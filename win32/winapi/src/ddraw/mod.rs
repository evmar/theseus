use std::{
    cell::{OnceCell, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use runtime::*;

mod ddraw1;
mod ddraw7;
mod types;

pub use ddraw1::*;
pub use ddraw7::*;
pub use types::*;

use crate::{
    kernel32,
    user32::{self, HWND},
};

pub const EXPORTS: [&'static str; 102] = [
    // IDirectDraw
    "IDirectDraw::QueryInterface",
    "IDirectDraw::AddRef",
    "IDirectDraw::Release",
    "IDirectDraw::Compact",
    "IDirectDraw::CreateClipper",
    "IDirectDraw::CreatePalette",
    "IDirectDraw::CreateSurface",
    "IDirectDraw::DuplicateSurface",
    "IDirectDraw::EnumDisplayModes",
    "IDirectDraw::EnumSurfaces",
    "IDirectDraw::FlipToGDISurface",
    "IDirectDraw::GetCaps",
    "IDirectDraw::GetDisplayMode",
    "IDirectDraw::GetFourCCCodes",
    "IDirectDraw::GetGDISurface",
    "IDirectDraw::GetMonitorFrequency",
    "IDirectDraw::GetScanLine",
    "IDirectDraw::GetVerticalBlankStatus",
    "IDirectDraw::Initialize",
    "IDirectDraw::RestoreDisplayMode",
    "IDirectDraw::SetCooperativeLevel",
    "IDirectDraw::SetDisplayMode",
    "IDirectDraw::WaitForVerticalBlank",
    // IDirectDraw7
    "IDirectDraw7::QueryInterface",
    "IDirectDraw7::AddRef",
    "IDirectDraw7::Release",
    "IDirectDraw7::Compact",
    "IDirectDraw7::CreateClipper",
    "IDirectDraw7::CreatePalette",
    "IDirectDraw7::CreateSurface",
    "IDirectDraw7::DuplicateSurface",
    "IDirectDraw7::EnumDisplayModes",
    "IDirectDraw7::EnumSurfaces",
    "IDirectDraw7::FlipToGDISurface",
    "IDirectDraw7::GetCaps",
    "IDirectDraw7::GetDisplayMode",
    "IDirectDraw7::GetFourCCCodes",
    "IDirectDraw7::GetGDISurface",
    "IDirectDraw7::GetMonitorFrequency",
    "IDirectDraw7::GetScanLine",
    "IDirectDraw7::GetVerticalBlankStatus",
    "IDirectDraw7::Initialize",
    "IDirectDraw7::RestoreDisplayMode",
    "IDirectDraw7::SetCooperativeLevel",
    "IDirectDraw7::SetDisplayMode",
    "IDirectDraw7::WaitForVerticalBlank",
    "IDirectDraw7::GetAvailableVidMem",
    "IDirectDraw7::GetSurfaceFromDC",
    "IDirectDraw7::RestoreAllSurfaces",
    "IDirectDraw7::TestCooperativeLevel",
    "IDirectDraw7::GetDeviceIdentifier",
    "IDirectDraw7::StartModeTest",
    "IDirectDraw7::EvaluateMode",
    "IDirectDrawSurface7::QueryInterface",
    "IDirectDrawSurface7::AddRef",
    "IDirectDrawSurface7::Release",
    "IDirectDrawSurface7::AddAttachedSurface",
    "IDirectDrawSurface7::AddOverlayDirtyRect",
    "IDirectDrawSurface7::Blt",
    "IDirectDrawSurface7::BltBatch",
    "IDirectDrawSurface7::BltFast",
    "IDirectDrawSurface7::DeleteAttachedSurface",
    "IDirectDrawSurface7::EnumAttachedSurfaces",
    "IDirectDrawSurface7::EnumOverlayZOrders",
    "IDirectDrawSurface7::Flip",
    "IDirectDrawSurface7::GetAttachedSurface",
    "IDirectDrawSurface7::GetBltStatus",
    "IDirectDrawSurface7::GetCaps",
    "IDirectDrawSurface7::GetClipper",
    "IDirectDrawSurface7::GetColorKey",
    "IDirectDrawSurface7::GetDC",
    "IDirectDrawSurface7::GetFlipStatus",
    "IDirectDrawSurface7::GetOverlayPosition",
    "IDirectDrawSurface7::GetPalette",
    "IDirectDrawSurface7::GetPixelFormat",
    "IDirectDrawSurface7::GetSurfaceDesc",
    "IDirectDrawSurface7::Initialize",
    "IDirectDrawSurface7::IsLost",
    "IDirectDrawSurface7::Lock",
    "IDirectDrawSurface7::ReleaseDC",
    "IDirectDrawSurface7::Restore",
    "IDirectDrawSurface7::SetClipper",
    "IDirectDrawSurface7::SetColorKey",
    "IDirectDrawSurface7::SetOverlayPosition",
    "IDirectDrawSurface7::SetPalette",
    "IDirectDrawSurface7::Unlock",
    "IDirectDrawSurface7::UpdateOverlay",
    "IDirectDrawSurface7::UpdateOverlayDisplay",
    "IDirectDrawSurface7::UpdateOverlayZOrder",
    "IDirectDrawSurface7::GetDDInterface",
    "IDirectDrawSurface7::PageLock",
    "IDirectDrawSurface7::PageUnlock",
    "IDirectDrawSurface7::SetSurfaceDesc",
    "IDirectDrawSurface7::SetPrivateData",
    "IDirectDrawSurface7::GetPrivateData",
    "IDirectDrawSurface7::FreePrivateData",
    "IDirectDrawSurface7::GetUniquenessValue",
    "IDirectDrawSurface7::ChangeUniquenessValue",
    "IDirectDrawSurface7::SetPriority",
    "IDirectDrawSurface7::GetPriority",
    "IDirectDrawSurface7::SetLOD",
    "IDirectDrawSurface7::GetLOD",
];

#[repr(C)]
#[derive(PartialEq, zerocopy::FromBytes)]
pub struct GUID(pub (u32, u16, u16, [u8; 8]));

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-",
            self.0.0,
            self.0.1,
            self.0.2,
            u16::from_le_bytes(self.0.3[..2].try_into().unwrap())
        )?;
        for b in &self.0.3[2..] {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
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

#[derive(Default)]
pub struct State {
    ddraw: RefCell<Option<DirectDraw>>,
    surf: RefCell<HashMap<u32, Rc<RefCell<Surface>>>>,
}

impl State {
    pub fn get_ddraw(&self, ptr: u32) -> RefMut<'_, DirectDraw> {
        let ddraw = RefMut::map(self.ddraw.borrow_mut(), |ddraw| ddraw.as_mut().unwrap());
        assert!(ptr == ddraw.addr);
        ddraw
    }
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| Default::default())
}

pub struct DirectDraw {
    addr: u32,
    window: Option<Rc<user32::Window>>,
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
        let window = self.window.as_ref().unwrap();
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
            let texture_creator = window.canvas.borrow_mut().texture_creator();
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

enum Target {
    Window(Rc<user32::Window>),
    Texture(sdl3::render::Texture),
}

pub struct Surface {
    addr: u32,
    width: u32,
    height: u32,
    target: Target,

    // How does surface attachment actually work?
    // Docs are unclear, and wine's comments are also full of speculation and frustration, ha.
    /// Present on surfaces attached to Target::Window
    primary: Option<Rc<RefCell<Surface>>>,
    /// Present on Target::Window, TODO should be vec
    attached: Option<Rc<RefCell<Surface>>>,

    /// Address of pixel data, when locked.
    pixels: Option<u32>,
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
