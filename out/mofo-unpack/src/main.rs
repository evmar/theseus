mod externs;
mod generated;

use std::sync::{LazyLock, Mutex};

use winapi::kernel32;

/// State gathered while running the unpacker.
#[derive(Default)]
struct State {
    /// DLLs loaded using LoadLibrary.
    /// modules[i] = the name of the DLL for HMODULE i+1.
    modules: Vec<String>,
    /// symbols imported using GetProcAddress
    syms: Vec<tc::Import>,
    /// address to assign to the next symbol imported
    next_addr: u32,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

/// Implementation of kernel32::DLLLoader (hooking LoadLibrary/GetProcAddress) that
/// succeeds and records the imports in the global STATE.
struct Loader;
impl kernel32::DLLLoader for Loader {
    fn load_library(&mut self, filename: &str) -> kernel32::HMODULE {
        let name = filename.to_lowercase();
        let name = name.trim_end_matches(".dll");
        let mut state = STATE.lock().unwrap();
        state.modules.push(name.to_owned());
        state.modules.len() as u32
    }

    fn get_proc_address(&mut self, hmodule: kernel32::HMODULE, proc_name: &str) -> u32 {
        let mut state = STATE.lock().unwrap();
        let dll = state.modules[hmodule as usize - 1].clone();
        assert!(proc_name.len() > 0);
        let addr = state.next_addr;
        state.next_addr += 1;
        state.syms.push(tc::Import {
            dll,
            func: proc_name.to_owned(),
            iat_addr: 0, // not known yet
            addr,
            data: false,
        });
        addr
    }
}

fn main() {
    let mut ctx = winapi::load(&generated::EXEDATA);

    kernel32::lock().dll_loader = Box::new(Loader);
    {
        // Give the imports an arbitrary strange address so they are easier to find in memory.
        let mut state = STATE.lock().unwrap();
        state.next_addr = 0xFAFB_FC00;
    }
    winapi::start(&mut ctx, &generated::EXEDATA);
}

/// Fill in the .iat_addr on functions by searching the memory for their addresses.
fn find_iat(
    functions: &mut [tc::Import],
    mappings: &[runtime::Mapping],
    memory: &[u8],
    image_base: u32,
) {
    for sym in functions.iter_mut() {
        let addr_bytes = sym.addr.to_le_bytes();
        let mut iat_addr = None;
        for mapping in mappings.iter() {
            if mapping.addr < image_base {
                continue;
            }
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
            log::info!("{}!{}: found at {:x}", sym.dll, sym.func, ofs);
            sym.iat_addr = ofs;
            sym.addr = 0;
        } else {
            log::info!("{}!{}: not found", sym.dll, sym.func);
        }
    }
}

pub fn do_unpack(ctx: &mut runtime::Context) {
    let mut syms = std::mem::take(&mut STATE.lock().unwrap().syms);

    let mut tc = tc::State::default();

    // Use the same mappings as the input file as sections of the output.
    // Filter to "fixed" sections to avoid serializing out the process heap etc.
    tc.mem.mappings = runtime::Mappings::from(
        kernel32::lock()
            .mappings
            .vec()
            .iter()
            .filter(|m| m.addr == 0 || m.section)
            .cloned()
            .collect::<Vec<_>>(),
    );

    let image_base = 0x40_0000;
    find_iat(
        &mut syms,
        &tc.mem.mappings.vec(),
        &ctx.memory.bytes,
        image_base,
    );

    tc.mem.bytes.resize(ctx.memory.bytes.len(), 0);
    tc.mem.bytes.copy_from_slice(ctx.memory.bytes);

    tc.module = tc::Module {
        bitness: 32,
        code_segment: None,
        image_base,
        entry_point: 0x0041f079,
        code_memory: 0x40_0000..tc.mem.bytes.len() as u32,
        resources: None,
        imports: syms,
        vtables: vec![],
    };
    tc.init_imports();

    tc.gather(tc::Gather {
        //scan_immediates: true,
        entry_points: [
            vec![
                tc::EntryPoint::Single(0x004068f0), // window proc
                tc::EntryPoint::Single(0x0041ec70), // sound thread proc
                tc::EntryPoint::Single(0x0041e8f0), // waveOutOpen callback
            ],
            // jump table
            (0x0041e8d4..=0x0041e8ec)
                .step_by(4)
                .map(|addr| tc::EntryPoint::Single(tc.mem.read::<u32>(addr)))
                .collect(),
        ]
        .concat()
        .to_vec(),
        ..Default::default()
    });

    tc.generate("out/mofo").unwrap();
}
