use runtime::*;

use crate::stub;

pub type HDC = u32;
pub type HGDIOBJ = u32;

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(_hdc: HDC) -> HDC {
    todo!()
}

#[win32_derive::dllexport]
pub fn DeleteDC(_hdc: HDC) -> bool {
    todo!()
}

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

#[win32_derive::dllexport]
pub fn StretchBlt(
    _hdcDest: HDC,
    _xDest: i32,
    _yDest: i32,
    _wDest: i32,
    _hDest: i32,
    _hdcSrc: HDC,
    _xSrc: i32,
    _ySrc: i32,
    _wSrc: i32,
    _hSrc: i32,
    _rop: u32, /* ROP_CODE */
) -> bool {
    todo!()
}
