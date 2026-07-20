use runtime::Context;

use super::HWND;
use crate::{POINT, Ptr, RECT};

#[win32_derive::dllexport]
pub fn OffsetRect(ctx: &mut Context, lprc: Ptr<RECT>, dx: i32, dy: i32) -> bool {
    let Some(mut rect) = lprc.read(&ctx.memory) else {
        return false;
    };
    rect.left += dx;
    rect.right += dx;
    rect.top += dy;
    rect.bottom += dy;
    lprc.write(&mut ctx.memory, rect).is_some()
}

#[win32_derive::dllexport]
pub fn ClientToScreen(_ctx: &mut Context, _hWnd: HWND, _lpPoint: Ptr<POINT>) -> bool {
    // The window's client area sits at the screen origin.
    true
}

#[win32_derive::dllexport]
pub fn PtInRect(ctx: &mut Context, lprc: Ptr<RECT>, x: i32, y: i32) -> bool {
    let rect = lprc.read(&ctx.memory).unwrap();
    let point = POINT { x, y };
    rect.contains(point)
}

#[win32_derive::dllexport]
pub fn SetRect(
    ctx: &mut Context,
    lprc: Ptr<RECT>,
    xLeft: i32,
    yTop: i32,
    xRight: i32,
    yBottom: i32,
) -> bool {
    lprc.write(
        &mut ctx.memory,
        RECT {
            left: xLeft,
            top: yTop,
            right: xRight,
            bottom: yBottom,
        },
    )
    .unwrap();
    true
}
