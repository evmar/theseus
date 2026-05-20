use runtime::Context;

use super::*;
use crate::{Ptr, RECT, stub};

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
pub fn CreateCursor(
    _ctx: &mut Context,
    _hInst: HINSTANCE,
    _xHotSpot: i32,
    _yHotSpot: i32,
    _nWidth: i32,
    _nHeight: i32,
    _pvANDPlane: Ptr<u8>,
    _pvXORPlane: Ptr<u8>,
) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn ReleaseCapture(_ctx: &mut Context) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn SetCapture(_ctx: &mut Context, _hWnd: HWND) -> HWND {
    stub!(HWND::null())
}

#[win32_derive::dllexport]
pub fn WinHelpW(
    _ctx: &mut Context,
    _hWndMain: HWND,
    _lpszHelp: Ptr<u16>, /* WSTR */
    _uCommand: u32,
    _dwData: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(_ctx: &mut Context, _hMenu: HMENU, _uIDCheckItem: u32, _uCheck: u32) -> u32 {
    stub!(0) // previously unchecked
}

pub type LRESULT = i32;

#[win32_derive::dllexport]
pub fn GetMenuItemRect(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hMenu: HMENU,
    _uItem: u32,
    _lprcItem: Ptr<RECT>,
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
    _lpText: Ptr<u16>,    /* WSTR */
    _lpCaption: Ptr<u16>, /* WSTR */
    _uType: u32,          /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn SetMenu(_ctx: &mut Context, _hWnd: HWND, _hMenu: HMENU) -> bool {
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn SetTimer(
    _ctx: &mut Context,
    _hWnd: HWND,
    _nIDEvent: u32,
    _uElapse: u32,
    _lpTimerFunc: Ptr<()>, /* TIMERPROC */
) -> u32 {
    stub!(0) // fail
}

// XXX: cdecl
#[win32_derive::dllexport]
pub fn wsprintfW(
    _ctx: &mut Context,
    _param0: Ptr<u16>, /* WSTR */
    _param1: Ptr<u16>, /* WSTR */
) -> i32 {
    todo!()
}
