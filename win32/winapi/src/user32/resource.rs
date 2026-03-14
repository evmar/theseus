use super::*;
use crate::{dllexport::win32flags, stub};
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

#[win32_derive::dllexport]
pub fn LoadImageA(
    hInst: HINSTANCE,
    _name: u32,
    typ: IMAGE,
    _cx: i32,
    _cy: i32,
    _fuLoad: LR,
) -> HANDLE {
    assert!(hInst == 0);
    assert!(typ == IMAGE::BITMAP);
    stub!(0)
}
