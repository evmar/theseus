use super::*;
use crate::{bitmap::BitmapInfo, dllexport::win32flags, kernel32, stub};
use runtime::*;

#[win32_derive::dllexport]
pub fn LoadCursorA(_hInstance: HINSTANCE, _lpCursorName: u32) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn LoadIconA(_hInstance: HINSTANCE, _lpIconName: u32) -> HICON {
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
    hInst: HINSTANCE,
    name: u32,
    typ: IMAGE,
    _cx: i32,
    _cy: i32,
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

    let section = unsafe { MACHINE.memory.slice(kernel32::state().resources.clone()) };
    let Some(span) = pe::find_resource(section, typ, name) else {
        log::warn!("LoadImage: resource not found");
        return 0;
    };
    let image_base = kernel32::state().image_base;
    let span = image_base + span.start..image_base + span.end;
    log::warn!("found image at {:x?}", span);

    let buf = unsafe { MACHINE.memory.slice(span) };
    let hdr = BitmapInfo::parse(buf);
    println!("loaded bitmap {:#x?}", hdr);

    stub!(0)
}
