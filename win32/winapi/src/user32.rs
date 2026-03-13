use runtime::*;

use crate::stub;

pub type HWND = u32;
pub type HMENU = u32;
pub type HINSTANCE = u32;
pub type HCURSOR = u32;
pub type HANDLE = u32;
pub type HICON = u32;

#[win32_derive::dllexport]
pub fn CreateWindowExA(
    _dwExStyle: u32, /* WINDOW_EX_STYLE */
    _lpClassName: u32,
    _lpWindowName: u32,
    _dwStyle: u32, /* WINDOW_STYLE */
    _X: i32,
    _Y: i32,
    _nWidth: i32,
    _nHeight: i32,
    _hWndParent: HWND,
    _hMenu: HMENU,
    _hInstance: HINSTANCE,
    _lpParam: u32,
) -> HWND {
    stub!(1)
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_hInstance: HINSTANCE, _lpCursorName: u32) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconA(_hInstance: HINSTANCE, _lpIconName: u32) -> HICON {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    _hInst: HINSTANCE,
    _name: u32,
    _type: u32, /* GDI_IMAGE_TYPE */
    _cx: i32,
    _cy: i32,
    _fuLoad: u32, /* IMAGE_FLAGS */
) -> HANDLE {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn MessageBoxA(
    _hWnd: HWND,
    _lpText: u32,
    _lpCaption: u32,
    _uType: u32, /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    todo!()
}

#[win32_derive::dllexport]
pub fn RegisterClassA(_lpWndClass: u32) -> u16 {
    stub!(1)
}

#[win32_derive::dllexport]
pub fn SetFocus(_hWnd: HWND) -> HWND {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn ShowWindow(_hWnd: HWND, _nCmdShow: u32 /* SHOW_WINDOW_CMD */) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_hWnd: HWND) -> bool {
    stub!(true)
}

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
pub fn GetSystemMetrics(nIndex: u32 /* SYSTEM_METRICS_INDEX */) -> i32 {
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
pub fn DefWindowProcA(_hWnd: HWND, _Msg: u32, _wParam: u32, _lParam: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_nExitCode: i32) {
    todo!()
}
