use runtime::Context;

use crate::stub;

pub type HKEY = u32;

const ERROR_FILE_NOT_FOUND: u32 = 2;

#[win32_derive::dllexport]
pub fn RegCloseKey(_ctx: &mut Context, _hKey: HKEY) -> u32 /* WIN32_ERROR */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn RegCreateKeyExW(
    _ctx: &mut Context,
    _hKey: HKEY,
    _lpSubKey: u32, /* WSTR */
    _Reserved: u32,
    _lpClass: u32,              /* WSTR */
    _dwOptions: u32,            /* REG_OPEN_CREATE_OPTIONS */
    _samDesired: u32,           /* REG_SAM_FLAGS */
    _lpSecurityAttributes: u32, /* SECURITY_ATTRIBUTES */
    _phkResult: HKEY,
    _lpdwDisposition: u32, /* REG_CREATE_KEY_DISPOSITION */
) -> u32 /* WIN32_ERROR */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn RegOpenKeyExA(
    _ctx: &mut Context,
    _hKey: HKEY,
    _lpSubKey: u32, /* STR */
    _ulOptions: u32,
    _samDesired: u32, /* REG_SAM_FLAGS */
    _phkResult: HKEY,
) -> u32 /* WIN32_ERROR */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn RegQueryValueExA(
    _ctx: &mut Context,
    _hKey: HKEY,
    _lpValueName: u32, /* STR */
    _lpReserved: u32,
    _lpType: u32, /* REG_VALUE_TYPE */
    _lpData: u32,
    _lpcbData: u32,
) -> u32 /* WIN32_ERROR */ {
    stub!(ERROR_FILE_NOT_FOUND)
}

#[win32_derive::dllexport]
pub fn RegQueryValueExW(
    _ctx: &mut Context,
    _hKey: HKEY,
    _lpValueName: u32, /* WSTR */
    _lpReserved: u32,
    _lpType: u32, /* REG_VALUE_TYPE */
    _lpData: u32,
    _lpcbData: u32,
) -> u32 /* WIN32_ERROR */ {
    stub!(ERROR_FILE_NOT_FOUND)
}

#[win32_derive::dllexport]
pub fn RegSetValueExW(
    _ctx: &mut Context,
    _hKey: HKEY,
    _lpValueName: u32, /* WSTR */
    _Reserved: u32,
    _dwType: u32, /* REG_VALUE_TYPE */
    _lpData: u32,
    _cbData: u32,
) -> u32 /* WIN32_ERROR */ {
    stub!(0)
}
