#![allow(unreachable_code)]
#![allow(unused_parens)]

use runtime::*;
use winapi::*;

fn init_mappings(ctx: &mut Context, mappings: &mut kernel32::Mappings) {
    mappings.alloc("null page".to_string(), Some(0x0), 0x1000);
    mappings.alloc("imported functions".to_string(), Some(0x1000), 0x1000);
    mappings.alloc("exe header".to_string(), Some(0x400000), 0x1000);
    let bytes = include_bytes!("../data/00400000.raw").as_slice();
    let out = &mut ctx.memory[0x400000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".text".to_string(), Some(0x401000), 0x1000);
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut ctx.memory[0x401000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rdata".to_string(), Some(0x402000), 0x1000);
    let bytes = include_bytes!("../data/00402000.raw").as_slice();
    let out = &mut ctx.memory[0x402000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".data".to_string(), Some(0x403000), 0x1000);
    let bytes = include_bytes!("../data/00403000.raw").as_slice();
    let out = &mut ctx.memory[0x403000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".reloc".to_string(), Some(0x404000), 0x1000);
    let bytes = include_bytes!("../data/00404000.raw").as_slice();
    let out = &mut ctx.memory[0x404000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
}
#[allow(unused_variables)]
pub fn x00401000(ctx: &mut Context) -> Cont {
    // 00401000 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401001 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401002 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401003 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401004 sub esp,408h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x408u32, &mut ctx.cpu.flags);
    // 0040100a mov esi,ecx
    ctx.cpu.regs.esi = ctx.cpu.regs.ecx;
    // 0040100c cmp dword ptr [ecx+4],0
    sub(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)),
        0x0u32,
        &mut ctx.cpu.flags,
    );
    // 00401010 jle near ptr 00401170h
    jle(ctx, Cont(x00401016), Cont(x00401170))
}

#[allow(unused_variables)]
pub fn x00401016(ctx: &mut Context) -> Cont {
    // 00401016 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00401018 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(ctx, 0x40101e, dst)
}

#[allow(unused_variables)]
pub fn x00401018(ctx: &mut Context) -> Cont {
    // 00401018 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(ctx, 0x40101e, dst)
}

#[allow(unused_variables)]
pub fn x0040101e(ctx: &mut Context) -> Cont {
    // 0040101e mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 00401020 mov ebp,esi
    ctx.cpu.regs.ebp = ctx.cpu.regs.esi;
    // 00401022 push dword ptr [esi+8]
    push(
        ctx,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32)),
    );
    // 00401025 call dword ptr ds:[402198h]
    let dst = Cont(kernel32::TlsGetValue_stdcall);
    call(ctx, 0x40102b, dst)
}

#[allow(unused_variables)]
pub fn x0040102b(ctx: &mut Context) -> Cont {
    // 0040102b mov [esp],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.eax);
    // 0040102e mov dword ptr [esp+404h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), 0x0u32);
    // 00401039 mov eax,0FFFFFFF6h
    ctx.cpu.regs.eax = 0xfffffff6u32;
    // 0040103e mov cl,[eax+402112h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402112u32)),
    );
    // 00401044 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 0040104b lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040104e mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401055 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401059 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040105a jne short 0040103Eh
    jne(ctx, Cont(x0040105c), Cont(x0040103e))
}

#[allow(unused_variables)]
pub fn x0040103e(ctx: &mut Context) -> Cont {
    // 0040103e mov cl,[eax+402112h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402112u32)),
    );
    // 00401044 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 0040104b lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040104e mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401055 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401059 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040105a jne short 0040103Eh
    jne(ctx, Cont(x0040105c), Cont(x0040103e))
}

#[allow(unused_variables)]
pub fn x0040105c(ctx: &mut Context) -> Cont {
    // 0040105c lea ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x4u32);
    // 00401060 mov edx,edi
    ctx.cpu.regs.edx = ctx.cpu.regs.edi;
    // 00401062 call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x401067, dst)
}

#[allow(unused_variables)]
pub fn x00401067(ctx: &mut Context) -> Cont {
    // 00401067 mov eax,0FFFFFFFAh
    ctx.cpu.regs.eax = 0xfffffffau32;
    // 0040106c mov cl,[eax+402107h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402107u32)),
    );
    // 00401072 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401079 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040107c mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401083 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401087 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401088 jne short 0040106Ch
    jne(ctx, Cont(x0040108a), Cont(x0040106c))
}

#[allow(unused_variables)]
pub fn x0040106c(ctx: &mut Context) -> Cont {
    // 0040106c mov cl,[eax+402107h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402107u32)),
    );
    // 00401072 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401079 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040107c mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401083 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401087 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401088 jne short 0040106Ch
    jne(ctx, Cont(x0040108a), Cont(x0040106c))
}

#[allow(unused_variables)]
pub fn x0040108a(ctx: &mut Context) -> Cont {
    // 0040108a mov eax,[ebp]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.ebp);
    // 0040108d cmp byte ptr [eax],0
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.eax),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 00401090 je short 004010C2h
    je(ctx, Cont(x00401092), Cont(x004010c2))
}

