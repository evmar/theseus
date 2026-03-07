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

pub fn dump_state() {
    unsafe {
        println!(
            "eax={:08x} ecx={:08x} edx={:08x} ebx={:08x}",
            REGS.eax, REGS.ecx, REGS.edx, REGS.ebx
        );
        println!("stack:");
        for i in 0..8 {
            let addr = REGS.esp + i * 4;
            println!(
                "{addr:#08x} {:#08x}",
                *(MEMORY.add(addr as usize) as *const u32)
            );
        }
    }
}
