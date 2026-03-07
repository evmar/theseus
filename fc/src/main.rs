#![allow(unused_must_use)]

mod memory;

use anyhow::{Result, anyhow, bail};
use std::collections::{HashMap, VecDeque};

use memory::*;

#[derive(Debug)]
struct State {
    pe_file: pe::File,
    mem: Memory,
    imports: HashMap<u32, (String, String)>,
}

impl State {
    fn new(buf: Vec<u8>) -> State {
        let f = pe::File::parse(&buf).unwrap();
        let mut mem = Memory::default();

        let image_base = AddrAbs(f.opt_header.ImageBase);
        mem.alloc("exe header".into(), image_base, 0x1000);
        mem.put(image_base, &buf[..0x1000.min(buf.len())]);
        for sec in &f.sections {
            let addr = AddrImage(sec.VirtualAddress).to_abs(image_base);
            let size = sec.SizeOfRawData.max(sec.VirtualSize);
            mem.alloc(sec.name().unwrap().into(), addr, size);
            let data = &buf[sec.PointerToRawData as usize
                ..(sec.PointerToRawData + sec.SizeOfRawData) as usize];
            mem.put(addr, data);
        }
        println!("{:#x?}", mem);

        State {
            pe_file: f,
            mem,
            imports: Default::default(),
        }
    }

    fn read_imports(&mut self) {
        let Some(imports) = self
            .pe_file
            .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT)
        else {
            return;
        };
        let image_base = self.image_base();
        let image = self.mem.slice_all(image_base);
        for imp in pe::read_imports(imports.as_slice(image).unwrap()) {
            let name = std::str::from_utf8(imp.image_name(image)).unwrap();
            println!("{name:?}");
            for (addr, entry) in imp.iat_iter(image) {
                let addr = AddrImage(addr);
                self.imports.insert(
                    addr.to_abs(image_base).0,
                    (name.into(), entry.as_import_symbol(image).to_string()),
                );
            }
        }
    }

    fn image_base(&self) -> AddrAbs {
        AddrAbs(self.pe_file.opt_header.ImageBase)
    }
}

fn is_abs_memory_ref(instr: &iced_x86::Instruction) -> Option<u32> {
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

fn gen_reg(r: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match r {
        EAX => format!("REGS.eax"),
        ECX => format!("REGS.ecx"),
        EDX => format!("REGS.edx"),
        EBX => format!("REGS.ebx"),

        ESI => format!("REGS.esi"),
        EDI => format!("REGS.edi"),
        ESP => format!("REGS.esp"),
        EBP => format!("REGS.ebp"),

        r => todo!("{:?}", r),
    }
}

fn gen_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Immediate8 => format!("{:#x}u8", instr.immediate8()),
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => gen_reg(instr.op_register(n)),
        Memory => {
            match instr.memory_segment() {
                iced_x86::Register::DS => {}
                iced_x86::Register::FS => return format!("todo!();"),
                iced_x86::Register::None => {}
                r => todo!("{r:?}"),
            }
            let mut expr = String::new();
            match instr.memory_base() {
                iced_x86::Register::None => {}
                r => expr = gen_reg(r),
            }
            if instr.memory_index() != iced_x86::Register::None {
                todo!();
            }
            let addr = instr.memory_displacement32();
            if !expr.is_empty() {
                expr = expr + "+";
            }
            expr = expr + &format!("{addr:#x}u32");
            let size = match instr.memory_size() {
                iced_x86::MemorySize::UInt8 => "u8",
                iced_x86::MemorySize::UInt32 => "u32",
                s => todo!("{s:?}"),
            };
            format!("*(({expr}) as *mut {size})")
        }
        k => {
            dbg!(instr);
            todo!("{:?}", k);
        }
    }
}

struct Block {
    instrs: Vec<iced_x86::Instruction>,
}

