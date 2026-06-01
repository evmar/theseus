//! Instruction stream traversal, scanning for basic blocks.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::{Block, BlockType, Import, Instr, Module, State, memory::Memory};

/// If the instruction looks like
///   foo [x]
/// where x is a constant, return the value of x.
pub fn is_abs_memory_ref(instr: &iced_x86::Instruction) -> Option<u32> {
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

#[derive(Clone)]
pub enum EntryPoint {
    Single(u32),
    Range(std::ops::Range<u32>),
}

#[derive(Default)]
pub struct Gather {
    pub externs: Vec<u32>,
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

struct Traverse<'a> {
    gather: &'a Gather,
    module: &'a Module,
    mem: &'a Memory,
    symbol_names: &'a HashMap<u32, String>,

    iat_refs: HashMap<u32, &'a Import>,
    queue: VecDeque<u32>,
    invalid: HashSet<u32>,
    blocks: BTreeMap<u32, Block>,
}

impl<'a> Traverse<'a> {
    pub fn new(state: &'a mut State, gather: &'a Gather) -> Traverse<'a> {
        Traverse {
            gather,
            module: &state.module,
            mem: &state.mem,
            symbol_names: &state.symbol_names,

            iat_refs: Default::default(),
            queue: VecDeque::new(),
            invalid: HashSet::new(),
            blocks: Default::default(),
        }
    }

    pub fn run(&mut self) {
        for import in &self.module.imports {
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

        for &addr in self.gather.externs.iter() {
            self.blocks.insert(
                addr,
                Block {
                    name: self.symbol_names.get(&addr).cloned(),
                    ty: BlockType::Extern(addr),
                },
            );
        }

        self.queue.push_back(self.module.entry_point);
        for entry_point in self.gather.entry_points.iter() {
            match entry_point {
                EntryPoint::Single(addr) => self.queue.push_back(*addr),
                EntryPoint::Range(r) => {
                    let mut ip = r.start;
                    while ip < r.end {
                        let Ok(block) = self.decode_one(ip) else {
                            log::warn!("failed to decode range {r:#x?} at {:#x}", ip);
                            break;
                        };
                        let BlockType::Instrs(instrs) = &block.ty else {
                            unreachable!();
                        };
                        let next = instrs.last().unwrap().next_ip();
                        self.blocks.insert(ip, block);
                        ip = next;
                    }
                }
            }
        }
        if self.gather.scan_memory {
            self.scan_for_pointers();
        }

        while let Some(ip) = self.queue.pop_front() {
            if self.blocks.contains_key(&ip) || self.invalid.contains(&ip) {
                continue;
            }

            // If this ip is contained within an existing block, it means it is a
            // jmp within some other code.
            // Re-queue the other block for re-parsing after this one so that it can be split.
            if let Some((&addr, block)) = self.blocks.range(0..ip).last() {
                if let BlockType::Instrs(instrs) = &block.ty {
                    let range =
                        instrs.first().unwrap().iced.ip32()..instrs.last().unwrap().next_ip();
                    if range.contains(&ip) {
                        self.blocks.remove(&addr);
                        self.queue.push_back(addr);
                    }
                }
            }

            match self.decode_one(ip) {
                Ok(block) => {
                    self.blocks.insert(ip, block);
                }
                Err(e) => {
                    log::warn!("omitting {ip:08x}: {e}");
                    self.invalid.insert(ip);
                }
            }
        }
    }

    fn decode_one(&mut self, ip: u32) -> anyhow::Result<Block> {
        if ip > self.mem.bytes.len() as u32 {
            anyhow::bail!("ip out of bounds");
        }
        let data = self.mem.slice_all(ip);
        if data[..0x10].iter().all(|&b| b == 0) {
            anyhow::bail!("block appears zero-filled");
        }

        let mut instrs = Vec::new();
        let decoder = iced_x86::Decoder::with_ip(
            self.module.bitness,
            data,
            ip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        for instr in decoder {
            // log::info!("{ip:08x} {instr}", ip = instr.ip32());
            if self.blocks.contains_key(&instr.ip32()) {
                // Hit a point covered by another block, e.g. a jump target
                break;
            }
            let new_instr = instrs.push_mut(Instr {
                iced: instr,
                hint: None,
            });

            if self.gather.scan_immediates {
                for i in 0..instr.op_count() {
                    if instr.op_kind(i) == iced_x86::OpKind::Immediate32 {
                        let imm = instr.immediate32();
                        if self.module.code_memory.contains(&imm) {
                            log::info!("{imm:x} looks like a code pointer");
                            self.queue.push_back(imm);
                        }
                    }
                }
            }

            if instr.flow_control() == iced_x86::FlowControl::Next
                || instr.mnemonic() == iced_x86::Mnemonic::Int
            {
                let next_bytes = &data[(instr.next_ip32() - ip) as usize..][..0x10];
                if next_bytes.iter().all(|&b| b == 0) {
                    anyhow::bail!("suspicious block of 0");
                }
                continue;
            }
            let ip = instr.ip32();
            use iced_x86::Mnemonic::*;
            match instr.mnemonic() {
                Call | Jmp | Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jge | Jecxz | Jg | Jle
                | Jo | Jno | Jp | Jnp | Jbe | Loop | Loope | Loopne => {
                    match instr.op0_kind() {
                        iced_x86::OpKind::NearBranch16 => {
                            self.queue.push_back(instr.near_branch16() as u32)
                        }
                        iced_x86::OpKind::NearBranch32 => {
                            self.queue.push_back(instr.near_branch32())
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
                                    log::warn!("{ip:08x} {instr}  ; indirect via memory");
                                }
                            } else {
                                log::warn!("{ip:08x} {instr}  ; indirect via memory");
                            }
                        }
                        iced_x86::OpKind::Register => {
                            log::warn!("{ip:08x} {instr}  ; indirect via register");
                        }
                        d => anyhow::bail!("unhandled jmp {d:?}"),
                    }
                    if instr.mnemonic() != Jmp {
                        self.queue.push_back(instr.next_ip32());
                    }
                }
                Ret | Retf => {}
                Into => {}        // terminates
                Int1 | Int3 => {} // breakpoint
                INVALID => {
                    anyhow::bail!("invalid code found");
                }
                _ => todo!("{ip:08x} control flow {}", instr),
            }
            break;
        }

        Ok(Block {
            name: self.symbol_names.get(&instrs[0].iced.ip32()).cloned(),
            ty: BlockType::Instrs(instrs),
        })
    }

    fn scan_for_pointers(&mut self) {
        for i in 0..self.mem.mappings.vec().len() {
            let mapping = &self.mem.mappings.vec()[i];
            if mapping.addr == 0 || mapping.addr == self.module.code_memory.start {
                continue;
            }
            log::info!("scanning mapping {:?}", mapping);
            let mapping_addr = mapping.addr;
            let data = self.mem.bytes[mapping.addr as usize..][..mapping.size as usize].to_vec();
            for ofs in 0..data.len() - 4 {
                let value =
                    u32::from_le_bytes([data[ofs], data[ofs + 1], data[ofs + 2], data[ofs + 3]]);
                if self.module.code_memory.contains(&value) {
                    log::info!(
                        "{addr:08x}: found possible code pointer {value:x}",
                        addr = mapping_addr + ofs as u32
                    );
                    self.queue.push_back(value);
                }
            }
        }
    }
}
