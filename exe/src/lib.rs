#![no_std]

use runtime::{REGS, push};

fn init_memory() {
    // It would be cool if we could just link a wasm object file that contains data sections
    // like
    //   (data (i32.const 0x400000) "....")
    // Unfortunately, wasm-lld only supports "relocatable" object files which means it moves
    // the location of such data at link time.  We could do it by postprocessing the wasm
    // file, maybe.
    unsafe {
        let sections = [
            (0x40_0000, include_bytes!("../data/00400000.raw").as_slice()),
            (0x40_1000, include_bytes!("../data/00401000.raw")),
            (0x40_2000, include_bytes!("../data/00402000.raw")),
            (0x40_3000, include_bytes!("../data/00403000.raw")),
        ];
        for (addr, data) in sections {
            let out = core::slice::from_raw_parts_mut(addr as *mut _, data.len());
            out.copy_from_slice(data);
        }
    }
}

fn x401000() {
    unsafe {
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
    init_memory();
    x401000();
}
