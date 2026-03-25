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

/// Call an x86 stdcall function, only returning once the function returns.
pub fn call_x86(ctx: &mut Context, mut f: Cont, args: Vec<u32>) {
    let esp = ctx.cpu.regs.esp;
    for arg in args.into_iter().rev() {
        push(ctx, arg);
    }
    // Note that return_from_x86 is never called.  When the x86 code returns
    // it, the stack will have been popped so that esp matches our initial
    // esp and we abort the loop before invoking the continuation.
    let return_addr = proc_addr(ctx, return_from_x86);
    push(ctx, return_addr);
    while ctx.cpu.regs.esp != esp {
        f = f.0(ctx);
    }
}

/// When making a call from host to to x86 code, we need a valid return address
/// so that the final 'ret' from the called function succeeds,
/// but we never invoke it.
pub fn return_from_x86(_ctx: &mut Context) -> Cont {
    panic!();
}