#[allow(unused_variables)]
pub fn x00401092(ctx: &mut Context) -> Cont {
    // 00401092 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401094 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401095 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401097 dec edi
    ctx.cpu.regs.edi = dec(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401098 cmp byte ptr [eax+edi+2],0
    sub(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.edi)
                .wrapping_add(0x2u32),
        ),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 0040109d mov ecx,edx
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx;
    // 0040109f lea edi,[edi+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add(0x1u32);
    // 004010a2 lea edx,[edx+1]
    ctx.cpu.regs.edx = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004010a5 jne short 00401098h
    jne(ctx, Cont(x004010a7), Cont(x00401098))
}

#[allow(unused_variables)]
pub fn x00401098(ctx: &mut Context) -> Cont {
    // 00401098 cmp byte ptr [eax+edi+2],0
    sub(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.edi)
                .wrapping_add(0x2u32),
        ),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 0040109d mov ecx,edx
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx;
    // 0040109f lea edi,[edi+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add(0x1u32);
    // 004010a2 lea edx,[edx+1]
    ctx.cpu.regs.edx = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004010a5 jne short 00401098h
    jne(ctx, Cont(x004010a7), Cont(x00401098))
}

#[allow(unused_variables)]
pub fn x004010a7(ctx: &mut Context) -> Cont {
    // 004010a7 mov dl,[eax]
    ctx.cpu.regs.set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 004010a9 mov esi,[esp+404h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004010b0 lea edi,[esi+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 004010b3 mov [esp+404h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.edi);
    // 004010ba mov [esp+esi+4],dl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_dl(),
    );
    // 004010be inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004010bf dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004010c0 jne short 004010A7h
    jne(ctx, Cont(x004010c2), Cont(x004010a7))
}

#[allow(unused_variables)]
pub fn x004010c2(ctx: &mut Context) -> Cont {
    // 004010c2 mov eax,0FFFFFFFBh
    ctx.cpu.regs.eax = 0xfffffffbu32;
    // 004010c7 mov cl,[eax+4020FCh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020fcu32)),
    );
    // 004010cd mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004010d4 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004010d7 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 004010de mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004010e2 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004010e3 jne short 004010C7h
    jne(ctx, Cont(x004010e5), Cont(x004010c7))
}

#[allow(unused_variables)]
pub fn x004010c7(ctx: &mut Context) -> Cont {
    // 004010c7 mov cl,[eax+4020FCh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020fcu32)),
    );
    // 004010cd mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004010d4 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004010d7 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 004010de mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004010e2 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004010e3 jne short 004010C7h
    jne(ctx, Cont(x004010e5), Cont(x004010c7))
}

#[allow(unused_variables)]
pub fn x004010e5(ctx: &mut Context) -> Cont {
    // 004010e5 lea ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x4u32);
    // 004010e9 mov edx,[esp]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.esp);
    // 004010ec call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x4010f1, dst)
}

#[allow(unused_variables)]
pub fn x004010f1(ctx: &mut Context) -> Cont {
    // 004010f1 mov eax,0FFFFFFFDh
    ctx.cpu.regs.eax = 0xfffffffdu32;
    // 004010f6 mov cl,[eax+402100h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402100u32)),
    );
    // 004010fc mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401103 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401106 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 0040110d mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401111 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401112 jne short 004010F6h
    jne(ctx, Cont(x00401114), Cont(x004010f6))
}

#[allow(unused_variables)]
pub fn x004010f6(ctx: &mut Context) -> Cont {
    // 004010f6 mov cl,[eax+402100h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402100u32)),
    );
    // 004010fc mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401103 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401106 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 0040110d mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401111 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401112 jne short 004010F6h
    jne(ctx, Cont(x00401114), Cont(x004010f6))
}

#[allow(unused_variables)]
pub fn x00401114(ctx: &mut Context) -> Cont {
    // 00401114 lea edi,[esp+4]
    ctx.cpu.regs.edi = ctx.cpu.regs.esp.wrapping_add(0x4u32);
    // 00401118 mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 0040111a mov edx,ebx
    ctx.cpu.regs.edx = ctx.cpu.regs.ebx;
    // 0040111c call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x401121, dst)
}

#[allow(unused_variables)]
pub fn x00401121(ctx: &mut Context) -> Cont {
    // 00401121 mov eax,[esp+404h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401128 lea ecx,[eax+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 0040112b mov [esp+404h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ecx);
    // 00401132 mov byte ptr [esp+eax+4],0Ah
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x4u32),
        0xau8,
    );
    // 00401137 mov esi,[esp+404h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 0040113e push 0FFFFFFF5h
    push(ctx, 0xfffffff5u32);
    // 00401140 call dword ptr ds:[40218Ch]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(ctx, 0x401146, dst)
}

