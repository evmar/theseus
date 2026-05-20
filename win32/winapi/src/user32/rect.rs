use runtime::Context;

use crate::{POINT, Ptr, RECT};

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
