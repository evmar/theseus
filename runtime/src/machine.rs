use crate::{Cont, ContFn, Flags, Memory, Regs, fpu::FPU, mmx::MMX};

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
    /// Given an address (jump target), look up the Cont registerd for it.
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
        let esp = self.cpu.regs.esp - 4;
        println!("stack:");
        for i in 0..8 {
            let addr = esp + i * 4;
            if addr + 4 > self.memory.bytes.len() as u32 {
                break;
            }
            println!("{addr:08x} {:08x}", self.memory.read::<u32>(addr));
        }
    }

    pub fn dump_stack16(&self) {
        let esp = self.cpu.regs.esp - 2;
        println!("stack:");
        for i in 0..8 {
            let addr = esp + i * 2;
            if addr + 2 > self.memory.bytes.len() as u32 {
                break;
            }
            println!("{addr:04x} {:04x}", self.memory.read::<u16>(addr));
        }
    }

    pub fn dump(&self) {
        self.cpu.dump();
        if self.cpu.real_mode {
            self.dump_stack16();
        } else {
            self.dump_stack32();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_ctx(ctx: &Context) {
    ctx.dump();
}
