use crate::{Cont, Host, MACHINE, Machine};

pub struct NativeHost {}
impl Host for NativeHost {
    fn init(&self, blocks: &'static [(u32, fn(&mut Machine) -> Cont)]) {
        logger::init();
        unsafe {
            let size = 32 << 20;
            let mem = std::alloc::alloc(std::alloc::Layout::from_size_align(size, 0x1000).unwrap());
            MACHINE.memory.bytes = std::slice::from_raw_parts_mut(mem, size);
            MACHINE.blocks = blocks;
        }
    }

    fn panic(&self, msg: &str) {
        panic!("{}", msg);
    }

    fn print(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }
}

pub static HOST: NativeHost = NativeHost {};
