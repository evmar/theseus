#![allow(static_mut_refs)]

#[cfg(feature = "wasm")]
mod wasm;

mod native;

mod machine;
mod ops;

pub use machine::{MACHINE, Memory, indirect, proc_addr};
pub use native::HOST;
pub use ops::*;

pub trait Host {
    fn init(&self, blocks: &'static [(u32, fn() -> Cont)]);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

pub struct Cont(pub fn() -> Cont);

pub fn run_loop(mut f: Cont) {
    push(0xf000_0000); // return_from_main
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
