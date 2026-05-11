use runtime::Context;

use crate::stub;

#[win32_derive::dllexport]
pub fn _XcptFilter(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn __getmainargs(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn __p__commode(_ctx: &mut Context) -> u32 {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn __p__fmode(_ctx: &mut Context) -> u32 {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn __set_app_type(_ctx: &mut Context, _at: i32) {}

#[win32_derive::dllexport]
pub fn __setusermatherr(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _acmdln(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _adjust_fdiv(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _controlfp(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _except_handler3(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _exit(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn _initterm(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn exit(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn rand(_ctx: &mut Context) {
    todo!()
}

#[win32_derive::dllexport]
pub fn srand(_ctx: &mut Context) {
    todo!()
}
