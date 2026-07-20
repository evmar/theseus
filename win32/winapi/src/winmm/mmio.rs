//! mmio*: RIFF (WAV) file reading, plus MCI. Currently stubs; see the
//! retrowin32 fork's winmm mmio implementation for a reference port.

use runtime::Context;

#[win32_derive::dllexport]
pub fn mciSendCommandA(
    _ctx: &mut Context,
    _mciId: u32,
    _uMsg: u32,
    _dwParam1: u32,
    _dwParam2: u32,
) -> u32 {
    // CD audio etc.; pretend success and play nothing.
    crate::stub!(0)
}

#[win32_derive::dllexport]
pub fn mmioOpenA(
    _ctx: &mut Context,
    _szFilename: u32,
    _lpmmioinfo: u32,
    _dwOpenFlags: u32,
) -> u32 {
    todo!("mmioOpenA")
}

#[win32_derive::dllexport]
pub fn mmioClose(_ctx: &mut Context, _hmmio: u32, _wFlags: u32) -> u32 {
    todo!("mmioClose")
}

#[win32_derive::dllexport]
pub fn mmioDescend(
    _ctx: &mut Context,
    _hmmio: u32,
    _lpck: u32,
    _lpckParent: u32,
    _wFlags: u32,
) -> u32 {
    todo!("mmioDescend")
}

#[win32_derive::dllexport]
pub fn mmioAscend(_ctx: &mut Context, _hmmio: u32, _lpck: u32, _wFlags: u32) -> u32 {
    todo!("mmioAscend")
}

#[win32_derive::dllexport]
pub fn mmioRead(_ctx: &mut Context, _hmmio: u32, _pch: u32, _cch: u32) -> i32 {
    todo!("mmioRead")
}

#[win32_derive::dllexport]
pub fn mmioSeek(_ctx: &mut Context, _hmmio: u32, _lOffset: i32, _iOrigin: i32) -> i32 {
    todo!("mmioSeek")
}

#[win32_derive::dllexport]
pub fn mmioGetInfo(_ctx: &mut Context, _hmmio: u32, _lpmmioinfo: u32, _wFlags: u32) -> u32 {
    todo!("mmioGetInfo")
}

#[win32_derive::dllexport]
pub fn mmioSetInfo(_ctx: &mut Context, _hmmio: u32, _lpmmioinfo: u32, _wFlags: u32) -> u32 {
    todo!("mmioSetInfo")
}

#[win32_derive::dllexport]
pub fn mmioAdvance(_ctx: &mut Context, _hmmio: u32, _lpmmioinfo: u32, _wFlags: u32) -> u32 {
    todo!("mmioAdvance")
}
