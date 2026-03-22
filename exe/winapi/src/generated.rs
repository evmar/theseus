#![allow(unreachable_code)]
#![allow(unused_parens)]

use runtime::*;
use winapi::*;

fn init_mappings(m: &mut Machine) {
    let mut mappings = kernel32::state().mappings.borrow_mut();
    mappings.alloc("null page".to_string(), Some(0x0), 0x1000);
    mappings.alloc("imported functions".to_string(), Some(0x1000), 0x1000);
    mappings.alloc("exe header".to_string(), Some(0x400000), 0x1000);
    let bytes = include_bytes!("../data/00400000.raw").as_slice();
    let out = &mut m.memory.bytes[0x400000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".text".to_string(), Some(0x401000), 0x1000);
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut m.memory.bytes[0x401000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rdata".to_string(), Some(0x402000), 0x1000);
    let bytes = include_bytes!("../data/00402000.raw").as_slice();
    let out = &mut m.memory.bytes[0x402000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".reloc".to_string(), Some(0x403000), 0x1000);
    let bytes = include_bytes!("../data/00403000.raw").as_slice();
    let out = &mut m.memory.bytes[0x403000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
}
#[allow(unused_variables)]
pub fn x00401000(m: &mut Machine) -> Cont {
    // 00401000 push 0FFFFFFF5h
    push(m, 0xfffffff5u32);
    // 00401002 call dword ptr ds:[402058h]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(m, 0x401008, dst)
}

#[allow(unused_variables)]
pub fn x00401008(m: &mut Machine) -> Cont {
    // 00401008 xor ecx,ecx
    m.regs.ecx = xor(m.regs.ecx, m.regs.ecx, &mut m.flags);
    // 0040100a push ecx
    push(m, m.regs.ecx);
    // 0040100b push ecx
    push(m, m.regs.ecx);
    // 0040100c push 6
    push(m, 0x6u32);
    // 0040100e push 402000h
    push(m, 0x402000u32);
    // 00401013 push eax
    push(m, m.regs.eax);
    // 00401014 call dword ptr ds:[40205Ch]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(m, 0x40101a, dst)
}

#[allow(unused_variables)]
pub fn x0040101a(m: &mut Machine) -> Cont {
    // 0040101a ret
    ret(m, 0)
}

const BLOCKS: [(u32, fn(&mut Machine) -> Cont); 6] = [
    (0x001001, kernel32::GetStdHandle_stdcall),
    (0x001002, kernel32::WriteFile_stdcall),
    (0x401000, x00401000),
    (0x401008, x00401008),
    (0x40101a, x0040101a),
    (0xf000_0000, runtime::return_from_main),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x00401000),
};
