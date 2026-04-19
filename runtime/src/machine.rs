use crate::{Cont, Flags, Memory, Regs, fpu::FPU, mmx::MMX};

pub struct Machine {
    pub memory: Memory,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
}

#[derive(Default)]
pub struct CPU {
    pub regs: Regs,
    pub flags: Flags,
    pub fpu: FPU,
    pub mmx: MMX,
}

pub struct Context {
    pub cpu: CPU,
    pub thread_id: u32,
    pub memory: Memory,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
}

pub fn indirect(ctx: &mut Context, addr: u32) -> Cont {
    if addr == 0 {
        panic!("jmp to null ptr");
    }
    // TODO: this would be faster as a hash table, or even a perfect hash if we really cared.
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
    pub fn dump_stack(&self, esp: u32) {
        println!("stack:");
        for i in 0..8 {
            let addr = esp + i * 4;
            println!("{addr:#08x} {:#08x}", self.memory.read::<u32>(addr));
        }
    }
}
