use crate::gdi32::HDC;

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(_hdc: HDC) -> HDC {
    todo!()
}

#[win32_derive::dllexport]
pub fn DeleteDC(_hdc: HDC) -> bool {
    todo!()
}
