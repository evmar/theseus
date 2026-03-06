mod memory;

use anyhow::{Result, anyhow, bail};
use std::collections::HashMap;

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

fn is_abs_addr(instr: iced_x86::Instruction) -> Option<u32> {
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

fn gen_op(instr: iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    use iced_x86::Register::*;
    match instr.op_kind(n) {
        Immediate8to32 => format!("{:#x}u32", instr.immediate8to32()),
        Immediate32 => format!("{:#x}u32", instr.immediate32()),
        Register => match instr.op_register(n) {
            EAX => format!("REGS.eax"),
            ECX => format!("REGS.ecx"),
            EDX => format!("REGS.edx"),
            EBX => format!("REGS.ebx"),
            r => todo!("{:?}", r),
        },
        k => todo!("{:?}", k),
    }
}

fn gen_block(w: &mut dyn std::fmt::Write, state: &State, buf: &[u8], ip: AddrAbs) {
    let mut decoder =
        iced_x86::Decoder::with_ip(32, buf, ip.0 as u64, iced_x86::DecoderOptions::NONE);

    for instr in &mut decoder {
        write!(w, "// {:08x} {}\n", AddrAbs(instr.ip32()).0, instr);
        match instr.mnemonic() {
            iced_x86::Mnemonic::Push => {
                write!(w, "push({});\n", gen_op(instr, 0));
            }
            iced_x86::Mnemonic::Call => {
                if let Some(addr) = is_abs_addr(instr) {
                    if let Some((dll, func)) = state.imports.get(&addr) {
                        write!(w, "todo!(\"{dll}:{func}\");\n");
                    } else {
                        todo!("{}", instr);
                    }
                } else {
                    todo!("{}", instr);
                }
            }
            iced_x86::Mnemonic::Xor => {
                let op0 = gen_op(instr, 0);
                let op1 = gen_op(instr, 1);
                write!(w, "{op0} ^= {op1};\n");
            }
            iced_x86::Mnemonic::Ret => {
                write!(w, "return None;\n");
            }

            c => todo!("{:?}", c),
        }
        if instr.flow_control() != iced_x86::FlowControl::Next {
            match instr.mnemonic() {
                iced_x86::Mnemonic::Call => {}
                _ => break,
            }
        }
    }
}

fn gen_file(state: &State, outdir: &str) -> Result<()> {
    use std::fmt::Write;
    let mut text = String::new();

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint).to_abs(state.image_base());
    write!(&mut text, "use runtime::{{REGS, push}};\n");

    write!(&mut text, "pub fn x{:08x}() -> Option<u32> {{\n", ip.0);
    write!(&mut text, "unsafe {{\n");
    gen_block(&mut text, &state, &state.mem.data[ip.0 as usize..], ip);
    write!(&mut text, "}}}}\n");

    let path = format!("{outdir}/src/generated.rs");
    let text = rustfmt(&text)?;
    std::fs::write(&path, text).map_err(|err| anyhow!("{path}: {err}"))?;
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