#[allow(unused_variables)]
pub fn x00401146(ctx: &mut Context) -> Cont {
    // 00401146 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00401148 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00401149 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040114a push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0040114b push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0040114c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040114d call dword ptr ds:[4021A0h]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(ctx, 0x401153, dst)
}

#[allow(unused_variables)]
pub fn x00401153(ctx: &mut Context) -> Cont {
    // 00401153 mov eax,3E8h
    ctx.cpu.regs.eax = 0x3e8u32;
    // 00401158 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040115a mov esi,ebp
    ctx.cpu.regs.esi = ctx.cpu.regs.ebp;
    // 0040115c idiv dword ptr [ebp+4]
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x4u32)) as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040115f push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00401160 call dword ptr ds:[402190h]
    let dst = Cont(kernel32::Sleep_stdcall);
    call(ctx, 0x401166, dst)
}

#[allow(unused_variables)]
pub fn x00401166(ctx: &mut Context) -> Cont {
    // 00401166 inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00401167 cmp ebx,[ebp+4]
    sub(
        ctx.cpu.regs.ebx,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x4u32)),
        &mut ctx.cpu.flags,
    );
    // 0040116a jl near ptr 00401018h
    jl(ctx, Cont(x00401170), Cont(x00401018))
}

#[allow(unused_variables)]
pub fn x00401170(ctx: &mut Context) -> Cont {
    // 00401170 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(ctx, 0x401176, dst)
}

#[allow(unused_variables)]
pub fn x00401176(ctx: &mut Context) -> Cont {
    // 00401176 mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00401178 push dword ptr [esi+8]
    push(
        ctx,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32)),
    );
    // 0040117b call dword ptr ds:[402198h]
    let dst = Cont(kernel32::TlsGetValue_stdcall);
    call(ctx, 0x401181, dst)
}

#[allow(unused_variables)]
pub fn x00401181(ctx: &mut Context) -> Cont {
    // 00401181 mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 00401183 mov dword ptr [esp+404h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), 0x0u32);
    // 0040118e mov eax,0FFFFFFF6h
    ctx.cpu.regs.eax = 0xfffffff6u32;
    // 00401193 mov cl,[eax+402112h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402112u32)),
    );
    // 00401199 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004011a0 lea ebp,[edx+1]
    ctx.cpu.regs.ebp = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004011a3 mov [esp+404h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebp);
    // 004011aa mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004011ae inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011af jne short 00401193h
    jne(ctx, Cont(x004011b1), Cont(x00401193))
}

#[allow(unused_variables)]
pub fn x00401193(ctx: &mut Context) -> Cont {
    // 00401193 mov cl,[eax+402112h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402112u32)),
    );
    // 00401199 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004011a0 lea ebp,[edx+1]
    ctx.cpu.regs.ebp = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004011a3 mov [esp+404h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebp);
    // 004011aa mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004011ae inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011af jne short 00401193h
    jne(ctx, Cont(x004011b1), Cont(x00401193))
}

#[allow(unused_variables)]
pub fn x004011b1(ctx: &mut Context) -> Cont {
    // 004011b1 lea ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x4u32);
    // 004011b5 mov edx,ebx
    ctx.cpu.regs.edx = ctx.cpu.regs.ebx;
    // 004011b7 call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x4011bc, dst)
}

#[allow(unused_variables)]
pub fn x004011bc(ctx: &mut Context) -> Cont {
    // 004011bc mov eax,0FFFFFFFAh
    ctx.cpu.regs.eax = 0xfffffffau32;
    // 004011c1 mov cl,[eax+402107h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402107u32)),
    );
    // 004011c7 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004011ce lea ebx,[edx+1]
    ctx.cpu.regs.ebx = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004011d1 mov [esp+404h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebx);
    // 004011d8 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004011dc inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011dd jne short 004011C1h
    jne(ctx, Cont(x004011df), Cont(x004011c1))
}

#[allow(unused_variables)]
pub fn x004011c1(ctx: &mut Context) -> Cont {
    // 004011c1 mov cl,[eax+402107h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x402107u32)),
    );
    // 004011c7 mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 004011ce lea ebx,[edx+1]
    ctx.cpu.regs.ebx = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 004011d1 mov [esp+404h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebx);
    // 004011d8 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 004011dc inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011dd jne short 004011C1h
    jne(ctx, Cont(x004011df), Cont(x004011c1))
}

#[allow(unused_variables)]
pub fn x004011df(ctx: &mut Context) -> Cont {
    // 004011df mov eax,[esi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004011e1 cmp byte ptr [eax],0
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.eax),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 004011e4 je short 00401214h
    je(ctx, Cont(x004011e6), Cont(x00401214))
}

#[allow(unused_variables)]
pub fn x004011e6(ctx: &mut Context) -> Cont {
    // 004011e6 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004011e8 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004011e9 cmp byte ptr [eax+ecx+2],0
    sub(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x2u32),
        ),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 004011ee lea ecx,[ecx+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_add(0x1u32);
    // 004011f1 jne short 004011E9h
    jne(ctx, Cont(x004011f3), Cont(x004011e9))
}

