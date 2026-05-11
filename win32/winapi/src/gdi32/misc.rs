use runtime::Context;

use crate::gdi32::HDC;

#[win32_derive::dllexport]
pub fn BitBlt(
    _ctx: &mut Context,
    _hdc: HDC,
    _x: i32,
    _y: i32,
    _cx: i32,
    _cy: i32,
    _hdcSrc: HDC,
    _x1: i32,
    _y1: i32,
    _rop: u32, /* ROP_CODE */
) -> bool {
    todo!()
}

pub type HBITMAP = u32;

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(_ctx: &mut Context, _hdc: HDC, _cx: i32, _cy: i32) -> HBITMAP {
    todo!()
}

pub type COLORREF = u32;
pub type HGDIOBJ = u32;
pub type HPEN = u32;

#[win32_derive::dllexport]
pub fn CreatePen(
    _ctx: &mut Context,
    _iStyle: u32, /* PEN_STYLE */
    _cWidth: i32,
    _color: COLORREF,
) -> HPEN {
    todo!()
}

#[win32_derive::dllexport]
pub fn DeleteObject(_ctx: &mut Context, _ho: HGDIOBJ) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDeviceCaps(_ctx: &mut Context, _hdc: HDC, _index: i32) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetLayout(_ctx: &mut Context, _hdc: HDC) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn LineTo(_ctx: &mut Context, _hdc: HDC, _x: i32, _y: i32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MoveToEx(
    _ctx: &mut Context,
    _hdc: HDC,
    _x: i32,
    _y: i32,
    _lppt: u32, /* POINT */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    _ctx: &mut Context,
    _hdc: HDC,
    _xDest: i32,
    _yDest: i32,
    _w: u32,
    _h: u32,
    _xSrc: i32,
    _ySrc: i32,
    _StartScan: u32,
    _cLines: u32,
    _lpvBits: u32,
    _lpbmi: u32,    /* BITMAPINFO */
    _ColorUse: u32, /* DIB_USAGE */
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetLayout(_ctx: &mut Context, _hdc: HDC, _l: u32 /* DC_LAYOUT */) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPixel(_ctx: &mut Context, _hdc: HDC, _x: i32, _y: i32, _color: COLORREF) -> COLORREF {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetROP2(_ctx: &mut Context, _hdc: HDC, _rop2: u32 /* R2_MODE */) -> i32 {
    todo!()
}
