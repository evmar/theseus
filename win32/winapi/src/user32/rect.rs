use runtime::Context;
use zerocopy::{FromBytes, IntoBytes};

use crate::{POINT, RECT};

#[win32_derive::dllexport]
pub fn PtInRect(ctx: &mut Context, lprc: u32 /* RECT */, x: i32, y: i32) -> bool {
    let rect = RECT::read_from_prefix(&ctx.memory[lprc..]).unwrap().0;
    let point = POINT { x, y };
    rect.contains(point)
}

#[win32_derive::dllexport]
pub fn SetRect(
    ctx: &mut Context,
    lprc: u32, /* RECT */
    xLeft: i32,
    yTop: i32,
    xRight: i32,
    yBottom: i32,
) -> bool {
    RECT {
        left: xLeft,
        top: yTop,
        right: xRight,
        bottom: yBottom,
    }
    .write_to_prefix(&mut ctx.memory[lprc..])
    .unwrap();
    true
}
