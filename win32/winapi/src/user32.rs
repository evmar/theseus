use runtime::*;

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
    todo!()
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_hInstance: HINSTANCE, _lpCursorName: u32) -> HCURSOR {
    todo!()
}

#[win32_derive::dllexport]
pub fn LoadIconA(_hInstance: HINSTANCE, _lpIconName: u32) -> HICON {
    todo!()
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
    todo!()
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
    todo!()
}

#[win32_derive::dllexport]
pub fn SetFocus(_hWnd: HWND) -> HWND {
    todo!()
}

#[win32_derive::dllexport]
pub fn ShowWindow(_hWnd: HWND, _nCmdShow: u32 /* SHOW_WINDOW_CMD */) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_hWnd: HWND) -> bool {
    todo!()
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
    todo!()
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_nIndex: u32 /* SYSTEM_METRICS_INDEX */) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn DefWindowProcA(_hWnd: HWND, _Msg: u32, _wParam: u32, _lParam: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_nExitCode: i32) {
    todo!()
}