#[allow(unused_variables)]
pub fn x004011e9(ctx: &mut Context) -> Cont {
    // 004011e9 cmp byte ptr [eax+ecx+2],0
    sub(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x2u32),
        ),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 004011ee lea ecx,[ecx+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_add(0x1u32);
    // 004011f1 jne short 004011E9h
    jne(ctx, Cont(x004011f3), Cont(x004011e9))
}

#[allow(unused_variables)]
pub fn x004011f3(ctx: &mut Context) -> Cont {
    // 004011f3 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004011f5 dec edx
    ctx.cpu.regs.edx = dec(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004011f6 mov bl,[eax+edx+1]
    ctx.cpu.regs.set_bl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.edx)
                .wrapping_add(0x1u32),
        ),
    );
    // 004011fa mov esi,[esp+404h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401201 lea ebp,[esi+1]
    ctx.cpu.regs.ebp = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 00401204 mov [esp+404h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebp);
    // 0040120b mov [esp+esi+4],bl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_bl(),
    );
    // 0040120f inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401210 cmp ecx,edx
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401212 jne short 004011F6h
    jne(ctx, Cont(x00401214), Cont(x004011f6))
}

#[allow(unused_variables)]
pub fn x004011f6(ctx: &mut Context) -> Cont {
    // 004011f6 mov bl,[eax+edx+1]
    ctx.cpu.regs.set_bl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.edx)
                .wrapping_add(0x1u32),
        ),
    );
    // 004011fa mov esi,[esp+404h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401201 lea ebp,[esi+1]
    ctx.cpu.regs.ebp = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 00401204 mov [esp+404h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ebp);
    // 0040120b mov [esp+esi+4],bl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_bl(),
    );
    // 0040120f inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401210 cmp ecx,edx
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401212 jne short 004011F6h
    jne(ctx, Cont(x00401214), Cont(x004011f6))
}

#[allow(unused_variables)]
pub fn x00401214(ctx: &mut Context) -> Cont {
    // 00401214 mov eax,0FFFFFFFBh
    ctx.cpu.regs.eax = 0xfffffffbu32;
    // 00401219 mov cl,[eax+4020FCh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020fcu32)),
    );
    // 0040121f mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401226 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401229 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401230 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401234 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401235 jne short 00401219h
    jne(ctx, Cont(x00401237), Cont(x00401219))
}

#[allow(unused_variables)]
pub fn x00401219(ctx: &mut Context) -> Cont {
    // 00401219 mov cl,[eax+4020FCh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020fcu32)),
    );
    // 0040121f mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401226 lea esi,[edx+1]
    ctx.cpu.regs.esi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401229 mov [esp+404h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.esi);
    // 00401230 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401234 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401235 jne short 00401219h
    jne(ctx, Cont(x00401237), Cont(x00401219))
}

#[allow(unused_variables)]
pub fn x00401237(ctx: &mut Context) -> Cont {
    // 00401237 lea esi,[esp+4]
    ctx.cpu.regs.esi = ctx.cpu.regs.esp.wrapping_add(0x4u32);
    // 0040123b mov ecx,esi
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi;
    // 0040123d mov edx,edi
    ctx.cpu.regs.edx = ctx.cpu.regs.edi;
    // 0040123f call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x401244, dst)
}

#[allow(unused_variables)]
pub fn x00401244(ctx: &mut Context) -> Cont {
    // 00401244 mov eax,0FFFFFFF6h
    ctx.cpu.regs.eax = 0xfffffff6u32;
    // 00401249 mov cl,[eax+4020EAh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020eau32)),
    );
    // 0040124f mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401256 lea edi,[edx+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401259 mov [esp+404h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.edi);
    // 00401260 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401264 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401265 jne short 00401249h
    jne(ctx, Cont(x00401267), Cont(x00401249))
}

#[allow(unused_variables)]
pub fn x00401249(ctx: &mut Context) -> Cont {
    // 00401249 mov cl,[eax+4020EAh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020eau32)),
    );
    // 0040124f mov edx,[esp+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401256 lea edi,[edx+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00401259 mov [esp+404h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.edi);
    // 00401260 mov [esp+edx+4],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.edx)
            .wrapping_add(0x4u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00401264 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401265 jne short 00401249h
    jne(ctx, Cont(x00401267), Cont(x00401249))
}

#[allow(unused_variables)]
pub fn x00401267(ctx: &mut Context) -> Cont {
    // 00401267 mov eax,[esp+404h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 0040126e lea ecx,[eax+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 00401271 mov [esp+404h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32), ctx.cpu.regs.ecx);
    // 00401278 mov byte ptr [esp+eax+4],0Ah
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x4u32),
        0xau8,
    );
    // 0040127d mov edi,[esp+404h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x404u32));
    // 00401284 push 0FFFFFFF5h
    push(ctx, 0xfffffff5u32);
    // 00401286 call dword ptr ds:[40218Ch]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(ctx, 0x40128c, dst)
}

