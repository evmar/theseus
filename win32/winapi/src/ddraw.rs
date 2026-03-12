use runtime::*;

use crate::{ABIReturn, stub};
use zerocopy::IntoBytes;

#[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
#[repr(C)]
struct IDirectDraw7_VTable {
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
pub fn IDirectDraw7_QueryInterface() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_AddRef() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_Release() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_Compact() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_CreateClipper() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_CreatePalette() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_CreateSurface() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_DuplicateSurface() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_EnumDisplayModes() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_EnumSurfaces() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_FlipToGDISurface() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetCaps() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetDisplayMode() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetFourCCCodes() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetGDISurface() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetMonitorFrequency() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetScanLine() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetVerticalBlankStatus() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_Initialize() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_RestoreDisplayMode() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_SetCooperativeLevel() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_SetDisplayMode() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_WaitForVerticalBlank() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetAvailableVidMem() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetSurfaceFromDC() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_RestoreAllSurfaces() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_TestCooperativeLevel() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_GetDeviceIdentifier() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_StartModeTest() {
    todo!()
}

#[win32_derive::dllexport]
pub fn IDirectDraw7_EvaluateMode() {
    todo!()
}

pub const EXPORTS: [&'static str; 30] = [
    "IDirectDraw7_QueryInterface",
    "IDirectDraw7_AddRef",
    "IDirectDraw7_Release",
    "IDirectDraw7_Compact",
    "IDirectDraw7_CreateClipper",
    "IDirectDraw7_CreatePalette",
    "IDirectDraw7_CreateSurface",
    "IDirectDraw7_DuplicateSurface",
    "IDirectDraw7_EnumDisplayModes",
    "IDirectDraw7_EnumSurfaces",
    "IDirectDraw7_FlipToGDISurface",
    "IDirectDraw7_GetCaps",
    "IDirectDraw7_GetDisplayMode",
    "IDirectDraw7_GetFourCCCodes",
    "IDirectDraw7_GetGDISurface",
    "IDirectDraw7_GetMonitorFrequency",
    "IDirectDraw7_GetScanLine",
    "IDirectDraw7_GetVerticalBlankStatus",
    "IDirectDraw7_Initialize",
    "IDirectDraw7_RestoreDisplayMode",
    "IDirectDraw7_SetCooperativeLevel",
    "IDirectDraw7_SetDisplayMode",
    "IDirectDraw7_WaitForVerticalBlank",
    "IDirectDraw7_GetAvailableVidMem",
    "IDirectDraw7_GetSurfaceFromDC",
    "IDirectDraw7_RestoreAllSurfaces",
    "IDirectDraw7_TestCooperativeLevel",
    "IDirectDraw7_GetDeviceIdentifier",
    "IDirectDraw7_StartModeTest",
    "IDirectDraw7_EvaluateMode",
];

pub fn vtable(buf: &mut [u8]) {
    let addr = runtime::proc_addr(stdcall_IDirectDraw7_QueryInterface);
    let vtable = IDirectDraw7_VTable {
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
            vtable(&mut []);
            stub!(0)
        }
        _ => panic!(),
    };

    unsafe {
        MACHINE.memory.write(lplpDD, ddraw);
    }
    DD::OK
}
