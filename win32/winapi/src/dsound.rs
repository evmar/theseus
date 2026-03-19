#[win32_derive::dllexport]
pub fn DirectSoundCreate(_lpGuid: u32, _ppDS: u32, _pUnkOuter: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn ordinal1(lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    DirectSoundCreate(lpGuid, ppDS, pUnkOuter)
}
