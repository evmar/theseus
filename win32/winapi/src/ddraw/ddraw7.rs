use crate::heap::Heap;
use std::cell::RefMut;

use runtime::*;
use zerocopy::{FromBytes as _, IntoBytes};

use crate::{
    RECT,
    ddraw::{GUID, Target, state, types::*},
    gdi32,
    gdi32::{DIB, HDC},
    kernel32, stub,
    user32::HWND,
};

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
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Compact(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper(
        _ctx: &mut Context,
        _this: u32,
        _flags: u32,
        _lplpClipper: u32,
        _pUnkOuter: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette(
        _ctx: &mut Context,
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
        ctx: &mut Context,
        this: u32,
        lpDDSurfaceDesc2: u32,
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> DD {
        let mut ddraw = state().get_ddraw(this);
        let desc = <DDSURFACEDESC2>::read_from_prefix(&ctx.memory[lpDDSurfaceDesc2..])
            .unwrap()
            .0;

        let mut lock = kernel32::lock();
        let surface = ddraw.create_surface(&desc, &mut || {
            IDirectDrawSurface7::new(ctx, &mut lock.process_heap)
        });
        ctx.memory.write(lplpDDSurface, surface.borrow().addr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface(
        _ctx: &mut Context,
        _this: u32,
        _lpDDSurface: u32,
        _lplpDupDDSurface: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes(
        _ctx: &mut Context,
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
        _ctx: &mut Context,
        _this: u32,
        _flags: u32,
        _lpSurfaceDesc2: u32,
        _lpContext: u32,
        _lpEnumCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32, _lpDDDriverCaps: u32, _lpDDEmulCaps: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(_ctx: &mut Context, _this: u32, _lpDDSurfaceDesc2: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes(_ctx: &mut Context, _this: u32, _lpNumCodes: u32, _lpCodes: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface(_ctx: &mut Context, _this: u32, _lplpGDISurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency(_ctx: &mut Context, _this: u32, _lpdwFrequency: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine(_ctx: &mut Context, _this: u32, _lpdwScanLine: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus(_ctx: &mut Context, _this: u32, _lpbIsInVB: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32, _lpGUID: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_ctx: &mut Context, this: u32, hwnd: HWND, flags: u32) -> DD {
        state().get_ddraw(this).set_cooperative_level(hwnd, flags);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        _ctx: &mut Context,
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
    pub fn WaitForVerticalBlank(_ctx: &mut Context, _this: u32, _flags: u32, _hEvent: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetAvailableVidMem(
        _ctx: &mut Context,
        _this: u32,
        _lpDDSCaps2: u32,
        _lpdwTotal: u32,
        _lpdwFree: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceFromDC(_ctx: &mut Context, _this: u32, _hdc: HDC, _lplpDDSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RestoreAllSurfaces(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn TestCooperativeLevel(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceIdentifier(
        _ctx: &mut Context,
        _this: u32,
        _lpDDDeviceIdentifier: u32,
        _flags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn StartModeTest(
        _ctx: &mut Context,
        _this: u32,
        _lpModesToTest: u32,
        _numEntries: u32,
        _flags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EvaluateMode(
        _ctx: &mut Context,
        _this: u32,
        _flags: u32,
        _pSecondsUntilTimeout: u32,
    ) -> DD {
        todo!()
    }

    fn vtable(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let vtable_addr = heap.alloc(&mut ctx.memory, std::mem::size_of::<VTable>() as u32);
        let func_addr = runtime::proc_addr(ctx, QueryInterface_stdcall);
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
            .write_to_prefix(&mut ctx.memory[vtable_addr..])
            .unwrap();
        vtable_addr
    }

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        let vtable = vtable(ctx, heap);
        ctx.memory.write(addr, vtable);
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
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddAttachedSurface(_ctx: &mut Context, _this: u32, _lpDDSAttachedSurface: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddOverlayDirtyRect(_ctx: &mut Context, _this: u32, _lpRect: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Blt(
        _ctx: &mut Context,
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
    pub fn BltBatch(
        _ctx: &mut Context,
        _this: u32,
        _lpDDBltBatch: u32,
        _dwCount: u32,
        _dwFlags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltFast(
        ctx: &mut Context,
        this: u32,
        dwX: u32,
        dwY: u32,
        lpDDSrcSurface: u32,
        lpSrcRect: u32,
        _dwTrans: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut dst_surface = surfaces.get(&this).unwrap().borrow_mut();
        let src_rect = <RECT>::ref_from_prefix(&ctx.memory[lpSrcRect..]).unwrap().0;
        let src_surface = surfaces.get(&lpDDSrcSurface).unwrap().borrow();

        let Target::Texture(dst_texture) = &mut dst_surface.target else {
            unreachable!()
        };
        let Target::Texture(src_texture) = &src_surface.target else {
            unreachable!()
        };

        // To render to a texture, we need to start with a canvas, which we can only get from
        // a window or a surface for some reason.  Use the window in case it has some sort of
        // GPU context attached.
        let ddraw = state().ddraw.borrow();
        let mut canvas = RefMut::map(
            ddraw
                .as_ref()
                .unwrap()
                .window
                .as_ref()
                .unwrap()
                .borrow_mut(),
            |w| &mut w.canvas,
        );

        canvas
            .with_texture_canvas(dst_texture, |canvas| {
                canvas
                    .copy(
                        src_texture,
                        sdl3::rect::Rect::new(
                            src_rect.left as i32,
                            src_rect.top as i32,
                            (src_rect.right - src_rect.left) as u32,
                            (src_rect.bottom - src_rect.top) as u32,
                        ),
                        sdl3::render::FRect::new(
                            dwX as f32,
                            dwY as f32,
                            (src_rect.right - src_rect.left) as f32,
                            (src_rect.bottom - src_rect.top) as f32,
                        ),
                    )
                    .unwrap();
            })
            .unwrap();

        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn DeleteAttachedSurface(
        _ctx: &mut Context,
        _this: u32,
        _dwFlags: u32,
        _lpDDSAttachedSurface: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumAttachedSurfaces(
        _ctx: &mut Context,
        _this: u32,
        _lpContext: u32,
        _lpEnumSurfacesCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumOverlayZOrders(
        _ctx: &mut Context,
        _this: u32,
        _dwFlags: u32,
        _lpContext: u32,
        _lpfnCallback: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Flip(
        _ctx: &mut Context,
        this: u32,
        _lpDDSurfaceTargetOverride: u32,
        _dwFlags: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        surface.flip();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        ctx: &mut Context,
        this: u32,
        _lpDDSCaps: u32,
        lplpDDAttachedSurface: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let surface = surfaces.get(&this).unwrap().borrow();
        ctx.memory.write(
            lplpDDAttachedSurface,
            surface.attached.as_ref().unwrap().borrow().addr,
        );
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetBltStatus(_ctx: &mut Context, _this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32, _lpDDSCaps: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetClipper(_ctx: &mut Context, _this: u32, _lplpDDClipper: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetColorKey(_ctx: &mut Context, _this: u32, _dwFlags: u32, _lpDDColorKey: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDC(ctx: &mut Context, this: u32, lphDC: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        let pixels = surface.lock(&mut ctx.memory);
        let dc = gdi32::state()
            .dcs
            .borrow_mut()
            .add(gdi32::new_memory_dc(DIB {
                width: surface.width,
                height: surface.height,
                pixels,
            }));
        ctx.memory.write(lphDC, dc.to_raw());
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn GetFlipStatus(_ctx: &mut Context, _this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetOverlayPosition(_ctx: &mut Context, _this: u32, _lplX: u32, _lplY: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPalette(_ctx: &mut Context, _this: u32, _lplpDDPalette: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(_ctx: &mut Context, _this: u32, _lpDDPixelFormat: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(ctx: &mut Context, this: u32, lpDDSurfaceDesc2: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let surface = surfaces.get(&this).unwrap().borrow();
        let size = ctx.memory.read::<u32>(lpDDSurfaceDesc2);
        assert_eq!(size, std::mem::size_of::<DDSURFACEDESC2>() as u32);
        ctx.memory.write(
            lpDDSurfaceDesc2,
            DDSURFACEDESC2 {
                dwSize: std::mem::size_of::<DDSURFACEDESC2>() as u32,
                dwFlags: DDSD::WIDTH | DDSD::HEIGHT,
                dwWidth: surface.width,
                dwHeight: surface.height,
                ..Default::default()
            },
        );

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32, _lpDD: u32, _lpDDSurfaceDesc: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn IsLost(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        _ctx: &mut Context,
        _this: u32,
        _lpDestRect: u32,
        _lpDDSurfaceDesc2: u32,
        _dwFlags: u32,
        _hEvent: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(ctx: &mut Context, this: u32, hDC: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        gdi32::state().dcs.borrow_mut().remove(HDC::from_raw(hDC));
        surface.unlock(&mut ctx.memory);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Restore(_ctx: &mut Context, _this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(_ctx: &mut Context, _this: u32, _lpDDClipper: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(_ctx: &mut Context, _this: u32, _dwFlags: u32, _lpDDColorKey: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetOverlayPosition(_ctx: &mut Context, _this: u32, _lX: i32, _lY: i32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(_ctx: &mut Context, _this: u32, _lpDDPalette: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Unlock(_ctx: &mut Context, _this: u32, _lpSurfaceData: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlay(
        _ctx: &mut Context,
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
    pub fn UpdateOverlayDisplay(_ctx: &mut Context, _this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayZOrder(
        _ctx: &mut Context,
        _this: u32,
        _dwFlags: u32,
        _lpDDSurfaceReference: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDDInterface(_ctx: &mut Context, _this: u32, _lplpDD: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn PageLock(_ctx: &mut Context, _this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn PageUnlock(_ctx: &mut Context, _this: u32, _dwFlags: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetSurfaceDesc(
        _ctx: &mut Context,
        _this: u32,
        _lpDDSurfaceDesc2: u32,
        _dwFlags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPrivateData(
        _ctx: &mut Context,
        _this: u32,
        _guidTag: u32,
        _lpData: u32,
        _cbSize: u32,
        _dwFlags: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPrivateData(
        _ctx: &mut Context,
        _this: u32,
        _guidTag: u32,
        _lpBuffer: u32,
        _lpcbBufferSize: u32,
    ) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FreePrivateData(_ctx: &mut Context, _this: u32, _guidTag: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetUniquenessValue(_ctx: &mut Context, _this: u32, _lpValue: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn ChangeUniquenessValue(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPriority(_ctx: &mut Context, _this: u32, _dwPriority: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPriority(_ctx: &mut Context, _this: u32, _lpdwPriority: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetLOD(_ctx: &mut Context, _this: u32, _dwMaxLOD: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetLOD(_ctx: &mut Context, _this: u32, _lpdwMaxLOD: u32) -> DD {
        todo!()
    }

    fn vtable(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let vtable_addr = heap.alloc(&mut ctx.memory, std::mem::size_of::<VTable>() as u32);
        let func_addr = runtime::proc_addr(ctx, QueryInterface_stdcall);
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
            .write_to_prefix(&mut ctx.memory[vtable_addr..])
            .unwrap();
        vtable_addr
    }

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        let vtable = vtable(ctx, heap);
        ctx.memory.write(addr, vtable);
        addr
    }
}
