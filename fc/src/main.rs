#![allow(unused_must_use)]

mod memory;

use anyhow::{Result, anyhow, bail};
use std::collections::{HashMap, VecDeque};

use memory::*;

struct State {
    pe_file: pe::File,
    mem: Memory,
    imports: HashMap<u32, (String, String)>,
    blocks: HashMap<u32, Block>,
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
            blocks: Default::default(),
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
        EAX => format!("MACHINE.regs.eax"),
        ECX => format!("MACHINE.regs.ecx"),
        EDX => format!("MACHINE.regs.edx"),
        EBX => format!("MACHINE.regs.ebx"),

        ESI => format!("MACHINE.regs.esi"),
        EDI => format!("MACHINE.regs.edi"),
        ESP => format!("MACHINE.regs.esp"),
        EBP => format!("MACHINE.regs.ebp"),

        r => todo!("{:?}", r),
    }
}

fn gen_addr(instr: &iced_x86::Instruction) -> String {
    let mut expr = Vec::new();
    match instr.memory_segment() {
        iced_x86::Register::DS | iced_x86::Register::SS => {}
        iced_x86::Register::FS => expr.push(format!("MACHINE.regs.fs_base")),
        iced_x86::Register::None => {}
        r => todo!("{r:?}"),
    }
    match instr.memory_base() {
        iced_x86::Register::None => {}
        r => expr.push(gen_reg(r)),
    }
    if instr.memory_index() != iced_x86::Register::None {
        todo!();
    }
    let addr = instr.memory_displacement32();
    expr.push(format!("{addr:#x}u32"));
    expr.join(" + ")
}

fn gen_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Immediate8 => format!("{:#x}u8", instr.immediate8()),
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => gen_reg(instr.op_register(n)),
        Memory => {
            let addr = gen_addr(instr);
            let size = match instr.memory_size() {
                iced_x86::MemorySize::UInt8 => "u8",
                iced_x86::MemorySize::UInt32 => "u32",
                s => todo!("{s:?}"),
            };
            format!("*(MACHINE.memory.add(({addr}) as usize) as *mut {size})")
        }
        k => {
            dbg!(instr);
            todo!("{:?}", k);
        }
    }
}

fn gen_abs_jmp(state: &State, addr: u32) -> String {
    if state.blocks.contains_key(&addr) {
        format!("Cont(x{:08x})", addr)
    } else {
        format!("indirect({:#08x}u32)", addr)
    }
}

fn gen_jmp(state: &State, instr: &iced_x86::Instruction) -> String {
    match instr.op_kind(0) {
        iced_x86::OpKind::NearBranch32 => {
            let addr = instr.near_branch32();
            gen_abs_jmp(state, addr)
        }
        iced_x86::OpKind::Memory => {
            if let Some(addr) = is_abs_memory_ref(instr) {
                if let Some((dll, func)) = state.imports.get(&addr) {
                    let dll = dll.to_lowercase();
                    let dll = dll.trim_end_matches(".dll");
                    format!("Cont({dll}::stdcall_{func})")
                } else {
                    format!("*(MACHINE.memory.add({addr:#x}u32 as usize) as *const u32)")
                }
            } else {
                todo!("indirect jmp");
            }
        }
        k => todo!("{:?}", k),
    }
}

struct Block {
    instrs: Vec<iced_x86::Instruction>,
}

