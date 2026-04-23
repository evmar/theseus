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

    // Give the imports an arbitrary strange address so they are easier to find in memory.
    {
        let mut syms = SYMS.lock().unwrap();
        syms.next_addr = 0xFAFB_FC00;
        kernel32::lock().dll_loader = Box::new(Loader);
    }
    winapi::start(&mut ctx, &generated::EXEDATA);
}

/// Fill in the .iat_addr on functions by searching the memory for their addresses.
fn find_iat(functions: &mut [tc::Import], mappings: &[kernel32::Mapping], memory: &[u8]) {
    for sym in functions.iter_mut() {
        log::info!("finding: {}!{}", sym.dll, sym.func);
        let addr_bytes = sym.func_addr.to_le_bytes();
        let mut iat_addr = None;
        for mapping in mappings.iter() {
            for (ofs, _) in memory[mapping.addr as usize..][..mapping.size as usize]
                .windows(4)
                .enumerate()
                .filter(|(_, w)| *w == addr_bytes)
            {
                let ofs = mapping.addr + ofs as u32;
                match iat_addr {
                    None => iat_addr = Some(ofs),
                    Some(_) => {
                        log::error!("multiple matches: {:x} and {:x}", ofs, iat_addr.unwrap());
                        break;
                    }
                }
            }
        }
        if let Some(ofs) = iat_addr {
            sym.iat_addr = ofs;
            log::info!("found: {:x}", ofs);
        } else {
            log::error!("not found");
        }
    }
}

pub fn do_unpack(ctx: &mut runtime::Context) {
    let mut syms = SYMS.lock().unwrap();
    let syms = &mut *syms;

    let mut state = tc::State::default();

    {
        let kernel32 = kernel32::lock();
        state.mem.mappings = kernel32::Mappings::from(
            kernel32
                .mappings
                .vec()
                .iter()
                .filter(|m| m.fixed)
                .map(|m| m.clone())
                .collect::<Vec<_>>(),
        );
    }

    find_iat(
        &mut syms.functions,
        &state.mem.mappings.vec(),
        &ctx.memory.bytes,
    );

    state.image_base = 0x0040_0000;
    state.entry_point = 0x0040_85dd;
    state.mem.data.resize(ctx.memory.bytes.len(), 0);
    state.mem.data.copy_from_slice(ctx.memory.bytes);
    state.imports = syms
        .functions
        .iter()
        .map(|imp| (imp.iat_addr, imp.clone()))
        .collect();

    let mut next_addr = syms.next_addr;
    for (lib, exports) in [
        ("ddraw", winapi::ddraw::EXPORTS.as_slice()),
        ("dsound", winapi::dsound::EXPORTS.as_slice()),
    ] {
        for func in exports {
            state.imports.insert(
                next_addr,
                tc::Import {
                    dll: lib.to_string(),
                    func: func.to_string(),
                    iat_addr: 0,
                    func_addr: next_addr,
                },
            );
            next_addr += 1;
        }
    }

    for import in state.imports.values() {
        state.blocks.insert(
            import.func_addr,
            tc::Block::Stdcall(format!("{}::{}", import.dll, import.func)),
        );
    }

    let mut traverse = tc::Traverse::new(&mut state, false, 0x0040_85dd);
    traverse.run();

    tc::generate(&mut state, "exe/chillin").unwrap();
}
