//! Instruction stream traversal, scanning for basic blocks.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use runtime::SegOfs;

use crate::{AddrInfo, Block, BlockType, Import, Instr, Module, State, memory::Memory};

/// If the instruction looks like
///   foo [x]
/// where x is a constant, return the value of x.
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum IP {
    Flat(u32),
    Seg(SegOfs),
}

impl From<u32> for IP {
    fn from(addr: u32) -> Self {
        IP::Flat(addr)
    }
}

impl From<(u16, u16)> for IP {
    fn from(tuple: (u16, u16)) -> Self {
        IP::Seg(tuple.into())
    }
}

impl IP {
    /// called by code that hasn't been updated to be segmentation-aware
    pub fn todo_segmenting(_addr: u32) -> IP {
        todo!();
        //IP::Flat(addr)
    }

    pub fn seg(&self) -> u16 {
        match *self {
            IP::Flat(_) => unreachable!(),
            IP::Seg(addr) => addr.seg,
        }
    }

    pub fn to_addr(&self) -> u32 {
        match *self {
            IP::Flat(addr) => addr,
            IP::Seg(addr) => addr.abs(),
        }
    }

    pub fn local(&self) -> u32 {
        match *self {
            IP::Flat(ip) => ip,
            IP::Seg(addr) => addr.ofs as u32,
        }
    }

    pub fn with_local(&self, local: u32) -> IP {
        match *self {
            IP::Flat(_) => IP::Flat(local),
            IP::Seg(addr) => IP::Seg((addr.seg, local as u16).into()),
        }
    }
}

impl std::fmt::Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IP::Flat(ip) => write!(f, "{ip:08x}"),
            IP::Seg(addr) => write!(f, "{addr}"),
        }
    }
}

impl std::fmt::Debug for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
pub enum EntryPoint {
    Single(IP),
    Range(std::ops::Range<IP>),
}

#[derive(Default)]
pub struct Gather {
    pub scan_immediates: bool,
    pub scan_memory: bool,

    pub entry_points: Vec<EntryPoint>,
}

impl Gather {
    pub fn run(self, state: &mut State) -> HashMap<u32, Block> {
        let mut traverse = Traverse::new(state, &self);
        traverse.run();
        traverse.blocks.into_iter().collect()
    }
}

/// Wrap a VecDeque<IP> just so we add some logic every time it's called.
#[derive(Default)]
struct IPQueue(VecDeque<IP>);
impl IPQueue {
    pub fn enqueue(&mut self, ip: IP) {
        // log::info!("enqueue {ip}");
        // let IP::Seg(seg, ofs) = ip else { panic!() };
        // if ofs > 0x8000 {
        //     panic!();
        // }
        self.0.push_back(ip);
    }
    pub fn pop(&mut self) -> Option<IP> {
        self.0.pop_front()
    }
}

struct Traverse<'a> {
    gather: &'a Gather,
    module: &'a Module,
    mem: &'a Memory,
    addr_info: &'a HashMap<u32, AddrInfo>,

    iat_refs: HashMap<u32, &'a Import>,
    queue: IPQueue,
    invalid: HashSet<u32>,
    blocks: BTreeMap<u32, Block>,
}

