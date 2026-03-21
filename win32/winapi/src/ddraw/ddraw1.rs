use crate::ddraw::state;
use crate::user32::HWND;
use crate::{ddraw::DD, kernel32, stub};
use runtime::MACHINE;
use zerocopy::IntoBytes;

pub mod IDirectDraw {
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
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(_this: u32, _riid: u32, _ppvObject: u32) -> DD {
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
    pub fn CreateClipper(_this: u32) -> DD {
        todo!()
    }
    #[win32_derive::dllexport]
    pub fn CreatePalette(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(_this: u32, _desc: u32, _lplpDDSurface: u32, _pUnkOuter: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumSurfaces(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(this: u32, hwnd: HWND, flags: u32) -> DD {
        state().get_ddraw(this).set_cooperative_level(hwnd, flags);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(_this: u32, _width: u32, _height: u32, _bpp: u32) -> DD {
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(_this: u32) -> DD {
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
