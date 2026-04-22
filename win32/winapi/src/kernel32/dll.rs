use runtime::Context;

use crate::{kernel32::lock, stub};

pub type HMODULE = u32;

/// DLLLoader provides LoadLibrary and GetProcAddress implementations,
/// with a default impl that just fails.
pub trait DLLLoader: Send {
    fn load_library(&mut self, filename: &str) -> HMODULE;
    fn get_proc_address(&mut self, hmodule: HMODULE, proc_name: &str) -> u32;
}

impl DLLLoader for () {
    fn load_library(&mut self, filename: &str) -> HMODULE {
        log::warn!("LoadLibrary({filename}): not supported, returning null");
        0
    }

    fn get_proc_address(&mut self, hmodule: HMODULE, proc_name: &str) -> u32 {
        log::warn!("GetProcAddress({hmodule:#x}, {proc_name}): not supported, returning null");
        0
    }
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    _ctx: &mut Context,
    _hModule: HMODULE,
    _lpFilename: u32,
    _nSize: u32,
) -> u32 {
    /*
    get_module_file_name(sys, hModule, &mut EncoderAnsi::new(&mut filename))
    */
    stub!(0)
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(_ctx: &mut Context, _lpModuleName: u32) -> HMODULE {
    stub!(0)
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
pub fn LoadLibraryA(ctx: &mut Context, lpLibFileName: u32) -> HMODULE {
    let filename = ctx.memory.read_str(lpLibFileName);
    lock().dll_loader.load_library(&filename)
}

#[win32_derive::dllexport]
pub fn GetProcAddress(ctx: &mut Context, hModule: HMODULE, lpProcName: u32) -> u32 {
    let name = if lpProcName < 0x1000 {
        format!("ordinal{}", lpProcName)
    } else {
        ctx.memory.read_str(lpProcName).to_owned()
    };
    lock().dll_loader.get_proc_address(hModule, &name)
}
