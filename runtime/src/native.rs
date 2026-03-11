use crate::{Cont, Host, MACHINE};

struct Logger {}
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use colored::Colorize;
        use log::Level::*;
        let level = match record.level() {
            Error => format!("{:<5}", record.level()).red(),
            Warn => format!("{:<5}", record.level()).yellow(),
            Info => format!("{:<5}", record.level()).cyan(),
            Debug => format!("{:<5}", record.level()).purple(),
            Trace => format!("{:<5}", record.level()).normal(),
        };
        println!(
            "{} {}:{} {}",
            level,
            record.file().unwrap_or("?"),
            record.line().unwrap_or(0),
            record.args()
        );
    }

    fn flush(&self) {}
}
static LOGGER: Logger = Logger {};

pub struct NativeHost {}
impl Host for NativeHost {
    fn init(&self, blocks: &'static [(u32, fn() -> Cont)]) {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Debug);
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
