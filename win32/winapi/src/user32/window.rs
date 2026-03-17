use std::rc::Rc;

use runtime::MACHINE;

use crate::{
    stub,
    user32::{HINSTANCE, HMENU, HWND, state},
};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub canvas: sdl3::render::WindowCanvas,
}

#[win32_derive::dllexport]
pub fn CreateWindowExA(
    _dwExStyle: u32, /* WINDOW_EX_STYLE */
    _lpClassName: u32,
    lpWindowName: u32,
    _dwStyle: u32, /* WINDOW_STYLE */
    _X: i32,
    _Y: i32,
    nWidth: i32,
    nHeight: i32,
    _hWndParent: HWND,
    _hMenu: HMENU,
    _hInstance: HINSTANCE,
    _lpParam: u32,
) -> HWND {
    let name = unsafe { MACHINE.memory.read_str(lpWindowName) };

    const CW_USEDEFAULT: i32 = 0x8000_0000u32 as i32;
    let width = if nWidth == CW_USEDEFAULT {
        640
    } else {
        nWidth as u32
    };
    let height = if nHeight == CW_USEDEFAULT {
        480
    } else {
        nHeight as u32
    };

    let mut window = Window {
        width,
        height,
        canvas: state()
            .video
            .window(name, width, height)
            .build()
            .unwrap()
            .into_canvas(),
    };
    window.canvas.clear();
    window.canvas.set_draw_color(sdl3::pixels::Color::GREEN);
    window
        .canvas
        .fill_rect(sdl3::render::FRect::new(10.0, 10.0, 100.0, 100.0))
        .unwrap();
    window.canvas.present();

    *state().window.borrow_mut() = Some(Rc::new(window));
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
