use std::rc::Rc;

use runtime::Context;

use crate::{
    HANDLE,
    gdi32::{Bitmap, BitmapType, DIB, State, state},
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
