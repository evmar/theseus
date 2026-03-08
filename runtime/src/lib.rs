#![allow(static_mut_refs)]

#[cfg(feature = "wasm")]
mod wasm;

mod native;

mod machine;
mod ops;

pub use machine::MACHINE;
pub use native::HOST;
pub use ops::*;

pub trait Host {
    fn init(&self);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

pub struct Cont(pub fn() -> Cont);
