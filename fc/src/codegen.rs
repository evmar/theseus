#![allow(unused_must_use)]

use anyhow::{Result, anyhow, bail};

use crate::{Block, State, is_abs_memory_ref, memory::AddrAbs};

fn get_reg(r: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match r {
        EAX | ECX | EDX | EBX | ESI | EDI | ESP | EBP => {
            let reg = format!("{r:?}").to_ascii_lowercase();
            format!("MACHINE.regs.{reg}")
        }

        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX => {
            let reg = format!("{r:?}").to_ascii_lowercase();
            format!("MACHINE.regs.get_{reg}()")
        }

        r => todo!("{r:?}"),
    }
}

fn set_reg(r: iced_x86::Register, expr: String) -> String {
    use iced_x86::Register::*;
    match r {
        EAX | ECX | EDX | EBX | ESI | EDI | ESP | EBP => {
            let reg = format!("{r:?}").to_ascii_lowercase();
            format!("MACHINE.regs.{reg} = {expr};")
        }

        AL | AH | AX | CL | CH | CX | DL | DH | DX | BL | BH | BX => {
            let reg = format!("{r:?}").to_ascii_lowercase();
            format!("MACHINE.regs.set_{reg}({expr});")
        }
        r => todo!("{r:?}"),
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
        r => expr.push(get_reg(r)),
    }
    if instr.memory_index() != iced_x86::Register::None {
        expr.push(format!(
            "({}*{})",
            get_reg(instr.memory_index()),
            instr.memory_index_scale()
        ));
    }
    let addr = instr.memory_displacement32();
    expr.push(format!("{addr:#x}u32"));
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

fn get_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Immediate8 => format!("{:#x}u8", instr.immediate8()),
        Immediate16 => format!("{:#x}u16", instr.immediate16()),
        Immediate8to16 => format!("{:#x}u16", instr.immediate8to16()),
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => get_reg(instr.op_register(n)),
        Memory => {
            let addr = gen_addr(instr);
            let size = match instr.memory_size() {
                iced_x86::MemorySize::UInt8 => "u8",
                iced_x86::MemorySize::UInt16 => "u16",
                iced_x86::MemorySize::UInt32 => "u32",
                s => todo!("{s:?}"),
            };
            format!("*(MACHINE.memory.add({addr} as usize) as *mut {size})")
        }
        k => {
            dbg!(instr);
            todo!("{:?}", k);
        }
    }
}

fn set_op(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => set_reg(instr.op_register(n), expr),
        Memory => {
            let addr = gen_addr(instr);
            let size = match instr.memory_size() {
                iced_x86::MemorySize::UInt8 => "u8",
                iced_x86::MemorySize::UInt16 => "u16",
                iced_x86::MemorySize::UInt32 => "u32",
                s => todo!("{s:?}"),
            };
            format!("*(MACHINE.memory.add({addr} as usize) as *mut {size}) = {expr};")
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
                    format!(
                        "(MACHINE.indirect)(*(MACHINE.memory.add({addr:#x}u32 as usize) as *const u32))"
                    )
                }
            } else {
                format!("indirect({})", gen_addr(instr))
            }
        }
        iced_x86::OpKind::Register => {
            format!("indirect({})", get_reg(instr.op0_register()))
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
        writeln!(w, "// {:08x} {}", AddrAbs(instr.ip32()).0, instr);
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Push => {
                writeln!(w, "push({});", get_op(instr, 0));
            }
            Pop => {
                writeln!(w, "{};", set_op(instr, 0, "pop()".into()));
            }

            Jmp => {
                writeln!(w, "{}", gen_jmp(state, instr));
            }

            Call => {
                writeln!(
                    w,
                    "call({:#08x}, {})",
                    instr.next_ip32(),
                    gen_jmp(state, instr)
                );
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
                writeln!(w, "ret({n})");
            }

            Xor => {
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                writeln!(w, "{op0} ^= {op1};");
            }

            // Binary operations.
            And | Or | Add | Sub | Sbb | Shl | Shr => {
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                let func = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                let _ = writeln!(w, "{};", set_op(instr, 0, format!("{func}({op0}, {op1})")));
            }

            Cmp => {
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                writeln!(w, "sub({op0}, {op1});");
            }
            Test => {
                writeln!(w, "and({}, {});", get_op(instr, 0), get_op(instr, 1));
            }

            Mov => {
                writeln!(w, "{};", set_op(instr, 0, get_op(instr, 1)));
            }

            // Conditional jumps.
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe => {
                let next = gen_abs_jmp(state, instr.next_ip32());
                let dst = gen_jmp(state, instr);
                let func = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                writeln!(w, "{func}({next}, {dst})");
            }

            Lea => {
                writeln!(w, "{} = {};", get_op(instr, 0), gen_addr(instr));
            }
            Neg => {
                writeln!(
                    w,
                    "{};",
                    set_op(instr, 0, format!("neg({})", get_op(instr, 0)))
                );
            }

            Stosd => {
                writeln!(w, "stosd();");
            }
            Scasb => {
                writeln!(w, "scasb();");
            }
            Cmpsb => {
                writeln!(w, "cmpsb();");
            }
            Movzx => {
                writeln!(w, "movzx();");
            }
            Movsx => {
                writeln!(w, "movsx();");
            }
            Movsd => {
                writeln!(w, "movsd();");
            }
            Std => {
                writeln!(w, "std();");
            }
            Cld => {
                writeln!(w, "cld();");
            }
            Stosb => {
                writeln!(w, "stosb();");
            }
            Div => {
                writeln!(w, "div();");
            }
            Leave => {
                writeln!(w, "leave();");
            }
            Dec => {
                writeln!(w, "dec();");
            }
            Inc => {
                writeln!(w, "inc();");
            }
            Sete => {
                writeln!(w, "{};", set_op(instr, 0, "sete()".into()));
            }
            Sar => {
                writeln!(w, "sar();");
            }
            Imul => {
                writeln!(w, "imul();");
            }
            Not => {
                writeln!(w, "not();");
            }
            Setge => {
                writeln!(w, "setge();");
            }
            Int => {
                writeln!(w, "int();");
            }
            Cdq => {
                writeln!(w, "cdq();");
            }
            Idiv => {
                writeln!(w, "idiv();");
            }
            Int3 => {
                writeln!(w, "int3();");
            }
            Xchg => {
                writeln!(w, "xchg();");
            }
            Cmpxchg => {
                writeln!(w, "cmpxchg();");
            }
            Pushfd => {
                writeln!(w, "pushfd();");
            }
            Setne => {
                writeln!(w, "setne();");
            }
            Cpuid => {
                writeln!(w, "cpuid();");
            }
            Nop => {
                writeln!(w, "nop();");
            }
            Xgetbv => {
                writeln!(w, "xgetbv();");
            }
            Setg => {
                writeln!(w, "setg();");
            }
            Bt => {
                writeln!(w, "bt();");
            }
            Movsb => {
                writeln!(w, "movsb();");
            }
            Movq => {
                writeln!(w, "movq();");
            }
            Movdqa => {
                writeln!(w, "movdqa();");
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
    write!(&mut text, "#![allow(unused_parens)]\n\n");

    write!(&mut text, "use runtime::*;\n");
    write!(&mut text, "use winapi::*;\n\n");

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
