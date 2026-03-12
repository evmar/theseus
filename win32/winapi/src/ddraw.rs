use runtime::*;

use crate::{ABIReturn, stub};
use zerocopy::IntoBytes;

pub mod IDirectDraw7 {
    use super::*;

    #[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
    #[repr(C)]
    pub struct VTable {
        QueryInterface: u32,
        AddRef: u32,
        Release: u32,
        Compact: u32,
        CreateClipper: u32,
        CreatePalette: u32,
        CreateSurface: u32,
        DuplicateSurface: u32,
        EnumDisplayModes: u32,
        EnumSurfaces: u32,
        FlipToGDISurface: u32,
        GetCaps: u32,
        GetDisplayMode: u32,
        GetFourCCCodes: u32,
        GetGDISurface: u32,
        GetMonitorFrequency: u32,
        GetScanLine: u32,
        GetVerticalBlankStatus: u32,
        Initialize: u32,
        RestoreDisplayMode: u32,
        SetCooperativeLevel: u32,
        SetDisplayMode: u32,
        WaitForVerticalBlank: u32,
        GetAvailableVidMem: u32,
        GetSurfaceFromDC: u32,
        RestoreAllSurfaces: u32,
        TestCooperativeLevel: u32,
        GetDeviceIdentifier: u32,
        StartModeTest: u32,
        EvaluateMode: u32,
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Compact() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumSurfaces() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetAvailableVidMem() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceFromDC() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreAllSurfaces() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn TestCooperativeLevel() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceIdentifier() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn StartModeTest() {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EvaluateMode() {
        todo!()
    }

    pub fn vtable(buf: &mut [u8]) {
        let addr = runtime::proc_addr(QueryInterface_stdcall);
        let vtable = VTable {
            QueryInterface: addr + 0,
            AddRef: addr + 1,
            Release: addr + 2,
            Compact: addr + 3,
            CreateClipper: addr + 4,
            CreatePalette: addr + 5,
            CreateSurface: addr + 6,
            DuplicateSurface: addr + 7,
            EnumDisplayModes: addr + 8,
            EnumSurfaces: addr + 9,
            FlipToGDISurface: addr + 10,
            GetCaps: addr + 11,
            GetDisplayMode: addr + 12,
            GetFourCCCodes: addr + 13,
            GetGDISurface: addr + 14,
            GetMonitorFrequency: addr + 15,
            GetScanLine: addr + 16,
            GetVerticalBlankStatus: addr + 17,
            Initialize: addr + 18,
            RestoreDisplayMode: addr + 19,
            SetCooperativeLevel: addr + 20,
            SetDisplayMode: addr + 21,
            WaitForVerticalBlank: addr + 22,
            GetAvailableVidMem: addr + 23,
            GetSurfaceFromDC: addr + 24,
            RestoreAllSurfaces: addr + 25,
            TestCooperativeLevel: addr + 26,
            GetDeviceIdentifier: addr + 27,
            StartModeTest: addr + 28,
            EvaluateMode: addr + 29,
        };
        vtable.write_to_prefix(buf).unwrap();
    }
}

pub const EXPORTS: [&'static str; 30] = [
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
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DD {
    OK = 0,
    E_NOINTERFACE = 0x80004002,
    ERR_GENERIC = 0x80004005,
}

impl Into<ABIReturn> for DD {
    fn into(self) -> ABIReturn {
        (self as u32).into()
    }
}

pub const IID_IDirectDraw7: GUID = GUID((
    0x15e65ec0,
    0x3b9c,
    0x11d2,
    [0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b],
));

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

    let ddraw: u32 = match iid {
        Some(IID_IDirectDraw7) => {
            IDirectDraw7::vtable(&mut []);
            stub!(0)
        }
        _ => panic!(),
    };

    unsafe {
        MACHINE.memory.write(lplpDD, ddraw);
    }
    DD::OK
}
