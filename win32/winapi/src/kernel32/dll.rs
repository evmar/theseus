use std::sync::Mutex;

use runtime::Context;

use crate::{Ptr, kernel32::lock, stub};

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

/// Functions the translated program can look up by name at runtime.
///
/// A statically linked import is dispatched by the translator, but a program
/// that goes through LoadLibrary/GetProcAddress needs an actual address to
/// call. The translator picks one per function (the same synthetic addresses it
/// uses for imports, so they resolve through the block table) and registers it
/// here from the generated init code.
#[derive(Default)]
struct Exports {
    /// (dll, function) -> address, with dll lowercased and without ".dll".
    functions: Vec<(String, String, u32)>,
    /// Module handles handed out by LoadLibrary, in registration order.
    modules: Vec<String>,
}

static EXPORTS: Mutex<Option<Exports>> = Mutex::new(None);

/// Module handles are synthetic; the value only has to be non-null and
/// distinguishable.
const MODULE_HANDLE_BASE: HMODULE = 0xd11_0000;

fn normalize_module_name(name: &str) -> String {
    let name = name.rsplit(['\\', '/']).next().unwrap();
    // Lowercase first: stripping ".dll" before folding case misses "Foo.Dll".
    let name = name.to_ascii_lowercase();
    name.strip_suffix(".dll").unwrap_or(&name).to_string()
}

/// Register a function as available through GetProcAddress. Called from
/// generated init code, once per function the translator reserved an address
/// for.
pub fn register_export(dll: &str, func: &str, addr: u32) {
    let mut exports = EXPORTS.lock().unwrap();
    let exports = exports.get_or_insert_with(Default::default);
    let dll = normalize_module_name(dll);
    if !exports.modules.contains(&dll) {
        exports.modules.push(dll.clone());
    }
    exports.functions.push((dll, func.to_string(), addr));
}

fn module_handle(name: &str) -> Option<HMODULE> {
    let exports = EXPORTS.lock().unwrap();
    let exports = exports.as_ref()?;
    let name = normalize_module_name(name);
    let index = exports.modules.iter().position(|module| *module == name)?;
    Some(MODULE_HANDLE_BASE + index as u32)
}

fn module_name(handle: HMODULE) -> Option<String> {
    let index = handle.checked_sub(MODULE_HANDLE_BASE)? as usize;
    let exports = EXPORTS.lock().unwrap();
    exports.as_ref()?.modules.get(index).cloned()
}

fn proc_address(module: &str, func: &str) -> Option<u32> {
    let exports = EXPORTS.lock().unwrap();
    exports
        .as_ref()?
        .functions
        .iter()
        .find_map(|(dll, name, addr)| (dll == module && name == func).then_some(*addr))
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
    let Some(name) =
        (lpModuleName.addr != 0).then(|| ctx.memory.read_str(lpModuleName.addr).to_owned())
    else {
        // A null name asks for the running executable itself.
        return lock().image_base;
    };
    match module_handle(&name) {
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
    // A DLL we implement is already "loaded"; anything else falls through to
    // whatever loader the host installed.
    if let Some(handle) = module_handle(&filename) {
        return handle;
    }
    lock().dll_loader.load_library(&filename)
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
    if let Some(module) = module_name(hModule) {
        if let Some(addr) = proc_address(&module, &name) {
            return addr;
        }
        log::warn!("GetProcAddress({module}, {name}): not implemented, returning null");
        return 0;
    }
    lock().dll_loader.get_proc_address(hModule, &name)
}
