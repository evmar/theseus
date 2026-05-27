#[cfg(feature = "wasm")]
mod wasm;

mod flags;
mod fpu;
mod machine;
mod memory;
mod mmx;
mod ops;
mod registers;

pub use flags::Flags;
pub use machine::{CPU, Context};
pub use memory::Memory;
pub use ops::*;
pub use registers::Regs;

pub type ContFn = fn(&mut Context) -> Cont;

#[derive(Clone, Copy)]
pub struct Cont(pub ContFn);

/// When making a call from host to to x86 code, we need a valid return address
/// that is associated with a real function so that the final 'ret' from the
/// called function succeeds, but we never invoke it.
pub const RETURN_FROM_X86_ADDR: u32 = 0xffff_fffe;

impl Context {
    /// Call an x86 stdcall function, only returning once the function returns.
    pub fn call32_x86(&mut self, mut f: Cont, args: Vec<u32>) {
        let esp = self.cpu.regs.esp;
        for arg in args.into_iter().rev() {
            self.push32(arg);
        }
        // Note that return_from_x86 is never called.  When the x86 code returns
        // to it, the stack will have been popped so that esp matches our initial
        // esp and we abort the loop before invoking the continuation.
        self.push32(RETURN_FROM_X86_ADDR);

        let mut i = 0;
        while self.cpu.regs.esp != esp {
            self.recent[i] = f.0;
            i = (i + 1) % self.recent.len();
            f = f.0(self);
        }
    }

    pub fn cpu_loop(&mut self, mut f: Cont, target_esp: u32) {
        let mut i = 0;
        while self.cpu.regs.esp != target_esp {
            self.recent[i] = f.0;
            i = (i + 1) % self.recent.len();
            f = f.0(self);
        }
    }

    pub fn return_from_x86(&mut self) -> Cont {
        panic!();
    }
}
