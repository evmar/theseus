use runtime::Machine;

use crate::HANDLE;

#[win32_derive::dllexport]
pub fn CreateThread(
    _m: &mut Machine,
    _lpThreadAttributes: u32,
    _dwStackSize: u32,
    _lpStartAddress: u32, /* LPTHREAD_START_ROUTINE */
    _lpParameter: u32,
    _dwCreationFlags: u32, /* THREAD_CREATION_FLAGS */
    _lpThreadId: u32,
) -> HANDLE {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(_m: &mut Machine) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TlsAlloc(_m: &mut Machine) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TlsGetValue(_m: &mut Machine, _dwTlsIndex: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TlsSetValue(_m: &mut Machine, _dwTlsIndex: u32, _lpTlsValue: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(
    _m: &mut Machine,
    _hThread: HANDLE,
    _nPriority: u32, /* THREAD_PRIORITY */
) -> bool {
    todo!()
}
