#![allow(static_mut_refs)]

#[cfg(feature = "wasm")]
mod wasm;

mod native;

mod machine;
mod ops;

pub use machine::*;
pub use native::HOST;
pub use ops::*;

pub trait Host {
    fn init(&self);
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}
