use runtime::Context;

use crate::{
    Ptr, stub,
    user32::{HINSTANCE, HWND},
};

#[win32_derive::dllexport]
pub fn DialogBoxParamA(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTemplateName: Ptr<u8>,
    _hWndParent: HWND,
    _lpDialogFunc: Ptr<()>, /* DLGPROC */
    _dwInitParam: u32,
) -> i32 {
    stub!(1) // return value from dialog proc
}

#[win32_derive::dllexport]
pub fn DialogBoxParamW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTemplateName: Ptr<u16>, /* WSTR */
    _hWndParent: HWND,
    _lpDialogFunc: Ptr<()>, /* DLGPROC */
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
    _lpTranslated: Ptr<u32>,
    _bSigned: bool,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDlgItemTextW(
    _ctx: &mut Context,
    _hDlg: HWND,
    _nIDDlgItem: i32,
    _lpString: Ptr<u16>, /* WSTR */
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
    _lpString: Ptr<u16>, /* WSTR */
) -> bool {
    todo!()
}
