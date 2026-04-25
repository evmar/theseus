//! Instruction stream traversal, scanning for basic blocks.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Block, Module, State, is_abs_memory_ref, memory::Memory};

pub struct Traverse<'a> {
    module: &'a Module,
    mem: &'a Memory,
    iat_refs: &'a HashMap<u32, String>,
    scan_immediates: bool,
    queue: VecDeque<u32>,
    invalid: HashSet<u32>,
    blocks: &'a mut HashMap<u32, Block>,
}

impl<'a> Traverse<'a> {
    pub fn new(state: &'a mut State, scan_immediates: bool) -> Traverse<'a> {
        let mut traverse = Traverse {
            module: &state.module,
            blocks: &mut state.blocks,
            iat_refs: &state.iat_refs,
            mem: &state.mem,
            scan_immediates,
            queue: VecDeque::new(),
            invalid: HashSet::new(),
        };
        traverse.enqueue(state.module.entry_point);
        traverse
    }

    pub fn add_extern(&mut self, addr: u32) {
        self.blocks.insert(addr, Block::Extern(addr));
    }

    fn enqueue(&mut self, ip: u32) {
        self.queue.push_back(ip);
    }

    pub fn run(&mut self) {
        while let Some(ip) = self.queue.pop_front() {
            if self.blocks.contains_key(&ip) || self.invalid.contains(&ip) {
                continue;
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

            if self.scan_immediates {
                for i in 0..instr.op_count() {
                    if instr.op_kind(i) == iced_x86::OpKind::Immediate32 {
                        let imm = instr.immediate32();
                        if self.module.code_memory.contains(&imm) {
                            log::info!("{imm:x} looks like a code pointer");
                            self.enqueue(imm);
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
                        iced_x86::OpKind::NearBranch32 => self.enqueue(instr.near_branch32()),
                        iced_x86::OpKind::Memory => {
                            if let Some(addr) = is_abs_memory_ref(&instr) {
                                if self.iat_refs.contains_key(&addr) {
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
                        self.enqueue(instr.next_ip32());
                    }
                }
                Ret => {}
                Int => {}         // terminates
                Int1 | Int3 => {} // breakpoint
                INVALID => {
                    anyhow::bail!("invalid code found");
                }
                _ => todo!("control flow {}", instr),
            }
            break;
        }

        Ok(Block::Instrs(instrs))
    }

    pub fn scan_for_pointers(&mut self) {
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
                    self.enqueue(value);
                }
            }
        }
    }
}
