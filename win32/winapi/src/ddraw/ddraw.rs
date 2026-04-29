use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use runtime::*;

use super::types::*;
use crate::{
    ddraw::{GUID, ddraw1, ddraw7, state},
    kernel32,
    user32::{self, HWND},
};

pub struct DirectDraw {
    pub addr: u32,
    pub bytes_per_pixel: u32,
    pub window: Option<Rc<RefCell<user32::Window>>>,
}

impl DirectDraw {
    pub fn set_cooperative_level(&mut self, _hwnd: HWND, _flags: u32) {
        let window = user32::state().window.borrow().as_ref().unwrap().clone();
        self.window = Some(window);
    }
}

struct SurfaceParams {
    is_primary: bool,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

impl DirectDraw {
    pub fn create_surface(
        &mut self,
        desc: &DDSURFACEDESC2,
        new_pointer: &mut dyn FnMut() -> u32,
    ) -> Rc<RefCell<Surface>> {
        let is_primary = desc.dwFlags.contains(DDSD::CAPS)
            && desc.ddsCaps.dwCaps.contains(DDSCAPS::PRIMARYSURFACE);

        let window = self.window.as_ref().unwrap().borrow();
        let width = if desc.dwFlags.contains(DDSD::WIDTH) {
            desc.dwWidth
        } else {
            window.width
        };
        let height = if desc.dwFlags.contains(DDSD::HEIGHT) {
            desc.dwHeight
        } else {
            window.height
        };
        drop(window);

        let bytes_per_pixel = if is_primary {
            self.bytes_per_pixel
        } else {
            log::warn!("creating surface assuming 32bpp");
            4
        };

        let surface = self.create_one_surface(
            new_pointer(),
            &SurfaceParams {
                is_primary,
                width,
                height,
                bytes_per_pixel,
            },
        );

        if let Some(count) = desc.back_buffer_count() {
            assert_eq!(count, 1);
            let back = self.create_one_surface(
                new_pointer(),
                &SurfaceParams {
                    is_primary: false,
                    width,
                    height,
                    bytes_per_pixel,
                },
            );
            back.borrow_mut().primary.replace(surface.clone());
            surface.borrow_mut().attached.replace(back);
        }

        surface
    }

    fn create_one_surface(&mut self, addr: u32, params: &SurfaceParams) -> Rc<RefCell<Surface>> {
        let window = self.window.as_ref().unwrap();
        let target = if params.is_primary {
            Target::Window(window.clone())
        } else {
            let texture_creator = window.borrow().canvas.texture_creator();
            let mut texture = texture_creator
                .create_texture_target(None, params.width, params.height)
                .unwrap();
            texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
            // FML, this means BGRA in memory order
            assert_eq!(texture.format(), sdl3::pixels::PixelFormat::ARGB8888);
            Target::Texture(texture)
        };

        let surf = Rc::new(RefCell::new(Surface {
            addr,
            width: params.width,
            height: params.height,
            bytes_per_pixel: params.bytes_per_pixel,
            target,
            primary: Default::default(),
            attached: Default::default(),
            pixels: None,
            palette: None,
        }));
        // TODO: move surf to ddraw
        state().surf.borrow_mut().insert(addr, surf.clone());
        surf
    }
}

pub enum Target {
    Window(Rc<RefCell<user32::Window>>),
    Texture(sdl3::render::Texture),
}

pub struct Surface {
    pub addr: u32,
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub target: Target,

    // How does surface attachment actually work?
    // Docs are unclear, and wine's comments are also full of speculation and frustration, ha.
    /// Present on surfaces attached to Target::Window
    pub primary: Option<Rc<RefCell<Surface>>>,
    /// Present on Target::Window, TODO should be vec
    pub attached: Option<Rc<RefCell<Surface>>>,

    /// Address of pixel data.
    pub pixels: Option<u32>,

    pub palette: Option<Rc<RefCell<Palette>>>,
}

impl Surface {
    pub fn lock(&mut self, mem: &mut Memory) -> u32 {
        match self.pixels {
            Some(addr) => addr,
            None => {
                let size = self.width * self.height * self.bytes_per_pixel;
                let addr = kernel32::lock().process_heap.alloc(mem, size);
                // scribble on pixels so we can see it
                mem[addr..][..size as usize].fill(0x8F);
                self.pixels = Some(addr);
                log::error!("surf {:x} pixels {:x}", self.addr, addr);
                addr
            }
        }
    }

