use runtime::Context;

use crate::{Ptr, kernel32::lock, stub};

pub type HMODULE = u32;

/// DLLs provides LoadLibrary and GetProcAddress implementations.
/// It's a trait so it can be hooked by unpackers that want to implement custom logic.
pub trait DLLs: Send {
    /// Register a function as available through GetProcAddress. Called from
    /// generated init code, once per function the translator reserved an
    /// address for.
    fn register_export(&mut self, dll: &str, func: &str, addr: u32);

    fn load_library(&mut self, filename: &str) -> HMODULE;
    fn module_handle(&self, dll: &str) -> Option<HMODULE>;
    fn get_proc_address(&mut self, hmodule: HMODULE, proc_name: &str) -> u32;
}

/// Default implementation of DLLs.
/// Functions the translated program can look up by name at runtime.
///
/// A statically linked import is dispatched by the translator, but a program
/// that goes through LoadLibrary/GetProcAddress needs an actual address to
/// call. The translator picks one per function (the same synthetic addresses it
/// uses for imports, so they resolve through the block table) and registers it
/// here from the generated init code.
#[derive(Default)]
pub struct Exports {
    /// (dll, function) -> address, with dll lowercased and without ".dll".
    functions: Vec<(String, String, u32)>,
    /// Module handles handed out by LoadLibrary, in registration order.
    modules: Vec<String>,
}

/// Module handles are synthetic; the value only has to be non-null and
/// distinguishable.
const MODULE_HANDLE_BASE: HMODULE = 0xd11_0000;

fn normalize_module_name(name: &str) -> String {
    let name = name.rsplit(['\\', '/']).next().unwrap();
    // Lowercase first: stripping ".dll" before folding case misses "Foo.Dll".
    let name = name.to_ascii_lowercase();
    name.strip_suffix(".dll").unwrap_or(&name).to_string()
}

impl DLLs for Exports {
    /// Register a function as available through GetProcAddress. Called from
    /// generated init code, once per function the translator reserved an
    /// address for.
    fn register_export(&mut self, dll: &str, func: &str, addr: u32) {
        let dll = normalize_module_name(dll);
        if !self.modules.contains(&dll) {
            self.modules.push(dll.clone());
        }
        self.functions.push((dll, func.to_string(), addr));
    }

    fn load_library(&mut self, filename: &str) -> HMODULE {
        let Some(hmodule) = self.module_handle(filename) else {
            return 0;
        };
        hmodule
    }

    fn module_handle(&self, name: &str) -> Option<HMODULE> {
        let name = normalize_module_name(name);
        let index = self.modules.iter().position(|module| *module == name)?;
        Some(MODULE_HANDLE_BASE + index as u32)
    }

    fn get_proc_address(&mut self, hmodule: HMODULE, proc_name: &str) -> u32 {
        let Some(index) = hmodule.checked_sub(MODULE_HANDLE_BASE) else {
            return 0;
        };
        let Some(module) = self.modules.get(index as usize) else {
            return 0;
        };
        let Some((_, _, addr)) = self
            .functions
            .iter()
            .find(|(dll, name, _)| dll == module && name == proc_name)
        else {
            return 0;
        };

        *addr
    }
}

/// Register a function as available through GetProcAddress; the entry point
/// the generated init code calls.
pub fn register_export(dll: &str, func: &str, addr: u32) {
    lock().dlls.register_export(dll, func, addr);
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    _ctx: &mut Context,
    _hModule: HMODULE,
    _lpFilename: Ptr<u8>,
    _nSize: u32,
) -> u32 {
    /*
    get_module_file_name(sys, hModule, &mut EncoderAnsi::new(&mut filename))
    */
    stub!(0)
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(ctx: &mut Context, lpModuleName: Ptr<u8>) -> HMODULE {
    let kernel32 = lock();
    let Some(name) =
        (lpModuleName.addr != 0).then(|| ctx.memory.read_str(lpModuleName.addr).to_owned())
    else {
        // A null name asks for the running executable itself.
        return kernel32.image_base;
    };
    match kernel32.dlls.module_handle(&name) {
        Some(handle) => handle,
        None => {
            log::warn!("GetModuleHandleA({name}): not loaded");
            0
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(ctx: &mut Context, lpLibFileName: Ptr<u8>) -> HMODULE {
    let filename = ctx.memory.read_str(lpLibFileName.addr).to_owned();
    let addr = lock().dlls.load_library(&filename);
    if addr == 0 {
        log::warn!("LoadLibrary({filename}): not supported, returning null");
    }
    addr
}

#[win32_derive::dllexport]
pub fn FreeLibrary(_ctx: &mut Context, _hLibModule: HMODULE) -> bool {
    // Our modules are always resident.
    true
}

#[win32_derive::dllexport]
pub fn GetProcAddress(ctx: &mut Context, hModule: HMODULE, lpProcName: Ptr<u8>) -> u32 {
    // A name below 0x1000 is really an ordinal, per the API's convention.
    let name = if lpProcName.addr < 0x1000 {
        format!("ordinal{}", lpProcName.addr)
    } else {
        ctx.memory.read_str(lpProcName.addr).to_owned()
    };
    let addr = lock().dlls.get_proc_address(hModule, &name);
    if addr == 0 {
        log::warn!("GetProcAddress({hModule:#x}, {name}): not supported, returning null");
    }
    addr
}
