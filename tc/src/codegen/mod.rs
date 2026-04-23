use anyhow::{Result, anyhow, bail};
mod control_flow;
mod fpu;
mod math;
mod misc;
mod mmx;
mod string;

use crate::{Block, State, write_if_changed};

fn reg_name(r: iced_x86::Register) -> String {
    format!("{r:?}").to_ascii_lowercase()
}

fn instr_name(instr: &iced_x86::Instruction) -> String {
    format!("{:?}", instr.mnemonic()).to_ascii_lowercase()
}

pub fn get_reg(r: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match r {
        EAX | ECX | EDX | EBX | ESI | EDI | ESP | EBP => {
            format!("ctx.cpu.regs.{reg}", reg = reg_name(r))
        }
        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX | DI | SI => {
            format!("ctx.cpu.regs.get_{reg}()", reg = reg_name(r))
        }
        r => todo!("{r:?}"),
    }
}

pub fn set_reg(r: iced_x86::Register, expr: String) -> String {
    use iced_x86::Register::*;
    match r {
        EAX | ECX | EDX | EBX | ESI | EDI | ESP | EBP => {
            format!("ctx.cpu.regs.{reg} = {expr};", reg = reg_name(r))
        }
        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX | DI | SI => {
            format!("ctx.cpu.regs.set_{reg}({expr});", reg = reg_name(r))
        }
        r => todo!("{r:?}"),
    }
}

pub fn gen_addr(instr: &iced_x86::Instruction) -> String {
    let mut expr = Vec::new();
    match instr.memory_segment() {
        iced_x86::Register::DS | iced_x86::Register::SS => {}
        iced_x86::Register::FS => expr.push(format!("ctx.cpu.regs.fs_base")),
        iced_x86::Register::None => {}
        r => todo!("{r:?}"),
    }
    match instr.memory_base() {
        iced_x86::Register::None => {}
        r => expr.push(get_reg(r)),
    }
    if instr.memory_index() != iced_x86::Register::None {
        if instr.memory_index_scale() != 1 {
            expr.push(format!(
                "({}*{})",
                get_reg(instr.memory_index()),
                instr.memory_index_scale()
            ));
        } else {
            expr.push(format!("{}", get_reg(instr.memory_index()),));
        }
    }
    let offset = instr.memory_displacement32();
    if offset != 0 {
        expr.push(format!("{offset:#x}u32"));
    }
    expr.into_iter()
        .enumerate()
        .map(|(i, e)| {
            if i == 0 {
                e
            } else {
                format!(".wrapping_add({e})")
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn get_mem(typ: String, addr: String) -> String {
    format!("ctx.memory.read::<{typ}>({addr})")
}

pub fn set_mem(typ: String, addr: String, expr: String) -> String {
    format!("ctx.memory.write::<{typ}>({addr}, {expr});")
}

pub fn get_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Immediate8 => format!("{:#x}u8", instr.immediate8()),
        Immediate16 => format!("{:#x}u16", instr.immediate16()),
        Immediate8to16 => format!("{:#x}u16", instr.immediate8to16()),
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => get_reg(instr.op_register(n)),
        Memory => get_mem(format!("u{}", mem_size(instr)), gen_addr(instr)),
        k => todo!("{:?}", k),
    }
}

pub fn reg_size(r: iced_x86::Register) -> usize {
    use iced_x86::Register::*;
    match r {
        AL | AH | BL | BH | CL | CH | DL | DH => 8,
        AX | BX | CX | DX | SI | DI | BP | SP => 16,
        EAX | EBX | ECX | EDX | ESI | EDI | ESP | EBP => 32,
        MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7 => 64,
        r => todo!("{r:?}"),
    }
}

pub fn mem_size(instr: &iced_x86::Instruction) -> usize {
    use iced_x86::MemorySize::*;
    match instr.memory_size() {
        UInt8 | Int8 => 8,
        UInt16 | Int16 => 16,
        UInt32 | Int32 => 32,
        UInt64 | Int64 => 64,
        Float32 => 32,
        Float64 => 64,
        Packed32_UInt8 => 32,
        Packed64_Int8 | Packed64_Int16 => 64,
        s => todo!("{s:?}"),
    }
}

pub fn op_size(instr: &iced_x86::Instruction, n: u32) -> usize {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => reg_size(instr.op_register(n)),
        Memory => mem_size(instr),
        Immediate8to32 => 32,
        Immediate32 => 32,
        k => todo!("{k:?}"),
    }
}

pub fn set_op(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => set_reg(instr.op_register(n), expr),
        Memory => {
            let addr = gen_addr(instr);
            let size = mem_size(instr);
            set_mem(format!("u{size}"), addr, expr)
        }
        k => todo!("{:?}", k),
    }
}

