use runtime::Context;

use crate::stub;

#[win32_derive::dllexport]
pub fn waveOutOpen(
    _ctx: &mut Context,
    _phwo: u32,
    _uDeviceID: u32,
    _pwfx: u32,
    _dwCallback: u32,
    _dwInstance: u32,
    _fdwOpen: u32, /* MIDI_WAVE_OPEN_TYPE */
) -> u32 {
    const MMSYSERR_NODRIVER: u32 = 0x00000002;
    stub!(MMSYSERR_NODRIVER)
}

#[win32_derive::dllexport]
pub fn waveOutReset(_ctx: &mut Context, _hwo: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutClose(_ctx: &mut Context, _hwo: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_ctx: &mut Context) -> u32 {
    stub!(0)
}

// module: USER32.dll

#[win32_derive::dllexport]
pub fn waveOutUnprepareHeader(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    todo!()
}
#[win32_derive::dllexport]
pub fn waveOutWrite(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    todo!()
}
#[win32_derive::dllexport]
pub fn waveOutPrepareHeader(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    todo!()
}
#[win32_derive::dllexport]
pub fn waveOutGetDevCapsA(_ctx: &mut Context, _uDeviceID: u32, _pwoc: u32, _cbwoc: u32) -> u32 {
    todo!()
}
