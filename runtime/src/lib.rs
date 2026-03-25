#[cfg(feature = "wasm")]
mod wasm;

mod flags;
mod fpu;
mod machine;
mod memory;
mod mmx;
mod native;
mod ops;
mod registers;

pub use flags::Flags;
pub use machine::{CPU, Context, Machine, indirect, proc_addr};
pub use memory::Memory;
pub use native::HOST;
pub use ops::*;
pub use registers::Regs;

pub trait Host {
    fn init(&self);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

#[derive(Clone, Copy)]
pub struct Cont(pub fn(&mut Context) -> Cont);

pub fn run_loop(ctx: &mut Context, mut f: Cont) {
    loop {
        f = f.0(ctx);
    }
}

/// Call an x86 function, only returning once the x86 function returns.
pub fn call_nested(ctx: &mut Context, mut f: Cont, args: Vec<u32>) {
    let esp = ctx.cpu.regs.esp;
    for arg in args.into_iter().rev() {
        push(ctx, arg);
    }
    // We need a valid return address (so that the final 'ret' from the called function succeeds)
    // but we never invoke it, and instead abort the loop by noticing the stack was popped.
    let return_addr = proc_addr(ctx, return_from_main);
    push(ctx, return_addr);
    while ctx.cpu.regs.esp != esp {
        f = f.0(ctx);
    }
}

pub fn return_from_main(_ctx: &mut Context) -> Cont {
    log::warn!("entry point returned without exiting; TODO: wait for threads");

    std::process::exit(0);
}
