use runtime::*;

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

    let span = {
        let state = kernel32::lock();
        let section = &ctx.memory[state.resources.clone()];
        let Some(span) = pe::find_resource(section, typ, name) else {
            log::warn!("LoadImage: resource not found");
            return HANDLE::null();
        };
        let image_base = state.image_base;
        image_base + span.start..image_base + span.end
    };

    let buf = &ctx.memory[span];
    let bitmap = gdi32::parse_bitmap(buf);

    let BitmapType::DDB(ddb) = &bitmap.typ else {
        unreachable!()
    };
    assert_eq!(ddb.width, cx);
    assert_eq!(ddb.height, cy);

    bitmap.handle
}
