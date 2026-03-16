use crate::{gdi32::HDC, stub};

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(hdc: HDC) -> HDC {
    if hdc.is_null() {
        // memory DC compatible with screen
        stub!(HDC::from_raw(1))
    } else {
        todo!()
    }
}

#[win32_derive::dllexport]
pub fn DeleteDC(_hdc: HDC) -> bool {
    todo!()
}
