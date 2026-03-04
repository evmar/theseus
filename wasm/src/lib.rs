#![no_std]

use kernel32::REGS;

fn push(x: u32) {
    unsafe {
        REGS.esp -= 4;
        *(REGS.esp as *mut u32) = x;
    }
}

fn x401000() {
    unsafe {
        let mem: *mut u8 = 0x402000 as *mut u8;
        *mem = b'h';

        /*
        00401000 push 0FFFFFFF5h
        00401002 call dword ptr ds:[402058h]
        00401008 xor ecx,ecx
        0040100a push ecx
        0040100b push ecx
        0040100c push 6
        0040100e push 402000h
        00401013 push eax
        00401014 call dword ptr ds:[40205Ch]
        0040101a ret
        */

        push(-1i32 as u32);
        kernel32::stdcall_GetStdHandle();
        REGS.ecx ^= REGS.ecx;
        push(REGS.ecx);
        push(REGS.ecx);
        push(6);
        push(0x402000);
        push(REGS.eax);
        kernel32::stdcall_WriteFile();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn entry_point() {
    x401000();
}
