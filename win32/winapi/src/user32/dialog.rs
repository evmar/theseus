use runtime::Context;

use crate::{
    stub,
    user32::{HINSTANCE, HWND},
};

#[win32_derive::dllexport]
pub fn MessageBoxA(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpText: u32,
    _lpCaption: u32,
    _uType: u32, /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    todo!()
}

#[win32_derive::dllexport]
pub fn DialogBoxParamA(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTemplateName: u32,
    _hWndParent: HWND,
    _lpDialogFunc: u32, /* DLGPROC */
    _dwInitParam: u32,
) -> i32 {
    stub!(1) // return value from dialog proc
}

#[win32_derive::dllexport]
pub fn CheckDlgButton(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDButton: i32,
    _uCheck: u32, /* DLG_BUTTON_CHECK_STATE */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn EndDialog(_ctx: &mut Context, _hDlg: HWND, _nResult: i32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn IsDlgButtonChecked(_ctx: &mut Context, _hDlg: HWND, _nIDButton: i32) -> u32 {
    todo!()
}
