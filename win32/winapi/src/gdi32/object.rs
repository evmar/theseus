use runtime::MACHINE;

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
pub fn GetObjectA(handle: HGDIOBJ, size: u32, lpOut: u32) -> u32 {
    let bitmap = state().objects.borrow().get(handle).unwrap().clone();
    assert!(size == std::mem::size_of::<BITMAP>() as u32);
    unsafe {
        MACHINE.memory.write(
            lpOut,
            BITMAP {
                bmType: 0,
                bmWidth: bitmap.header.width,
                bmHeight: bitmap.header.height,
                bmWidthBytes: 0,
                bmPlanes: 0,
                bmBitsPixel: bitmap.header.bit_count as u16,
                bmBits: 0,
            },
        );
    }
    size
}

#[win32_derive::dllexport]
pub fn GetStockObject(_i: u32 /* GET_STOCK_OBJECT_FLAGS */) -> HGDIOBJ {
    stub!(HGDIOBJ::null())
}

#[win32_derive::dllexport]
pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
    let object = state().objects.borrow().get(h).unwrap().clone();
    let prev = state()
        .dcs
        .borrow_mut()
        .get_mut(hdc)
        .unwrap()
        .bitmap
        .replace(object);
    match prev {
        None => HGDIOBJ::null(),
        Some(bitmap) => bitmap.handle,
    }
}
