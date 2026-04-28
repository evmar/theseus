//! Instruction stream traversal, scanning for basic blocks.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::{Block, Module, State, is_abs_memory_ref, memory::Memory};

#[derive(Default)]
pub struct Gather {
    pub scan_immediates: bool,
    pub scan_memory: bool,
    pub entry_points: Vec<u32>,
    pub jump_tables: Vec<std::ops::RangeInclusive<u32>>,
    pub externs: Vec<u32>,
}

impl Gather {
    pub fn run(self, state: &mut State) -> HashMap<u32, Block> {
        let mut traverse = Traverse::new(state, self);
        traverse.run();
        traverse.blocks.into_iter().collect()
    }
}

struct Traverse<'a> {
    gather: Gather,
    module: &'a Module,
    mem: &'a Memory,

    iat_refs: HashSet<u32>,
    queue: VecDeque<u32>,
    invalid: HashSet<u32>,
    blocks: BTreeMap<u32, Block>,
}

impl<'a> Traverse<'a> {
    pub fn new(state: &'a mut State, gather: Gather) -> Traverse<'a> {
        Traverse {
            gather,
            module: &state.module,
            mem: &state.mem,

            iat_refs: Default::default(),
            queue: VecDeque::new(),
            invalid: HashSet::new(),
            blocks: Default::default(),
        }
    }

    pub fn run(&mut self) {
        for import in &self.module.imports {
            let func = format!("{}::{}", import.dll, import.func);
            self.blocks
                .insert(import.func_addr, Block::Stdcall(func.clone()));
            self.iat_refs.insert(import.iat_addr);
        }

        for &addr in self.gather.externs.iter() {
            self.blocks.insert(addr, Block::Extern(addr));
        }
        self.queue.push_back(self.module.entry_point);
        for &addr in self.gather.entry_points.iter() {
            self.queue.push_back(addr);
        }
        for range in self.gather.jump_tables.iter() {
            for addr in range.clone().step_by(4) {
                let addr = self.mem.read::<u32>(addr);
                self.queue.push_back(addr);
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
            if let Some((&addr, Block::Instrs(instrs))) = self.blocks.range(0..ip).last() {
                let range = instrs.first().unwrap().ip32()..instrs.last().unwrap().next_ip32();
                if range.contains(&ip) {
                    self.blocks.remove(&addr);
                    self.queue.push_back(addr);
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
        let decoder =
            iced_x86::Decoder::with_ip(32, data, ip as u64, iced_x86::DecoderOptions::NONE);
        for instr in decoder {
            if self.blocks.contains_key(&instr.ip32()) {
                // Hit a point covered by another block, e.g. a jump target
                break;
            }
            instrs.push(instr);

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

            if instr.flow_control() == iced_x86::FlowControl::Next {
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
                        iced_x86::OpKind::NearBranch32 => {
                            self.queue.push_back(instr.near_branch32())
                        }
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if self.iat_refs.contains(&addr) {
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
                        d => anyhow::bail!("unhandled jmp {d:?}"),
                    }
                    if instr.mnemonic() != Jmp {
                        self.queue.push_back(instr.next_ip32());
                    }
                }
                Ret | Retf => {}
                Int | Into => {}  // terminates
                Int1 | Int3 => {} // breakpoint
                INVALID => {
                    anyhow::bail!("invalid code found");
                }
                _ => todo!("{ip:08x} control flow {}", instr),
            }
            break;
        }

        Ok(Block::Instrs(instrs))
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
