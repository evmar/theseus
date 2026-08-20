use std::{cell::RefCell, rc::Rc};

use runtime::Context;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    ddraw::{DD, GUID, Palette, get_pixel_format, state, types::*},
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
    pub fn QueryInterface(ctx: &mut Context, _this: u32, riid: u32, _ppvObject: u32) -> DD {
        let iid = crate::Ptr::<GUID>::new(riid).read(&ctx.memory);
        log::warn!("IDirectDraw::QueryInterface({iid:?}): not supported");
        DD::E_NOINTERFACE
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        // We don't reference count; the single DirectDraw object lives as long
        // as the process.
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        0
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
    pub fn EnumDisplayModes(
        ctx: &mut Context,
        _this: u32,
        _dwFlags: u32,
        lpSurfaceDesc: u32,
        lpContext: u32,
        lpEnumCallback: u32,
    ) -> DD {
        if lpSurfaceDesc != 0 {
            todo!("EnumDisplayModes with a filter desc");
        }

        // Report the standard display modes; games match these against their
        // internal mode tables by width/height/bit count.
        const RESOLUTIONS: &[(u32, u32)] = &[(640, 480), (800, 600), (1024, 768)];
        // Only depths the surface code can actually convert to rgba.
        const BIT_DEPTHS: &[u32] = &[8, 16, 32];

        for &(width, height) in RESOLUTIONS {
            for &bpp in BIT_DEPTHS {
                let mut desc = DDSURFACEDESC::default();
                desc.dwSize = std::mem::size_of::<DDSURFACEDESC>() as u32;
                desc.dwFlags = DDSD::WIDTH | DDSD::HEIGHT | DDSD::PIXELFORMAT | DDSD::PITCH;
                desc.dwWidth = width;
                desc.dwHeight = height;
                desc.lPitch_dwLinearSize = width * bpp.div_ceil(8);

                // DDPF_RGB = 0x40, DDPF_PALETTEINDEXED8 = 0x20.
                let (flags, r, g, b) = match bpp {
                    8 => (0x40 | 0x20, 0, 0, 0),
                    16 => (0x40, 0xF800, 0x07E0, 0x001F), // 5-6-5
                    _ => (0x40, 0xFF0000, 0x00FF00, 0x0000FF), // 24/32
                };
                desc.ddpfPixelFormat = DDPIXELFORMAT {
                    dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
                    dwFlags: flags,
                    dwFourCC: 0,
                    dwRGBBitCount: bpp,
                    dwRBitMask: r,
                    dwGBitMask: g,
                    dwBBitMask: b,
                    dwRGBAlphaBitMask: 0,
                };

                let desc_addr = kernel32::lock()
                    .process_heap
                    .alloc(&mut ctx.memory, desc.dwSize);
                ctx.memory.write(desc_addr, desc);
                let callback = ctx.indirect(lpEnumCallback);
                ctx.call32_x86(callback, vec![desc_addr, lpContext]);
                let ret = ctx.cpu.regs.eax;
                kernel32::lock()
                    .process_heap
                    .free(&mut ctx.memory, desc_addr);

                // DDENUMRET_CANCEL (0) means stop enumerating.
                if ret == 0 {
                    return DD::OK;
                }
            }
        }

        DD::OK
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
    pub fn Initialize(
        _ctx: &mut Context,
        _this: u32,
        _lpDD: u32,
        _dwFlags: u32,
        _lpDDColorTable: u32,
    ) -> DD {
        // Nothing to do: the object is fully constructed when it's created.
        DD::OK
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
    pub fn WaitForVerticalBlank(_ctx: &mut Context, _this: u32, _dwFlags: u32, _hEvent: u32) -> DD {
        DD::OK // pretend the vblank already happened
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
    pub fn QueryInterface(ctx: &mut Context, _this: u32, riid: u32, _ppvObject: u32) -> DD {
        let iid = crate::Ptr::<GUID>::new(riid).read(&ctx.memory);
        log::warn!("IDirectDrawSurface::QueryInterface({iid:?}): not supported");
        DD::E_NOINTERFACE
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, this: u32) -> u32 {
        match state().surf.borrow_mut().get(&this) {
            Some(surface) => {
                let mut surface = surface.borrow_mut();
                surface.refs += 1;
                surface.refs
            }
            None => 0,
        }
    }

    #[win32_derive::dllexport]
    pub fn Release(ctx: &mut Context, this: u32) -> u32 {
        let surfaces = state().surf.borrow_mut();
        let Some(surface) = surfaces.get(&this) else {
            return 0;
        };
        let remaining = {
            let mut surface = surface.borrow_mut();
            surface.refs = surface.refs.saturating_sub(1);
            surface.refs
        };
        drop(surfaces);
        if remaining > 0 {
            return remaining;
        }
        let Some(surface) = state().surf.borrow_mut().remove(&this) else {
            return 0;
        };
        // Games recreate surfaces when changing screens, so returning the
        // pixels keeps the heap from growing without bound.
        if let Some(pixels) = surface.borrow_mut().pixels.take() {
            kernel32::lock().process_heap.free(&mut ctx.memory, pixels);
        }
        0
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
    pub fn Blt(
        ctx: &mut Context,
        this: u32,
        lpDstRect: u32,
        lpDDSrcSurface: u32,
        lpSrcRect: u32,
        dwFlags: u32,
        lpDDBLTFX: u32,
    ) -> DD {
        crate::ddraw::ddraw::blt(
            ctx,
            this,
            lpDstRect,
            lpDDSrcSurface,
            lpSrcRect,
            dwFlags,
            lpDDBLTFX,
        )
    }

    #[win32_derive::dllexport]
    pub fn BltBatch(_ctx: &mut Context, _this: u32) -> DD {
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
        dwTrans: u32,
    ) -> DD {
        crate::ddraw::ddraw::blt_fast(ctx, this, dwX, dwY, lpDDSrcSurface, lpSrcRect, dwTrans)
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
        {
            let surfaces = state().surf.borrow_mut();
            let mut surface = surfaces.get(&this).unwrap().borrow_mut();
            surface.flip(&mut ctx.memory);
        }
        // A frame flip is the one thing a game does every frame no matter what
        // it's doing, so it's where we keep the audio mixer fed.
        crate::dsound::pump(ctx);
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
    pub fn GetCaps(ctx: &mut Context, _this: u32, lpDDSCaps: u32) -> DD {
        let caps = DDSCAPS::BACKBUFFER | DDSCAPS::COMPLEX | DDSCAPS::FLIP | DDSCAPS::VIDEOMEMORY;
        ctx.memory.write::<u32>(lpDDSCaps, caps.bits());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetClipper(_ctx: &mut Context, _this: u32) -> DD {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetColorKey(ctx: &mut Context, this: u32, dwFlags: u32, lpDDColorKey: u32) -> DD {
        crate::ddraw::ddraw::get_color_key(ctx, this, dwFlags, lpDDColorKey)
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(ctx: &mut Context, this: u32, dwFlags: u32, lpDDColorKey: u32) -> DD {
        crate::ddraw::ddraw::set_color_key(ctx, this, dwFlags, lpDDColorKey)
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
    pub fn GetSurfaceDesc(ctx: &mut Context, this: u32, lpDDSurfaceDesc: u32) -> DD {
        let desc = {
            let surfaces = state().surf.borrow_mut();
            let surface = surfaces.get(&this).unwrap().borrow();
            let bpp = surface.bytes_per_pixel * 8;
            let pixel_format = if bpp == 8 {
                DDPIXELFORMAT {
                    dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
                    dwFlags: 0x40 | 0x20, // DDPF_RGB | DDPF_PALETTEINDEXED8
                    dwFourCC: 0,
                    dwRGBBitCount: 8,
                    dwRBitMask: 0,
                    dwGBitMask: 0,
                    dwBBitMask: 0,
                    dwRGBAlphaBitMask: 0,
                }
            } else {
                get_pixel_format()
            };
            DDSURFACEDESC {
                dwSize: std::mem::size_of::<DDSURFACEDESC>() as u32,
                dwFlags: DDSD::WIDTH | DDSD::HEIGHT | DDSD::PITCH | DDSD::PIXELFORMAT,
                dwWidth: surface.width,
                dwHeight: surface.height,
                lPitch_dwLinearSize: surface.width * surface.bytes_per_pixel,
                ddpfPixelFormat: pixel_format,
                ..DDSURFACEDESC::default()
            }
        };
        desc.write_to_prefix(&mut ctx.memory[lpDDSurfaceDesc..])
            .unwrap();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Initialize(
        _ctx: &mut Context,
        _this: u32,
        _lpDD: u32,
        _dwFlags: u32,
        _lpDDColorTable: u32,
    ) -> DD {
        // Nothing to do: the object is fully constructed when it's created.
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn IsLost(_ctx: &mut Context, _this: u32) -> DD {
        DD::OK // our surfaces are never lost
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
        // unlock presents window-backed surfaces itself.
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
    pub fn QueryInterface(ctx: &mut Context, _this: u32, riid: u32, _ppvObject: u32) -> DD {
        let iid = crate::Ptr::<GUID>::new(riid).read(&ctx.memory);
        log::warn!("IDirectDrawPalette::QueryInterface({iid:?}): not supported");
        DD::E_NOINTERFACE
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(ctx: &mut Context, this: u32) -> u32 {
        // Surfaces hold their own reference to the palette, so dropping it from
        // the table doesn't disturb anything still displaying it.
        state().palette.borrow_mut().remove(&this);
        kernel32::lock().process_heap.free(&mut ctx.memory, this);
        0
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(ctx: &mut Context, _this: u32, lpdwCaps: u32) -> DD {
        // We only ever create 8-bit palettes with all 256 entries settable.
        let caps = DDPCAPS::_8BIT | DDPCAPS::ALLOW256;
        ctx.memory.write::<u32>(lpdwCaps, caps.bits());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetEntries(
        ctx: &mut Context,
        this: u32,
        _dwFlags: u32,
        dwBase: u32,
        dwNumEntries: u32,
        lpEntries: u32,
    ) -> DD {
        let palettes = state().palette.borrow();
        let Some(palette) = palettes.get(&this) else {
            return DD::ERR_GENERIC;
        };
        let palette = palette.borrow();
        for i in 0..dwNumEntries {
            let Some(entry) = palette.entries.get((dwBase + i) as usize) else {
                break;
            };
            ctx.memory.write(lpEntries + i * 4, entry.clone());
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Initialize(
        _ctx: &mut Context,
        _this: u32,
        _lpDD: u32,
        _dwFlags: u32,
        _lpDDColorTable: u32,
    ) -> DD {
        // Nothing to do: the object is fully constructed when it's created.
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetEntries(
        ctx: &mut Context,
        this: u32,
        _dwFlags: u32,
        dwStartingEntry: u32,
        dwCount: u32,
        lpEntries: u32,
    ) -> DD {
        let new_entries = <[PALETTEENTRY]>::ref_from_prefix_with_elems(
            &ctx.memory[lpEntries..],
            dwCount as usize,
        )
        .unwrap()
        .0
        .to_vec();
        let palettes = state().palette.borrow_mut();
        let mut palette = palettes.get(&this).unwrap().borrow_mut();
        for (i, entry) in new_entries.into_iter().enumerate() {
            let index = dwStartingEntry as usize + i;
            if index < palette.entries.len() {
                palette.entries[index] = entry;
            }
        }
        DD::OK
    }

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}
