use runtime::Context;

use crate::{HANDLE, kernel32::lock, stub};

pub enum Object {
    Thread,
    Event,
}

#[win32_derive::dllexport]
pub fn WaitForSingleObject(_ctx: &mut Context, _hHandle: HANDLE, _dwMilliseconds: u32) -> u32 /* WAIT_EVENT */
{
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    _ctx: &mut Context,
    _lpEventAttributes: u32,
    _bManualReset: bool,
    _bInitialState: bool,
    _lpName: u32,
) -> HANDLE {
    let mut kernel32 = lock();
    let handle = kernel32.objects.add(Object::Event);
    stub!(handle)
}

#[win32_derive::dllexport]
pub fn SetEvent(_ctx: &mut Context, _hEvent: HANDLE) -> bool {
    stub!(true)
}
