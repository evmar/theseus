#![no_std]

#[macro_use]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    #[link_name = "panic"]
    safe fn host_panic(s: u32, len: u32);
    safe fn print(s: u32, len: u32);
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
        print(text.as_ptr() as u32, text.len() as u32);
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

#[repr(C)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,
}

pub static mut REGS: Regs = Regs {
    eax: 0,
    ecx: 0,
    edx: 0,
    ebx: 0,

    esi: 0,
    edi: 0,
    esp: 0x2000,
    ebp: 0x2000,
};
//const REGS: &mut Regs = unsafe { &mut *(0x1000 as *mut Regs) };

pub fn push(x: u32) {
    unsafe {
        REGS.esp -= 4;
        *(REGS.esp as *mut u32) = x;
    }
}

pub fn pop() -> u32 {
    unsafe {
        let x = *(REGS.esp as *mut u32);
        REGS.esp += 4;
        x
    }
}

pub fn call(addr: u32) -> Option<u32> {
    todo!("call");
    return Some(addr);
}
