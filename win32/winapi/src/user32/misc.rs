use super::*;
use crate::stub;
use runtime::*;

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
