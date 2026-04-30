#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use runtime::*;
use winapi::*;

fn init_memory(ctx: &mut Context, mappings: &mut kernel32::Mappings) {
    mappings.reserve(winapi::kernel32::Mapping {
        desc: "null page".to_string(),
        addr: 0x0,
        size: 0x1000,
        section: false,
    });
    mappings.reserve(winapi::kernel32::Mapping {
        desc: "vtables".to_string(),
        addr: 0x1000,
        size: 0x1000,
        section: false,
    });
    mappings.reserve(winapi::kernel32::Mapping {
        desc: "exe header".to_string(),
        addr: 0x400000,
        size: 0x1000,
        section: true,
    });
    let bytes = include_bytes!("../data/00400000.raw").as_slice();
    let out = &mut ctx.memory[0x400000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.reserve(winapi::kernel32::Mapping {
        desc: ".text".to_string(),
        addr: 0x401000,
        size: 0x1000,
        section: true,
    });
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut ctx.memory[0x401000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.reserve(winapi::kernel32::Mapping {
        desc: ".rdata".to_string(),
        addr: 0x402000,
        size: 0x1000,
        section: true,
    });
    let bytes = include_bytes!("../data/00402000.raw").as_slice();
    let out = &mut ctx.memory[0x402000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.reserve(winapi::kernel32::Mapping {
        desc: ".reloc".to_string(),
        addr: 0x403000,
        size: 0x1000,
        section: true,
    });
    let bytes = include_bytes!("../data/00403000.raw").as_slice();
    let out = &mut ctx.memory[0x403000..][..bytes.len()];
    out.copy_from_slice(bytes);
}

pub fn x401000(ctx: &mut Context) -> Cont {
    // 00401000 push 0FFFFFFF5h
    push(ctx, 0xfffffff5u32);
    // 00401002 call dword ptr ds:[402058h]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(ctx, 0x401008, dst)
}

pub fn x401008(ctx: &mut Context) -> Cont {
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

pub fn x40101a(ctx: &mut Context) -> Cont {
    // 0040101a ret
    ret(ctx, 0)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 6] = [
    (0x401000, x401000),
    (0x401008, x401008),
    (0x40101a, x40101a),
    (0xfafbfc00, kernel32::GetStdHandle_stdcall),
    (0xfafbfc01, kernel32::WriteFile_stdcall),
    (runtime::RETURN_FROM_X86_ADDR, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0x0..0x0,
    blocks: &BLOCKS,
    init_memory,
    entry_point: Cont(x401000),
};
