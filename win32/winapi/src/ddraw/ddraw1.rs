use runtime::MACHINE;
use runtime::Machine;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    ddraw::{DD, get_pixel_format, state, types::*},
    kernel32, stub,
    user32::HWND,
};

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
    pub fn QueryInterface(_m: &mut Machine, _this: u32, _riid: u32, _ppvObject: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_m: &mut Machine, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_m: &mut Machine, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Compact(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }
    #[win32_derive::dllexport]
    pub fn CreatePalette(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        m: &mut Machine,
        this: u32,
        desc: u32,
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> DD {
        let mut ddraw = state().get_ddraw(this);
        let desc = <DDSURFACEDESC>::ref_from_prefix(m.memory.slice_from(desc))
            .unwrap()
            .0;
        let desc2 = DDSURFACEDESC2::from_desc(&desc);
        let surface = ddraw.create_surface(&desc2, &mut || IDirectDrawSurface::new());
        m.memory.write(lplpDDSurface, surface.borrow().addr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumSurfaces(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_m: &mut Machine, this: u32, hwnd: HWND, flags: u32) -> DD {
        state().get_ddraw(this).set_cooperative_level(hwnd, flags);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(_m: &mut Machine, this: u32, width: u32, height: u32, _bpp: u32) -> DD {
        let ddraw = state().get_ddraw(this);
        ddraw
            .window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .resize(width, height);
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(_m: &mut Machine, _this: u32) -> DD {
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

pub mod IDirectDrawSurface {
    use super::*;

    #[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
    #[repr(C)]
    pub struct VTable {
        QueryInterface: u32,
        AddRef: u32,
        Release: u32,
        AddAttachedSurface: u32,
        AddOverlayDirtyRect: u32,
        Blt: u32,
        BltBatch: u32,
        BltFast: u32,
        DeleteAttachedSurface: u32,
        EnumAttachedSurfaces: u32,
        EnumOverlayZOrders: u32,
        Flip: u32,
        GetAttachedSurface: u32,
        GetBltStatus: u32,
        GetCaps: u32,
        GetClipper: u32,
        GetColorKey: u32,
        GetDC: u32,
        GetFlipStatus: u32,
        GetOverlayPosition: u32,
        GetPalette: u32,
        GetPixelFormat: u32,
        GetSurfaceDesc: u32,
        Initialize: u32,
        IsLost: u32,
        Lock: u32,
        ReleaseDC: u32,
        Restore: u32,
        SetClipper: u32,
        SetColorKey: u32,
        SetOverlayPosition: u32,
        SetPalette: u32,
        Unlock: u32,
        UpdateOverlay: u32,
        UpdateOverlayDisplay: u32,
        UpdateOverlayZOrder: u32,
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_m: &mut Machine, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_m: &mut Machine, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddAttachedSurface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddOverlayDirtyRect(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Blt(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltBatch(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltFast(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DeleteAttachedSurface(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumAttachedSurfaces(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumOverlayZOrders(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Flip(_m: &mut Machine, this: u32, _lpDDSurfaceTargetOverride: u32, _dwFlags: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        surface.flip();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        m: &mut Machine,
        this: u32,
        _lpDDSCaps: u32,
        lplpDDAttachedSurface: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let surface = surfaces.get(&this).unwrap().borrow();
        m.memory.write(
            lplpDDAttachedSurface,
            surface.attached.as_ref().unwrap().borrow().addr,
        );
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetBltStatus(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetClipper(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetColorKey(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDC(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFlipStatus(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetOverlayPosition(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPalette(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(m: &mut Machine, _this: u32, lpDDPixelFormat: u32) -> DD {
        m.memory.write(lpDDPixelFormat, get_pixel_format());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn IsLost(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        m: &mut Machine,
        this: u32,
        rect: u32,
        lpDesc: u32,
        _flags: u32,
        _unused: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        assert_eq!(rect, 0);

        let pixels = surface.lock();
        let desc = DDSURFACEDESC {
            dwSize: std::mem::size_of::<DDSURFACEDESC>() as u32,
            lPitch_dwLinearSize: surface.width * 4,
            lpSurface: pixels,
            ..DDSURFACEDESC::default()
        };
        desc.write_to_prefix(m.memory.slice_mut_from(lpDesc))
            .unwrap();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Restore(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetOverlayPosition(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Unlock(_m: &mut Machine, this: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        surface.unlock();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlay(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayDisplay(_m: &mut Machine, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayZOrder(_m: &mut Machine, _this: u32) -> DD {
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
            AddAttachedSurface: func_addr + 3,
            AddOverlayDirtyRect: func_addr + 4,
            Blt: func_addr + 5,
            BltBatch: func_addr + 6,
            BltFast: func_addr + 7,
            DeleteAttachedSurface: func_addr + 8,
            EnumAttachedSurfaces: func_addr + 9,
            EnumOverlayZOrders: func_addr + 10,
            Flip: func_addr + 11,
            GetAttachedSurface: func_addr + 12,
            GetBltStatus: func_addr + 13,
            GetCaps: func_addr + 14,
            GetClipper: func_addr + 15,
            GetColorKey: func_addr + 16,
            GetDC: func_addr + 17,
            GetFlipStatus: func_addr + 18,
            GetOverlayPosition: func_addr + 19,
            GetPalette: func_addr + 20,
            GetPixelFormat: func_addr + 21,
            GetSurfaceDesc: func_addr + 22,
            Initialize: func_addr + 23,
            IsLost: func_addr + 24,
            Lock: func_addr + 25,
            ReleaseDC: func_addr + 26,
            Restore: func_addr + 27,
            SetClipper: func_addr + 28,
            SetColorKey: func_addr + 29,
            SetOverlayPosition: func_addr + 30,
            SetPalette: func_addr + 31,
            Unlock: func_addr + 32,
            UpdateOverlay: func_addr + 33,
            UpdateOverlayDisplay: func_addr + 34,
            UpdateOverlayZOrder: func_addr + 35,
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