#[allow(unused_variables)]
pub fn x0040128c(ctx: &mut Context) -> Cont {
    // 0040128c xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040128e push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040128f push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00401290 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401291 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401292 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00401293 call dword ptr ds:[4021A0h]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(ctx, 0x401299, dst)
}

#[allow(unused_variables)]
pub fn x00401299(ctx: &mut Context) -> Cont {
    // 00401299 add esp,408h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x408u32, &mut ctx.cpu.flags);
    // 0040129f pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004012a0 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 004012a1 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004012a2 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 004012a3 ret
    ret(ctx, 0)
}

#[allow(unused_variables)]
pub fn x004012a4(ctx: &mut Context) -> Cont {
    // 004012a4 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004012a5 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004012a6 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004012a7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004012a8 mov esi,edx
    ctx.cpu.regs.esi = ctx.cpu.regs.edx;
    // 004012aa test edx,edx
    and(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004012ac jns short 004012C3h
    jns(ctx, Cont(x004012ae), Cont(x004012c3))
}

#[allow(unused_variables)]
pub fn x004012ae(ctx: &mut Context) -> Cont {
    // 004012ae mov eax,[ecx+400h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012b4 lea edx,[eax+1]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 004012b7 mov [ecx+400h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32), ctx.cpu.regs.edx);
    // 004012bd mov byte ptr [ecx+eax],2Dh
    ctx.memory
        .write::<u8>(ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.eax), 0x2du8);
    // 004012c1 neg esi
    ctx.cpu.regs.esi = neg(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004012c3 mov edi,[ecx+400h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012c9 mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    ctx.cpu.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    let x = ctx.cpu.regs.eax;
    let y = ctx.cpu.regs.edx;
    let out = mul(x as u64, y as u64, &mut ctx.cpu.flags);
    ctx.cpu.regs.edx = (out >> 32) as u32;
    ctx.cpu.regs.eax = out as u32;
    // 004012d2 shr edx,3
    ctx.cpu.regs.edx = shr(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 004012d5 lea eax,[edx+edx]
    ctx.cpu.regs.eax = ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 004012db mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012dd sub eax,ebp
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004012df or al,30h
    ctx.cpu
        .regs
        .set_al(or(ctx.cpu.regs.get_al(), 0x30u8, &mut ctx.cpu.flags));
    // 004012e1 mov ebp,[ecx+400h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    ctx.cpu.regs.ebx = ctx.cpu.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32), ctx.cpu.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.ebp),
        ctx.cpu.regs.get_al(),
    );
    // 004012f3 cmp esi,9
    sub(ctx.cpu.regs.esi, 0x9u32, &mut ctx.cpu.flags);
    // 004012f6 mov esi,edx
    ctx.cpu.regs.esi = ctx.cpu.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(ctx, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012c3(ctx: &mut Context) -> Cont {
    // 004012c3 mov edi,[ecx+400h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012c9 mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    ctx.cpu.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    let x = ctx.cpu.regs.eax;
    let y = ctx.cpu.regs.edx;
    let out = mul(x as u64, y as u64, &mut ctx.cpu.flags);
    ctx.cpu.regs.edx = (out >> 32) as u32;
    ctx.cpu.regs.eax = out as u32;
    // 004012d2 shr edx,3
    ctx.cpu.regs.edx = shr(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 004012d5 lea eax,[edx+edx]
    ctx.cpu.regs.eax = ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 004012db mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012dd sub eax,ebp
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004012df or al,30h
    ctx.cpu
        .regs
        .set_al(or(ctx.cpu.regs.get_al(), 0x30u8, &mut ctx.cpu.flags));
    // 004012e1 mov ebp,[ecx+400h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    ctx.cpu.regs.ebx = ctx.cpu.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32), ctx.cpu.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.ebp),
        ctx.cpu.regs.get_al(),
    );
    // 004012f3 cmp esi,9
    sub(ctx.cpu.regs.esi, 0x9u32, &mut ctx.cpu.flags);
    // 004012f6 mov esi,edx
    ctx.cpu.regs.esi = ctx.cpu.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(ctx, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012c9(ctx: &mut Context) -> Cont {
    // 004012c9 mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    ctx.cpu.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    let x = ctx.cpu.regs.eax;
    let y = ctx.cpu.regs.edx;
    let out = mul(x as u64, y as u64, &mut ctx.cpu.flags);
    ctx.cpu.regs.edx = (out >> 32) as u32;
    ctx.cpu.regs.eax = out as u32;
    // 004012d2 shr edx,3
    ctx.cpu.regs.edx = shr(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 004012d5 lea eax,[edx+edx]
    ctx.cpu.regs.eax = ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 004012db mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 004012dd sub eax,ebp
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004012df or al,30h
    ctx.cpu
        .regs
        .set_al(or(ctx.cpu.regs.get_al(), 0x30u8, &mut ctx.cpu.flags));
    // 004012e1 mov ebp,[ecx+400h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    ctx.cpu.regs.ebx = ctx.cpu.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32), ctx.cpu.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.ebp),
        ctx.cpu.regs.get_al(),
    );
    // 004012f3 cmp esi,9
    sub(ctx.cpu.regs.esi, 0x9u32, &mut ctx.cpu.flags);
    // 004012f6 mov esi,edx
    ctx.cpu.regs.esi = ctx.cpu.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(ctx, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012fa(ctx: &mut Context) -> Cont {
    // 004012fa mov eax,[ecx+400h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 00401300 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401301 cmp eax,edi
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401303 jle short 0040131Eh
    jle(ctx, Cont(x00401305), Cont(x0040131e))
}

#[allow(unused_variables)]
pub fn x00401305(ctx: &mut Context) -> Cont {
    // 00401305 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401306 mov dl,[ecx+edi-1]
    ctx.cpu.regs.set_dl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .ecx
                .wrapping_add(ctx.cpu.regs.edi)
                .wrapping_add(0xffffffffu32),
        ),
    );
    // 0040130a mov dh,[ecx+eax]
    ctx.cpu.regs.set_dh(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.eax)),
    );
    // 0040130d mov [ecx+edi-1],dh
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .ecx
            .wrapping_add(ctx.cpu.regs.edi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_dh(),
    );
    // 00401311 mov [ecx+eax],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.eax),
        ctx.cpu.regs.get_dl(),
    );
    // 00401314 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401315 lea edx,[eax-1]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0xffffffffu32);
    // 00401318 cmp eax,edi
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040131a mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 0040131c jg short 00401306h
    jg(ctx, Cont(x0040131e), Cont(x00401306))
}

#[allow(unused_variables)]
pub fn x00401306(ctx: &mut Context) -> Cont {
    // 00401306 mov dl,[ecx+edi-1]
    ctx.cpu.regs.set_dl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .ecx
                .wrapping_add(ctx.cpu.regs.edi)
                .wrapping_add(0xffffffffu32),
        ),
    );
    // 0040130a mov dh,[ecx+eax]
    ctx.cpu.regs.set_dh(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.eax)),
    );
    // 0040130d mov [ecx+edi-1],dh
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .ecx
            .wrapping_add(ctx.cpu.regs.edi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_dh(),
    );
    // 00401311 mov [ecx+eax],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.eax),
        ctx.cpu.regs.get_dl(),
    );
    // 00401314 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401315 lea edx,[eax-1]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0xffffffffu32);
    // 00401318 cmp eax,edi
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040131a mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 0040131c jg short 00401306h
    jg(ctx, Cont(x0040131e), Cont(x00401306))
}

