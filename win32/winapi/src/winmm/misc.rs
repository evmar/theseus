use runtime::Context;

use crate::kernel32::HMODULE;

#[win32_derive::dllexport]
pub fn PlaySoundW(
    _ctx: &mut Context,
    _pszSound: u32, /* WSTR */
    _hmod: HMODULE,
    _fdwSound: u32, /* SND_FLAGS */
) -> bool {
    todo!()
}
