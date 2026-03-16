use runtime::*;
use winit::{
    event_loop, platform::pump_events::EventLoopExtPumpEvents as _, window::WindowAttributes,
};

use crate::{
    stub,
    user32::{HINSTANCE, HMENU, HWND, state},
};

struct H {}

pub struct Window {
    winit_window: winit::window::Window,
}

impl winit::application::ApplicationHandler for H {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();
        *state().window.borrow_mut() = Some(Window {
            winit_window: window,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        //dbg!(event);
    }
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
    let status = state()
        .event_loop
        .borrow_mut()
        .pump_app_events(None, &mut H {});
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