#[allow(unused_variables)]
pub fn x0040131e(ctx: &mut Context) -> Cont {
    // 0040131e mov eax,ecx
    ctx.cpu.regs.eax = ctx.cpu.regs.ecx;
    // 00401320 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401321 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401322 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00401323 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00401324 ret
    ret(ctx, 0)
}

#[allow(unused_variables)]
pub fn x00401325(ctx: &mut Context) -> Cont {
    // 00401325 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401326 mov esi,[esp+8]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 0040132a push 2
    push(ctx, 0x2u32);
    // 0040132c push dword ptr [esi+8]
    push(
        ctx,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32)),
    );
    // 0040132f call dword ptr ds:[40219Ch]
    let dst = Cont(kernel32::TlsSetValue_stdcall);
    call(ctx, 0x401335, dst)
}

#[allow(unused_variables)]
pub fn x00401335(ctx: &mut Context) -> Cont {
    // 00401335 mov ecx,esi
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi;
    // 00401337 call 00401000h
    let dst = Cont(x00401000);
    call(ctx, 0x40133c, dst)
}

#[allow(unused_variables)]
pub fn x0040133c(ctx: &mut Context) -> Cont {
    // 0040133c xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040133e pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040133f ret 4
    ret(ctx, 4)
}

#[allow(unused_variables)]
pub fn x00401342(ctx: &mut Context) -> Cont {
    // 00401342 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401343 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401344 sub esp,404h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x404u32, &mut ctx.cpu.flags);
    // 0040134a call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(ctx, 0x401350, dst)
}

#[allow(unused_variables)]
pub fn x00401350(ctx: &mut Context) -> Cont {
    // 00401350 mov dword ptr [esp+400h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), 0x0u32);
    // 0040135b mov ecx,0FFFFFFF6h
    ctx.cpu.regs.ecx = 0xfffffff6u32;
    // 00401360 mov dl,[ecx+402112h]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(0x402112u32)),
    );
    // 00401366 mov esi,[esp+400h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 0040136d lea edi,[esi+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 00401370 mov [esp+400h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), ctx.cpu.regs.edi);
    // 00401377 mov [esp+esi],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esp.wrapping_add(ctx.cpu.regs.esi),
        ctx.cpu.regs.get_dl(),
    );
    // 0040137a inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040137b jne short 00401360h
    jne(ctx, Cont(x0040137d), Cont(x00401360))
}

