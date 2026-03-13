use crate::ddraw::GUID;
use crate::{ddraw::DD, gdi32::HDC, kernel32, stub, user32::HWND};
use runtime::*;
use zerocopy::IntoBytes;

pub const IID_IDirectDraw7: GUID = GUID((
    0x15e65ec0,
    0x3b9c,
    0x11d2,
    [0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b],
));

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
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> DD {
        unsafe {
            MACHINE
                .memory
                .write(lplpDDSurface, IDirectDrawSurface7::new())
        };
        stub!(DD::OK)
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

pub mod IDirectDrawSurface7 {
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
        GetDDInterface: u32,
        PageLock: u32,
        PageUnlock: u32,
        SetSurfaceDesc: u32,
        SetPrivateData: u32,
        GetPrivateData: u32,
        FreePrivateData: u32,
        GetUniquenessValue: u32,
        ChangeUniquenessValue: u32,
        SetPriority: u32,
        GetPriority: u32,
        SetLOD: u32,
        GetLOD: u32,
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
    pub fn AddAttachedSurface(_this: u32, _lpDDSAttachedSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddOverlayDirtyRect(_this: u32, _lpRect: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Blt(
        _this: u32,
        _lpDestRect: u32,
        _lpDDSrcSurface: u32,
        _lpSrcRect: u32,
        _dwFlags: u32,
        _lpDDBltFx: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltBatch(_this: u32, _lpDDBltBatch: u32, _dwCount: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltFast(
        _this: u32,
        _dwX: u32,
        _dwY: u32,
        _lpDDSrcSurface: u32,
        _lpSrcRect: u32,
        _dwTrans: u32,
    ) -> DD {
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn DeleteAttachedSurface(_this: u32, _dwFlags: u32, _lpDDSAttachedSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumAttachedSurfaces(_this: u32, _lpContext: u32, _lpEnumSurfacesCallback: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumOverlayZOrders(
        _this: u32,
        _dwFlags: u32,
        _lpContext: u32,
        _lpfnCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Flip(_this: u32, _lpDDSurfaceTargetOverride: u32, _dwFlags: u32) -> DD {
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(_this: u32, _lpDDSCaps: u32, lplpDDAttachedSurface: u32) -> DD {
        unsafe {
            MACHINE.memory.write(lplpDDAttachedSurface, new());
        }
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn GetBltStatus(_this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_this: u32, _lpDDSCaps: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetClipper(_this: u32, _lplpDDClipper: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetColorKey(_this: u32, _dwFlags: u32, _lpDDColorKey: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDC(_this: u32, _lphDC: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFlipStatus(_this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetOverlayPosition(_this: u32, _lplX: u32, _lplY: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPalette(_this: u32, _lplpDDPalette: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(_this: u32, _lpDDPixelFormat: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(_this: u32, _lpDDSurfaceDesc2: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_this: u32, _lpDD: u32, _lpDDSurfaceDesc: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn IsLost(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        _this: u32,
        _lpDestRect: u32,
        _lpDDSurfaceDesc2: u32,
        _dwFlags: u32,
        _hEvent: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(_this: u32, _hDC: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Restore(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(_this: u32, _lpDDClipper: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(_this: u32, _dwFlags: u32, _lpDDColorKey: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetOverlayPosition(_this: u32, _lX: i32, _lY: i32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(_this: u32, _lpDDPalette: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Unlock(_this: u32, _lpSurfaceData: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlay(
        _this: u32,
        _lpSrcRect: u32,
        _lpDDDestSurface: u32,
        _lpDestRect: u32,
        _dwFlags: u32,
        _lpDDOverlayFx: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayDisplay(_this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayZOrder(_this: u32, _dwFlags: u32, _lpDDSurfaceReference: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDDInterface(_this: u32, _lplpDD: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn PageLock(_this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn PageUnlock(_this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetSurfaceDesc(_this: u32, _lpDDSurfaceDesc2: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPrivateData(
        _this: u32,
        _guidTag: u32,
        _lpData: u32,
        _cbSize: u32,
        _dwFlags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPrivateData(_this: u32, _guidTag: u32, _lpBuffer: u32, _lpcbBufferSize: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FreePrivateData(_this: u32, _guidTag: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetUniquenessValue(_this: u32, _lpValue: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn ChangeUniquenessValue(_this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPriority(_this: u32, _dwPriority: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPriority(_this: u32, _lpdwPriority: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetLOD(_this: u32, _dwMaxLOD: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetLOD(_this: u32, _lpdwMaxLOD: u32) -> DD {
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
            GetDDInterface: func_addr + 36,
            PageLock: func_addr + 37,
            PageUnlock: func_addr + 38,
            SetSurfaceDesc: func_addr + 39,
            SetPrivateData: func_addr + 40,
            GetPrivateData: func_addr + 41,
            FreePrivateData: func_addr + 42,
            GetUniquenessValue: func_addr + 43,
            ChangeUniquenessValue: func_addr + 44,
            SetPriority: func_addr + 45,
            GetPriority: func_addr + 46,
            SetLOD: func_addr + 47,
            GetLOD: func_addr + 48,
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
