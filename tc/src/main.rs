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
        let mut code_range = 0u32..0u32;
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
            if flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::MEM_EXECUTE) {
                code_range.start = code_range.start.min(addr.0);
                code_range.end = code_range.end.max(addr.0 + sec.SizeOfRawData);
            }
        }

        let resource_dir = f
            .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
            .map(|dir| (AddrImage(dir.VirtualAddress).to_abs(image_base).0, dir.Size));

        State {
            pe_file: f,
            mem,
            code_memory: code_range,
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

        for (lib, exports) in [
            ("ddraw", winapi::ddraw::EXPORTS.as_slice()),
            ("dsound", winapi::dsound::EXPORTS.as_slice()),
        ] {
            if imports.iter().find(|i| i.dll == lib).is_some() {
                for func in exports {
                    self.blocks.insert(
                        import_func_addr,
                        Block::Stdcall(format!("{lib}::{}", func.to_string())),
                    );
                    import_func_addr += 1;
                }
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
    Invalid, // used to negatively cache invalid locations
    Instrs(Vec<iced_x86::Instruction>),
    Stdcall(String),
    Extern(u32),
}

impl Block {
    pub fn name(&self) -> String {
        match self {
            Block::Invalid => unimplemented!(),
            Block::Instrs(instrs) => format!("x{:08x}", instrs[0].ip32()),
            Block::Stdcall(func) => format!("{}_stdcall", func),
            Block::Extern(ip) => format!("x{:08x}", ip),
        }
    }
}

pub struct Traverse<'a> {
    state: &'a mut State,
    queue: VecDeque<u32>,
}

impl<'a> Traverse<'a> {
    fn new(state: &'a mut State, start: u32) -> Traverse<'a> {
        let mut traverse = Traverse {
            state,
            queue: VecDeque::new(),
        };
        Self::enqueue(&mut traverse.queue, start);
        traverse
    }

    fn enqueue(queue: &mut VecDeque<u32>, ip: u32) {
        queue.push_back(ip);
    }

    fn run(&mut self) {
        while let Some(ip) = self.queue.pop_front() {
            let block_ip = ip;
            if self.state.blocks.contains_key(&block_ip) {
                continue;
            }
            match self.decode_one(block_ip) {
                Ok(block) => {
                    self.state.blocks.insert(block_ip, block);
                }
                Err(e) => {
                    log::warn!("omitting {block_ip:08x}: {e}");
                    self.state.blocks.insert(block_ip, Block::Invalid);
                }
            }
        }
    }

    fn decode_one(&mut self, ip: u32) -> anyhow::Result<Block> {
        let data = self.state.mem.slice_all(AddrAbs(ip));
        if data[..0x10].iter().all(|&b| b == 0) {
            anyhow::bail!("block appears zero-filled");
        }

        let mut instrs = Vec::new();
        let decoder =
            iced_x86::Decoder::with_ip(32, data, ip as u64, iced_x86::DecoderOptions::NONE);
        for instr in decoder {
            instrs.push(instr);

            if self.state.scan_immediates {
                for i in 0..instr.op_count() {
                    if instr.op_kind(i) == iced_x86::OpKind::Immediate32 {
                        let imm = instr.immediate32();
                        if self.state.code_memory.contains(&imm) {
                            log::info!("{imm:x} looks like a code pointer");
                            Self::enqueue(&mut self.queue, imm);
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
                | Jo | Jno | Jp | Jnp | Jbe | Loop => {
                    match instr.op0_kind() {
                        iced_x86::OpKind::NearBranch32 => {
                            Self::enqueue(&mut self.queue, instr.near_branch32())
                        }
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if self.state.imports.contains_key(&addr) {
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
                    }
                    if instr.mnemonic() != Jmp {
                        Self::enqueue(&mut self.queue, instr.next_ip32());
                    }
                }
                Ret => {}
                Int => {}  // terminates
                Int3 => {} // breakpoint
                INVALID => {
                    anyhow::bail!("invalid code found");
                }
                _ => todo!("control flow {}", instr),
            }
            break;
        }

        Ok(Block::Instrs(instrs))
    }

    fn scan_for_pointers(&mut self) {
        for i in 0..self.state.mem.mappings.vec().len() {
            let mapping = &self.state.mem.mappings.vec()[i];
            if mapping.addr == 0 || mapping.addr == self.state.code_memory.start {
                continue;
            }
            log::info!("scanning mapping {:?}", mapping);
            let mapping_addr = mapping.addr;
            let data =
                self.state.mem.data[mapping.addr as usize..][..mapping.size as usize].to_vec();
            for ofs in 0..data.len() - 4 {
                let value =
                    u32::from_le_bytes([data[ofs], data[ofs + 1], data[ofs + 2], data[ofs + 3]]);
                if self.state.code_memory.contains(&value) {
                    log::info!(
                        "{addr:08x}: found possible code pointer {value:x}",
                        addr = mapping_addr + ofs as u32
                    );
                    Self::enqueue(&mut self.queue, value);
                }
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
    let mut traverse = Traverse::new(&mut state, ip.0);
    if args.scan {
        traverse.scan_for_pointers();
    }
    traverse.run();

    let outdir = &args.out;
    codegen::gen_file(&mut state, outdir)?;

    let data_dir = format!("{outdir}/data");
    std::fs::create_dir_all(&data_dir)?;
    for map in state.mem.mappings.vec().iter() {
        let buf = state.mem.slice(AddrAbs(map.addr), map.size);
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
