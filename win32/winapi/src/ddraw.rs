use runtime::*;

use crate::ABIReturn;
use zerocopy::IntoBytes;

pub mod IDirectDraw7 {
    use crate::{gdi32::HDC, kernel32, stub, user32::HWND};

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
    pub fn QueryInterface(_this: u32, _riid: u32, _ppv: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Compact(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper(_this: u32, _flags: u32, _lplpClipper: u32, _pUnkOuter: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette(
        _this: u32,
        _flags: u32,
        _lpColorTable: u32,
        _lplpPalette: u32,
        _pUnkOuter: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        _this: u32,
        _lpDDSurfaceDesc2: u32,
        _lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface(_this: u32, _lpDDSurface: u32, _lplpDupDDSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes(
        _this: u32,
        _flags: u32,
        _lpSurfaceDesc2: u32,
        _lpContext: u32,
        _lpEnumCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumSurfaces(
        _this: u32,
        _flags: u32,
        _lpSurfaceDesc2: u32,
        _lpContext: u32,
        _lpEnumCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_this: u32, _lpDDDriverCaps: u32, _lpDDEmulCaps: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(_this: u32, _lpDDSurfaceDesc2: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes(_this: u32, _lpNumCodes: u32, _lpCodes: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface(_this: u32, _lplpGDISurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency(_this: u32, _lpdwFrequency: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine(_this: u32, _lpdwScanLine: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus(_this: u32, _lpbIsInVB: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_this: u32, _lpGUID: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_this: u32, _hwnd: HWND, _flags: u32) -> DD {
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        _this: u32,
        _width: u32,
        _height: u32,
        _bpp: u32,
        _refresh: u32,
        _flags: u32,
    ) -> DD {
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(_this: u32, _flags: u32, _hEvent: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetAvailableVidMem(_this: u32, _lpDDSCaps2: u32, _lpdwTotal: u32, _lpdwFree: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceFromDC(_this: u32, _hdc: HDC, _lplpDDSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreAllSurfaces(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn TestCooperativeLevel(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceIdentifier(_this: u32, _lpDDDeviceIdentifier: u32, _flags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn StartModeTest(_this: u32, _lpModesToTest: u32, _numEntries: u32, _flags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EvaluateMode(_this: u32, _flags: u32, _pSecondsUntilTimeout: u32) -> DD {
        todo!()
    }

    fn vtable() -> u32 {
        let vtable_addr = kernel32::state().process_heap.borrow().alloc(
            unsafe { &mut MACHINE.memory },
            std::mem::size_of::<VTable>() as u32,
        );
        let func_addr = runtime::proc_addr(QueryInterface_stdcall);
        let vtable = VTable {
            QueryInterface: func_addr + 0,
            AddRef: func_addr + 1,
            Release: func_addr + 2,
            Compact: func_addr + 3,
            CreateClipper: func_addr + 4,
            CreatePalette: func_addr + 5,
            CreateSurface: func_addr + 6,
            DuplicateSurface: func_addr + 7,
            EnumDisplayModes: func_addr + 8,
            EnumSurfaces: func_addr + 9,
            FlipToGDISurface: func_addr + 10,
            GetCaps: func_addr + 11,
            GetDisplayMode: func_addr + 12,
            GetFourCCCodes: func_addr + 13,
            GetGDISurface: func_addr + 14,
            GetMonitorFrequency: func_addr + 15,
            GetScanLine: func_addr + 16,
            GetVerticalBlankStatus: func_addr + 17,
            Initialize: func_addr + 18,
            RestoreDisplayMode: func_addr + 19,
            SetCooperativeLevel: func_addr + 20,
            SetDisplayMode: func_addr + 21,
            WaitForVerticalBlank: func_addr + 22,
            GetAvailableVidMem: func_addr + 23,
            GetSurfaceFromDC: func_addr + 24,
            RestoreAllSurfaces: func_addr + 25,
            TestCooperativeLevel: func_addr + 26,
            GetDeviceIdentifier: func_addr + 27,
            StartModeTest: func_addr + 28,
            EvaluateMode: func_addr + 29,
        };
        vtable
            .write_to_prefix(unsafe { &mut MACHINE.memory.bytes[vtable_addr as usize..] })
            .unwrap();
        vtable_addr
    }

    pub fn new() -> u32 {
        let addr = kernel32::state()
            .process_heap
            .borrow()
            .alloc(unsafe { &mut MACHINE.memory }, 4);
        unsafe {
            MACHINE.memory.write(addr, vtable());
        }
        addr
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
        Some(IID_IDirectDraw7) => IDirectDraw7::new(),
        _ => panic!(),
    };

    unsafe {
        MACHINE.memory.write(lplpDD, ddraw);
    }
    DD::OK
}
