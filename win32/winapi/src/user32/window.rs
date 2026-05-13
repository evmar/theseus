use std::{cell::RefCell, rc::Rc};

use runtime::Context;

use crate::{
    FromABIParam,
    gdi32::{self, HDC},
    stub,
    user32::{HINSTANCE, HMENU, HWND, State, state},
};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub canvas: sdl3::render::WindowCanvas,
}

impl Window {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let window = self.canvas.window_mut();
        let scale = window.display_scale();
        window
            .set_size(
                (width as f32 * scale) as u32,
                (height as f32 * scale) as u32,
            )
            .unwrap();
    }
}

#[derive(Default)]
struct CreateWindowArgs {
    name: String,
    width: Option<u32>,
    height: Option<u32>,
}

const CW_USEDEFAULT: u32 = 0x8000_0000;

pub struct CW(u32);
impl CW {
    fn value(&self) -> Option<u32> {
        if self.0 == CW_USEDEFAULT {
            None
        } else {
            Some(self.0)
        }
    }
}
impl FromABIParam for CW {
    fn from_abi(val: u32) -> Self {
        Self(val)
    }
}
impl std::fmt::Debug for CW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == CW_USEDEFAULT {
            write!(f, "CW_USEDEFAULT")
        } else {
            write!(f, "{:#x}", self.0)
        }
    }
}

impl State {
    fn create_window(&self, args: CreateWindowArgs) -> HWND {
        let width = args.width.unwrap_or(640);
        let height = args.height.unwrap_or(480);

        let mut window = Window {
            width,
            height,
            canvas: state()
                .video
                .window(&args.name, width, height)
                .high_pixel_density()
                .build()
                .unwrap()
                .into_canvas(),
        };
        window.canvas.clear();
        *self.window.borrow_mut() = Some(Rc::new(RefCell::new(window)));
        stub!(HWND::from_raw(1))
    }
}

#[win32_derive::dllexport]
pub fn CreateWindowExA(
    ctx: &mut Context,
    _dwExStyle: u32, /* WINDOW_EX_STYLE */
    _lpClassName: u32,
    lpWindowName: u32,
    _dwStyle: u32, /* WINDOW_STYLE */
    _X: i32,
    _Y: i32,
    nWidth: CW,
    nHeight: CW,
    _hWndParent: HWND,
    _hMenu: HMENU,
    _hInstance: HINSTANCE,
    _lpParam: u32,
) -> HWND {
    let name = ctx.memory.read_str(lpWindowName);
    state().create_window(CreateWindowArgs {
        name: name.into(),
        width: nWidth.value(),
        height: nHeight.value(),
    })
}

#[win32_derive::dllexport]
pub fn CreateWindowExW(
    ctx: &mut Context,
    _dwExStyle: u32,   /* WINDOW_EX_STYLE */
    _lpClassName: u32, /* WSTR */
    lpWindowName: u32, /* WSTR */
    _dwStyle: u32,     /* WINDOW_STYLE */
    _X: i32,
    _Y: i32,
    nWidth: CW,
    nHeight: CW,
    _hWndParent: HWND,
    _hMenu: HMENU,
    _hInstance: HINSTANCE,
    _lpParam: u32,
) -> HWND {
    let name = ctx.memory.read_wstr(lpWindowName);
    state().create_window(CreateWindowArgs {
        name: name.to_string_lossy(),
        width: nWidth.value(),
        height: nHeight.value(),
    })
}

#[win32_derive::dllexport]
pub fn DestroyWindow(_ctx: &mut Context, _hWnd: HWND) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn ShowWindow(
    _ctx: &mut Context,
    _hWnd: HWND,
    _nCmdShow: u32, /* SHOW_WINDOW_CMD */
) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_ctx: &mut Context, _hWnd: HWND) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn DefWindowProcA(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: u32,
    _lParam: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetFocus(_ctx: &mut Context, _hWnd: HWND) -> HWND {
    stub!(HWND::null())
}

#[win32_derive::dllexport]
pub fn RegisterClassA(_ctx: &mut Context, _lpWndClass: u32) -> u16 {
    stub!(1)
}

#[win32_derive::dllexport]
pub fn RegisterClassW(_ctx: &mut Context, _lpWndClass: u32 /* WNDCLASSW */) -> u16 {
    stub!(1)
}

#[win32_derive::dllexport]
pub fn BeginPaint(_ctx: &mut Context, _hWnd: HWND, _lpPaint: u32 /* PAINTSTRUCT */) -> HDC {
    todo!()
}

#[win32_derive::dllexport]
pub fn EndPaint(_ctx: &mut Context, _hWnd: HWND, _lpPaint: u32 /* PAINTSTRUCT */) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDC(_ctx: &mut Context, hWnd: HWND) -> HDC {
    if hWnd.is_null() {
        // desktop window
        return stub!(HDC::null());
    }

    let state = state();
    let window = state.window.borrow();
    let window = window.as_ref().unwrap().borrow();

    let bitmap = gdi32::DIB::new(window.width, window.height);
    let hdc = gdi32::state()
        .dcs
        .borrow_mut()
        .add(gdi32::new_memory_dc(bitmap));
    stub!(hdc)
}

#[win32_derive::dllexport]
pub fn ReleaseDC(_ctx: &mut Context, _hWnd: HWND, hDC: HDC) -> i32 {
    gdi32::state().release_dc(hDC);
    1 // success
}
