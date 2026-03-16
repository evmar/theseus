use runtime::*;

use crate::{
    stub,
    user32::{HINSTANCE, HMENU, HWND, state},
};

pub struct Window {
    pub winit_window: winit::window::Window,
}

pub fn create_pending_windows(event_loop: &winit::event_loop::ActiveEventLoop) {
    if state().window.borrow().is_some() {
        return;
    }

    let window = event_loop
        .create_window(winit::window::WindowAttributes::default())
        .unwrap();
    *state().window.borrow_mut() = Some(Window {
        winit_window: window,
    });
}

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
pub fn ShowWindow(_hWnd: HWND, _nCmdShow: u32 /* SHOW_WINDOW_CMD */) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_hWnd: HWND) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn DefWindowProcA(_hWnd: HWND, _Msg: u32, _wParam: u32, _lParam: u32) -> u32 {
    todo!()
}
