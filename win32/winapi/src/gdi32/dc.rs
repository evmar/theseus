use runtime::Context;
use std::rc::Rc;

use crate::{
    HANDLE,
    gdi32::{Bitmap, BitmapType, DIB, HDC, state},
    stub,
};

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
        todo!()
    }
}

#[win32_derive::dllexport]
pub fn DeleteDC(_ctx: &mut Context, _hdc: HDC) -> bool {
    stub!(true)
}
