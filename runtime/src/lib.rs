#![allow(static_mut_refs)]

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
pub use machine::{MACHINE, Machine, indirect, proc_addr};
pub use memory::Memory;
pub use native::HOST;
pub use ops::*;
pub use registers::Regs;

pub trait Host {
    fn init(&self, blocks: &'static [(u32, fn(&mut Machine) -> Cont)]);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

#[derive(Clone, Copy)]
pub struct Cont(pub fn(&mut Machine) -> Cont);

pub fn run_loop(m: &mut Machine, mut f: Cont) {
    loop {
        f = f.0(m);
    }
}

pub fn null_pointer_error() -> Cont {
    panic!("jmp to null pointer");
}

pub fn return_from_main(_m: &mut Machine) -> Cont {
    std::process::exit(0);
}