#[allow(unused_variables)]
pub fn x00401360(ctx: &mut Context) -> Cont {
    // 00401360 mov dl,[ecx+402112h]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(0x402112u32)),
    );
    // 00401366 mov esi,[esp+400h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 0040136d lea edi,[esi+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 00401370 mov [esp+400h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), ctx.cpu.regs.edi);
    // 00401377 mov [esp+esi],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esp.wrapping_add(ctx.cpu.regs.esi),
        ctx.cpu.regs.get_dl(),
    );
    // 0040137a inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040137b jne short 00401360h
    jne(ctx, Cont(x0040137d), Cont(x00401360))
}

#[allow(unused_variables)]
pub fn x0040137d(ctx: &mut Context) -> Cont {
    // 0040137d mov esi,esp
    ctx.cpu.regs.esi = ctx.cpu.regs.esp;
    // 0040137f mov ecx,esi
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi;
    // 00401381 mov edx,eax
    ctx.cpu.regs.edx = ctx.cpu.regs.eax;
    // 00401383 call 004012A4h
    let dst = Cont(x004012a4);
    call(ctx, 0x401388, dst)
}

#[allow(unused_variables)]
pub fn x00401388(ctx: &mut Context) -> Cont {
    // 00401388 mov eax,0FFFFFFE6h
    ctx.cpu.regs.eax = 0xffffffe6u32;
    // 0040138d mov cl,[eax+4020DFh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020dfu32)),
    );
    // 00401393 mov edx,[esp+400h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 0040139a lea edi,[edx+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040139d mov [esp+400h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), ctx.cpu.regs.edi);
    // 004013a4 mov [esp+edx],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esp.wrapping_add(ctx.cpu.regs.edx),
        ctx.cpu.regs.get_cl(),
    );
    // 004013a7 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004013a8 jne short 0040138Dh
    jne(ctx, Cont(x004013aa), Cont(x0040138d))
}

#[allow(unused_variables)]
pub fn x0040138d(ctx: &mut Context) -> Cont {
    // 0040138d mov cl,[eax+4020DFh]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4020dfu32)),
    );
    // 00401393 mov edx,[esp+400h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 0040139a lea edi,[edx+1]
    ctx.cpu.regs.edi = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 0040139d mov [esp+400h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), ctx.cpu.regs.edi);
    // 004013a4 mov [esp+edx],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esp.wrapping_add(ctx.cpu.regs.edx),
        ctx.cpu.regs.get_cl(),
    );
    // 004013a7 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004013a8 jne short 0040138Dh
    jne(ctx, Cont(x004013aa), Cont(x0040138d))
}

#[allow(unused_variables)]
pub fn x004013aa(ctx: &mut Context) -> Cont {
    // 004013aa mov eax,[esp+400h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 004013b1 lea ecx,[eax+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 004013b4 mov [esp+400h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32), ctx.cpu.regs.ecx);
    // 004013bb mov byte ptr [esp+eax],0Ah
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esp.wrapping_add(ctx.cpu.regs.eax), 0xau8);
    // 004013bf mov edi,[esp+400h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x400u32));
    // 004013c6 push 0FFFFFFF5h
    push(ctx, 0xfffffff5u32);
    // 004013c8 call dword ptr ds:[40218Ch]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(ctx, 0x4013ce, dst)
}

#[allow(unused_variables)]
pub fn x004013ce(ctx: &mut Context) -> Cont {
    // 004013ce xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004013d0 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004013d1 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004013d2 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004013d3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004013d4 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004013d5 call dword ptr ds:[4021A0h]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(ctx, 0x4013db, dst)
}

#[allow(unused_variables)]
pub fn x004013db(ctx: &mut Context) -> Cont {
    // 004013db xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004013dd add esp,404h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x404u32, &mut ctx.cpu.flags);
    // 004013e3 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004013e4 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 004013e5 ret 4
    ret(ctx, 4)
}

#[allow(unused_variables)]
pub fn x004013e8(ctx: &mut Context) -> Cont {
    // 004013e8 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004013e9 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004013ea push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004013eb sub esp,18h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 004013ee xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004013f0 mov edi,ds:[402184h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x402184u32);
    // 004013f6 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004013f7 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004013f8 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004013f9 push 401342h
    push(ctx, 0x401342u32);
    // 004013fe push 1000h
    push(ctx, 0x1000u32);
    // 00401403 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401404 call edi
    let dst = indirect(ctx, ctx.cpu.regs.edi);
    call(ctx, 0x401406, dst)
}

#[allow(unused_variables)]
pub fn x00401406(ctx: &mut Context) -> Cont {
    // 00401406 call dword ptr ds:[402194h]
    let dst = Cont(kernel32::TlsAlloc_stdcall);
    call(ctx, 0x40140c, dst)
}

