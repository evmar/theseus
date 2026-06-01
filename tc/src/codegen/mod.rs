use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};
mod control_flow;
mod fpu;
mod math;
mod misc;
mod mmx;
mod string;

use crate::{Block, BlockType, Instr, Module, State, memory::Memory, write_if_changed};

fn reg_name(r: iced_x86::Register) -> String {
    format!("{r:?}").to_ascii_lowercase()
}

fn instr_name(instr: &iced_x86::Instruction) -> String {
    if instr.mnemonic() == iced_x86::Mnemonic::Loop {
        return "loop_".to_string();
    }
    format!("{:?}", instr.mnemonic()).to_ascii_lowercase()
}

pub fn get_reg(r: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match r {
        EAX | ECX | EDX | EBX | ESI | EDI | ESP | EBP => {
            format!("ctx.cpu.regs.{reg}", reg = reg_name(r))
        }
        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX |
        DI | SI | SP | BP | // comment to disable formatting
        CS | DS | ES | SS => {
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
        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX |
        DI | SI | SP | BP | // comment to disable formatting
        CS | DS | ES | SS => {
            format!("ctx.cpu.regs.set_{reg}({expr});", reg = reg_name(r))
        }
        r => todo!("{r:?}"),
    }
}

/// Code generate a memory address reference.
/// Even for 16-bit code we generate a 32-bit memory address, because the computed
/// address can go beyond a 16-bit address range.
pub fn gen_addr(instr: &iced_x86::Instruction) -> String {
    let mut expr = Vec::new();
    match instr.memory_segment() {
        iced_x86::Register::CS
        | iced_x86::Register::DS
        | iced_x86::Register::SS
        | iced_x86::Register::GS => {}
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
        expr.push(format!("{offset:#x}"));
    }

    let needs_cast = match instr.memory_size() {
        iced_x86::MemorySize::Unknown => {
            // e.g. lea
            false
        }
        _ => mem_size(instr) == 16,
    };

    if needs_cast {
        expr.into_iter()
            .enumerate()
            .map(|(i, e)| {
                if i == 0 {
                    format!("({e} as u32)")
                } else {
                    format!(".wrapping_add({e} as u32)")
                }
            })
            .collect::<Vec<_>>()
            .join("")
    } else {
        expr.into_iter()
            .enumerate()
            .map(|(i, e)| {
                if i == 0 {
                    format!("{e}")
                } else {
                    format!(".wrapping_add({e})")
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
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
        CS | DS | ES | FS | GS | SS => 16,
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
        DwordOffset => 32, // e.g. `call dword ptr [...]`
        s => todo!("{s:?}"),
    }
}

pub fn op_size(instr: &iced_x86::Instruction, n: u32) -> usize {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => reg_size(instr.op_register(n)),
        Memory => mem_size(instr),
        Immediate16 => 16,
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

pub struct CodeGen<'a> {
    module: &'a Module,
    mem: &'a Memory,
    blocks: &'a HashMap<u32, Block>,
    buf: String,
}

impl<'a> CodeGen<'a> {
    pub fn new(state: &'a State) -> Self {
        Self {
            module: &state.module,
            mem: &state.mem,
            blocks: &state.blocks,
            buf: Default::default(),
        }
    }

    pub fn line(&mut self, s: impl AsRef<str>) {
        self.buf.push_str(s.as_ref());
        self.buf.push('\n');
    }

    pub fn todo(&mut self) {
        self.line("todo!();");
    }
}

impl<'a> CodeGen<'a> {
    fn gen_block(&mut self, block: &Block) {
        match &block.ty {
            BlockType::Instrs(instrs) => {
                self.line(format!(
                    "pub fn {name}(ctx: &mut Context) -> Cont {{",
                    name = block.name()
                ));
                // self.line(format!("println!(\"{name}\");", name = block.name()));
                // self.line(format!("ctx.dump_dosbox(0{});", block.name()));
                for instr in instrs {
                    if let Err(e) = self.gen_instr(instr) {
                        self.line(format!("// {}", e));
                        self.todo();
                        break;
                    }
                }

                let last = instrs.last().unwrap();
                if last.iced.flow_control() == iced_x86::FlowControl::Next
                    || (last.iced.mnemonic() == iced_x86::Mnemonic::Call && last.hint.is_some())
                {
                    self.line(format!("Cont(x{:x})", last.next_ip()));
                }

                self.line("}\n");
            }
            BlockType::Stdcall(_) | BlockType::Extern(_) => {
                // no emit
            }
        }
    }

    fn gen_instr(&mut self, instr: &Instr) -> anyhow::Result<()> {
        // log::info!("gen: {:08x} {}", instr.iced.ip32(), instr.iced);
        self.line(format!("// {:08x} {}", instr.iced.ip32(), instr.iced));
        if self.codegen_control_flow(instr) {
        } else if self.codegen_math(&instr.iced) {
        } else if self.codegen_string(&instr.iced) {
        } else if self.codegen_misc(&instr.iced) {
        } else if self.codegen_fpu(&instr.iced) {
        } else if self.codegen_mmx(&instr.iced) {
        } else {
            anyhow::bail!("{:?} not implemented", instr.iced.mnemonic());
        }
        Ok(())
    }

    fn gen_init_memory(&mut self) {
        // It would be cool if we could just link a wasm object file that contains data sections
        // like
        //   (data (i32.const 0x400000) "....")
        // Unfortunately, wasm-lld only supports "relocatable" object files which means it moves
        // the location of such data at link time.  We could do it by postprocessing the wasm
        // file, maybe.

        self.line("fn init_memory(ctx: &mut Context, mappings: &mut kernel32::Mappings) {");

        for map in self.mem.mappings.vec().iter() {
            let addr = map.addr;
            let buf = self.mem.slice(map.addr, map.size);
            let zeroed = buf.iter().all(|&b| b == 0);

            self.line(format!(
                "mappings.reserve(winapi::kernel32::Mapping {{
                    desc: {desc:?}.to_string(),
                    addr: {addr:#x},
                    size: {size:#x},
                    section: {section},
                }});",
                desc = map.desc,
                size = buf.len(),
                section = map.section,
            ));
            if !zeroed {
                self.line(format!(
                    "let bytes = include_bytes!(\"../data/{addr:08x}.raw\").as_slice();
let out = &mut ctx.memory.bytes[{addr:#x}..][..bytes.len()];
out.copy_from_slice(bytes);",
                ));
            }
        }

        if !self.module.vtables.is_empty() {
            self.line("unsafe {");
            for (module, val) in &self.module.vtables {
                self.line(format!("winapi::{module}::VTABLE = {val:#x};"));
            }
            self.line("}");
        }

        self.line("}");
        self.line("");
    }

    fn gen_blocks(&mut self) {
        let mut ips = self.blocks.keys().copied().collect::<Vec<_>>();
        ips.sort();
        for &ip in &ips {
            let block = self.blocks.get(&ip).unwrap();
            self.gen_block(&block);
        }

        self.line(format!(
            "const BLOCKS: [(u32, ContFn); {}] = [\n",
            ips.len() + 1,
        ));
        for &ip in &ips {
            let block = self.blocks.get(&ip).unwrap();
            self.line(format!("({ip:#x}, {name}),", name = block.name()));
        }
        self.line("(runtime::RETURN_FROM_X86_ADDR, Context::return_from_x86),");
        self.line("];");
        self.line("");
    }

    pub fn gen_file(&mut self, outdir: &str) -> Result<()> {
        self.line(
            "//! this module was generated by tc

#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use runtime::*;
use winapi::*;
",
        );

        if self
            .blocks
            .values()
            .any(|b| matches!(b.ty, BlockType::Extern(_)))
        {
            self.line("use crate::externs::*;");
        }
        self.line("");

        self.gen_init_memory();

        self.gen_blocks();

        let resources = self.module.resources.clone().unwrap_or(0..0);

        let entry_point = self.blocks.get(&self.module.entry_point).unwrap();
        self.line(format!(
            "pub const EXEDATA: EXEData = EXEData {{
            bitness: {bitness},
            image_base: {image_base:#x},
            resources: {res_start:#x}..{res_end:#x},
            blocks: &BLOCKS,
            init_memory,
            entry_point: Cont({entry_point}),
        }};\n\n",
            bitness = self.module.bitness,
            image_base = self.module.image_base,
            res_start = resources.start,
            res_end = resources.end,
            entry_point = entry_point.name(),
        ));

        std::fs::create_dir_all(format!("{outdir}/src"))?;
        let path = format!("{outdir}/src/generated.rs");
        let text = rustfmt(&self.buf)?;
        write_if_changed(&path, text.as_bytes()).map_err(|err| anyhow!("write {path}: {err}"))?;
        Ok(())
    }
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
