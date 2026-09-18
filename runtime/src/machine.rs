use std::cell::RefCell;

use crate::{Cont, ContFn, Flags, Memory, Regs, SegOfs, fpu::FPU, mmx::MMX, segofs};

#[derive(Default)]
pub struct CPU {
    pub regs: Regs,
    pub flags: Flags,
    pub fpu: FPU,
    pub mmx: MMX,
    pub real_mode: bool,
}

impl CPU {
    pub fn dump(&self) {
        self.regs.dump();
        if self.real_mode {
            self.regs.dump_segments();
        }
        // self.flags.dump();
        // self.fpu.dump();
        // self.mmx.dump();
    }
}

/// Cache of recently taken indirect jumps.
/// Often when these are hot it's because they're in some sort of loop, so
/// even a simple cache is effective.  One program I tested in retrowin32
/// had cache hits on 99.7% of lookups.
pub struct BlockCache {
    /// Simple hash table based on low bits of address.
    slots: [Option<(u32, ContFn)>; BlockCache::SIZE],
}

impl BlockCache {
    const SIZE: usize = 64;

    fn slot(addr: u32) -> usize {
        addr as usize & (Self::SIZE - 1)
    }

    fn get(&self, addr: u32) -> Option<ContFn> {
        match self.slots[Self::slot(addr)] {
            Some((cached, func)) if cached == addr => Some(func),
            _ => None,
        }
    }

    fn insert(&mut self, addr: u32, func: ContFn) {
        self.slots[Self::slot(addr)] = Some((addr, func));
    }
}

impl Default for BlockCache {
    fn default() -> Self {
        BlockCache {
            slots: std::array::from_fn(|_| Default::default()),
        }
    }
}

pub struct Context {
    pub cpu: CPU,
    pub thread_handle: u32,
    pub thread_id: u32,
    // TODO: we currently use a single leaked static memory to allow it to be shared between threads.
    pub memory: Memory<'static>,
    pub blocks: &'static [(u32, ContFn)],
    pub cache: RefCell<BlockCache>,
    pub recent: [ContFn; 4],
}

impl Context {
    /// Given an address (jump target), look up the Cont registered for it.
    pub fn indirect16(&self, addr: SegOfs) -> Cont {
        self.indirect(addr.abs(), || format!("{addr}"))
    }

    /// Given an address (jump target), look up the Cont registered for it.
    pub fn indirect32(&self, addr: u32) -> Cont {
        self.indirect(addr, || format!("{addr:x}"))
    }

    /// Given an address (jump target), look up the Cont registered for it.
    fn indirect(&self, addr: u32, addr_str: impl FnOnce() -> String) -> Cont {
        if addr == 0 {
            self.dump();
            panic!("jmp to null ptr");
        }
        let mut cache = self.cache.borrow_mut();
        if let Some(func) = cache.get(addr) {
            return Cont(func);
        }
        // TODO: this would be faster as a perfect hash if we really cared.
        let Ok(index) = self.blocks.binary_search_by_key(&addr, |(addr, _)| *addr) else {
            self.dump();
            crate::log_missing_addr(addr);
            panic!(
                "jmp to unknown addr {}; \
                re-run tc with --entry-points-file (see THESEUS_MISSING_ADDRS)",
                addr_str()
            );
        };
        let func = self.blocks[index].1;
        cache.insert(addr, func);
        Cont(func)
    }

    pub fn proc_addr(&mut self, func: ContFn) -> u32 {
        self.blocks
            .iter()
            .find(|&(_, f)| std::ptr::fn_addr_eq(*f, func))
            .unwrap()
            .0
    }
}

impl Context {
    pub fn dump_stack32(&self) {
        let esp = self.cpu.regs.esp;
        println!("stack:");
        for i in 0..8 {
            let addr = esp + i * 4;
            if addr + 4 > self.memory.bytes.len() as u32 {
                break;
            }
            println!("{addr:08x} {:08x}", self.memory.read::<u32>(addr));
        }
    }

    pub fn dump_memory16(&self, seg: u16, ofs: u16, count: u16) {
        for i in 0..count {
            let Some(ofs) = ofs.checked_add(i * 2) else {
                break;
            };
            let addr = segofs(seg, ofs);
            if addr + 2 > self.memory.bytes.len() as u32 {
                break;
            }
            println!("{seg:04x}:{ofs:04x} {:04x}", self.memory.read::<u16>(addr));
        }
    }

    pub fn dump_stack16(&self) {
        let seg = self.cpu.regs.get_ss();
        let sp = self.cpu.regs.get_sp();
        println!("stack:");
        self.dump_memory16(seg, sp, 8);
    }

    pub fn dump(&self) {
        self.cpu.dump();
        if self.cpu.real_mode {
            self.dump_stack16();
        } else {
            self.dump_stack32();
        }
    }

    pub fn dump_dosbox(&self, ip: u16) {
        // 0813:0000FF30  xchg si,ax
        // EAX:0000000C EBX:00000001 ECX:00000005 EDX:00000D0B
        // ESI:0000F060 EDI:0000011F EBP:00000100 ESP:0000FFF4
        // DS:0813 ES:0813 FS:0000 GS:0000 SS:0813 CF:1 ZF:0 SF:0 OF:0 IF:1
        println!("{cs:04X}:{ip:08X}", cs = self.cpu.regs.cs);
        println!(
            "EAX:{:08X} EBX:{:08X} ECX:{:08X} EDX:{:08X}",
            self.cpu.regs.eax, self.cpu.regs.ebx, self.cpu.regs.ecx, self.cpu.regs.edx
        );
        println!(
            "ESI:{:08X} EDI:{:08X} EBP:{:08X} ESP:{:08X}",
            self.cpu.regs.esi, self.cpu.regs.edi, self.cpu.regs.ebp, self.cpu.regs.esp
        );
        println!(
            "DS:{:04X} ES:{:04X} FS:{:04X} GS:{:04X} SS:{:04X}",
            self.cpu.regs.ds,
            self.cpu.regs.es,
            self.cpu.regs.fs,
            self.cpu.regs.gs,
            self.cpu.regs.ss
        );
    }

    pub fn dump_memory(&self, addr: u32) {
        let len = 256;
        let stride = 16u32;
        let mut addr = addr;
        for _ in 0..(len / stride) {
            let mem = &self.memory[addr..];
            let mem = &mem[..(stride as usize).min(mem.len())];
            print!("{addr:x}");
            for (i, b) in mem.iter().enumerate() {
                print!(" {b:02x}");
                if i == 8 {
                    print!(" ");
                }
            }
            println!();
            addr += stride;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_ctx(ctx: &Context) {
    ctx.dump();
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_mem32(ctx: &Context, ofs: u32) {
    ctx.dump_memory(ofs);
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_mem16(ctx: &Context, seg: u16, ofs: u16) {
    ctx.dump_memory(SegOfs::new(seg, ofs).abs());
}
