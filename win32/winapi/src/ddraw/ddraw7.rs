use std::cell::RefMut;

use runtime::*;
use zerocopy::FromBytes;

use crate::{
    RECT,
    ddraw::{GUID, Target, state, types::*},
    gdi32,
    gdi32::HDC,
    heap::Heap,
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

    pub const VTABLE_ENTRIES: [&str; 30] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "Compact",
        "CreateClipper",
        "CreatePalette",
        "CreateSurface",
        "DuplicateSurface",
        "EnumDisplayModes",
        "EnumSurfaces",
        "FlipToGDISurface",
        "GetCaps",
        "GetDisplayMode",
        "GetFourCCCodes",
        "GetGDISurface",
        "GetMonitorFrequency",
        "GetScanLine",
        "GetVerticalBlankStatus",
        "Initialize",
        "RestoreDisplayMode",
        "SetCooperativeLevel",
        "SetDisplayMode",
        "WaitForVerticalBlank",
        "GetAvailableVidMem",
        "GetSurfaceFromDC",
        "RestoreAllSurfaces",
        "TestCooperativeLevel",
        "GetDeviceIdentifier",
        "StartModeTest",
        "EvaluateMode",
    ];

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

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}

pub mod IDirectDrawSurface7 {
    use super::*;

    pub const VTABLE_ENTRIES: [&str; 49] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "AddAttachedSurface",
        "AddOverlayDirtyRect",
        "Blt",
        "BltBatch",
        "BltFast",
        "DeleteAttachedSurface",
        "EnumAttachedSurfaces",
        "EnumOverlayZOrders",
        "Flip",
        "GetAttachedSurface",
        "GetBltStatus",
        "GetCaps",
        "GetClipper",
        "GetColorKey",
        "GetDC",
        "GetFlipStatus",
        "GetOverlayPosition",
        "GetPalette",
        "GetPixelFormat",
        "GetSurfaceDesc",
        "Initialize",
        "IsLost",
        "Lock",
        "ReleaseDC",
        "Restore",
        "SetClipper",
        "SetColorKey",
        "SetOverlayPosition",
        "SetPalette",
        "Unlock",
        "UpdateOverlay",
        "UpdateOverlayDisplay",
        "UpdateOverlayZOrder",
        "GetDDInterface",
        "PageLock",
        "PageUnlock",
        "SetSurfaceDesc",
        "SetPrivateData",
        "GetPrivateData",
        "FreePrivateData",
        "GetUniquenessValue",
        "ChangeUniquenessValue",
        "SetPriority",
        "GetPriority",
        "SetLOD",
        "GetLOD",
    ];

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
        ctx: &mut Context,
        this: u32,
        _lpDDSurfaceTargetOverride: u32,
        _dwFlags: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        surface.flip(&mut ctx.memory);
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
        let dc = gdi32::lock().new_memory_dc(gdi32::Bitmap::new_simple(
            surface.width,
            surface.height,
            pixels,
        ));
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
    pub fn ReleaseDC(ctx: &mut Context, this: u32, hDC: HDC) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        gdi32::lock().release_dc(hDC);
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

    pub static mut VTABLE: u32 = 0;
    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}
