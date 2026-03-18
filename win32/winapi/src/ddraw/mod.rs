use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use runtime::*;

mod ddraw7;
mod types;

pub use ddraw7::*;
pub use types::*;

use crate::user32;

pub const EXPORTS: [&'static str; 79] = [
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
pub fn DirectDrawCreateEx(lpGuid: u32, lplpDD: u32, iid: u32, _pUnkOuter: u32) -> DD {
    assert!(lpGuid == 0);
    let iid = if iid == 0 {
        None
    } else {
        Some(unsafe { MACHINE.memory.read::<GUID>(iid) })
    };

    let addr: u32 = match iid {
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

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| Default::default())
}

struct DirectDraw {
    addr: u32,
    window: Option<Rc<user32::Window>>,
}

struct SurfaceParams {
    is_primary: bool,
    width: u32,
    height: u32,
}

impl DirectDraw {
    fn create_surface(&mut self, addr: u32, params: &SurfaceParams) -> Rc<RefCell<Surface>> {
        let window = self.window.as_ref().unwrap();
        let target = if params.is_primary {
            log::info!("primary {addr:x}");
            Target::Window(window.clone())
        } else {
            log::info!("back {addr:x}");
            let texture_creator = window.canvas.borrow_mut().texture_creator();
            assert_eq!(
                texture_creator.default_pixel_format(),
                sdl3::pixels::PixelFormat::ARGB8888
            );
            let mut texture = texture_creator
                .create_texture_target(None, params.width, params.height)
                .unwrap();
            let mut pixels = Vec::new();
            pixels.resize((params.width * params.height * 4) as usize, 0xff);
            texture
                .update(None, &pixels, params.width as usize * 4)
                .unwrap();
            Target::Texture(texture)
        };

        let surf = Rc::new(RefCell::new(Surface {
            addr,
            target,
            primary: Default::default(),
            attached: Default::default(),
        }));
        state().surf.borrow_mut().insert(addr, surf.clone());
        surf
    }
}

enum Target {
    Window(Rc<user32::Window>),
    Texture(sdl3::render::Texture),
}

struct Surface {
    addr: u32,
    target: Target,

    // How does surface attachment actually work?
    // Docs are unclear, and wine's comments are also full of speculation and frustration, ha.
    /// Present on surfaces attached to Target::Window
    primary: Option<Rc<RefCell<Surface>>>,
    /// Present on Target::Window, TODO should be vec
    attached: Option<Rc<RefCell<Surface>>>,
}
