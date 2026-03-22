use std::{
    cell::{OnceCell, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

mod ddraw;
mod ddraw1;
mod ddraw7;
pub mod types;

pub use ddraw::*;
pub use ddraw1::*;
pub use ddraw7::*;
pub use types::DD;

pub const EXPORTS: [&'static str; 138] = [
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
    // IDirectDrawSurface
    "IDirectDrawSurface::QueryInterface",
    "IDirectDrawSurface::AddRef",
    "IDirectDrawSurface::Release",
    "IDirectDrawSurface::AddAttachedSurface",
    "IDirectDrawSurface::AddOverlayDirtyRect",
    "IDirectDrawSurface::Blt",
    "IDirectDrawSurface::BltBatch",
    "IDirectDrawSurface::BltFast",
    "IDirectDrawSurface::DeleteAttachedSurface",
    "IDirectDrawSurface::EnumAttachedSurfaces",
    "IDirectDrawSurface::EnumOverlayZOrders",
    "IDirectDrawSurface::Flip",
    "IDirectDrawSurface::GetAttachedSurface",
    "IDirectDrawSurface::GetBltStatus",
    "IDirectDrawSurface::GetCaps",
    "IDirectDrawSurface::GetClipper",
    "IDirectDrawSurface::GetColorKey",
    "IDirectDrawSurface::GetDC",
    "IDirectDrawSurface::GetFlipStatus",
    "IDirectDrawSurface::GetOverlayPosition",
    "IDirectDrawSurface::GetPalette",
    "IDirectDrawSurface::GetPixelFormat",
    "IDirectDrawSurface::GetSurfaceDesc",
    "IDirectDrawSurface::Initialize",
    "IDirectDrawSurface::IsLost",
    "IDirectDrawSurface::Lock",
    "IDirectDrawSurface::ReleaseDC",
    "IDirectDrawSurface::Restore",
    "IDirectDrawSurface::SetClipper",
    "IDirectDrawSurface::SetColorKey",
    "IDirectDrawSurface::SetOverlayPosition",
    "IDirectDrawSurface::SetPalette",
    "IDirectDrawSurface::Unlock",
    "IDirectDrawSurface::UpdateOverlay",
    "IDirectDrawSurface::UpdateOverlayDisplay",
    "IDirectDrawSurface::UpdateOverlayZOrder",
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

#[derive(Default)]
pub struct State {
    pub ddraw: RefCell<Option<DirectDraw>>,
    pub surf: RefCell<HashMap<u32, Rc<RefCell<Surface>>>>,
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
