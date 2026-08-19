use std::{cell::RefCell, rc::Rc};

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

        // An offscreen surface takes the display mode's format unless the app
        // asks for a specific one, which is what lets a palettized game blit
        // between its buffers without conversion.
        let bytes_per_pixel = if desc.dwFlags.contains(DDSD::PIXELFORMAT) {
            let bits = desc.ddpfPixelFormat.dwRGBBitCount;
            if bits == 0 {
                self.bytes_per_pixel
            } else {
                bits.div_ceil(8)
            }
        } else {
            self.bytes_per_pixel
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

        if desc.dwFlags.contains(DDSD::CKSRCBLT) {
            surface.borrow_mut().src_color_key = Some(ColorKey {
                low: desc.ddckCKSrcBlt.dwColorSpaceLowValue,
                high: desc.ddckCKSrcBlt.dwColorSpaceHighValue,
            });
        }
        if desc.dwFlags.contains(DDSD::CKDESTBLT) {
            surface.borrow_mut().dst_color_key = Some(ColorKey {
                low: desc.ddckCKDestBlt.dwColorSpaceLowValue,
                high: desc.ddckCKDestBlt.dwColorSpaceHighValue,
            });
        }

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
            let texture = window
                .borrow_mut()
                .host
                .create_surface(params.width, params.height);
            Target::Texture(texture)
        };

        let surf = Rc::new(RefCell::new(Surface {
            addr,
            refs: 1,
            width: params.width,
            height: params.height,
            bytes_per_pixel: params.bytes_per_pixel,
            target,
            primary: Default::default(),
            attached: Default::default(),
            pixels: None,
            palette: None,
            src_color_key: None,
            dst_color_key: None,
        }));
        // TODO: move surf to ddraw
        state().surf.borrow_mut().insert(addr, surf.clone());
        surf
    }
}

pub enum Target {
    Window(Rc<RefCell<user32::Window>>),
    Texture(host::Surface),
}

/// A DDCOLORKEY: the inclusive range of pixel values a blit treats as
/// transparent.
#[derive(Copy, Clone, Debug)]
pub struct ColorKey {
    pub low: u32,
    pub high: u32,
}

impl ColorKey {
    pub fn matches(&self, pixel: u32) -> bool {
        (self.low..=self.high).contains(&pixel)
    }
}

pub struct Surface {
    pub addr: u32,
    /// COM reference count. An app that balances AddRef/Release expects the
    /// surface to outlive the matching Release, so this has to be real.
    pub refs: u32,
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

    /// Pixel values that read as transparent when this surface is the source
    /// of a blit — how sprites get their transparent background.
    pub src_color_key: Option<ColorKey>,
    /// Pixel values that may be overwritten when this surface is the
    /// destination of a blit.
    pub dst_color_key: Option<ColorKey>,
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
                addr
            }
        }
    }

    pub fn unlock(&mut self, mem: &mut Memory) {
        match self.target {
            // Writes to the primary surface go straight to the screen.
            Target::Window(_) => self.present(mem),
            Target::Texture(_) => self.update_texture(mem, &None),
        }
    }

    /// Convert this surface's pixels to the RGBA the host wants, using
    /// `palette` for palettized formats. Borrows the pixels directly when they
    /// are already RGBA. Returns None when there is nothing to show, e.g. an
    /// 8-bit surface with no palette attached yet.
    fn to_rgba<'a>(
        &self,
        mem: &'a Memory,
        palette: &Option<Rc<RefCell<Palette>>>,
    ) -> Option<std::borrow::Cow<'a, [u8]>> {
        let addr = self.pixels?;
        let size = self.width * self.height * self.bytes_per_pixel;
        let pixels = &mem[addr..][..size as usize];
        Some(match self.bytes_per_pixel {
            1 => {
                let palette = palette.as_ref()?;
                let entries = &palette.borrow().entries;
                let mut buf = Vec::with_capacity(pixels.len() * 4);
                for &p in pixels {
                    let entry = &entries[p as usize];
                    // ABGR8888 layout: R,G,B,A in byte order.
                    buf.push(entry.peRed);
                    buf.push(entry.peGreen);
                    buf.push(entry.peBlue);
                    buf.push(0);
                }
                buf.into()
            }
            2 => {
                // RGB565, the standard 16-bit display format.
                let mut buf = Vec::with_capacity(pixels.len() * 2);
                for pixel in pixels.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([pixel[0], pixel[1]]);
                    let (r, g, b) = (pixel >> 11, (pixel >> 5) & 0x3f, pixel & 0x1f);
                    // Replicate the high bits into the low ones so full-scale
                    // values stay full-scale.
                    buf.push((r << 3 | r >> 2) as u8);
                    buf.push((g << 2 | g >> 4) as u8);
                    buf.push((b << 3 | b >> 2) as u8);
                    buf.push(0);
                }
                buf.into()
            }
            4 => pixels.into(),
            bpp => {
                log::warn!("unsupported surface format: {bpp} bytes per pixel");
                return None;
            }
        })
    }

    // App can write pixels to back buffer but attach palette to front buffer,
    // so take palette as an argument.
    fn update_texture(&mut self, mem: &mut Memory, palette: &Option<Rc<RefCell<Palette>>>) {
        let Some(pixels) = self.to_rgba(mem, palette) else {
            return;
        };
        let width = self.width;
        match &mut self.target {
            Target::Window(_) => unreachable!(),
            Target::Texture(texture) => {
                texture.set_pixels(&pixels, width * 4);
            }
        }
    }

    /// Present this surface's own pixel buffer to the window it targets, used
    /// when an app draws directly to the primary surface (via Lock or Blt)
    /// instead of flipping. No-op for non-primary surfaces.
    pub fn present(&mut self, mem: &mut Memory) {
        let Target::Window(window) = &self.target else {
            return;
        };
        // We have no texture of our own; borrow the back buffer's.
        let Some(back) = self.attached.clone() else {
            return;
        };
        let Some(pixels) = self.to_rgba(mem, &self.palette) else {
            return;
        };
        let mut back = back.borrow_mut();
        let Target::Texture(texture) = &mut back.target else {
            return;
        };
        texture.set_pixels(&pixels, self.width * 4);
        window.borrow_mut().host.render(texture);
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

        let mut window = window.borrow_mut();
        window.host.render(texture);
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