    pub fn unlock(&mut self, mem: &mut Memory) {
        self.update_texture(mem, &None);
    }

    // App can write pixels to back buffer but attach palette to front buffer,
    // so take palette as an argument.
    fn update_texture(&mut self, mem: &mut Memory, palette: &Option<Rc<RefCell<Palette>>>) {
        let Some(addr) = self.pixels else {
            return;
        };
        let size = self.width * self.height * self.bytes_per_pixel;
        let pixels = &mem[addr..][..size as usize];
        let mut buf = vec![];
        let pixels32 = match self.bytes_per_pixel {
            1 => {
                let Some(palette) = palette.as_ref() else {
                    // e.g. unlock on a back buffer with no palette attached
                    return;
                };
                let palette = palette.borrow();
                let entries = &palette.entries;
                for &p in pixels {
                    let entry = &entries[p as usize];
                    buf.push(entry.peBlue);
                    buf.push(entry.peGreen);
                    buf.push(entry.peRed);
                    buf.push(0);
                }
                buf.as_slice()
            }
            4 => pixels,
            _ => todo!(),
        };
        match &mut self.target {
            Target::Window(_) => unreachable!(),
            Target::Texture(texture) => {
                texture
                    .update(None, pixels32, self.width as usize * 4)
                    .unwrap();
            }
        }
    }

    pub fn flip(&mut self, mem: &mut Memory) {
        // "Flip can be called only for a surface that has the DDSCAPS_FLIP and DDSCAPS_FRONTBUFFER capabilities."
        let Target::Window(window) = &self.target else {
            unreachable!()
        };

        let mut back = self.attached.as_ref().unwrap().borrow_mut();
        if self.palette.is_some() {
            back.update_texture(mem, &self.palette);
        }
        let Target::Texture(texture) = &mut back.target else {
            unreachable!()
        };

        // Ignore any alpha in the input when doing the final render copy.
        texture.set_blend_mode(sdl3::render::BlendMode::None);

        let mut canvas = RefMut::map(window.borrow_mut(), |w| &mut w.canvas);
        // For debugging, can verify that the flip covers the entire canvas by starting with red:
        // canvas.set_draw_color(sdl3::pixels::Color::RED);
        // canvas.clear();
        canvas.copy(texture, None, None).unwrap();
        canvas.present();
    }
}

pub struct Palette {
    pub entries: Vec<PALETTEENTRY>,
}

pub fn get_pixel_format() -> DDPIXELFORMAT {
    DDPIXELFORMAT {
        dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
        dwFlags: 0x00000040,
        dwFourCC: 0,
        dwRGBBitCount: 32,
        dwRBitMask: 0x0000_00FF,
        dwGBitMask: 0x0000_FF00,
        dwBBitMask: 0x00FF_0000,
        dwRGBAlphaBitMask: 0xFF00_0000,
    }
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(ctx: &mut Context, lpGUID: u32, lplpDD: u32, pUnkOuter: u32) -> DD {
    DirectDrawCreateEx(ctx, lpGUID, lplpDD, 0, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    ctx: &mut Context,
    lpGuid: u32,
    lplpDD: u32,
    iid: u32,
    _pUnkOuter: u32,
) -> DD {
    assert!(lpGuid == 0);
    let iid = if iid == 0 {
        None
    } else {
        Some(ctx.memory.read::<GUID>(iid))
    };

    let mut kernel32 = kernel32::lock();
    let addr: u32 = match iid {
        None => ddraw1::IDirectDraw::new(ctx, &mut kernel32.process_heap),
        Some(ddraw7::IID_IDirectDraw7) => {
            ddraw7::IDirectDraw7::new(ctx, &mut kernel32.process_heap)
        }
        _ => panic!(),
    };

    let mut ddraw = state().ddraw.borrow_mut();
    assert!(ddraw.is_none());
    *ddraw = Some(DirectDraw {
        addr,
        bytes_per_pixel: 4,
        window: None,
    });

    ctx.memory.write(lplpDD, addr);
    DD::OK
}
