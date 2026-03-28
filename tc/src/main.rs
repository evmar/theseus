mod codegen;
mod memory;

use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use memory::*;

struct Import {
    dll: String,
    func: String,
    // address to write func_addr to
    iat_addr: u32,
    // address of code
    func_addr: u32,
}

pub struct State {
    pe_file: pe::File,
    mem: Memory,
    code_memory: std::ops::Range<u32>,
    imports: HashMap<u32, Import>,
    blocks: HashMap<u32, Block>,
    resources: Option<(u32, u32)>,

    scan_immediates: bool,
}

impl State {
    fn new(buf: Vec<u8>) -> State {
        let f = pe::File::parse(&buf).unwrap();
        let mut mem = Memory::default();

        let image_base = AddrAbs(f.opt_header.ImageBase);
        mem.alloc("exe header".into(), image_base, 0x1000);
        mem.put(image_base, &buf[..0x1000.min(buf.len())]);
        let mut code_range = None;
        for sec in &f.sections {
            let addr = AddrImage(sec.VirtualAddress).to_abs(image_base);
            let size = winapi::kernel32::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
            mem.alloc(sec.name().unwrap().into(), addr, size);

            let flags = sec.characteristics().unwrap();
            let load_data = flags.contains(pe::IMAGE_SCN::CODE)
                || flags.contains(pe::IMAGE_SCN::INITIALIZED_DATA);
            if load_data {
                let data = &buf[sec.PointerToRawData as usize..][..sec.SizeOfRawData as usize];
                mem.put(addr, data);
            }
            if flags.contains(pe::IMAGE_SCN::CODE) {
                assert!(code_range.is_none());
                code_range = Some(addr.0..(addr.0 + sec.SizeOfRawData));
            }
        }

        let resource_dir = f
            .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
            .map(|dir| (AddrImage(dir.VirtualAddress).to_abs(image_base).0, dir.Size));

        State {
            pe_file: f,
            mem,
            code_memory: code_range.unwrap(),
            imports: Default::default(),
            blocks: Default::default(),
            resources: resource_dir,
            scan_immediates: false,
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
                let addr = AddrImage(addr).to_abs(image_base);
                self.imports.insert(
                    addr.0,
                    Import {
                        dll: name.to_string(),
                        func: match entry.as_import_symbol(image) {
                            pe::ImportSymbol::Name(name) => {
                                std::str::from_utf8(name).unwrap().to_string()
                            }
                            pe::ImportSymbol::Ordinal(n) => format!("ordinal{n}"),
                        },
                        iat_addr: addr.0,
                        func_addr: 0,
                    },
                );
            }
        }

        // Reserve some fake addresses for imported functions so they can be assigned addresses.
        // If we never write to the memory it stays zero and doesn't end up in the output.
        let mut import_func_addr =
            self.mem
                .mappings
                .alloc("imported functions".into(), None, self.imports.len() as u32);

        let mut imports = self.imports.values_mut().collect::<Vec<_>>();
        imports.sort_by_key(|i| i.iat_addr);
        for import in imports.iter_mut() {
            import.func_addr = import_func_addr;
            import_func_addr += 1;
            self.mem.write::<u32>(import.iat_addr, import.func_addr);
            self.blocks.insert(
                import.func_addr,
                Block::Stdcall(format!("{}::{}", import.dll, import.func)),
            );
        }

        if imports.iter().find(|i| i.dll == "ddraw").is_some() {
            for func in winapi::ddraw::EXPORTS {
                self.blocks.insert(
                    import_func_addr,
                    Block::Stdcall(format!("ddraw::{}", func.to_string())),
                );
                import_func_addr += 1;
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

fn traverse(state: &mut State, start: u32) {
    if state.blocks.contains_key(&start) {
        return;
    }

    let mut queue = VecDeque::<u32>::new();
    queue.push_back(start);

    'queue: while let Some(ip) = queue.pop_front() {
        if state.blocks.contains_key(&ip) {
            continue;
        }

        let mut instrs = Vec::new();
        let decoder = iced_x86::Decoder::with_ip(
            32,
            state.mem.slice_all(AddrAbs(ip)),
            ip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        for instr in decoder {
            instrs.push(instr);

            if state.scan_immediates {
                for i in 0..instr.op_count() {
                    if instr.op_kind(i) == iced_x86::OpKind::Immediate32 {
                        let imm = instr.immediate32();
                        if state.code_memory.contains(&imm) {
                            log::info!("{imm:x} looks like a code pointer");
                            queue.push_back(imm);
                        }
                    }
                }
            }

            if instr.flow_control() == iced_x86::FlowControl::Next {
                continue;
            }
            let ip = instr.ip32();
            use iced_x86::Mnemonic::*;
            match instr.mnemonic() {
                Call | Jmp | Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jge | Jecxz | Jg | Jle
                | Jno | Jnp | Jbe | Loop => {
                    match instr.op0_kind() {
                        iced_x86::OpKind::NearBranch32 => queue.push_back(instr.near_branch32()),
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if state.imports.contains_key(&addr) {
                                    // ok
                                } else {
                                    log::warn!("{ip:08x} {instr}  ; indirect via memory");
                                }
                            } else {
                                log::warn!("{ip:08x} {instr}  ; indirect via memory");
                            }
                        }
                        iced_x86::OpKind::Register => {
                            log::warn!("{ip:08x} {instr}  ; indirect via register");
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
                INVALID => {
                    log::error!("aborting block at {start:x}, invalid code found");
                    continue 'queue;
                }
                _ => todo!("control flow {}", instr),
            }
            break;
        }

        state.blocks.insert(ip, Block::Instrs(instrs));
    }
}

fn scan_for_pointers(state: &mut State) {
    for i in 0..state.mem.mappings.vec().len() {
        let mapping = &state.mem.mappings.vec()[i];
        if mapping.addr == 0 || mapping.addr == state.code_memory.start {
            continue;
        }
        log::info!("scanning mapping {:?}", mapping);
        let mapping_addr = mapping.addr;
        let data = state.mem.data[mapping.addr as usize..][..mapping.size as usize].to_vec();
        for ofs in 0..data.len() - 4 {
            let value =
                u32::from_le_bytes([data[ofs], data[ofs + 1], data[ofs + 2], data[ofs + 3]]);
            if state.code_memory.contains(&value) {
                log::info!(
                    "{addr:08x}: found possible code pointer {value:x}",
                    addr = mapping_addr + ofs as u32
                );
                traverse(state, value);
            }
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

fn run() -> Result<()> {
    logger::init();
    let args: Args = argh::from_env();

    let buf = std::fs::read(args.exe).unwrap();
    let mut state = State::new(buf);
    if args.scan_immediates {
        state.scan_immediates = true;
    }

    for addr in args.externs {
        log::info!("extern: {addr:#x}");
        state.blocks.insert(addr, Block::Extern(addr));
    }

    state
        .mem
        .mappings
        .alloc("null page".into(), Some(0), 0x1000);
    state.read_imports();

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint).to_abs(state.image_base());
    traverse(&mut state, ip.0);
    if args.scan {
        scan_for_pointers(&mut state);
    }

    let outdir = &args.out;
    codegen::gen_file(&mut state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in state.mem.mappings.vec().iter() {
        let buf = state.mem.slice(AddrAbs(map.addr), map.size);
        if buf.iter().all(|&b| b == 0) {
            continue;
        }
        std::fs::write(format!("{outdir}/data/{:08x}.raw", map.addr), buf)?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        log::error!("error: {err}");
        std::process::exit(1);
    }
}
