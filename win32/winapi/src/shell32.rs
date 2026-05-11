use runtime::Context;

use crate::user32::{HICON, HWND};

#[win32_derive::dllexport]
pub fn ShellAboutW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _szApp: u32,        /* WSTR */
    _szOtherStuff: u32, /* WSTR */
    _hIcon: HICON,
) -> i32 {
    todo!()
}