#[derive(Default)]
pub struct Writer {
    buf: String,
}

impl Writer {
    pub fn line(&mut self, s: impl AsRef<str>) {
        self.buf.push_str(s.as_ref());
        self.buf.push('\n');
    }

    #[allow(unused)]
    pub fn todo(&mut self) {
        self.line("todo!();");
    }
}

fn gen_block(w: &mut Writer, state: &State, ip: u32, block: &Block) {
    match block {
        Block::Instrs(instrs) => {
            w.line(format!("pub fn x{ip:x}(ctx: &mut Context) -> Cont {{"));
            for instr in instrs {
                gen_instr(w, state, instr);
            }
            w.line("}\n");
        }
        Block::Stdcall(_) | Block::Extern(_) => {
            // no emit
        }
    }
}

fn gen_instr(w: &mut Writer, state: &State, instr: &iced_x86::Instruction) {
    w.line(format!("// {:08x} {}", instr.ip32(), instr));
    if control_flow::codegen(w, state, instr) {
    } else if math::codegen(w, state, instr) {
    } else if string::codegen(w, state, instr) {
    } else if misc::codegen(w, state, instr) {
    } else if fpu::codegen(w, state, instr) {
    } else if mmx::codegen(w, state, instr) {
    } else {
        todo!("{:?} in {}", instr.mnemonic(), instr);
    }
}

pub fn gen_file(state: &mut State, outdir: &str) -> Result<()> {
    let mut w = Writer::default();

    w.line(
        "#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use runtime::*;
use winapi::*;
",
    );

    if state.blocks.values().any(|b| matches!(b, Block::Extern(_))) {
        w.line("use crate::externs::*;");
    }

    // It would be cool if we could just link a wasm object file that contains data sections
    // like
    //   (data (i32.const 0x400000) "....")
    // Unfortunately, wasm-lld only supports "relocatable" object files which means it moves
    // the location of such data at link time.  We could do it by postprocessing the wasm
    // file, maybe.
    w.line("fn init_mappings(ctx: &mut Context, mappings: &mut kernel32::Mappings) {");
    for map in state.mem.mappings.vec().iter() {
        let addr = map.addr;
        let buf = state.mem.slice(map.addr, map.size);
        let zeroed = buf.iter().all(|&b| b == 0);

        w.line(format!(
            "mappings.alloc(
                {desc:?}.to_string(),
                Some({addr:#x}),
                {size:#x}
            );",
            desc = map.desc,
            size = buf.len(),
        ));
        if !zeroed {
            w.line(format!(
                "let bytes = include_bytes!(\"../data/{addr:08x}.raw\").as_slice();
let out = &mut ctx.memory[{addr:#x}..][..bytes.len()];
out.copy_from_slice(bytes);",
            ));
        }
    }

    w.line("}");

    let mut ips = state.blocks.keys().copied().collect::<Vec<_>>();
    ips.sort();
    for &ip in &ips {
        let block = state.blocks.get(&ip).unwrap();
        gen_block(&mut w, &state, ip, &block);
    }

    w.line(format!(
        "const BLOCKS: [(u32, fn(&mut Context) -> Cont); {}] = [\n",
        ips.len() + 1,
    ));
    for &ip in &ips {
        let block = state.blocks.get(&ip).unwrap();
        w.line(format!("({ip:#x}, {}),", block.name()));
    }
    w.line("(runtime::RETURN_FROM_X86_ADDR, runtime::return_from_x86),");
    w.line("];\n");

    let resources = match state.resources {
        Some((addr, size)) => format!("{addr:#x}..{end:#x}", end = addr + size),
        None => "0..0".to_string(),
    };

    w.line(format!(
        "pub const EXEDATA: EXEData = EXEData {{
            image_base: {image_base:#x},
            resources: {resources},
            blocks: &BLOCKS,
            init_mappings,
            entry_point: Cont(x{entry_point:x}),
        }};\n\n",
        image_base = state.image_base,
        entry_point = state.entry_point,
    ));

    std::fs::create_dir_all(format!("{outdir}/src"))?;
    let path = format!("{outdir}/src/generated.rs");
    let text = rustfmt(&w.buf)?;
    write_if_changed(&path, text.as_bytes()).map_err(|err| anyhow!("write {path}: {err}"))?;
    Ok(())
}

fn rustfmt(text: &str) -> Result<String> {
    use std::io::Write;
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(text.as_bytes())?;
    drop(stdin);
    let output = child.wait_with_output()?;

    if !output.status.success() {
        bail!("rustfmt failed: {}", std::str::from_utf8(&output.stderr)?);
    }
    Ok(String::from_utf8(output.stdout)?)
}
