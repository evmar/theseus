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
    let out = &mut ctx.memory[0x400000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".text".to_string(), Some(0x401000), 0x1000);
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut ctx.memory[0x401000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rdata".to_string(), Some(0x402000), 0x1000);
    let bytes = include_bytes!("../data/00402000.raw").as_slice();
    let out = &mut ctx.memory[0x402000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".data".to_string(), Some(0x403000), 0x1000);
    mappings.alloc(".reloc".to_string(), Some(0x404000), 0x1000);
    let bytes = include_bytes!("../data/00404000.raw").as_slice();
    let out = &mut ctx.memory[0x404000..][..bytes.len()];
    out.copy_from_slice(bytes);
}
pub fn x00401000(ctx: &mut Context) -> Cont {
    // 00401000 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401001 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00401003 and esp,0FFFFFFF0h
    ctx.cpu.regs.esp = and(ctx.cpu.regs.esp, 0xfffffff0u32, &mut ctx.cpu.flags);
    // 00401006 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401009 call 00401015h
    let dst = Cont(x00401015);
    call(ctx, 0x40100e, dst)
}

pub fn x0040100e(ctx: &mut Context) -> Cont {
    // 0040100e push 0
    push(ctx, 0x0u32);
    // 00401010 call 00401077h
    let dst = Cont(x00401077);
    call(ctx, 0x401015, dst)
}

pub fn x00401015(ctx: &mut Context) -> Cont {
    // 00401015 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401016 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401017 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401018 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401019 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040101a cmp byte ptr ds:[403000h],0
    sub(
        ctx.memory.read::<u8>(0x403000u32),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 00401021 jne short 0040102Ah
    jne(ctx, Cont(x00401023), Cont(x0040102a))
}

pub fn x00401023(ctx: &mut Context) -> Cont {
    // 00401023 mov byte ptr ds:[403000h],1
    ctx.memory.write::<u8>(0x403000u32, 0x1u8);
    // 0040102a mov eax,fs:[18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.fs_base.wrapping_add(0x18u32));
    // 00401030 mov eax,[eax+30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x30u32));
    // 00401033 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401035 mov eax,[eax+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x10u32));
    // 00401038 mov esi,[eax+20h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x20u32));
    // 0040103b push 0Eh
    push(ctx, 0xeu32);
    // 0040103d pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040103e mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00401040 cmp edi,0Eh
    sub(ctx.cpu.regs.edi, 0xeu32, &mut ctx.cpu.flags);
    // 00401043 je short 00401068h
    je(ctx, Cont(x00401045), Cont(x00401068))
}

pub fn x0040102a(ctx: &mut Context) -> Cont {
    // 0040102a mov eax,fs:[18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.fs_base.wrapping_add(0x18u32));
    // 00401030 mov eax,[eax+30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x30u32));
    // 00401033 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401035 mov eax,[eax+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x10u32));
    // 00401038 mov esi,[eax+20h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x20u32));
    // 0040103b push 0Eh
    push(ctx, 0xeu32);
    // 0040103d pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040103e mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00401040 cmp edi,0Eh
    sub(ctx.cpu.regs.edi, 0xeu32, &mut ctx.cpu.flags);
    // 00401043 je short 00401068h
    je(ctx, Cont(x00401045), Cont(x00401068))
}

pub fn x00401040(ctx: &mut Context) -> Cont {
    // 00401040 cmp edi,0Eh
    sub(ctx.cpu.regs.edi, 0xeu32, &mut ctx.cpu.flags);
    // 00401043 je short 00401068h
    je(ctx, Cont(x00401045), Cont(x00401068))
}

pub fn x00401045(ctx: &mut Context) -> Cont {
    // 00401045 mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 00401047 lea eax,[edi+402000h]
    ctx.cpu.regs.eax = ctx.cpu.regs.edi.wrapping_add(0x402000u32);
    // 0040104d sub ecx,edi
    ctx.cpu.regs.ecx = sub(ctx.cpu.regs.ecx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040104f push 0
    push(ctx, 0x0u32);
    // 00401051 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401052 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00401053 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00401054 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401055 call 00401083h
    let dst = Cont(x00401083);
    call(ctx, 0x40105a, dst)
}

pub fn x0040105a(ctx: &mut Context) -> Cont {
    // 0040105a test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040105c je short 00401063h
    je(ctx, Cont(x0040105e), Cont(x00401063))
}

pub fn x0040105e(ctx: &mut Context) -> Cont {
    // 0040105e add edi,[esp]
    ctx.cpu.regs.edi = add(
        ctx.cpu.regs.edi,
        ctx.memory.read::<u32>(ctx.cpu.regs.esp),
        &mut ctx.cpu.flags,
    );
    // 00401061 jmp short 00401040h
    Cont(x00401040)
}

pub fn x00401063(ctx: &mut Context) -> Cont {
    // 00401063 call 0040107Dh
    let dst = Cont(x0040107d);
    call(ctx, 0x401068, dst)
}

pub fn x00401068(ctx: &mut Context) -> Cont {
    // 00401068 mov byte ptr ds:[403000h],0
    ctx.memory.write::<u8>(0x403000u32, 0x0u8);
    // 0040106f add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00401072 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401073 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401074 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00401075 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00401076 ret
    ret(ctx, 0)
}

pub fn x00401077(ctx: &mut Context) -> Cont {
    // 00401077 jmp dword ptr ds:[402048h]
    Cont(kernel32::ExitProcess_stdcall)
}

pub fn x0040107d(ctx: &mut Context) -> Cont {
    // 0040107d jmp dword ptr ds:[40204Ch]
    Cont(kernel32::GetLastError_stdcall)
}

pub fn x00401083(ctx: &mut Context) -> Cont {
    // 00401083 jmp dword ptr ds:[402050h]
    Cont(kernel32::WriteFile_stdcall)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 18] = [
    (0x001000, kernel32::ExitProcess_stdcall),
    (0x001001, kernel32::GetLastError_stdcall),
    (0x001002, kernel32::WriteFile_stdcall),
    (0x401000, x00401000),
    (0x40100e, x0040100e),
    (0x401015, x00401015),
    (0x401023, x00401023),
    (0x40102a, x0040102a),
    (0x401040, x00401040),
    (0x401045, x00401045),
    (0x40105a, x0040105a),
    (0x40105e, x0040105e),
    (0x401063, x00401063),
    (0x401068, x00401068),
    (0x401077, x00401077),
    (0x40107d, x0040107d),
    (0x401083, x00401083),
    (0xf000_0000, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x00401000),
};
