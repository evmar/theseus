use runtime::*;

use super::*;
use crate::{Ptr, dllexport::win32flags, gdi32, handle::HANDLE, kernel32, stub};

#[win32_derive::dllexport]
pub fn LoadCursorA(_ctx: &mut Context, _hInstance: HINSTANCE, _lpCursorName: Ptr<u8>) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconA(_ctx: &mut Context, _hInstance: HINSTANCE, _lpIconName: Ptr<u8>) -> HICON {
    stub!(0)
}

#[derive(Debug, PartialEq, Eq, win32_derive::ABIEnum)]
pub enum IMAGE {
    BITMAP = 0,
    ICON = 1,
    CURSOR = 2,
}

win32flags! {
    pub struct LR {
        // TODO: add flags
    }
}

fn is_intresource(x: u32) -> bool {
    x >> 16 == 0
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    ctx: &mut Context,
    hInst: HINSTANCE,
    name: Ptr<u8>,
    typ: IMAGE,
    cx: u32,
    cy: u32,
    fuLoad: LR,
) -> HANDLE {
    assert!(hInst == 0);

    assert!(is_intresource(name.addr));
    let name = exe::ResourceName::Id(name.addr);

    assert!(typ == IMAGE::BITMAP);
    let typ = exe::ResourceName::Id(match typ {
        IMAGE::CURSOR => exe::RT::CURSOR,
        IMAGE::BITMAP => exe::RT::BITMAP,
        IMAGE::ICON => exe::RT::ICON,
    } as u32);

    // assert!(cx == 0);
    // assert!(cy == 0);
    assert!(fuLoad.is_empty());

    let Some(buf) = kernel32::lock().find_resource(ctx, typ, name) else {
        log::warn!("LoadImage: resource not found");
        return HANDLE::null();
    };
    let (mut bitmap, pixels) = gdi32::Bitmap::parse(buf);
    assert_eq!(bitmap.width, cx);
    assert_eq!(bitmap.height, cy);

    let pixels = unsafe { pixels.as_ptr().offset_from_unsigned(ctx.memory.as_ptr()) };
    bitmap.pixels = pixels as u32;

    gdi32::lock().new_bitmap_handle(bitmap).0
}

pub type HCURSOR = u32;
pub type HICON = u32;
pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn LoadAcceleratorsW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTableName: Ptr<u16>, /* WSTR */
) -> HACCEL {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadCursorW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpCursorName: Ptr<u16>, /* WSTR */
) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpIconName: Ptr<u16>, /* WSTR */
) -> HICON {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadMenuW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpMenuName: Ptr<u16>, /* WSTR */
) -> HMENU {
    stub!(0)
}

fn find_string(ctx: &Context, uID: u32) -> Option<&[u8]> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let mut block = kernel32::lock().find_resource(
        ctx,
        exe::ResourceName::Id(exe::RT::STRING as u32),
        exe::ResourceName::Id(resource_id),
    )?;

    use zerocopy::FromBytes;
    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    for i in 0.. {
        let (len, rest) = <u16>::read_from_prefix(block).unwrap();
        let (cur, next) = rest.split_at(len as usize * 2);
        if i == index {
            return Some(cur);
        }
        block = next;
    }
    unreachable!()
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    ctx: &mut Context,
    hInstance: HINSTANCE,
    uID: u32,
    lpBuffer: Ptr<u16>, /* WSTR */
    cchBufferMax: i32,
) -> i32 {
    assert_eq!(hInstance, 0);
    assert!(cchBufferMax > 0);
    let Some(bytes) = find_string(ctx, uID) else {
        panic!();
    };
    let buf = Vec::from(bytes);
    let out = &mut ctx.memory[lpBuffer.addr..][..cchBufferMax as usize * 2];
    // TODO: handle case where buf.len() > cchBufferMax
    out[..buf.len()].copy_from_slice(&buf);
    buf.len() as i32 / 2
}
