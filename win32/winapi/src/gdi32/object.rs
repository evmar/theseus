use runtime::Context;

use crate::{
    gdi32::{HDC, HGDIOBJ, state},
    stub,
};

#[repr(C)]
#[derive(zerocopy::Immutable, zerocopy::IntoBytes)]
struct BITMAP {
    bmType: u32,
    bmWidth: u32,
    bmHeight: u32,
    bmWidthBytes: u32,
    bmPlanes: u16,
    bmBitsPixel: u16,
    bmBits: u32,
}

#[win32_derive::dllexport]
pub fn GetObjectA(ctx: &mut Context, handle: HGDIOBJ, size: u32, lpOut: u32) -> u32 {
    let bitmap = state().objects.borrow().get(handle).unwrap().clone();
    assert!(size == std::mem::size_of::<BITMAP>() as u32);
    let fields = BITMAP {
        bmType: 0,
        bmWidth: bitmap.width,
        bmHeight: bitmap.height,
        bmWidthBytes: 0,
        bmPlanes: 0,
        bmBitsPixel: bitmap.bit_count as u16,
        bmBits: 0,
    };
    ctx.memory.write(lpOut, fields);
    size
}

#[win32_derive::dllexport]
pub fn GetStockObject(_ctx: &mut Context, _i: u32 /* GET_STOCK_OBJECT_FLAGS */) -> HGDIOBJ {
    stub!(HGDIOBJ::null())
}

#[win32_derive::dllexport]
pub fn SelectObject(_ctx: &mut Context, hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
    let object = state().objects.borrow().get(h).unwrap().clone();
    let mut dcs = state().dcs.borrow_mut();
    let dc = dcs.get_mut(hdc).unwrap();
    // let prev = dc.bitmap;
    dc.bitmap = object;
    stub!(HGDIOBJ::null())
}
