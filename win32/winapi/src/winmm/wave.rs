use runtime::Context;

use crate::{FromABIParam, stub, winmm::state};

const MMSYSERR_NOERROR: u32 = 0;

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_ctx: &mut Context) -> u32 {
    1
}

#[repr(C)]
#[derive(Debug, zerocopy::Immutable, zerocopy::IntoBytes)]
pub struct WAVEOUTCAPS {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    // TODO: TCHAR, could this be unicode based on cbwoc param?
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}

enum WAVE_FORMAT {
    _4M16 = 0x0000_0400,
}

#[win32_derive::dllexport]
pub fn waveOutGetDevCapsA(ctx: &mut Context, _uDeviceID: u32, pwoc: u32, cbwoc: u32) -> u32 {
    assert_eq!(cbwoc, std::mem::size_of::<WAVEOUTCAPS>() as u32);

    ctx.memory.write(
        pwoc,
        WAVEOUTCAPS {
            wMid: 0,
            wPid: 0,
            vDriverVersion: 1,
            szPname: [0; 32],
            dwFormats: WAVE_FORMAT::_4M16 as u32,
            wChannels: 1, // mono
            wReserved1: 0,
            dwSupport: 0, // no features
        },
    );
    MMSYSERR_NOERROR
}

/// The types of callbacks that can be used with waveOutOpen.
#[derive(Debug, PartialEq, Eq, win32_derive::ABIEnum)]
pub enum CALLBACK {
    NULL = 0x00000000,
    WINDOW = 0x00010000,
    TASK = 0x00020000,
    FUNCTION = 0x00030000,
    EVENT = 0x00050000,
}

#[win32_derive::dllexport]
pub fn waveOutOpen(
    ctx: &mut Context,
    phwo: u32,
    _uDeviceID: u32,
    _pwfx: u32,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: u32,
) -> u32 {
    if fdwOpen & !0x000F_0000 != 0 {
        todo!("{fdwOpen:x?}");
    }

    let callback = CALLBACK::from_abi(fdwOpen);
    match callback {
        CALLBACK::NULL => {}
        CALLBACK::FUNCTION => {
            state().wave_callback = Some((dwCallback, dwInstance));
        }
        _ => todo!("{callback:?}"),
    }

    ctx.memory.write::<u32>(phwo, 1);

    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutReset(_ctx: &mut Context, _hwo: u32) -> u32 {
    stub!(MMSYSERR_NOERROR)
}

#[win32_derive::dllexport]
pub fn waveOutClose(_ctx: &mut Context, _hwo: u32) -> u32 {
    stub!(MMSYSERR_NOERROR)
}

#[repr(C)]
pub struct WAVEHDR {
    lpData: u32,
    dwBufferLength: u32,
    dwBytesRecorded: u32,
    dwUser: u32,
    dwFlags: u32,
    dwLoops: u32,
    lpNext: u32,
    reserved: u32,
}

#[win32_derive::dllexport]
pub fn waveOutPrepareHeader(_ctx: &mut Context, _hwo: u32, _pwh: u32, cbwh: u32) -> u32 {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    // This function is supposed to fill in fields in the WAVEHDR, but there's nothing
    // for us to do here.
    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutUnprepareHeader(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutWrite(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    todo!()
}
