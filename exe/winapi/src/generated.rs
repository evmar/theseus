#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(static_mut_refs)]

use runtime::*;
use winapi::*;

pub fn x00401000() -> Cont {
    unsafe {
        // 00401000 push 0FFFFFFF5h
        push(0xfffffff5u32);
        // 00401002 call dword ptr ds:[402058h]
        call(0x401008, Cont(kernel32::stdcall_GetStdHandle))
    }
}

pub fn x00401008() -> Cont {
    unsafe {
        // 00401008 xor ecx,ecx
        MACHINE.regs.ecx ^= MACHINE.regs.ecx;
        // 0040100a push ecx
        push(MACHINE.regs.ecx);
        // 0040100b push ecx
        push(MACHINE.regs.ecx);
        // 0040100c push 6
        push(0x6u32);
        // 0040100e push 402000h
        push(0x402000u32);
        // 00401013 push eax
        push(MACHINE.regs.eax);
        // 00401014 call dword ptr ds:[40205Ch]
        call(0x40101a, Cont(kernel32::stdcall_WriteFile))
    }
}

pub fn x0040101a() -> Cont {
    unsafe {
        // 0040101a ret
        indirect(pop())
    }
}

pub fn init_memory() {
    unsafe {
        let sections = [
            (0x400000, include_bytes!("../data/00400000.raw").as_slice()),
            (0x401000, include_bytes!("../data/00401000.raw").as_slice()),
            (0x402000, include_bytes!("../data/00402000.raw").as_slice()),
            (0x403000, include_bytes!("../data/00403000.raw").as_slice()),
        ];

        for (addr, data) in sections {
            let out = core::slice::from_raw_parts_mut(MACHINE.memory.add(addr), data.len());
            out.copy_from_slice(data);
        }
    }
}

const BLOCKS: [(u32, fn() -> Cont); 5] = [
    (0, runtime::null_pointer_error),
    (0x401000, x00401000),
    (0x401008, x00401008),
    (0x40101a, x0040101a),
    (0xf000_0000, runtime::return_from_main),
];

pub fn indirect(addr: u32) -> Cont {
    let index = BLOCKS
        .binary_search_by_key(&addr, |(addr, _)| *addr)
        .unwrap();
    Cont(BLOCKS[index].1)
}
