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
pub fn DialogBoxParamW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTemplateName: u32, /* WSTR */
    _hWndParent: HWND,
    _lpDialogFunc: u32, /* DLGPROC */
    _dwInitParam: u32,
) -> i32 {
    todo!()
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

#[win32_derive::dllexport]
pub fn GetDlgItem(_ctx: &mut Context, _hDlg: HWND, _nIDDlgItem: i32) -> HWND {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDlgItemInt(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDDlgItem: i32,
    _lpTranslated: bool,
    _bSigned: bool,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDlgItemTextW(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDDlgItem: i32,
    _lpString: u32, /* WSTR */
    _cchMax: i32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetDlgItemInt(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDDlgItem: i32,
    _uValue: u32,
    _bSigned: bool,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetDlgItemTextW(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDDlgItem: i32,
    _lpString: u32, /* WSTR */
) -> bool {
    todo!()
}
