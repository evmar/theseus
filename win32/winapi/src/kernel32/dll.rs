use crate::ABIReturn;
use runtime::{Cont, MACHINE};

pub type HMODULE = u32;

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(_hModule: HMODULE, _lpFilename: u32, _nSize: u32) -> u32 {
    /*
    get_module_file_name(sys, hModule, &mut EncoderAnsi::new(&mut filename))
    */
    0
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(_lpModuleName: u32) -> HMODULE {
    todo!();
    /*
    let state = get_state(sys);
        let Some(name) = lpModuleName else {
            return HMODULE::from_raw(state.image_base);
        };
        let name = normalize_module_name(name);

        let Some((&hmodule, _)) = state.modules.iter().find(|(_, dll)| dll.name == name) else {
            sys.set_last_error(ERROR::MOD_NOT_FOUND);
            return HMODULE::null();
        };

        hmodule
        */
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(_lpLibFileName: u32) -> HMODULE {
    0
    // load_library(sys, filename.unwrap()).await
}
