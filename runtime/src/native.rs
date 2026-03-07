use crate::{Host, machine::MEMORY};

pub struct NativeHost {}
impl Host for NativeHost {
    fn init(&self) {
        unsafe {
            MEMORY =
                std::alloc::alloc(std::alloc::Layout::from_size_align(32 << 20, 0x1000).unwrap());
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
