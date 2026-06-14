use crate::{Cont, ContFn, Flags, Memory, Regs, fpu::FPU, mmx::MMX, segofs};

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

pub struct Context {
    pub cpu: CPU,
    pub thread_handle: u32,
    pub thread_id: u32,
    pub memory: Memory,
    pub blocks: &'static [(u32, ContFn)],
    pub recent: [ContFn; 4],
}

impl Context {
    /// Given an address (jump target), look up the Cont registered for it.
    pub fn indirect16(&mut self, addr: u16) -> Cont {
        self.indirect(segofs(self.cpu.regs.get_cs(), addr))
    }

    /// Given an address (jump target), look up the Cont registered for it.
    pub fn indirect(&mut self, addr: u32) -> Cont {
        if addr == 0 {
            self.dump();
            panic!("jmp to null ptr");
        }
        // TODO: this would be faster as a hash table, or even a perfect hash if we really cared.
        let Ok(index) = self.blocks.binary_search_by_key(&addr, |(addr, _)| *addr) else {
            self.dump();
            panic!("jmp to unknown addr {addr:#08x}");
        };
        Cont(self.blocks[index].1)
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
        println!("{ip:08X}");
        println!(
            "EAX:{:08X} EBX:{:08X} ECX:{:08X} EDX:{:08X}",
            self.cpu.regs.eax, self.cpu.regs.ebx, self.cpu.regs.ecx, self.cpu.regs.edx
        );
        println!(
            "ESI:{:08X} EDI:{:08X} EBP:{:08X} ESP:{:08X}",
            self.cpu.regs.esi, self.cpu.regs.edi, self.cpu.regs.ebp, self.cpu.regs.esp
        );
        // println!(
        //     "DS:{:04x} ES:{:04x} FS:{:04x} GS:{:04x} SS:{:04x} CF:0 ZF:0 SF:0 OF:0 IF:0",
        //     0x813, 0x813, 0, 0, 0x813
        // );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_ctx(ctx: &Context) {
    ctx.dump();
}
