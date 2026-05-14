use std::rc::Rc;

use runtime::Context;

use crate::{
    HANDLE,
    gdi32::{Bitmap, BitmapType, COLORREF, DIB, State, state},
    stub,
};

pub type HDC = HANDLE;

impl State {
    pub fn release_dc(&self, hdc: HDC) {
        self.dcs.borrow_mut().remove(hdc);
    }
}

#[derive(Default)]
pub struct DC {
    pub bitmap: Option<Rc<Bitmap>>,
}

pub fn new_memory_dc(dib: DIB) -> DC {
    DC {
        bitmap: Some(Rc::new(Bitmap {
            handle: HANDLE::null(),
            typ: BitmapType::DIB(dib),
        })),
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(_ctx: &mut Context, hdc: HDC) -> HDC {
    if hdc.is_null() {
        // memory DC compatible with screen
        state().dcs.borrow_mut().add(DC::default())
    } else {
        // memory DC compatible with hdc
        state().dcs.borrow_mut().add(DC::default())
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
