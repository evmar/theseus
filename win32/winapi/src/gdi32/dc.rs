use std::rc::Rc;

use crate::{
    gdi32::{Bitmap, HDC, state},
    stub,
};

#[derive(Default)]
pub struct DC {
    pub bitmap: Option<Rc<Bitmap>>,
}

pub fn new_memory_dc() -> DC {
    DC::default()
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(hdc: HDC) -> HDC {
    if hdc.is_null() {
        // memory DC compatible with screen
        state().dcs.borrow_mut().add(DC::default())
    } else {
        todo!()
    }
}

#[win32_derive::dllexport]
pub fn DeleteDC(_hdc: HDC) -> bool {
    stub!(true)
}
