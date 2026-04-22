mod externs;
mod generated;

use std::sync::{LazyLock, Mutex};

use winapi::kernel32;

#[derive(Default)]
struct Symbols {
    next_addr: u32,
    modules: Vec<String>,
    functions: Vec<tc::Import>,
}

static SYMS: LazyLock<Mutex<Symbols>> = LazyLock::new(|| Mutex::new(Symbols::default()));

struct Loader;

impl kernel32::DLLLoader for Loader {
    fn load_library(&mut self, filename: &str) -> kernel32::HMODULE {
        let name = filename.to_lowercase();
        let name = name.trim_end_matches(".dll");
        let mut syms = SYMS.lock().unwrap();
        syms.modules.push(name.to_owned());
        syms.modules.len() as u32
    }

    fn get_proc_address(&mut self, hmodule: kernel32::HMODULE, proc_name: &str) -> u32 {
        let mut syms = SYMS.lock().unwrap();
        let dll = syms.modules[hmodule as usize - 1].clone();
        assert!(proc_name.len() > 0);
        let func_addr = syms.next_addr;
        syms.next_addr += 1;
        syms.functions.push(tc::Import {
            dll,
            func: proc_name.to_owned(),
            iat_addr: 0,
            func_addr,
        });
        func_addr
    }
}

fn main() {
    let mut ctx = winapi::load(&generated::EXEDATA);

    SYMS.lock().unwrap().next_addr = 0xFFF0_0001;
    kernel32::lock().dll_loader = Box::new(Loader);
    winapi::start(&mut ctx, &generated::EXEDATA);
}

pub fn do_unpack() {
    let syms = SYMS.lock().unwrap();
    println!("{:#x?}", syms.functions);
    kernel32::lock().mappings.dump();
}
