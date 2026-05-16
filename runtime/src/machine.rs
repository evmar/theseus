use crate::{Cont, ContFn, Flags, Host, Memory, Regs, fpu::FPU, mmx::MMX};

#[derive(Default)]
pub struct CPU {
    pub regs: Regs,
    pub flags: Flags,
    pub fpu: FPU,
    pub mmx: MMX,
}

pub struct Context {
    pub host: Box<dyn Host>,
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
            panic!("jmp to null ptr");
        }
        // TODO: this would be faster as a hash table, or even a perfect hash if we really cared.
        let Ok(index) = self.blocks.binary_search_by_key(&addr, |(addr, _)| *addr) else {
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

    pub fn dump(&self) {
        self.cpu.regs.dump();
        self.dump_stack();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dump_ctx(ctx: &Context) {
    ctx.dump();
}
