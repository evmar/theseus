use super::types::*;
use crate::{
    ddraw::{GUID, ddraw1, ddraw7, state},
    kernel32,
    user32::{self, HWND},
};
use runtime::*;
use std::{cell::RefCell, rc::Rc};

pub struct DirectDraw {
    pub addr: u32,
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
}

impl DirectDraw {
    pub fn create_surface(
        &mut self,
        desc: &DDSURFACEDESC2,
        new_pointer: &mut dyn FnMut() -> u32,
    ) -> Rc<RefCell<Surface>> {
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

        let surface = self.create_one_surface(
            new_pointer(),
            &SurfaceParams {
                is_primary: desc.dwFlags.contains(DDSD::CAPS)
                    && desc.ddsCaps.dwCaps.contains(DDSCAPS::PRIMARYSURFACE),
                width,
                height,
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
            log::info!("primary {addr:x}");
            Target::Window(window.clone())
        } else {
            log::info!("back {addr:x}");
            let texture_creator = window.borrow().canvas.texture_creator();
            let mut texture = texture_creator
                .create_texture_target(None, params.width, params.height)
                .unwrap();
            // FML, this means BGRA in memory order
            assert_eq!(texture.format(), sdl3::pixels::PixelFormat::ARGB8888);
            let mut pixels = Vec::new();
            pixels.resize((params.width * params.height) as usize, 0xff000000u32);
            use zerocopy::IntoBytes;
            texture
                .update(None, pixels.as_bytes(), params.width as usize * 4)
                .unwrap();
            Target::Texture(texture)
        };

        let surf = Rc::new(RefCell::new(Surface {
            addr,
            width: params.width,
            height: params.height,
            target,
            primary: Default::default(),
            attached: Default::default(),
            pixels: None,
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
    pub target: Target,

    // How does surface attachment actually work?
    // Docs are unclear, and wine's comments are also full of speculation and frustration, ha.
    /// Present on surfaces attached to Target::Window
    pub primary: Option<Rc<RefCell<Surface>>>,
    /// Present on Target::Window, TODO should be vec
    pub attached: Option<Rc<RefCell<Surface>>>,

    /// Address of pixel data, when locked.
    pub pixels: Option<u32>,
}

impl Surface {
    pub fn lock(&mut self) -> u32 {
        assert_eq!(self.pixels, None);
        let size = self.width * self.height * 4;
        let pixels = kernel32::state()
            .process_heap
            .borrow()
            .alloc(unsafe { &mut MACHINE.memory }, size);
        // scribble on pixels so we can see it
        unsafe {
            MACHINE.memory.slice_mut(pixels..pixels + size).fill(0x8F);
        }
        self.pixels = Some(pixels);
        pixels
    }

    pub fn unlock(&mut self) {
        let pixels = self.pixels.unwrap();
        let pixel_data = unsafe {
            MACHINE
                .memory
                .slice(pixels..pixels + (self.width * self.height * 4))
        };
        match &mut self.target {
            Target::Window(_) => unreachable!(),
            Target::Texture(texture) => {
                texture
                    .update(None, pixel_data, self.width as usize * 4)
                    .unwrap();
            }
        }

        kernel32::state()
            .process_heap
            .borrow()
            .free(unsafe { &mut MACHINE.memory }, self.pixels.unwrap());
        self.pixels = None;
    }
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
pub fn DirectDrawCreate(lpGUID: u32, lplpDD: u32, pUnkOuter: u32) -> DD {
    DirectDrawCreateEx(lpGUID, lplpDD, 0, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(lpGuid: u32, lplpDD: u32, iid: u32, _pUnkOuter: u32) -> DD {
    assert!(lpGuid == 0);
    let iid = if iid == 0 {
        None
    } else {
        Some(unsafe { MACHINE.memory.read::<GUID>(iid) })
    };

    let addr: u32 = match iid {
        None => ddraw1::IDirectDraw::new(),
        Some(ddraw7::IID_IDirectDraw7) => ddraw7::IDirectDraw7::new(),
        _ => panic!(),
    };

    let mut ddraw = state().ddraw.borrow_mut();
    assert!(ddraw.is_none());
    *ddraw = Some(DirectDraw { addr, window: None });

    unsafe {
        MACHINE.memory.write(lplpDD, addr);
    }
    DD::OK
}
