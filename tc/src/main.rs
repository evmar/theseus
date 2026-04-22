mod codegen;
mod load;
mod memory;
mod traverse;

use std::collections::HashMap;

use anyhow::Result;
use memory::*;

use crate::{load::load_pe, traverse::Traverse};

struct Import {
    dll: String,
    func: String,
    /// address to write func_addr to
    iat_addr: u32,
    /// address of code
    func_addr: u32,
}

#[derive(Default)]
pub struct State {
    image_base: u32,
    entry_point: u32,
    mem: Memory,
    code_memory: std::ops::Range<u32>,
    imports: HashMap<u32, Import>,
    blocks: HashMap<u32, Block>,
    resources: Option<(u32, u32)>,

    scan_immediates: bool,
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

enum Block {
    Instrs(Vec<iced_x86::Instruction>),
    Stdcall(String),
    Extern(u32),
}

impl Block {
    pub fn name(&self) -> String {
        match self {
            Block::Instrs(instrs) => format!("x{:08x}", instrs[0].ip32()),
            Block::Stdcall(func) => format!("{}_stdcall", func),
            Block::Extern(ip) => format!("x{:08x}", ip),
        }
    }
}

fn hex(val: &str) -> Result<u32, String> {
    if !val.starts_with("0x") {
        return Err("hex value must start with 0x".into());
    }
    u32::from_str_radix(&val[2..], 16).map_err(|err| err.to_string())
}

#[derive(argh::FromArgs)]
/// theseus compiler
struct Args {
    /// scan data sections for code-looking pointers
    #[argh(switch)]
    scan: bool,

    /// scan immediates for code-looking pointers
    #[argh(switch)]
    scan_immediates: bool,

    /// path to input executable
    #[argh(option)]
    exe: String,

    /// path to output directory
    #[argh(option)]
    out: String,

    /// blocks written by hand
    #[argh(option, long = "extern", from_str_fn(hex))]
    externs: Vec<u32>,
}

pub fn write_if_changed(path: &str, contents: &[u8]) -> Result<()> {
    let existing = std::fs::read(&path).unwrap_or_default();
    if existing != contents {
        std::fs::write(path, contents)?;
    }
    Ok(())
}

fn run() -> Result<()> {
    logger::init();
    let args: Args = argh::from_env();

    let mut state = State::default();
    state
        .mem
        .mappings
        .alloc("null page".into(), Some(0), 0x1000);
    if args.scan_immediates {
        state.scan_immediates = true;
    }

    for addr in args.externs {
        log::info!("extern: {addr:#x}");
        state.blocks.insert(addr, Block::Extern(addr));
    }

    let buf = std::fs::read(args.exe).unwrap();
    load_pe(&mut state, buf);

    let start = state.entry_point;
    let mut traverse = Traverse::new(&mut state, start);
    if args.scan {
        traverse.scan_for_pointers();
    }
    traverse.run();

    let outdir = &args.out;
    codegen::gen_file(&mut state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in state.mem.mappings.vec().iter() {
        let buf = state.mem.slice(map.addr, map.size);
        if buf.iter().all(|&b| b == 0) {
            continue;
        }
        write_if_changed(&format!("{outdir}/data/{:08x}.raw", map.addr), buf)?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        log::error!("error: {err}");
        std::process::exit(1);
    }
}
