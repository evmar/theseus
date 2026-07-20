use runtime::Context;

#[win32_derive::dllexport]
pub fn CoInitialize(_ctx: &mut Context, _pvReserved: u32) -> u32 /* HRESULT */ {
    0 // S_OK
}

#[win32_derive::dllexport]
pub fn CoUninitialize(_ctx: &mut Context) {}

#[win32_derive::dllexport]
pub fn CoCreateInstance(
    _ctx: &mut Context,
    _rclsid: u32,
    _pUnkOuter: u32,
    _dwClsContext: u32,
    _riid: u32,
    _ppv: u32,
) -> u32 /* HRESULT */ {
    todo!("CoCreateInstance")
}
