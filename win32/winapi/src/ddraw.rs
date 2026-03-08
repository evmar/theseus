use crate::ABIReturn;
use runtime::*;

pub type HRESULT = u32;

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(_lpGuid: u32, _lplpDD: u32, _iid: u32, _pUnkOuter: u32) -> HRESULT {
    todo!()
}
