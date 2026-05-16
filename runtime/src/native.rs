use crate::Host;

pub struct NativeHost {}
impl Host for NativeHost {
    fn init(&self) {
        logger::init();
    }

    fn print(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }

    fn clone(&self) -> Box<dyn Host> {
        Box::new(NativeHost {})
    }
}
