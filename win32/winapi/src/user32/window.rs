use std::{cell::RefCell, rc::Rc};

use runtime::Context;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    FromABIParam, RECT,
    gdi32::{self, HBRUSH, HDC},
    kernel32, stub,
    user32::{HCURSOR, HICON, HINSTANCE, HMENU, HWND, State, state},
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

    pub fn rect(&self) -> RECT {
        RECT {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        }
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
        let hwnd = HWND::from_raw(1);
        *self.window.borrow_mut() = Some(Rc::new(RefCell::new(window)));
        self.message_queue.borrow_mut().hwnd = hwnd;
        stub!(hwnd)
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
pub fn MoveWindow(
    _ctx: &mut Context,
    _hWnd: HWND,
    _X: i32,
    _Y: i32,
    nWidth: i32,
    nHeight: i32,
    bRepaint: bool,
) -> bool {
    let state = state();
    let window = state.window.borrow();
    let mut window = window.as_ref().unwrap().borrow_mut();
    window.resize(nWidth as u32, nHeight as u32);
    if bRepaint {
        // ...
    };
    true // sucess
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
    0
}

#[win32_derive::dllexport]
pub fn SetFocus(_ctx: &mut Context, _hWnd: HWND) -> HWND {
    stub!(HWND::null())
}

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes)]
struct WNDCLASS {
    style: u32,       /* WNDCLASS_STYLES */
    lpfnWndProc: u32, /* WNDPROC */
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: u32,
    lpszClassName: u32,
}

pub struct WndClass {
    pub wndproc: runtime::Cont,
    pub background: Option<gdi32::Brush>,
}

impl State {
    pub fn register_class(&self, wnd_class: WndClass) -> u16 {
        *self.wndclass.borrow_mut() = Some(wnd_class);
        0
    }
}

#[win32_derive::dllexport]
pub fn RegisterClassA(ctx: &mut Context, lpWndClass: u32) -> u16 {
    RegisterClassW(ctx, lpWndClass)
}

#[win32_derive::dllexport]
pub fn RegisterClassW(ctx: &mut Context, lpWndClass: u32 /* WNDCLASSW */) -> u16 {
    let wndclass = <WNDCLASS>::read_from_prefix(&ctx.memory[lpWndClass..])
        .unwrap()
        .0;
    let background = if wndclass.hbrBackground.is_null() {
        None
    } else {
        Some(
            gdi32::lock()
                .objects
                .get(wndclass.hbrBackground)
                .unwrap()
                .unwrap_brush(),
        )
    };
    state().register_class(WndClass {
        wndproc: ctx.indirect(wndclass.lpfnWndProc),
        background,
    });
    stub!(1)
}

#[repr(C)]
#[derive(Debug, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::FromBytes)]
struct PAINTSTRUCT {
    hdc: HDC,
    fErase: u32,
    rcPaint: RECT,
    reserved: [u32; 10],
}

#[win32_derive::dllexport]
pub fn BeginPaint(ctx: &mut Context, hWnd: HWND, lpPaint: u32 /* PAINTSTRUCT */) -> HDC {
    let wndclass = state().wndclass.borrow();
    let wndclass = wndclass.as_ref().unwrap();
    if wndclass.background.is_some() {
        // TODO: send WM_ERASEBKGND, let DefWindowProc handle it
    };

    let window = state().window.borrow();
    let window = window.as_ref().unwrap().borrow();

    let hdc = GetDC(ctx, hWnd);
    PAINTSTRUCT {
        hdc,
        fErase: wndclass.background.is_none() as u32,
        rcPaint: window.rect(),
        reserved: [0; 10],
    }
    .write_to_prefix(&mut ctx.memory[lpPaint..])
    .unwrap();
    hdc
}

#[win32_derive::dllexport]
pub fn EndPaint(ctx: &mut Context, _hWnd: HWND, lpPaint: u32 /* PAINTSTRUCT */) -> bool {
    let paint = <PAINTSTRUCT>::read_from_prefix(&ctx.memory[lpPaint..])
        .unwrap()
        .0;
    gdi32::lock().release_dc(paint.hdc);
    true
}

#[win32_derive::dllexport]
pub fn GetDC(ctx: &mut Context, hWnd: HWND) -> HDC {
    if hWnd.is_null() {
        // desktop window
        return stub!(HDC::null());
    }

    let state = state();
    let window = state.window.borrow();
    let window = window.as_ref().unwrap().borrow();

    let pixels = kernel32::lock()
        .process_heap
        .alloc(&mut ctx.memory, window.width * window.height * 4);
    let bitmap = gdi32::Bitmap::new_simple(window.width, window.height, pixels);
    gdi32::lock().new_memory_dc(bitmap)
}

#[win32_derive::dllexport]
pub fn ReleaseDC(_ctx: &mut Context, _hWnd: HWND, hDC: HDC) -> i32 {
    gdi32::lock().release_dc(hDC);
    1 // success
}

#[win32_derive::dllexport]
pub fn InvalidateRect(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpRect: u32, /* RECT */
    _bErase: bool,
) -> bool {
    stub!(true)
}
