use crate::stub;
use runtime::Machine;

const fn make_dhsresult(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

const DSERR_NODRIVER: u32 = make_dhsresult(120);

#[win32_derive::dllexport]
pub fn DirectSoundCreate(_m: &mut Machine, _lpGuid: u32, _ppDS: u32, _pUnkOuter: u32) -> u32 {
    stub!(DSERR_NODRIVER)
}

#[win32_derive::dllexport]
pub fn ordinal1(m: &mut Machine, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    DirectSoundCreate(m, lpGuid, ppDS, pUnkOuter)
}
