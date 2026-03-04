#![no_std]
#![allow(non_snake_case)]

use runtime::{HOST, Host};

#[macro_use]
extern crate alloc;

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
    let buf = format!("WriteFile({hFile:x} {lpBuffer:x} {n:x} {nr:x} {o:x})");
    HOST.print(buf.as_bytes());

    if hFile == 1 {
        HOST.print(unsafe { core::slice::from_raw_parts(lpBuffer as *const u8, n as usize) });
    } else {
        todo!("WriteFile(hFile={hFile:x})");
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
