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
pub use machine::{Context, Machine, indirect, proc_addr};
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

pub fn null_pointer_error() -> Cont {
    panic!("jmp to null pointer");
}

pub fn return_from_main(_ctx: &mut Context) -> Cont {
    std::process::exit(0);
}
