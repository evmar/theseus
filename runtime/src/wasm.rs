#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate alloc;

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    #[link_name = "panic"]
    safe fn host_panic(s: u32, len: u32);
    safe fn print(s: u32, len: u32);
}

pub struct WasmHost {}
impl Host for WasmHost {
    fn panic(&self, msg: &str) {
        host_panic(msg.as_ptr() as u32, msg.len() as u32);
    }

    fn print(&self, text: &[u8]) {
        print(text.as_ptr() as u32, text.len() as u32);
    }
}

pub static HOST: WasmHost = WasmHost {};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let buf = format!("{}", info);
    HOST.panic(&buf);
    // safety: we're already in the panic handler; if the host doesn't properly handle the panic
    // it's not like panicking would do anything.
    unsafe {
        core::hint::unreachable_unchecked();
    }
}
