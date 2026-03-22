use crate::stub;

#[win32_derive::dllexport]
pub fn timeSetEvent(
    _uDelay: u32,
    _uResolution: u32,
    _fptc: u32, /* LPTIMECALLBACK */
    _dwUser: u32,
    _fuEvent: u32,
) -> u32 {
    stub!(1)
}

#[win32_derive::dllexport]
pub fn timeKillEvent(_uTimerID: u32) -> u32 {
    todo!()
}
