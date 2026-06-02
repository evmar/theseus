use std::{cell::RefCell, rc::Rc};

use runtime::Context;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    ddraw::{DD, Palette, get_pixel_format, state, types::*},
    heap::Heap,
    kernel32, stub,
    user32::HWND,
};

pub mod IDirectDraw {
    use super::*;

    pub const VTABLE_ENTRIES: [&str; 23] = [
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
    ];

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppvObject: u32) -> DD {
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
    pub fn CreateClipper(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette(
        ctx: &mut Context,
        _this: u32,
        flags: DDPCAPS,
        lpEntries: u32,
        lplpPal: u32,
        pUnkOuter: u32,
    ) -> DD {
        assert_eq!(pUnkOuter, 0);
        assert!(flags.contains(DDPCAPS::_8BIT));

        let mut kernel32 = kernel32::lock();
        let ptr = IDirectDrawPalette::new(ctx, &mut kernel32.process_heap);

        let entries = <[PALETTEENTRY]>::ref_from_prefix_with_elems(&ctx.memory[lpEntries..], 256)
            .unwrap()
            .0;
        state().palette.borrow_mut().insert(
            ptr,
            Rc::new(RefCell::new(Palette {
                entries: entries.into_iter().cloned().collect(),
            })),
        );
        ctx.memory.write::<u32>(lplpPal, ptr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        ctx: &mut Context,
        this: u32,
        desc: u32,
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> DD {
        let mut ddraw = state().get_ddraw(this);
        let desc = <DDSURFACEDESC>::ref_from_prefix(&ctx.memory[desc..])
            .unwrap()
            .0;
        let desc2 = DDSURFACEDESC2::from_desc(&desc);
        let mut state = kernel32::lock();
        let surface = ddraw.create_surface(&desc2, &mut || {
            IDirectDrawSurface::new(ctx, &mut state.process_heap)
        });
        ctx.memory.write(lplpDDSurface, surface.borrow().addr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumDisplayModes(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumSurfaces(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn FlipToGDISurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFourCCCodes(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetGDISurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetMonitorFrequency(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetScanLine(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVerticalBlankStatus(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32) -> DD {
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
    pub fn SetDisplayMode(ctx: &mut Context, this: u32, width: u32, height: u32, bpp: u32) -> DD {
        let mut ddraw = state().get_ddraw(this);
        ddraw
            .window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .resize(ctx, width, height);
        assert!(bpp % 8 == 0);
        ddraw.bytes_per_pixel = bpp / 8;
        stub!(DD::OK)
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}

pub mod IDirectDrawSurface {
    use super::*;

    pub const VTABLE_ENTRIES: [&str; 36] = [
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
    ];

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32) -> DD {
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
    pub fn AddAttachedSurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddOverlayDirtyRect(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Blt(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltBatch(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn BltFast(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DeleteAttachedSurface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumAttachedSurfaces(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumOverlayZOrders(_ctx: &mut Context, _this: u32) -> DD {
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
    pub fn GetBltStatus(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetClipper(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetColorKey(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDC(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFlipStatus(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetOverlayPosition(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPalette(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(ctx: &mut Context, _this: u32, lpDDPixelFormat: u32) -> DD {
        ctx.memory.write(lpDDPixelFormat, get_pixel_format());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn IsLost(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        ctx: &mut Context,
        this: u32,
        rect: u32,
        lpDesc: u32,
        _flags: u32,
        _unused: u32,
    ) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        assert_eq!(rect, 0);

        let pixels = surface.lock(&mut ctx.memory);
        let desc = DDSURFACEDESC {
            dwSize: std::mem::size_of::<DDSURFACEDESC>() as u32,
            lPitch_dwLinearSize: surface.width * surface.bytes_per_pixel,
            lpSurface: pixels,
            ..DDSURFACEDESC::default()
        };
        desc.write_to_prefix(&mut ctx.memory[lpDesc..]).unwrap();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Restore(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetOverlayPosition(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(_ctx: &mut Context, this: u32, lpPalette: u32) -> DD {
        let state = state();
        let surfaces = state.surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        let palettes = state.palette.borrow_mut();
        let palette = palettes.get(&lpPalette).unwrap();
        surface.palette = Some(palette.clone());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(ctx: &mut Context, this: u32, _lpRect: u32) -> DD {
        let surfaces = state().surf.borrow_mut();
        let mut surface = surfaces.get(&this).unwrap().borrow_mut();
        surface.unlock(&mut ctx.memory);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlay(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayDisplay(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn UpdateOverlayZOrder(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}

pub mod IDirectDrawPalette {
    use super::*;

    pub const VTABLE_ENTRIES: [&str; 7] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "GetCaps",
        "GetEntries",
        "Initialize",
        "SetEntries",
    ];

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetEntries(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetEntries(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}