impl<'a> Traverse<'a> {
    fn new(state: &'a mut State, gather: &'a Gather) -> Traverse<'a> {
        Traverse {
            gather,
            module: &state.module,
            mem: &state.mem,
            addr_info: &state.addr_info,

            iat_refs: Default::default(),
            queue: IPQueue::default(),
            invalid: HashSet::new(),
            blocks: Default::default(),
        }
    }

    fn run(&mut self) {
        if let Module::Windows(module) = self.module {
            for import in &module.imports {
                if !import.data {
                    let func = format!("{}::{}", import.dll, import.func);
                    self.blocks.insert(
                        import.addr,
                        Block {
                            name: None, // block.name() will use the stdcall name
                            ty: BlockType::Stdcall(func),
                        },
                    );
                }
                self.iat_refs.insert(import.iat_addr, &import);
            }
        }

        for (&addr, info) in self.addr_info.iter() {
            if info.is_extern {
                self.blocks.insert(
                    addr,
                    Block {
                        name: Some(info.name.clone()),
                        ty: BlockType::Extern(addr),
                    },
                );
            }
        }

        self.queue.enqueue(self.module.entry_point());
        for entry_point in self.gather.entry_points.iter() {
            match entry_point {
                EntryPoint::Single(addr) => self.queue.enqueue(*addr),
                EntryPoint::Range(r) => {
                    let mut ip = r.start;
                    while ip < r.end {
                        let Ok(block) = self.decode_one(ip) else {
                            log::warn!("failed to decode range {r:#?} at {}", ip);
                            break;
                        };
                        let BlockType::Instrs(instrs) = &block.ty else {
                            unreachable!();
                        };
                        let next = instrs.last().unwrap().next_ip();
                        self.blocks.insert(ip.to_addr(), block);
                        ip = next;
                    }
                }
            }
        }
        if self.gather.scan_memory {
            self.scan_for_pointers();
        }

        while let Some(ip) = self.queue.pop() {
            let addr = ip.to_addr();
            if self.blocks.contains_key(&addr) || self.invalid.contains(&addr) {
                continue;
            }

            // If this ip is contained within an existing block, it means it is a
            // jmp within some other code.
            // Re-queue the other block for re-parsing after this one so that it can be split.
            if let Some((&baddr, block)) = self.blocks.range(0..addr).last() {
                if let BlockType::Instrs(instrs) = &block.ty {
                    let range = instrs.first().unwrap().ip.to_addr()
                        ..instrs.last().unwrap().next_ip().to_addr();
                    if range.contains(&addr) {
                        self.queue.enqueue(instrs[0].ip);
                        self.blocks.remove(&baddr);
                    }
                }
            }

            match self.decode_one(ip) {
                Ok(block) => {
                    self.blocks.insert(addr, block);
                }
                Err(e) => {
                    log::warn!("omitting {ip}: {e}");
                    self.invalid.insert(addr);
                }
            }
        }
    }

    fn decode_one(&mut self, block_ip: IP) -> anyhow::Result<Block> {
        // log::info!("decode block {block_ip}");
        let block_addr = block_ip.to_addr();
        if block_addr > self.mem.bytes.len() as u32 {
            anyhow::bail!("ip out of bounds");
        }
        let data = self.mem.slice_all(block_addr);
        if data.len() > 0x10 && data[..0x10].iter().all(|&b| b == 0) {
            anyhow::bail!("block appears zero-filled");
        }

        let mut instrs = Vec::new();
        let decoder = iced_x86::Decoder::with_ip(
            self.module.bitness(),
            data,
            block_ip.local() as u64,
            iced_x86::DecoderOptions::NONE,
        );
        for instr in decoder {
            let ip = block_ip.with_local(instr.ip32());
            // log::info!("{ip:08x} {instr}", ip = instr.ip32());
            if self.blocks.contains_key(&ip.to_addr()) {
                // Hit a point covered by another block, e.g. a jump target
                break;
            }

            if instr.mnemonic() == iced_x86::Mnemonic::Out && !self.module.is_dos() {
                anyhow::bail!("'out' instruction in non-DOS code");
            }

            instrs.push(Instr {
                ip,
                iced: instr,
                hint: None,
            });
            let new_instr = instrs.last_mut().unwrap();

            if self.gather.scan_immediates {
                for i in 0..instr.op_count() {
                    if instr.op_kind(i) == iced_x86::OpKind::Immediate32 {
                        let imm = instr.immediate32();
                        if self.module.code_memory().contains(&imm) {
                            log::info!("{imm:x} looks like a code pointer");
                            assert!(!self.module.segment_addressed());
                            self.queue.enqueue(IP::Flat(imm));
                        }
                    }
                }
            }

            if instr.flow_control() == iced_x86::FlowControl::Next {
                let next_ip = block_ip.with_local(instr.next_ip32());
                let next_bytes = &data[(next_ip.to_addr() - block_addr) as usize..];
                if next_bytes.len() > 0x10 && next_bytes[..0x10].iter().all(|&b| b == 0) {
                    anyhow::bail!("suspicious block of 0");
                }
                continue;
            }
            let ip = block_ip.with_local(instr.ip32());
            use iced_x86::Mnemonic::*;
            match instr.mnemonic() {
                Call | Jmp | Jcxz | Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jge | Jecxz | Jg
                | Jle | Jo | Jno | Jp | Jnp | Jbe | Loop | Loope | Loopne => {
                    match instr.op0_kind() {
                        iced_x86::OpKind::NearBranch16 => self
                            .queue
                            .enqueue(block_ip.with_local(instr.near_branch16() as u32)),
                        iced_x86::OpKind::NearBranch32 => self
                            .queue
                            .enqueue(block_ip.with_local(instr.near_branch32())),
                        iced_x86::OpKind::FarBranch16 => {
                            let ip =
                                IP::Seg((instr.far_branch_selector(), instr.far_branch16()).into());
                            self.queue.enqueue(ip);
                        }
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if let Some(imp) = self.iat_refs.get(&addr) {
                                    new_instr.hint =
                                        Some(format!("{}::{}_stdcall", imp.dll, imp.func));
                                    if instr.mnemonic() == iced_x86::Mnemonic::Call {
                                        continue; // don't end block here
                                    }
                                } else {
                                    log::warn!("{ip} {instr}  ; indirect via memory");
                                }
                            } else {
                                log::warn!("{ip} {instr}  ; indirect via memory");
                            }
                        }
                        iced_x86::OpKind::Register => {
                            log::warn!("{ip} {instr}  ; indirect via register");
                        }
                        d => anyhow::bail!("unhandled jmp {d:?}"),
                    }
                    if instr.mnemonic() != Jmp {
                        self.queue.enqueue(block_ip.with_local(instr.next_ip32()));
                    }
                }
                Ret | Retf | Iret => {}
                Into => {}        // terminates
                Int1 | Int3 => {} // breakpoint
                Int => {
                    self.queue.enqueue(block_ip.with_local(instr.next_ip32()));
                }
                Syscall | Sysexit | Sysret => anyhow::bail!("syscall not implemented"),
                INVALID => anyhow::bail!("invalid code found"),
                _ => todo!("{ip} control flow {}", instr),
            }
            break;
        }

        let info = self.addr_info.get(&block_ip.to_addr());
        Ok(Block {
            name: info.map(|info| info.name.clone()),
            ty: BlockType::Instrs(instrs),
        })
    }

    fn scan_for_pointers(&mut self) {
        for i in 0..self.mem.mappings.vec().len() {
            let mapping = &self.mem.mappings.vec()[i];
            if mapping.addr == 0 || mapping.addr == self.module.code_memory().start {
                continue;
            }
            log::info!("scanning mapping {:?}", mapping);
            let mapping_addr = mapping.addr;
            if self.module.segment_addressed() {
                todo!();
            }
            let data = self.mem.bytes[mapping.addr as usize..][..mapping.size as usize].to_vec();
            for ofs in 0..data.len() - 4 {
                let value =
                    u32::from_le_bytes([data[ofs], data[ofs + 1], data[ofs + 2], data[ofs + 3]]);
                if self.module.code_memory().contains(&value) {
                    log::info!(
                        "{addr:08x}: found possible code pointer {value:x}",
                        addr = mapping_addr + ofs as u32
                    );
                    self.queue.enqueue(IP::Flat(value));
                }
            }
        }
    }
}