#[allow(unused_variables)]
pub fn x0040140c(ctx: &mut Context) -> Cont {
    // 0040140c mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 0040140e push 1
    push(ctx, 0x1u32);
    // 00401410 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00401411 call dword ptr ds:[40219Ch]
    let dst = Cont(kernel32::TlsSetValue_stdcall);
    call(ctx, 0x401417, dst)
}

#[allow(unused_variables)]
pub fn x00401417(ctx: &mut Context) -> Cont {
    // 00401417 lea eax,[esp+0Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0xcu32);
    // 0040141b mov dword ptr [eax],4020EBh
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, 0x4020ebu32);
    // 00401421 mov dword ptr [eax+4],5
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), 0x5u32);
    // 00401428 mov [eax+8],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.esi);
    // 0040142b push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040142c push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040142d push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040142e push 401325h
    push(ctx, 0x401325u32);
    // 00401433 push 8000h
    push(ctx, 0x8000u32);
    // 00401438 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401439 call edi
    let dst = indirect(ctx, ctx.cpu.regs.edi);
    call(ctx, 0x40143b, dst)
}

#[allow(unused_variables)]
pub fn x0040143b(ctx: &mut Context) -> Cont {
    // 0040143b mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 0040143d mov dword ptr [ecx],4020C0h
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, 0x4020c0u32);
    // 00401443 mov dword ptr [ecx+4],0Ah
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), 0xau32);
    // 0040144a mov [ecx+8],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.esi);
    // 0040144d call 00401000h
    let dst = Cont(x00401000);
    call(ctx, 0x401452, dst)
}

#[allow(unused_variables)]
pub fn x00401452(ctx: &mut Context) -> Cont {
    // 00401452 add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 00401455 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401456 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401457 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00401458 ret
    ret(ctx, 0)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 78] = [
    (0x001000, kernel32::CreateThread_stdcall),
    (0x001001, kernel32::GetCurrentThreadId_stdcall),
    (0x001002, kernel32::GetStdHandle_stdcall),
    (0x001003, kernel32::Sleep_stdcall),
    (0x001004, kernel32::TlsAlloc_stdcall),
    (0x001005, kernel32::TlsGetValue_stdcall),
    (0x001006, kernel32::TlsSetValue_stdcall),
    (0x001007, kernel32::WriteFile_stdcall),
    (0x401000, x00401000),
    (0x401016, x00401016),
    (0x401018, x00401018),
    (0x40101e, x0040101e),
    (0x40102b, x0040102b),
    (0x40103e, x0040103e),
    (0x40105c, x0040105c),
    (0x401067, x00401067),
    (0x40106c, x0040106c),
    (0x40108a, x0040108a),
    (0x401092, x00401092),
    (0x401098, x00401098),
    (0x4010a7, x004010a7),
    (0x4010c2, x004010c2),
    (0x4010c7, x004010c7),
    (0x4010e5, x004010e5),
    (0x4010f1, x004010f1),
    (0x4010f6, x004010f6),
    (0x401114, x00401114),
    (0x401121, x00401121),
    (0x401146, x00401146),
    (0x401153, x00401153),
    (0x401166, x00401166),
    (0x401170, x00401170),
    (0x401176, x00401176),
    (0x401181, x00401181),
    (0x401193, x00401193),
    (0x4011b1, x004011b1),
    (0x4011bc, x004011bc),
    (0x4011c1, x004011c1),
    (0x4011df, x004011df),
    (0x4011e6, x004011e6),
    (0x4011e9, x004011e9),
    (0x4011f3, x004011f3),
    (0x4011f6, x004011f6),
    (0x401214, x00401214),
    (0x401219, x00401219),
    (0x401237, x00401237),
    (0x401244, x00401244),
    (0x401249, x00401249),
    (0x401267, x00401267),
    (0x40128c, x0040128c),
    (0x401299, x00401299),
    (0x4012a4, x004012a4),
    (0x4012ae, x004012ae),
    (0x4012c3, x004012c3),
    (0x4012c9, x004012c9),
    (0x4012fa, x004012fa),
    (0x401305, x00401305),
    (0x401306, x00401306),
    (0x40131e, x0040131e),
    (0x401325, x00401325),
    (0x401335, x00401335),
    (0x40133c, x0040133c),
    (0x401342, x00401342),
    (0x401350, x00401350),
    (0x401360, x00401360),
    (0x40137d, x0040137d),
    (0x401388, x00401388),
    (0x40138d, x0040138d),
    (0x4013aa, x004013aa),
    (0x4013ce, x004013ce),
    (0x4013db, x004013db),
    (0x4013e8, x004013e8),
    (0x401406, x00401406),
    (0x40140c, x0040140c),
    (0x401417, x00401417),
    (0x40143b, x0040143b),
    (0x401452, x00401452),
    (0xf000_0000, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x004013e8),
};
