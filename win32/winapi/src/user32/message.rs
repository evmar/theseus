use runtime::*;

use crate::{stub, user32::HWND};

#[win32_derive::dllexport]
pub fn DispatchMessageA(_lpMsg: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_lpMsg: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    _lpMsg: u32,
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    stub!(false)
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_nExitCode: i32) {
    todo!()
}
