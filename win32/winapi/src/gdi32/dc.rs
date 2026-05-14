use std::sync::Arc;

use runtime::Context;

use crate::{
    HANDLE,
    gdi32::{self, Bitmap, COLORREF, State},
    stub,
};

pub type HDC = HANDLE;

impl State {
    pub fn release_dc(&mut self, hdc: HDC) {
        self.dcs.remove(hdc);
    }
}

pub struct DC {
    pub bitmap: Arc<Bitmap>,
}

pub fn new_memory_dc(bitmap: Arc<Bitmap>) -> DC {
    DC { bitmap }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(_ctx: &mut Context, hdc: HDC) -> HDC {
    // 1x1 monochrome bitmap
    let bitmap = Bitmap {
        width: 1,
        height: 1,
        is_bottom_up: false,
        bit_count: 1,
        palette: Box::new([[0; 4]]),
        pixels: 0,
    };
    let dc = new_memory_dc(Arc::new(bitmap));
    if hdc.is_null() {
        // memory DC compatible with screen
        gdi32::lock().dcs.add(dc)
    } else {
        // memory DC compatible with hdc
        gdi32::lock().dcs.add(dc)
    }
}

#[win32_derive::dllexport]
pub fn DeleteDC(_ctx: &mut Context, _hdc: HDC) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn GetLayout(_ctx: &mut Context, _hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn SetROP2(_ctx: &mut Context, _hdc: HDC, _rop2: u32 /* R2_MODE */) -> i32 {
    stub!(0)
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
pub fn SetLayout(_ctx: &mut Context, _hdc: HDC, _l: u32 /* DC_LAYOUT */) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPixel(_ctx: &mut Context, _hdc: HDC, _x: i32, _y: i32, _color: COLORREF) -> COLORREF {
    todo!()
}
