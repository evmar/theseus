use crate::{Cont, Flags, Memory, Regs, fpu::FPU, mmx::MMX};

#[derive(Default)]
pub struct CPU {
    pub regs: Regs,
    pub flags: Flags,
    pub fpu: FPU,
    pub mmx: MMX,
}

pub struct Context {
    pub cpu: CPU,
    pub thread_handle: u32,
    pub thread_id: u32,
    pub memory: Memory,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
    pub recent: [fn(&mut Context) -> Cont; 4],
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

fn dump_stack(memory: &Memory, esp: u32) {
    println!("stack:");
    for i in 0..8 {
        let addr = esp + i * 4;
        println!("{addr:#08x} {:#08x}", memory.read::<u32>(addr));
    }
}

impl Context {
    pub fn dump_stack(&self) {
        dump_stack(&self.memory, self.cpu.regs.esp - 4);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_ctx(ctx: &Context) {
    ctx.cpu.regs.dump();
    ctx.dump_stack();
}
