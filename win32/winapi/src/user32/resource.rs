use runtime::*;
use zerocopy::FromBytes;

use super::*;
use crate::{
    dllexport::win32flags,
    gdi32::{self, BitmapType},
    handle::HANDLE,
    kernel32, stub,
};

#[win32_derive::dllexport]
pub fn LoadCursorA(_ctx: &mut Context, _hInstance: HINSTANCE, _lpCursorName: u32) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconA(_ctx: &mut Context, _hInstance: HINSTANCE, _lpIconName: u32) -> HICON {
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

fn find_resource<'ctx>(
    ctx: &'ctx Context,
    typ: pe::ResourceName,
    name: pe::ResourceName,
) -> Option<&'ctx [u8]> {
    let state = kernel32::lock();
    let section = &ctx.memory[state.resources.clone()];
    let span = pe::find_resource(section, typ, name)?;
    let image_base = state.image_base;
    Some(&ctx.memory[image_base + span.start..image_base + span.end])
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    ctx: &mut Context,
    hInst: HINSTANCE,
    name: u32,
    typ: IMAGE,
    cx: u32,
    cy: u32,
    fuLoad: LR,
) -> HANDLE {
    assert!(hInst == 0);

    assert!(is_intresource(name));
    let name = pe::ResourceName::Id(name);

    assert!(typ == IMAGE::BITMAP);
    let typ = pe::ResourceName::Id(match typ {
        IMAGE::CURSOR => pe::RT::CURSOR,
        IMAGE::BITMAP => pe::RT::BITMAP,
        IMAGE::ICON => pe::RT::ICON,
    } as u32);

    // assert!(cx == 0);
    // assert!(cy == 0);
    assert!(fuLoad.is_empty());

    let Some(buf) = find_resource(ctx, typ, name) else {
        log::warn!("LoadImage: resource not found");
        return HANDLE::null();
    };
    let bitmap = gdi32::parse_bitmap(buf);

    let BitmapType::DDB(ddb) = &bitmap.typ else {
        unreachable!()
    };
    assert_eq!(ddb.width, cx);
    assert_eq!(ddb.height, cy);

    bitmap.handle
}

pub type HACCEL = u32;
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn LoadAcceleratorsW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpTableName: u32, /* WSTR */
) -> HACCEL {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadCursorW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpCursorName: u32, /* WSTR */
) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpIconName: u32, /* WSTR */
) -> HICON {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadMenuW(
    _ctx: &mut Context,
    _hInstance: HINSTANCE,
    _lpMenuName: u32, /* WSTR */
) -> HMENU {
    stub!(0)
}

fn find_string(ctx: &Context, uID: u32) -> Option<&[u8]> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let mut block = find_resource(
        ctx,
        pe::ResourceName::Id(pe::RT::STRING as u32),
        pe::ResourceName::Id(resource_id),
    )?;

    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    for _ in 0..index {
        let len = <u16>::read_from_prefix(block).unwrap().0;
        block = &block[(1 + len as usize) * 2..];
    }
    let (len, rest) = <u16>::read_from_prefix(block).unwrap();
    Some(&rest[..len as usize * 2])
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    ctx: &mut Context,
    hInstance: HINSTANCE,
    uID: u32,
    lpBuffer: u32, /* WSTR */
    cchBufferMax: i32,
) -> i32 {
    assert_eq!(hInstance, 0);
    assert!(cchBufferMax > 0);
    let Some(bytes) = find_string(ctx, uID) else {
        panic!();
    };
    let buf = Vec::from(bytes);
    let out = &mut ctx.memory[lpBuffer..][..cchBufferMax as usize * 2];
    // TODO: handle case where buf.len() > cchBufferMax
    out[..buf.len()].copy_from_slice(&buf);
    buf.len() as i32 / 2
}
