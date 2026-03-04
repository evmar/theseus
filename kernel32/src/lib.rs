#![no_std]
#![allow(non_snake_case)]

extern crate alloc;

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    safe fn console_log(s: u32, len: u32);
    #[link_name = "panic"]
    safe fn panic_(s: u32, len: u32);
}

pub fn GetStdHandle(_x: u32) -> u32 {
    return 1;
}

#[repr(C)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esp: u32,
}

pub static mut REGS: Regs = Regs {
    eax: 0,
    ecx: 0,
    edx: 0,
    ebx: 0,
    esp: 0x2000,
};
//const REGS: &mut Regs = unsafe { &mut *(0x1000 as *mut Regs) };

pub fn stdcall_GetStdHandle() {
    unsafe {
        let stack: *mut u32 = REGS.esp as *mut u32;
        REGS.eax = GetStdHandle(*stack);
        REGS.esp += 4;
    }
}

pub fn WriteFile(hFile: u32, lpBuffer: u32, n: u32, nr: u32, o: u32) -> u32 {
    let buf = alloc::format!("WriteFile {hFile:x} {lpBuffer:x} {n:x} {nr:x} {o:x}");
    console_log(buf.as_ptr() as u32, buf.len() as u32);

    if hFile == 1 {
        console_log(lpBuffer, n);
    } else {
        panic_("writefile".as_ptr() as u32, 9);
    }
    return 1;
}

pub fn stdcall_WriteFile() {
    unsafe {
        let stack: *mut u32 = REGS.esp as *mut u32;
        REGS.eax = WriteFile(
            *stack.add(0),
            *stack.add(1),
            *stack.add(2),
            *stack.add(3),
            *stack.add(4),
        );
        REGS.esp += 5 * 4;
    }
}
