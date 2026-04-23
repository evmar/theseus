use std::collections::HashMap;

use crate::memory::Memory;

mod codegen;
mod load;
pub use load::load_pe;
mod memory;
mod traverse;
pub use traverse::Traverse;

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
pub struct State {
    pub image_base: u32,
    pub entry_point: u32,
    pub mem: Memory,
    pub code_memory: std::ops::Range<u32>,
    /// iat addr => Import
    pub imports: HashMap<u32, Import>,
    pub blocks: HashMap<u32, Block>,
    pub resources: Option<(u32, u32)>,
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

pub fn generate(state: &mut State, outdir: &str) -> anyhow::Result<()> {
    codegen::gen_file(state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in state.mem.mappings.vec().iter() {
        let buf = state.mem.slice(map.addr, map.size);
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
