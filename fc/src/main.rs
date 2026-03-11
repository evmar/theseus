mod codegen;
mod memory;

use anyhow::Result;
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
            let name = std::str::from_utf8(imp.image_name(image))
                .unwrap()
                .to_lowercase();
            let name = name.trim_end_matches(".dll");
            for (addr, entry) in imp.iat_iter(image) {
                let addr = AddrImage(addr);
                self.imports.insert(
                    addr.to_abs(image_base).0,
                    (name.to_string(), entry.as_import_symbol(image).to_string()),
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

struct Block {
    instrs: Vec<iced_x86::Instruction>,
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
                Call | Jmp | Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jge | Jecxz | Jg | Jle
                | Jbe => {
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
                            let reg = instr.op_register(0);
                            println!("indirect via register dest: {reg:?}");
                        }
                        d => todo!("dest {d:?}"),
                    };
                    if instr.mnemonic() != Jmp {
                        queue.push_back(instr.next_ip32());
                    }
                }
                Ret => {}
                Int => {}  // terminates
                Int3 => {} // breakpoint
                _ => todo!("control flow {}", instr),
            }
            break;
        }

        let block = Block { instrs };
        blocks.insert(ip, block);
    }

    blocks
}

fn run() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, exe_path, outdir] = args.as_slice() else {
        println!("usage: {} exe outdir", args[0]);
        return Ok(());
    };

    let buf = std::fs::read(exe_path).unwrap();
    let mut state = State::new(buf);
    state.mem.mappings.alloc("null page".into(), 0, 0x1000);
    state.read_imports();

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint).to_abs(state.image_base());
    state.blocks = traverse(&state, ip.0);

    codegen::gen_file(&mut state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in state.mem.mappings.iter() {
        std::fs::write(
            format!("{outdir}/data/{:08x}.raw", map.addr),
            state.mem.slice(AddrAbs(map.addr), map.size),
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
