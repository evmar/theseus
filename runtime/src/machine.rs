use crate::{Cont, Flags, Memory, Regs, fpu::FPU, mmx::MMX};

#[derive(Default)]
pub struct Machine {
    pub cpu: CPU,
    pub memory: Memory,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
}

pub type Context = Machine;

#[derive(Default)]
pub struct CPU {
    pub regs: Regs,
    pub flags: Flags,
    pub fpu: FPU,
    pub mmx: MMX,
}

pub fn indirect(ctx: &mut Context, addr: u32) -> Cont {
    if addr == 0 {
        panic!("jmp to null ptr");
    }
    let index = ctx
        .blocks
        .binary_search_by_key(&addr, |(addr, _)| *addr)
        .unwrap_or_else(|_| panic!("jmp to unknown addr {addr:#08x}"));
    Cont(ctx.blocks[index].1)
}

pub fn proc_addr(ctx: &mut Context, func: fn(&mut Context) -> Cont) -> u32 {
    ctx.blocks
        .iter()
        .find(|&(_, f)| std::ptr::fn_addr_eq(*f, func))
        .unwrap()
        .0
}

impl Machine {
    pub fn dump_state(&self) {
        self.cpu.regs.dump();
        println!("stack:");
        for i in 0..8 {
            let addr = self.cpu.regs.esp + i * 4;
            println!("{addr:#08x} {:#08x}", self.memory.read::<u32>(addr));
        }
    }
}
