#![allow(static_mut_refs)]

#[cfg(feature = "wasm")]
mod wasm;

mod fpu;
mod machine;
mod memory;
mod native;
mod ops;
mod registers;

pub use machine::{MACHINE, indirect, proc_addr};
pub use memory::Memory;
pub use native::HOST;
pub use ops::*;
pub use registers::{Flags, Regs};

pub trait Host {
    fn init(&self, blocks: &'static [(u32, fn() -> Cont)]);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

#[derive(Clone, Copy)]
pub struct Cont(pub fn() -> Cont);

pub fn run_loop(mut f: Cont) {
    loop {
        f = f.0();
    }
}

pub fn null_pointer_error() -> Cont {
    panic!("jmp to null pointer");
}

pub fn return_from_main() -> Cont {
    std::process::exit(0);
}
