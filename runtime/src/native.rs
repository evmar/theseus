use crate::Host;

pub struct NativeHost {}
impl Host for NativeHost {
    fn init(&self) {
        logger::init();
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
