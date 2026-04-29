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
pub use machine::{CPU, Context, indirect, proc_addr};
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
    // to it, the stack will have been popped so that esp matches our initial
    // esp and we abort the loop before invoking the continuation.
    push(ctx, RETURN_FROM_X86_ADDR);

    let mut i = 0;
    while ctx.cpu.regs.esp != esp {
        ctx.recent[i] = f.0;
        i = (i + 1) % ctx.recent.len();
        f = f.0(ctx);
    }
}

/// When making a call from host to to x86 code, we need a valid return address
/// that is associated with a real function so that the final 'ret' from the
/// called function succeeds, but we never invoke it.
pub const RETURN_FROM_X86_ADDR: u32 = 0xffff_fffe;

pub fn return_from_x86(_ctx: &mut Context) -> Cont {
    panic!();
}