fn gen_block(w: &mut dyn std::fmt::Write, state: &State, ip: AddrAbs, block: &Block) {
    write!(w, "pub fn x{:08x}() -> u32 {{\n", ip.0);
    write!(w, "unsafe {{\n");

    for instr in &block.instrs {
        println!("gen: {}", instr);
        write!(w, "// {:08x} {}\n", AddrAbs(instr.ip32()).0, instr);
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Push => {
                write!(w, "push({});\n", gen_op(instr, 0));
            }
            Pop => {
                write!(w, "{} = pop();\n", gen_op(instr, 0));
            }
            Call => {
                if let Some(addr) = is_abs_memory_ref(instr) {
                    if let Some((dll, func)) = state.imports.get(&addr) {
                        let dll = dll.to_lowercase();
                        let dll = dll.trim_end_matches(".dll");
                        write!(w, "{dll}::stdcall_{func}();\n");
                    } else {
                        todo!("{}", instr);
                    }
                } else if instr.op0_kind() == iced_x86::OpKind::NearBranch32 {
                    write!(
                        w,
                        "call({:#08x}, {:#08x})\n",
                        instr.next_ip32(),
                        instr.near_branch32()
                    );
                } else {
                    todo!("{} {:?}", instr, instr.op0_kind());
                }
            }
            Xor => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} ^= {op1};\n");
            }
            And => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} &= {op1};\n");
            }
            Add => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} -= {op1};\n");
            }
            Sub => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = sub({op0}, {op1});\n");
            }
            Cmp => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "sub({op0}, {op1});\n");
            }
            Ret => {
                write!(w, "todo!(\"ret\");\n");
            }
            Mov => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = {op1};\n");
            }
            Jne => {
                write!(w, "jne({}, {})\n", instr.next_ip32(), instr.near_branch32());
            }
            Je => {
                write!(w, "todo!(\"{}\");\n", instr);
            }

            c => todo!("{:?} in {}", c, instr),
        }
        if instr.flow_control() != iced_x86::FlowControl::Next {
            match instr.mnemonic() {
                // iced_x86::Mnemonic::Call => {}
                _ => break,
            }
        }
    }
    write!(w, "}}}}\n\n");
}

fn traverse(state: &State, ip: u32) -> HashMap<u32, Block> {
    let mut blocks = HashMap::<u32, Block>::new();

    let mut queue = VecDeque::<u32>::new();
    queue.push_back(ip);

    while let Some(ip) = queue.pop_front() {
        if blocks.contains_key(&ip) {
            continue;
        }
        println!("visit {ip:#08x}");

        let mut instrs = Vec::new();
        let decoder = iced_x86::Decoder::with_ip(
            32,
            state.mem.slice_all(AddrAbs(ip)),
            ip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        for instr in decoder {
            println!("{:08x} {}", instr.ip32(), instr);
            instrs.push(instr);
            if instr.flow_control() == iced_x86::FlowControl::Next {
                continue;
            }
            use iced_x86::Mnemonic::*;
            match instr.mnemonic() {
                Call => match instr.op0_kind() {
                    iced_x86::OpKind::NearBranch32 => {
                        queue.push_back(instr.near_branch32());
                    }
                    _ => todo!("call dest {}", instr),
                },
                Je | Jne => match instr.op0_kind() {
                    iced_x86::OpKind::NearBranch32 => {
                        queue.push_back(instr.near_branch32());
                    }
                    _ => todo!("jne dest {}", instr),
                },
                Ret => {}
                _ => todo!("control flow {}", instr),
            }
            break;
        }

        let block = Block { instrs };
        blocks.insert(ip, block);
    }

    blocks
}

fn gen_file(state: &State, outdir: &str) -> Result<()> {
    use std::fmt::Write;
    let mut text = String::new();

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint).to_abs(state.image_base());

    let blocks = traverse(state, ip.0);

    write!(&mut text, "use runtime::*;\n");

    let mut ips = blocks.keys().copied().collect::<Vec<_>>();
    ips.sort();
    for &ip in &ips {
        let block = blocks.get(&ip).unwrap();
        gen_block(&mut text, &state, AddrAbs(ip), &block);
    }

    write!(
        &mut text,
        "pub const BLOCKS: [(u32, fn() -> u32); {}] = [\n",
        ips.len()
    );
    for &ip in &ips {
        write!(&mut text, "({ip:#08x}, x{ip:08x}),\n");
    }
    write!(&mut text, "];\n\n");

    std::fs::create_dir_all(format!("{outdir}/src"))?;
    let path = format!("{outdir}/src/generated.rs");
    let text = rustfmt(&text)?;
    std::fs::write(&path, text).map_err(|err| anyhow!("write {path}: {err}"))?;
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

fn run() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, exe_path, outdir] = args.as_slice() else {
        println!("usage: {} exe outdir", args[0]);
        return Ok(());
    };

    let buf = std::fs::read(exe_path).unwrap();
    let mut state = State::new(buf);
    state.read_imports();
    println!("{:#x?}", state.imports);

    gen_file(&state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in &state.mem.mappings {
        std::fs::write(
            format!("{outdir}/data/{:08x}.raw", map.addr.0),
            state.mem.slice(map.addr, map.len),
        )?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error: {err}");
    }
}
