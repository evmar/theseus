use runtime::Context;

use super::*;
use crate::stub;

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_ctx: &mut Context, nIndex: u32 /* SYSTEM_METRICS_INDEX */) -> i32 {
    // These were dumped from a win2k VM running at 640x480.
    // See exe/rust/bin/metrics.rs.
    const METRICS: [i32; 100] = [
        640, 480, 16, 16, 19, 1, 1, 3, 3, 16, 16, 32, 32, 32, 32, 19, 640, 433, 0, 1, 16, 16, 0, 0,
        0, 0, 0, 0, 112, 27, 18, 18, 4, 4, 112, 27, 4, 4, 75, 75, 0, 0, 0, 5, 0, 2, 2, 160, 24, 16,
        16, 16, 12, 15, 18, 18, 8, 160, 24, 652, 492, 648, 460, 3, 0, 0, 0, 0, 4, 4, 0, 13, 13, 0,
        0, 1, 0, 0, 640, 480, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    METRICS[nIndex as usize]
}

#[win32_derive::dllexport]
pub fn ShowCursor(_ctx: &mut Context, bShow: bool) -> i32 {
    if bShow { stub!(1) } else { stub!(0) }
}

#[win32_derive::dllexport]
pub fn ValidateRect(_ctx: &mut Context, _hWnd: HWND, _lpRect: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateCursor(
    _ctx: &mut Context,
    _hInst: HINSTANCE,
    _xHotSpot: i32,
    _yHotSpot: i32,
    _nWidth: i32,
    _nHeight: i32,
    _pvANDPlane: u32,
    _pvXORPlane: u32,
) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn InvalidateRect(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpRect: u32, /* RECT */
    _bErase: bool,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MapWindowPoints(
    _ctx: &mut Context,
    _hWndFrom: HWND,
    _hWndTo: HWND,
    _lpPoints: u32, /* POINT */
    _cPoints: u32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn PtInRect(_ctx: &mut Context, _lprc: u32 /* RECT */, _pt: u32 /* POINT */) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetRect(
    _ctx: &mut Context,
    _lprc: u32, /* RECT */
    _xLeft: i32,
    _yTop: i32,
    _xRight: i32,
    _yBottom: i32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn ReleaseCapture(_ctx: &mut Context) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetCapture(_ctx: &mut Context, _hWnd: HWND) -> HWND {
    todo!()
}

#[win32_derive::dllexport]
pub fn WinHelpW(
    _ctx: &mut Context,
    _hWndMain: HWND,
    _lpszHelp: u32, /* WSTR */
    _uCommand: u32,
    _dwData: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(_ctx: &mut Context, _hMenu: HMENU, _uIDCheckItem: u32, _uCheck: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateWindowExW(
    _ctx: &mut Context,
    _dwExStyle: u32,    /* WINDOW_EX_STYLE */
    _lpClassName: u32,  /* WSTR */
    _lpWindowName: u32, /* WSTR */
    _dwStyle: u32,      /* WINDOW_STYLE */
    _X: i32,
    _Y: i32,
    _nWidth: i32,
    _nHeight: i32,
    _hWndParent: HWND,
    _hMenu: HMENU,
    _hInstance: HINSTANCE,
    _lpParam: u32,
) -> HWND {
    todo!()
}

pub type WPARAM = u32;
pub type LPARAM = u32;
pub type LRESULT = i32;

#[win32_derive::dllexport]
pub fn DefWindowProcW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: WPARAM,
    _lParam: LPARAM,
) -> LRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn DialogBoxParamW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTemplateName: u32, /* WSTR */
    _hWndParent: HWND,
    _lpDialogFunc: u32, /* DLGPROC */
    _dwInitParam: LPARAM,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn DispatchMessageW(_ctx: &mut Context, _lpMsg: u32 /* MSG */) -> LRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDesktopWindow(_ctx: &mut Context) -> HWND {
    stub!(HWND::null())
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
pub fn GetMenuItemRect(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hMenu: HMENU,
    _uItem: u32,
    _lprcItem: u32, /* RECT */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetMessageW(
    _ctx: &mut Context,
    _lpMsg: u32, /* MSG */
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn KillTimer(_ctx: &mut Context, _hWnd: HWND, _uIDEvent: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MessageBoxW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpText: u32,    /* WSTR */
    _lpCaption: u32, /* WSTR */
    _uType: u32,     /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    todo!()
}

#[win32_derive::dllexport]
pub fn MoveWindow(
    _ctx: &mut Context,
    _hWnd: HWND,
    _X: i32,
    _Y: i32,
    _nWidth: i32,
    _nHeight: i32,
    _bRepaint: bool,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekMessageW(
    _ctx: &mut Context,
    _lpMsg: u32, /*MSG*/
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostMessageW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: WPARAM,
    _lParam: LPARAM,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SendMessageW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: WPARAM,
    _lParam: LPARAM,
) -> LRESULT {
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

#[win32_derive::dllexport]
pub fn SetMenu(_ctx: &mut Context, _hWnd: HWND, _hMenu: HMENU) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetTimer(
    _ctx: &mut Context,
    _hWnd: HWND,
    _nIDEvent: u32,
    _uElapse: u32,
    _lpTimerFunc: u32, /* TIMERPROC */
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TranslateAcceleratorW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hAccTable: HACCEL,
    _lpMsg: u32, /* MSG */
) -> i32 {
    todo!()
}

// XXX: cdecl
#[win32_derive::dllexport]
pub fn wsprintfW(
    _ctx: &mut Context,
    _param0: u32, /* WSTR */
    _param1: u32, /* WSTR */
) -> i32 {
    todo!()
}
