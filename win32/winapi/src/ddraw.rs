use runtime::*;

use crate::{ABIReturn, stub};
use zerocopy::IntoBytes;

#[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
#[repr(C)]
struct DDraw_VTable {
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

pub fn ddraw_init(addr: u32) {
    unsafe {
        let vtable = DDraw_VTable::default();
        vtable
            .write_to_prefix(&mut MACHINE.memory.bytes[addr as usize..])
            .unwrap();
    }
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
        Some(IID_IDirectDraw7) => stub!(0),
        _ => panic!(),
    };

    unsafe {
        MACHINE.memory.write(lplpDD, ddraw);
    }
    DD::OK
}
