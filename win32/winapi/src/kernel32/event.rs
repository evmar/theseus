use runtime::Context;

use crate::kernel32::HANDLE;

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
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEvent(_ctx: &mut Context, _hEvent: HANDLE) -> bool {
    todo!()
}
