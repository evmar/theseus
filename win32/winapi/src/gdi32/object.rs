use crate::{
    gdi32::{HDC, HGDIOBJ},
    stub,
};

#[win32_derive::dllexport]
pub fn GetObjectA(_h: HGDIOBJ, _c: i32, _pv: u32) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetStockObject(_i: u32 /* GET_STOCK_OBJECT_FLAGS */) -> HGDIOBJ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn SelectObject(_hdc: HDC, _h: HGDIOBJ) -> HGDIOBJ {
    todo!()
}
