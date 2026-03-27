#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use runtime::*;
use winapi::*;

fn init_mappings(ctx: &mut Context, mappings: &mut kernel32::Mappings) {
    mappings.alloc("null page".to_string(), Some(0x0), 0x1000);
    mappings.alloc("imported functions".to_string(), Some(0x1000), 0x1000);
    mappings.alloc("exe header".to_string(), Some(0x400000), 0x1000);
    let bytes = include_bytes!("../data/00400000.raw").as_slice();
    let out = &mut ctx.memory.bytes[0x400000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".text".to_string(), Some(0x401000), 0x1000);
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut ctx.memory.bytes[0x401000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rdata".to_string(), Some(0x402000), 0x1000);
    let bytes = include_bytes!("../data/00402000.raw").as_slice();
    let out = &mut ctx.memory.bytes[0x402000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".reloc".to_string(), Some(0x403000), 0x1000);
    let bytes = include_bytes!("../data/00403000.raw").as_slice();
    let out = &mut ctx.memory.bytes[0x403000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
}
pub fn x00401000(ctx: &mut Context) -> Cont {
    // 00401000 push 0FFFFFFF5h
    push(ctx, 0xfffffff5u32);
    // 00401002 call dword ptr ds:[402058h]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(ctx, 0x401008, dst)
}

pub fn x00401008(ctx: &mut Context) -> Cont {
    // 00401008 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040100a push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040100b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040100c push 6
    push(ctx, 0x6u32);
    // 0040100e push 402000h
    push(ctx, 0x402000u32);
    // 00401013 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00401014 call dword ptr ds:[40205Ch]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(ctx, 0x40101a, dst)
}

pub fn x0040101a(ctx: &mut Context) -> Cont {
    // 0040101a ret
    ret(ctx, 0)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 6] = [
    (0x001000, kernel32::GetStdHandle_stdcall),
    (0x001001, kernel32::WriteFile_stdcall),
    (0x401000, x00401000),
    (0x401008, x00401008),
    (0x40101a, x0040101a),
    (0xf000_0000, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x00401000),
};
