#![no_std]
#![allow(non_snake_case)]

use runtime::{HOST, Host, REGS};

#[macro_use]
extern crate alloc;

pub fn GetStdHandle(_x: u32) -> u32 {
    return 1;
}

pub fn stdcall_GetStdHandle() {
    unsafe {
        let stack: *mut u32 = REGS.esp as *mut u32;
        REGS.eax = GetStdHandle(*stack);
        REGS.esp += 4;
    }
}

pub fn WriteFile(hFile: u32, lpBuffer: u32, n: u32, nr: u32, o: u32) -> u32 {
    let buf = format!("WriteFile({hFile:x} {lpBuffer:x} {n:x} {nr:x} {o:x})\n");
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
