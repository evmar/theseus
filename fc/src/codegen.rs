use anyhow::{Result, anyhow, bail};

use crate::{Block, State, is_abs_memory_ref, memory::AddrAbs};

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

        r => format!("todo!(\"{:?}\")", r),
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
        expr.push(format!(
            "({}*{})",
            gen_reg(instr.memory_index()),
            instr.memory_index_scale()
        ));
    }
    let addr = instr.memory_displacement32();
    expr.push(format!("{addr:#x}u32"));
    expr.join(" + ")
}

fn gen_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Immediate8 => format!("{:#x}u8", instr.immediate8()),
        Immediate16 => format!("{:#x}u16", instr.immediate16()),
        Immediate8to16 => format!("{:#x}u16", instr.immediate8to16()),
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => gen_reg(instr.op_register(n)),
        Memory => {
            let addr = gen_addr(instr);
            let size = match instr.memory_size() {
                iced_x86::MemorySize::UInt8 => "u8",
                iced_x86::MemorySize::UInt16 => "u16",
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
                format!("indirect({})", gen_addr(instr))
            }
        }
        iced_x86::OpKind::Register => {
            format!("indirect({})", gen_reg(instr.op0_register()))
        }
        k => todo!("{:?}", k),
    }
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
            Sbb => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} = sbb({op0}, {op1});\n");
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
            Js => {
                write!(
                    w,
                    "js({}, {})\n",
                    gen_abs_jmp(state, instr.next_ip32()),
                    gen_jmp(state, instr)
                );
            }
            Jns => {
                write!(
                    w,
                    "jns({}, {})\n",
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
            Neg => {
                write!(w, "{} = neg({});\n", gen_op(instr, 0), gen_op(instr, 0));
            }
            Shl => {
                write!(
                    w,
                    "{} = shl({}, {});\n",
                    gen_op(instr, 0),
                    gen_op(instr, 0),
                    gen_op(instr, 1)
                );
            }
            Shr => {
                write!(
                    w,
                    "{} = shr({}, {});\n",
                    gen_op(instr, 0),
                    gen_op(instr, 0),
                    gen_op(instr, 1)
                );
            }
            Stosd | Scasb | Cmpsb | Movzx | Movsx | Movsd | Std | Cld | Stosb | Div | Leave
            | Dec | Inc | Sete | Sar | Imul | Not | Setge | Int | Cdq | Idiv | Int3 | Xchg
            | Cmpxchg | Pushfd | Setne | Cpuid | Nop | Xgetbv | Setg | Bt | Movsb | Movq
            | Movdqa => {
                write!(w, "todo!();\n");
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

pub fn gen_file(state: &State, outdir: &str) -> Result<()> {
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
