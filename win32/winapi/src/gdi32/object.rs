use runtime::Machine;

use crate::{
    gdi32::{BitmapType, HDC, HGDIOBJ, state},
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
pub fn GetObjectA(m: &mut Machine, handle: HGDIOBJ, size: u32, lpOut: u32) -> u32 {
    let bitmap = state().objects.borrow().get(handle).unwrap().clone();
    assert!(size == std::mem::size_of::<BITMAP>() as u32);
    let fields = match &bitmap.typ {
        BitmapType::DDB(ddb) => BITMAP {
            bmType: 0,
            bmWidth: ddb.width,
            bmHeight: ddb.height,
            bmWidthBytes: 0,
            bmPlanes: 0,
            bmBitsPixel: ddb.bit_count as u16,
            bmBits: 0,
        },
        BitmapType::DIB(_) => todo!(),
    };
    m.memory.write(lpOut, fields);
    size
}

#[win32_derive::dllexport]
pub fn GetStockObject(_m: &mut Machine, _i: u32 /* GET_STOCK_OBJECT_FLAGS */) -> HGDIOBJ {
    stub!(HGDIOBJ::null())
}

#[win32_derive::dllexport]
pub fn SelectObject(_m: &mut Machine, hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
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
