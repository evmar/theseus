use std::collections::HashMap;

use crate::memory::Memory;

mod codegen;
mod load;
pub use load::load_pe;
mod memory;
mod traverse;
pub use traverse::Gather;

#[derive(Debug, Clone)]
pub struct Import {
    pub dll: String,
    pub func: String,
    /// address to write func_addr to
    pub iat_addr: u32,
    /// address of code
    pub func_addr: u32,
}

#[derive(Default)]
pub struct Module {
    pub image_base: u32,
    pub entry_point: u32,
    pub code_memory: std::ops::Range<u32>,
    pub resources: Option<(u32, u32)>,
    pub imports: Vec<Import>,
}

#[derive(Default)]
pub struct State {
    pub module: Module,
    pub mem: Memory,
    pub blocks: HashMap<u32, Block>,
}

/// If any of the imports are from a DLL with exports, add the DLL's exports too.
pub fn add_dll_imports(imports: &mut Vec<Import>) {
    let mut next_addr = imports.last().unwrap().func_addr + 1;
    for (lib, exports) in [
        ("ddraw", winapi::ddraw::EXPORTS.as_slice()),
        ("dsound", winapi::dsound::EXPORTS.as_slice()),
    ] {
        if !imports.iter().any(|i| i.dll == lib) {
            continue;
        }
        for func in exports {
            imports.push(Import {
                dll: lib.to_string(),
                func: func.to_string(),
                iat_addr: 0,
                func_addr: next_addr,
            });
            next_addr += 1;
        }
    }
}

pub fn is_abs_memory_ref(instr: &iced_x86::Instruction) -> Option<u32> {
    let iced_x86::OpKind::Memory = instr.op0_kind() else {
        return None;
    };
    let iced_x86::Register::None = instr.memory_base() else {
        return None;
    };
    let iced_x86::Register::None = instr.memory_index() else {
        return None;
    };
    Some(instr.memory_displacement32())
}

pub enum Block {
    Instrs(Vec<iced_x86::Instruction>),
    Stdcall(String),
    Extern(u32),
}

impl Block {
    pub fn name(&self) -> String {
        match self {
            Block::Instrs(instrs) => format!("x{:x}", instrs[0].ip32()),
            Block::Stdcall(func) => format!("{}_stdcall", func),
            Block::Extern(ip) => format!("x{:x}", ip),
        }
    }
}

pub fn write_if_changed(path: &str, contents: &[u8]) -> anyhow::Result<()> {
    let existing = std::fs::read(&path).unwrap_or_default();
    if existing != contents {
        std::fs::write(path, contents)?;
    }
    Ok(())
}

impl State {
    pub fn gather(&mut self, gather: Gather) {
        self.blocks = gather.run(self);
    }

    pub fn generate(&mut self, outdir: &str) -> anyhow::Result<()> {
        let mut codegen = codegen::CodeGen::new(self);
        codegen.gen_file(outdir)?;

        let data_dir = format!("{outdir}/data");
        std::fs::create_dir_all(&data_dir)?;
        for map in self.mem.mappings.vec().iter() {
            let buf = self.mem.slice(map.addr, map.size);
            if buf.iter().all(|&b| b == 0) {
                continue;
            }
            log::info!(
                "section {:?} @{:x} ({:x} bytes)",
                map.desc,
                map.addr,
                map.size
            );
            write_if_changed(&format!("{outdir}/data/{:08x}.raw", map.addr), buf)?;
        }
        Ok(())
    }
}
