#![no_std]

#[macro_use]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    #[link_name = "panic"]
    safe fn host_panic(s: u32, len: u32);
    safe fn console_log(s: u32, len: u32);
}

pub trait Host {
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

pub struct WasmHost {}
impl Host for WasmHost {
    fn panic(&self, msg: &str) {
        host_panic(msg.as_ptr() as u32, msg.len() as u32);
    }

    fn print(&self, text: &[u8]) {
        console_log(text.as_ptr() as u32, text.len() as u32);
    }
}

pub static HOST: WasmHost = WasmHost {};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let buf = format!("{}", info);
    HOST.panic(&buf);
    unsafe {
        core::hint::unreachable_unchecked();
    }
}