fn gen_block(w: &mut dyn std::fmt::Write, state: &State, ip: AddrAbs, block: &Block) {
    println!("gen block: {:#08x}", ip.0);

    write!(w, "pub fn x{:08x}() -> Cont {{\n", ip.0);
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
                write!(
                    w,
                    "call({:#08x}, {})\n",
                    instr.next_ip32(),
                    gen_jmp(state, instr)
                );
            }
            Xor => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} ^= {op1};\n");
            }
            And => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = and({op0}, {op1});\n");
            }
            Or => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = or({op0}, {op1});\n");
            }
            Add => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} += {op1};\n");
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
                let n = match instr.op_count() {
                    0 => 0,
                    1 => {
                        assert!(instr.op0_kind() == iced_x86::OpKind::Immediate16);
                        instr.immediate16()
                    }
                    _ => todo!(),
                };
                write!(w, "ret({n})\n");
            }
            Mov => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = {op1};\n");
            }
            Je => {
                write!(
                    w,
                    "je({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jne => {
                write!(
                    w,
                    "jne({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jb => {
                write!(
                    w,
                    "jb({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Ja => {
                write!(
                    w,
                    "ja({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jae => {
                write!(
                    w,
                    "jae({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jl => {
                write!(
                    w,
                    "jl({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jge => {
                write!(
                    w,
                    "jge({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jecxz => {
                write!(
                    w,
                    "jecxz({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jg => {
                write!(
                    w,
                    "jg({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jle => {
                write!(
                    w,
                    "jle({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jbe => {
                write!(
                    w,
                    "jbe({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jmp => {
                write!(w, "{}\n", gen_jmp(state, instr));
            }
            Lea => {
                write!(w, "{} = {};\n", gen_op(instr, 0), gen_addr(instr));
            }
            Test => {
                write!(w, "and({}, {});\n", gen_op(instr, 0), gen_op(instr, 1));
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
                Call | Jmp | Je | Jne | Jb | Ja | Jae | Jl | Jge | Jecxz | Jg | Jle | Jbe => {
                    match instr.op0_kind() {
                        iced_x86::OpKind::NearBranch32 => queue.push_back(instr.near_branch32()),
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if state.imports.contains_key(&addr) {
                                    // ok
                                } else {
                                    println!("indirect jmp via memory {addr:x}");
                                }
                            } else {
                                println!("complex indirect jmp");
                            }
                        }
                        iced_x86::OpKind::Register => {
                            let reg = gen_reg(instr.op_register(0));
                            println!("indirect via register dest: {reg}");
                        }
                        d => todo!("dest {d:?}"),
                    };
                    if instr.mnemonic() != Jmp {
                        queue.push_back(instr.next_ip32());
                    }
                }
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

    write!(&mut text, "#![allow(unused_unsafe)]\n");
    write!(&mut text, "#![allow(unreachable_code)]\n\n");
    write!(&mut text, "#![allow(static_mut_refs)]\n\n");

    write!(&mut text, "use runtime::*;\n\n");

    let mut ips = state.blocks.keys().copied().collect::<Vec<_>>();
    ips.sort();
    for &ip in &ips {
        let block = state.blocks.get(&ip).unwrap();
        gen_block(&mut text, &state, AddrAbs(ip), &block);
    }

    // It would be cool if we could just link a wasm object file that contains data sections
    // like
    //   (data (i32.const 0x400000) "....")
    // Unfortunately, wasm-lld only supports "relocatable" object files which means it moves
    // the location of such data at link time.  We could do it by postprocessing the wasm
    // file, maybe.
    write!(&mut text, "pub fn init_memory() {{\n");
    write!(&mut text, "unsafe {{\n");
    write!(&mut text, "let sections = [\n");
    for map in &state.mem.mappings {
        write!(
            &mut text,
            "({:#x}, include_bytes!(\"../data/{:08x}.raw\").as_slice()),\n",
            map.addr.0, map.addr.0
        );
    }
    write!(&mut text, "];\n");
    write!(
        &mut text,
        "
        for (addr, data) in sections {{
            let out = core::slice::from_raw_parts_mut(MACHINE.memory.add(addr), data.len());
            out.copy_from_slice(data);
        }}
        }}
        }}

    "
    );

    write!(
        &mut text,
        "const BLOCKS: [(u32, fn() -> Cont); {}] = [\n",
        ips.len() + 2,
    );
    write!(&mut text, "(0, runtime::null_pointer_error),\n");
    for &ip in &ips {
        write!(&mut text, "({ip:#08x}, x{ip:08x}),\n");
    }
    write!(&mut text, "(0xf000_0000, runtime::return_from_main),\n");
    write!(&mut text, "];\n\n");
    write!(
        &mut text,
        "pub fn indirect(addr: u32) -> Cont {{
            let index = BLOCKS
                .binary_search_by_key(&addr, |(addr, _)| *addr)
                .unwrap();
            Cont(BLOCKS[index].1)
        }}"
    );

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

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint).to_abs(state.image_base());
    state.blocks = traverse(&state, ip.0);

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
        std::process::exit(1);
    }
}
