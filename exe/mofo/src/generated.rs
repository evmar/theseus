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
    mappings.alloc("UPX0".to_string(), Some(0x401000), 0x3f000);
    let bytes = include_bytes!("../data/00401000.raw").as_slice();
    let out = &mut ctx.memory[0x401000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc("UPX1".to_string(), Some(0x440000), 0xe000);
    let bytes = include_bytes!("../data/00440000.raw").as_slice();
    let out = &mut ctx.memory[0x440000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rsrc".to_string(), Some(0x44e000), 0x2000);
    let bytes = include_bytes!("../data/0044e000.raw").as_slice();
    let out = &mut ctx.memory[0x44e000..][..bytes.len()];
    out.copy_from_slice(bytes);
}
pub fn x401000(ctx: &mut Context) -> Cont {
    // 00401000 sub esp,8Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8cu32, &mut ctx.cpu.flags);
    // 00401006 fld dword ptr [esp+90h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x90u32)) as f64,
    );
    // 0040100d fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401013 mov eax,[esp+0A8h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 0040101a push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040101b mov edx,[esp+0A4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 00401022 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401023 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401024 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401025 fst dword ptr [esp+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401029 fld dword ptr [esp+0A4h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32)) as f64,
    );
    // 00401030 fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401036 mov edi,[esp+0B4h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb4u32));
    // 0040103d shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 00401040 add eax,edi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401042 fst dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401046 fld dword ptr [esp+0A8h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32)) as f64,
    );
    // 0040104d fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401053 shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 00401056 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401058 shl eax,2
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x2u8, &mut ctx.cpu.flags);
    // 0040105b fst dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040105f fld dword ptr [esp+0A0h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32)) as f64,
    );
    // 00401066 fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 0040106c mov ecx,[eax+4296C0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4296c0u32));
    // 00401072 mov edx,[eax+4296C4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4296c4u32));
    // 00401078 mov [esp+20h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ecx);
    // 0040107c mov ecx,[eax+429744h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x429744u32));
    // 00401082 mov [esp+30h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.edx);
    // 00401086 mov edx,[eax+429740h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x429740u32));
    // 0040108c fst dword ptr [esp+94h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x94u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401093 fst dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401097 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00401099 mov [esp+40h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32), ctx.cpu.regs.ecx);
    // 0040109d mov ecx,[eax+42A6C0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a6c0u32));
    // 004010a3 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004010a7 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 004010a9 mov [esp+50h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32), ctx.cpu.regs.edx);
    // 004010ad mov edx,[esp+94h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004010b4 fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004010b8 mov [esp+60h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32), ctx.cpu.regs.ecx);
    // 004010bc mov [esp+64h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32), ctx.cpu.regs.edx);
    // 004010c0 fstp dword ptr [esp+34h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x34u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004010c4 fld dword ptr [esp+0A4h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32)) as f64,
    );
    // 004010cb fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 004010d1 mov edx,[eax+42A6C4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a6c4u32));
    // 004010d7 mov [esp+70h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32), ctx.cpu.regs.edx);
    // 004010db fst dword ptr [esp+98h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x98u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004010e2 fst dword ptr [esp+38h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x38u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004010e6 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 004010e8 mov edx,[esp+98h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 004010ef fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004010f3 fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 004010f5 mov [esp+78h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32), ctx.cpu.regs.edx);
    // 004010f9 mov edx,[eax+42A744h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a744u32));
    // 004010ff fstp dword ptr [esp+44h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x44u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401103 mov [esp+80h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32), ctx.cpu.regs.edx);
    // 0040110a fstp dword ptr [esp+48h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x48u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040110e fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401112 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00401114 fstp dword ptr [esp+54h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x54u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401118 fst dword ptr [esp+58h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x58u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040111c fld dword ptr [esp+0A8h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32)) as f64,
    );
    // 00401123 fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401129 fst dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040112d fstp dword ptr [esp+5Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x5cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401131 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00401135 mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00401139 fstp dword ptr [esp+68h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x68u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040113d mov [esp+6Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32), ctx.cpu.regs.ecx);
    // 00401141 mov ecx,[esp+94h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 00401148 fstp dword ptr [esp+84h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x84u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040114f fld dword ptr [eax+42A740h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x42a740u32)) as f64,
    );
    // 00401155 mov [esp+74h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32), ctx.cpu.regs.ecx);
    // 00401159 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0040115d fst dword ptr [esp+90h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x90u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401164 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00401168 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040116e mov [esp+7Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32), ctx.cpu.regs.ecx);
    // 00401172 mov ecx,[esp+98h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 00401179 mov [esp+88h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32), ctx.cpu.regs.ecx);
    // 00401180 mov [esp+8Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8cu32), ctx.cpu.regs.edx);
    // 00401187 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401189 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040118c je short 00401195h
    je(ctx, Cont(x40118e), Cont(x401195))
}

pub fn x40118e(ctx: &mut Context) -> Cont {
    // 0040118e mov ecx,1
    ctx.cpu.regs.ecx = 0x1u32;
    // 00401193 jmp short 00401197h
    Cont(x401197)
}

pub fn x401195(ctx: &mut Context) -> Cont {
    // 00401195 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    Cont(x401197)
}

pub fn x401197(ctx: &mut Context) -> Cont {
    // 00401197 fld dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 0040119b fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004011a1 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004011a3 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004011a6 je short 004011AFh
    je(ctx, Cont(x4011a8), Cont(x4011af))
}

pub fn x4011a8(ctx: &mut Context) -> Cont {
    // 004011a8 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004011ad jmp short 004011B1h
    Cont(x4011b1)
}

pub fn x4011af(ctx: &mut Context) -> Cont {
    // 004011af xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x4011b1)
}

pub fn x4011b1(ctx: &mut Context) -> Cont {
    // 004011b1 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 004011b5 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004011bb add eax,eax
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011bd or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004011bf fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004011c1 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004011c4 je short 004011CDh
    je(ctx, Cont(x4011c6), Cont(x4011cd))
}

pub fn x4011c6(ctx: &mut Context) -> Cont {
    // 004011c6 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004011cb jmp short 004011CFh
    Cont(x4011cf)
}

pub fn x4011cd(ctx: &mut Context) -> Cont {
    // 004011cd xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x4011cf)
}

pub fn x4011cf(ctx: &mut Context) -> Cont {
    // 004011cf fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004011d3 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004011d9 lea edx,[eax*4]
    ctx.cpu.regs.edx = (ctx.cpu.regs.eax * 4);
    // 004011e0 or ecx,edx
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004011e2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004011e4 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004011e7 je short 004011F0h
    je(ctx, Cont(x4011e9), Cont(x4011f0))
}

pub fn x4011e9(ctx: &mut Context) -> Cont {
    // 004011e9 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004011ee jmp short 004011F2h
    Cont(x4011f2)
}

pub fn x4011f0(ctx: &mut Context) -> Cont {
    // 004011f0 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x4011f2)
}

pub fn x4011f2(ctx: &mut Context) -> Cont {
    // 004011f2 fld dword ptr [esp+60h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x60u32)) as f64,
    );
    // 004011f6 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004011fc shl eax,3
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x3u8, &mut ctx.cpu.flags);
    // 004011ff or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401201 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401203 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401206 je short 0040120Fh
    je(ctx, Cont(x401208), Cont(x40120f))
}

pub fn x401208(ctx: &mut Context) -> Cont {
    // 00401208 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040120d jmp short 00401211h
    Cont(x401211)
}

pub fn x40120f(ctx: &mut Context) -> Cont {
    // 0040120f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401211)
}

pub fn x401211(ctx: &mut Context) -> Cont {
    // 00401211 fld dword ptr [esp+70h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x70u32)) as f64,
    );
    // 00401215 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040121b shl eax,4
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x4u8, &mut ctx.cpu.flags);
    // 0040121e or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401220 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401222 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401225 je short 0040122Eh
    je(ctx, Cont(x401227), Cont(x40122e))
}

pub fn x401227(ctx: &mut Context) -> Cont {
    // 00401227 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040122c jmp short 00401230h
    Cont(x401230)
}

pub fn x40122e(ctx: &mut Context) -> Cont {
    // 0040122e xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401230)
}

pub fn x401230(ctx: &mut Context) -> Cont {
    // 00401230 fld dword ptr [esp+80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x80u32)) as f64,
    );
    // 00401237 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040123d shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 00401240 or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401242 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401244 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401247 je short 00401250h
    je(ctx, Cont(x401249), Cont(x401250))
}

pub fn x401249(ctx: &mut Context) -> Cont {
    // 00401249 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040124e jmp short 00401252h
    Cont(x401252)
}

pub fn x401250(ctx: &mut Context) -> Cont {
    // 00401250 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401252)
}

pub fn x401252(ctx: &mut Context) -> Cont {
    // 00401252 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401258 shl eax,6
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x6u8, &mut ctx.cpu.flags);
    // 0040125b or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040125d fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040125f test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401262 je short 0040126Bh
    je(ctx, Cont(x401264), Cont(x40126b))
}

pub fn x401264(ctx: &mut Context) -> Cont {
    // 00401264 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 00401269 jmp short 0040126Dh
    Cont(x40126d)
}

pub fn x40126b(ctx: &mut Context) -> Cont {
    // 0040126b xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x40126d)
}

pub fn x40126d(ctx: &mut Context) -> Cont {
    // 0040126d shl eax,7
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 00401270 or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401272 je near ptr 00401547h
    je(ctx, Cont(x401278), Cont(x401547))
}

pub fn x401278(ctx: &mut Context) -> Cont {
    // 00401278 cmp ecx,0FFh
    sub(ctx.cpu.regs.ecx, 0xffu32, &mut ctx.cpu.flags);
    // 0040127e je near ptr 00401547h
    je(ctx, Cont(x401284), Cont(x401547))
}

pub fn x401284(ctx: &mut Context) -> Cont {
    // 00401284 mov esi,[esp+84h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x84u32));
    // 0040128b mov edi,[esp+88h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32));
    // 00401292 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401295 mov ebx,[esp+9Ch]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 0040129c mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 0040129e mov ebp,[esp+0A0h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 004012a5 mov eax,[esp+74h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32));
    // 004012a9 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004012ac mov [ecx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.esi);
    // 004012ae mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 004012b0 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004012b3 mov [ecx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 004012b6 mov [ecx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 004012b9 mov [ecx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 004012bc mov ecx,[esp+98h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 004012c3 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 004012c5 mov eax,[esp+9Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 004012cc mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 004012cf mov ecx,[esp+0A0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 004012d6 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004012d9 mov eax,[esp+84h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x84u32));
    // 004012e0 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 004012e3 mov ecx,[esp+88h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32));
    // 004012ea mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 004012ec sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004012ef mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 004012f1 mov eax,[esp+9Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 004012f8 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 004012fb mov ecx,[esp+0A0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 00401302 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401305 mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00401309 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 0040130c mov ecx,[esp+58h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 00401310 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401312 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401314 mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 00401318 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040131b mov ecx,[esp+60h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 0040131f mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401322 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401325 call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x40132a, dst)
}

pub fn x40132a(ctx: &mut Context) -> Cont {
    // 0040132a add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 0040132d mov ecx,[esp+44h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 00401331 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401333 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401336 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401338 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040133b mov [edx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.esi);
    // 0040133d mov [edx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 00401340 mov [edx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 00401343 mov [edx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 00401346 mov edx,[esp+68h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32));
    // 0040134a mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 0040134c mov ecx,[esp+6Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32));
    // 00401350 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00401353 mov edx,[esp+70h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32));
    // 00401357 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040135a mov ecx,[esp+0A4h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 00401361 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401364 mov edx,[esp+0A8h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 0040136b mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040136d sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401370 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00401372 mov ecx,[esp+0BCh]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xbcu32));
    // 00401379 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040137c mov edx,[esp+0C0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc0u32));
    // 00401383 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401386 mov ecx,[esp+0A4h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 0040138d mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401390 mov edx,[esp+0A8h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 00401397 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401399 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 0040139b mov ecx,[esp+0ACh]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xacu32));
    // 004013a2 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004013a5 mov edx,[esp+0B0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb0u32));
    // 004013ac mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 004013af mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 004013b2 call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x4013b7, dst)
}

pub fn x4013b7(ctx: &mut Context) -> Cont {
    // 004013b7 mov ecx,[esp+74h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32));
    // 004013bb add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 004013be mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 004013c0 mov edx,[esp+48h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 004013c4 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 004013c6 mov ecx,[esp+4Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 004013ca sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004013cd mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004013d0 mov edx,[esp+60h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 004013d4 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 004013d7 mov ecx,[esp+44h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 004013db mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 004013de mov edx,[esp+48h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 004013e2 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 004013e4 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004013e7 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 004013e9 mov ecx,[esp+5Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 004013ed mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004013f0 mov edx,[esp+60h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 004013f4 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 004013f7 mov ecx,[esp+94h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004013fe mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401401 mov edx,[esp+98h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 00401408 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040140a sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040140d mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 0040140f mov ecx,[esp+0ACh]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xacu32));
    // 00401416 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00401419 mov edx,[esp+0B0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb0u32));
    // 00401420 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401423 mov ecx,[esp+54h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00401427 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 0040142a mov edx,[esp+58h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 0040142e mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401430 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00401432 mov ecx,[esp+5Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 00401436 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00401439 mov edx,[esp+60h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 0040143d mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401440 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401443 call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401448, dst)
}

pub fn x401448(ctx: &mut Context) -> Cont {
    // 00401448 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 0040144b mov edx,[esp+54h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 0040144f mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401451 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401454 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00401456 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401459 mov [eax],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.esi);
    // 0040145b mov [eax+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 0040145e mov [eax+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 00401461 mov [eax+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 00401464 mov eax,[esp+78h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32));
    // 00401468 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 0040146a mov edx,[esp+7Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 0040146e mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00401471 mov eax,[esp+80h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 00401478 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040147b mov edx,[esp+64h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32));
    // 0040147f mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 00401482 mov eax,[esp+68h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32));
    // 00401486 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00401488 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040148b mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 0040148d mov edx,[esp+7Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 00401491 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00401494 mov eax,[esp+80h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 0040149b mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040149e mov edx,[esp+54h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 004014a2 mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 004014a5 mov eax,[esp+58h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 004014a9 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004014ab mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004014ad mov edx,[esp+5Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 004014b1 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 004014b4 mov eax,[esp+60h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 004014b8 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 004014bb mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 004014be call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x4014c3, dst)
}

pub fn x4014c3(ctx: &mut Context) -> Cont {
    // 004014c3 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 004014c6 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004014c8 mov [ecx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.esi);
    // 004014ca mov [ecx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 004014cd mov [ecx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 004014d0 mov [ecx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 004014d3 mov eax,[esp+44h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 004014d7 mov ecx,[esp+48h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 004014db sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004014de mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 004014e0 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004014e3 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 004014e5 mov eax,[esp+6Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32));
    // 004014e9 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 004014ec mov ecx,[esp+70h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32));
    // 004014f0 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004014f3 mov eax,[esp+94h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004014fa mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 004014fd mov ecx,[esp+98h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 00401504 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401506 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401509 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 0040150b mov eax,[esp+0ACh]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xacu32));
    // 00401512 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401515 mov ecx,[esp+0B0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb0u32));
    // 0040151c mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040151f mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00401523 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401526 mov ecx,[esp+58h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 0040152a mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 0040152c mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 0040152e mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 00401532 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401535 mov ecx,[esp+60h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 00401539 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040153c mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 0040153f call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401544, dst)
}

pub fn x401544(ctx: &mut Context) -> Cont {
    // 00401544 add esp,40h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x40u32, &mut ctx.cpu.flags);
    Cont(x401547)
}

pub fn x401547(ctx: &mut Context) -> Cont {
    // 00401547 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401548 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401549 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040154a pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040154b add esp,8Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8cu32, &mut ctx.cpu.flags);
    // 00401551 ret
    ret(ctx, 0)
}

pub fn x401560(ctx: &mut Context) -> Cont {
    // 00401560 sub esp,8Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8cu32, &mut ctx.cpu.flags);
    // 00401566 fld dword ptr [esp+90h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x90u32)) as f64,
    );
    // 0040156d fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401573 mov eax,[esp+0A8h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 0040157a push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040157b mov edx,[esp+0A4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 00401582 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401583 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401584 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401585 fst dword ptr [esp+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401589 fld dword ptr [esp+0A4h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32)) as f64,
    );
    // 00401590 fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401596 mov edi,[esp+0B4h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb4u32));
    // 0040159d shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 004015a0 add eax,edi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004015a2 fst dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004015a6 fld dword ptr [esp+0A8h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32)) as f64,
    );
    // 004015ad fsub dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 004015b3 shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 004015b6 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004015b8 shl eax,2
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x2u8, &mut ctx.cpu.flags);
    // 004015bb fst dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004015bf fld dword ptr [esp+0A0h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32)) as f64,
    );
    // 004015c6 fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 004015cc mov ecx,[eax+4296C0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4296c0u32));
    // 004015d2 mov edx,[eax+4296C4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4296c4u32));
    // 004015d8 mov [esp+20h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ecx);
    // 004015dc mov ecx,[eax+429744h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x429744u32));
    // 004015e2 mov [esp+30h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.edx);
    // 004015e6 mov edx,[eax+429740h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x429740u32));
    // 004015ec fst dword ptr [esp+94h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x94u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004015f3 fst dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004015f7 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004015f9 mov [esp+40h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32), ctx.cpu.regs.ecx);
    // 004015fd mov ecx,[eax+42A6C0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a6c0u32));
    // 00401603 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401607 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00401609 mov [esp+50h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32), ctx.cpu.regs.edx);
    // 0040160d mov edx,[esp+94h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 00401614 fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401618 mov [esp+60h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32), ctx.cpu.regs.ecx);
    // 0040161c mov [esp+64h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32), ctx.cpu.regs.edx);
    // 00401620 fstp dword ptr [esp+34h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x34u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401624 fld dword ptr [esp+0A4h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32)) as f64,
    );
    // 0040162b fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401631 mov edx,[eax+42A6C4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a6c4u32));
    // 00401637 mov [esp+70h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32), ctx.cpu.regs.edx);
    // 0040163b fst dword ptr [esp+98h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x98u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401642 fst dword ptr [esp+38h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x38u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00401646 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00401648 mov edx,[esp+98h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 0040164f fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401653 fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 00401655 mov [esp+78h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32), ctx.cpu.regs.edx);
    // 00401659 mov edx,[eax+42A744h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x42a744u32));
    // 0040165f fstp dword ptr [esp+44h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x44u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401663 mov [esp+80h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32), ctx.cpu.regs.edx);
    // 0040166a fstp dword ptr [esp+48h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x48u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040166e fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401672 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00401674 fstp dword ptr [esp+54h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x54u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401678 fst dword ptr [esp+58h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x58u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040167c fld dword ptr [esp+0A8h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32)) as f64,
    );
    // 00401683 fadd dword ptr ds:[42009Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42009cu32) as f64,
    );
    // 00401689 fst dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040168d fstp dword ptr [esp+5Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x5cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401691 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00401695 mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00401699 fstp dword ptr [esp+68h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x68u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040169d mov [esp+6Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32), ctx.cpu.regs.ecx);
    // 004016a1 mov ecx,[esp+94h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004016a8 fstp dword ptr [esp+84h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x84u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004016af fld dword ptr [eax+42A740h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x42a740u32)) as f64,
    );
    // 004016b5 mov [esp+74h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32), ctx.cpu.regs.ecx);
    // 004016b9 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 004016bd fst dword ptr [esp+90h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x90u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 004016c4 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004016c8 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004016ce mov [esp+7Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32), ctx.cpu.regs.ecx);
    // 004016d2 mov ecx,[esp+98h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 004016d9 mov [esp+88h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32), ctx.cpu.regs.ecx);
    // 004016e0 mov [esp+8Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8cu32), ctx.cpu.regs.edx);
    // 004016e7 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004016e9 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004016ec je short 004016F5h
    je(ctx, Cont(x4016ee), Cont(x4016f5))
}

pub fn x4016ee(ctx: &mut Context) -> Cont {
    // 004016ee mov ecx,1
    ctx.cpu.regs.ecx = 0x1u32;
    // 004016f3 jmp short 004016F7h
    Cont(x4016f7)
}

pub fn x4016f5(ctx: &mut Context) -> Cont {
    // 004016f5 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    Cont(x4016f7)
}

pub fn x4016f7(ctx: &mut Context) -> Cont {
    // 004016f7 fld dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 004016fb fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401701 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401703 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401706 je short 0040170Fh
    je(ctx, Cont(x401708), Cont(x40170f))
}

pub fn x401708(ctx: &mut Context) -> Cont {
    // 00401708 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040170d jmp short 00401711h
    Cont(x401711)
}

pub fn x40170f(ctx: &mut Context) -> Cont {
    // 0040170f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401711)
}

pub fn x401711(ctx: &mut Context) -> Cont {
    // 00401711 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 00401715 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040171b add eax,eax
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040171d or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040171f fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401721 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401724 je short 0040172Dh
    je(ctx, Cont(x401726), Cont(x40172d))
}

pub fn x401726(ctx: &mut Context) -> Cont {
    // 00401726 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040172b jmp short 0040172Fh
    Cont(x40172f)
}

pub fn x40172d(ctx: &mut Context) -> Cont {
    // 0040172d xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x40172f)
}

pub fn x40172f(ctx: &mut Context) -> Cont {
    // 0040172f fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 00401733 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401739 lea edx,[eax*4]
    ctx.cpu.regs.edx = (ctx.cpu.regs.eax * 4);
    // 00401740 or ecx,edx
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401742 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401744 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401747 je short 00401750h
    je(ctx, Cont(x401749), Cont(x401750))
}

pub fn x401749(ctx: &mut Context) -> Cont {
    // 00401749 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040174e jmp short 00401752h
    Cont(x401752)
}

pub fn x401750(ctx: &mut Context) -> Cont {
    // 00401750 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401752)
}

pub fn x401752(ctx: &mut Context) -> Cont {
    // 00401752 fld dword ptr [esp+60h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x60u32)) as f64,
    );
    // 00401756 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040175c shl eax,3
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x3u8, &mut ctx.cpu.flags);
    // 0040175f or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401761 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401763 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401766 je short 0040176Fh
    je(ctx, Cont(x401768), Cont(x40176f))
}

pub fn x401768(ctx: &mut Context) -> Cont {
    // 00401768 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040176d jmp short 00401771h
    Cont(x401771)
}

pub fn x40176f(ctx: &mut Context) -> Cont {
    // 0040176f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401771)
}

pub fn x401771(ctx: &mut Context) -> Cont {
    // 00401771 fld dword ptr [esp+70h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x70u32)) as f64,
    );
    // 00401775 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040177b shl eax,4
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x4u8, &mut ctx.cpu.flags);
    // 0040177e or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401780 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401782 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401785 je short 0040178Eh
    je(ctx, Cont(x401787), Cont(x40178e))
}

pub fn x401787(ctx: &mut Context) -> Cont {
    // 00401787 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040178c jmp short 00401790h
    Cont(x401790)
}

pub fn x40178e(ctx: &mut Context) -> Cont {
    // 0040178e xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x401790)
}

pub fn x401790(ctx: &mut Context) -> Cont {
    // 00401790 fld dword ptr [esp+80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x80u32)) as f64,
    );
    // 00401797 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040179d shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 004017a0 or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004017a2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004017a4 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004017a7 je short 004017B0h
    je(ctx, Cont(x4017a9), Cont(x4017b0))
}

pub fn x4017a9(ctx: &mut Context) -> Cont {
    // 004017a9 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004017ae jmp short 004017B2h
    Cont(x4017b2)
}

pub fn x4017b0(ctx: &mut Context) -> Cont {
    // 004017b0 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x4017b2)
}

pub fn x4017b2(ctx: &mut Context) -> Cont {
    // 004017b2 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 004017b8 shl eax,6
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x6u8, &mut ctx.cpu.flags);
    // 004017bb or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004017bd fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004017bf test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004017c2 je short 004017CBh
    je(ctx, Cont(x4017c4), Cont(x4017cb))
}

pub fn x4017c4(ctx: &mut Context) -> Cont {
    // 004017c4 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004017c9 jmp short 004017CDh
    Cont(x4017cd)
}

pub fn x4017cb(ctx: &mut Context) -> Cont {
    // 004017cb xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x4017cd)
}

pub fn x4017cd(ctx: &mut Context) -> Cont {
    // 004017cd shl eax,7
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 004017d0 or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004017d2 je near ptr 00401AD4h
    je(ctx, Cont(x4017d8), Cont(x401ad4))
}

pub fn x4017d8(ctx: &mut Context) -> Cont {
    // 004017d8 cmp ecx,0FFh
    sub(ctx.cpu.regs.ecx, 0xffu32, &mut ctx.cpu.flags);
    // 004017de je near ptr 00401AD4h
    je(ctx, Cont(x4017e4), Cont(x401ad4))
}

pub fn x4017e4(ctx: &mut Context) -> Cont {
    // 004017e4 mov esi,[esp+24h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 004017e8 mov edi,[esp+28h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 004017ec sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004017ef mov ebx,[esp+3Ch]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 004017f3 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004017f5 mov ebp,[esp+40h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 004017f9 mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 004017fd sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401800 mov [ecx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.esi);
    // 00401802 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401804 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401807 mov [ecx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 0040180a mov [ecx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 0040180d mov [ecx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 00401810 mov ecx,[esp+78h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32));
    // 00401814 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401816 mov eax,[esp+7Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 0040181a mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040181d mov ecx,[esp+80h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 00401824 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401827 mov eax,[esp+84h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x84u32));
    // 0040182e mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401831 mov ecx,[esp+88h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32));
    // 00401838 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 0040183a sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040183d mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 0040183f mov eax,[esp+9Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 00401846 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401849 mov ecx,[esp+0A0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 00401850 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401853 mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00401857 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 0040185a mov ecx,[esp+58h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 0040185e mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401860 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401862 mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 00401866 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401869 mov ecx,[esp+60h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 0040186d mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401870 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401873 call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401878, dst)
}

pub fn x401878(ctx: &mut Context) -> Cont {
    // 00401878 mov eax,[esp+0A4h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 0040187f mov ecx,[esp+0A8h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 00401886 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00401889 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 0040188b sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040188e mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401890 mov eax,[esp+8Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8cu32));
    // 00401897 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040189a mov ecx,[esp+90h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x90u32));
    // 004018a1 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004018a4 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 004018a7 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 004018a9 mov ecx,[esp+94h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004018b0 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004018b3 mov [edx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.esi);
    // 004018b5 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 004018b7 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004018ba mov [edx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 004018bd mov [edx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 004018c0 mov [edx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 004018c3 mov edx,[esp+0B8h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb8u32));
    // 004018ca mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 004018cc mov ecx,[esp+0BCh]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xbcu32));
    // 004018d3 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004018d6 mov edx,[esp+0C0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc0u32));
    // 004018dd mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 004018e0 mov ecx,[esp+94h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 004018e7 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 004018ea mov edx,[esp+98h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 004018f1 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 004018f3 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 004018f5 mov ecx,[esp+9Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 004018fc mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004018ff mov edx,[esp+0A0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 00401906 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401909 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 0040190c call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401911, dst)
}

pub fn x401911(ctx: &mut Context) -> Cont {
    // 00401911 mov ecx,[esp+74h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32));
    // 00401915 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00401918 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040191a mov edx,[esp+48h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 0040191e mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00401920 mov ecx,[esp+4Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 00401924 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401927 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040192a mov edx,[esp+60h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 0040192e mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401931 mov ecx,[esp+64h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32));
    // 00401935 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401938 mov edx,[esp+68h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32));
    // 0040193c mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040193e sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401941 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00401943 mov ecx,[esp+7Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 00401947 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040194a mov edx,[esp+80h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 00401951 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401954 mov ecx,[esp+0A4h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 0040195b mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 0040195e mov edx,[esp+0A8h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 00401965 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401967 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0040196a mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 0040196c mov ecx,[esp+0BCh]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xbcu32));
    // 00401973 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00401976 mov edx,[esp+0C0h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc0u32));
    // 0040197d mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401980 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00401983 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401985 mov [eax],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.esi);
    // 00401987 mov [eax+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 0040198a mov [eax+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 0040198d mov [eax+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 00401990 call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401995, dst)
}

pub fn x401995(ctx: &mut Context) -> Cont {
    // 00401995 mov edx,[esp+0C4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc4u32));
    // 0040199c mov eax,[esp+0C8h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc8u32));
    // 004019a3 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 004019a6 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004019a8 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004019ab mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004019ad mov edx,[esp+0ACh]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xacu32));
    // 004019b4 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 004019b7 mov eax,[esp+0B0h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xb0u32));
    // 004019be mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 004019c1 mov edx,[esp+64h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32));
    // 004019c5 mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 004019c8 mov eax,[esp+68h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32));
    // 004019cc mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004019ce sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004019d1 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004019d3 mov edx,[esp+7Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 004019d7 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 004019da mov eax,[esp+80h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 004019e1 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 004019e4 mov edx,[esp+0A4h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 004019eb mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 004019ee mov eax,[esp+0A8h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 004019f5 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 004019f7 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004019fa mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004019fc mov edx,[esp+0BCh]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xbcu32));
    // 00401a03 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00401a06 mov eax,[esp+0C0h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc0u32));
    // 00401a0d mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00401a10 mov edx,[esp+94h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 00401a17 mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 00401a1a mov eax,[esp+98h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 00401a21 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00401a23 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00401a25 mov edx,[esp+9Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 00401a2c mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00401a2f mov eax,[esp+0A0h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 00401a36 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00401a39 mov [ecx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 00401a3c call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401a41, dst)
}

pub fn x401a41(ctx: &mut Context) -> Cont {
    // 00401a41 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00401a44 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00401a46 mov [ecx],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.esi);
    // 00401a48 mov [ecx+4],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edi);
    // 00401a4b mov [ecx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 00401a4e mov [ecx+0Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32), ctx.cpu.regs.ebp);
    // 00401a51 mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00401a55 mov ecx,[esp+58h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 00401a59 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401a5c mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401a5e sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401a61 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401a63 mov eax,[esp+7Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 00401a67 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401a6a mov ecx,[esp+80h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 00401a71 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401a74 mov eax,[esp+0A4h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa4u32));
    // 00401a7b mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401a7e mov ecx,[esp+0A8h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa8u32));
    // 00401a85 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401a87 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401a8a mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401a8c mov eax,[esp+0BCh]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xbcu32));
    // 00401a93 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401a96 mov ecx,[esp+0C0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xc0u32));
    // 00401a9d mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401aa0 mov eax,[esp+94h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x94u32));
    // 00401aa7 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401aaa mov ecx,[esp+98h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x98u32));
    // 00401ab1 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401ab3 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401ab5 mov eax,[esp+9Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x9cu32));
    // 00401abc mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00401abf mov ecx,[esp+0A0h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xa0u32));
    // 00401ac6 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00401ac9 mov [edx+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401acc call 00401F90h
    let dst = Cont(x401f90);
    call(ctx, 0x401ad1, dst)
}

pub fn x401ad1(ctx: &mut Context) -> Cont {
    // 00401ad1 add esp,40h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x40u32, &mut ctx.cpu.flags);
    Cont(x401ad4)
}

pub fn x401ad4(ctx: &mut Context) -> Cont {
    // 00401ad4 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401ad5 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401ad6 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00401ad7 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00401ad8 add esp,8Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8cu32, &mut ctx.cpu.flags);
    // 00401ade ret
    ret(ctx, 0)
}

pub fn x401ae0(ctx: &mut Context) -> Cont {
    // 00401ae0 sub esp,18h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 00401ae3 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401ae4 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401ae5 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00401ae7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401ae8 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401ae9 mov [esp+10h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ebx);
    Cont(x401aed)
}

pub fn x401aed(ctx: &mut Context) -> Cont {
    // 00401aed lea eax,[ebx-5]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx.wrapping_add(0xfffffffbu32);
    // 00401af0 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401af2 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 00401af6 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 00401afa fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 00401b00 fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x401b04)
}

pub fn x401b04(ctx: &mut Context) -> Cont {
    // 00401b04 lea ecx,[edi-5]
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi.wrapping_add(0xfffffffbu32);
    // 00401b07 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00401b09 mov [esp+14h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.ecx);
    // 00401b0d fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 00401b11 fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 00401b17 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401b1b mov ebp,[esp+1Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    Cont(x401b1f)
}

pub fn x401b1f(ctx: &mut Context) -> Cont {
    // 00401b1f lea edx,[esi-5]
    ctx.cpu.regs.edx = ctx.cpu.regs.esi.wrapping_add(0xfffffffbu32);
    // 00401b22 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00401b26 mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00401b2a push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401b2b fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00401b2f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00401b31 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401b32 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401b33 fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 00401b39 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00401b3d je short 00401B62h
    je(ctx, Cont(x401b3f), Cont(x401b62))
}

pub fn x401b3f(ctx: &mut Context) -> Cont {
    // 00401b3f mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00401b43 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401b46 mov edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00401b4a mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00401b4c mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00401b4e mov ecx,[esp+40h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00401b52 mov [eax+4],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.ebp);
    // 00401b55 mov [eax+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00401b58 mov [eax+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00401b5b call 00401000h
    let dst = Cont(x401000);
    call(ctx, 0x401b60, dst)
}

pub fn x401b60(ctx: &mut Context) -> Cont {
    // 00401b60 jmp short 00401B83h
    Cont(x401b83)
}

pub fn x401b62(ctx: &mut Context) -> Cont {
    // 00401b62 mov eax,[esp+24h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00401b66 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00401b69 mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00401b6d mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00401b6f mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00401b71 mov eax,[esp+40h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00401b75 mov [edx+4],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ebp);
    // 00401b78 mov [edx+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00401b7b mov [edx+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 00401b7e call 00401560h
    let dst = Cont(x401560);
    call(ctx, 0x401b83, dst)
}

pub fn x401b83(ctx: &mut Context) -> Cont {
    // 00401b83 mov edx,[esp+2Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 00401b87 add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    // 00401b8a xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00401b8c test edx,edx
    and(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00401b8e sete cl
    ctx.cpu.regs.set_cl(sete(ctx));
    // 00401b91 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00401b92 mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00401b96 cmp esi,0Ah
    sub(ctx.cpu.regs.esi, 0xau32, &mut ctx.cpu.flags);
    // 00401b99 jl short 00401B1Fh
    jl(ctx, Cont(x401b9b), Cont(x401b1f))
}

pub fn x401b9b(ctx: &mut Context) -> Cont {
    // 00401b9b inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00401b9c cmp edi,0Ah
    sub(ctx.cpu.regs.edi, 0xau32, &mut ctx.cpu.flags);
    // 00401b9f jl near ptr 00401B04h
    jl(ctx, Cont(x401ba5), Cont(x401b04))
}

pub fn x401ba5(ctx: &mut Context) -> Cont {
    // 00401ba5 inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00401ba6 cmp ebx,0Ah
    sub(ctx.cpu.regs.ebx, 0xau32, &mut ctx.cpu.flags);
    // 00401ba9 jl near ptr 00401AEDh
    jl(ctx, Cont(x401baf), Cont(x401aed))
}

pub fn x401baf(ctx: &mut Context) -> Cont {
    // 00401baf pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00401bb0 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00401bb1 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00401bb2 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00401bb3 add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 00401bb6 ret
    ret(ctx, 0)
}

pub fn x401f90(ctx: &mut Context) -> Cont {
    // 00401f90 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00401f94 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401f9a sub esp,240h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x240u32, &mut ctx.cpu.flags);
    // 00401fa0 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00401fa1 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00401fa2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401fa4 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00401fa5 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00401fa6 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401fa9 je short 00401FB2h
    je(ctx, Cont(x401fab), Cont(x401fb2))
}

pub fn x401fab(ctx: &mut Context) -> Cont {
    // 00401fab mov esi,1
    ctx.cpu.regs.esi = 0x1u32;
    // 00401fb0 jmp short 00401FB4h
    Cont(x401fb4)
}

pub fn x401fb2(ctx: &mut Context) -> Cont {
    // 00401fb2 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x401fb4)
}

pub fn x401fb4(ctx: &mut Context) -> Cont {
    // 00401fb4 fld dword ptr [esp+270h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x270u32)) as f64,
    );
    // 00401fbb fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401fc1 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401fc3 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401fc6 je short 00401FCFh
    je(ctx, Cont(x401fc8), Cont(x401fcf))
}

pub fn x401fc8(ctx: &mut Context) -> Cont {
    // 00401fc8 mov edx,1
    ctx.cpu.regs.edx = 0x1u32;
    // 00401fcd jmp short 00401FD1h
    Cont(x401fd1)
}

pub fn x401fcf(ctx: &mut Context) -> Cont {
    // 00401fcf xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    Cont(x401fd1)
}

pub fn x401fd1(ctx: &mut Context) -> Cont {
    // 00401fd1 fld dword ptr [esp+280h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x280u32)) as f64,
    );
    // 00401fd8 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401fde fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401fe0 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00401fe3 je short 00401FECh
    je(ctx, Cont(x401fe5), Cont(x401fec))
}

pub fn x401fe5(ctx: &mut Context) -> Cont {
    // 00401fe5 mov ecx,1
    ctx.cpu.regs.ecx = 0x1u32;
    // 00401fea jmp short 00401FEEh
    Cont(x401fee)
}

pub fn x401fec(ctx: &mut Context) -> Cont {
    // 00401fec xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    Cont(x401fee)
}

pub fn x401fee(ctx: &mut Context) -> Cont {
    // 00401fee fld dword ptr [esp+290h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x290u32)) as f64,
    );
    // 00401ff5 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00401ffb fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00401ffd test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00402000 je short 00402009h
    je(ctx, Cont(x402002), Cont(x402009))
}

pub fn x402002(ctx: &mut Context) -> Cont {
    // 00402002 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 00402007 jmp short 0040200Bh
    Cont(x40200b)
}

pub fn x402009(ctx: &mut Context) -> Cont {
    // 00402009 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x40200b)
}

pub fn x40200b(ctx: &mut Context) -> Cont {
    // 0040200b shl eax,3
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x3u8, &mut ctx.cpu.flags);
    // 0040200e shl ecx,2
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x2u8, &mut ctx.cpu.flags);
    // 00402011 or eax,ecx
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00402013 add edx,edx
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00402015 or eax,edx
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00402017 or eax,esi
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00402019 je near ptr 00403310h
    je(ctx, Cont(x40201f), Cont(x403310))
}

pub fn x40201f(ctx: &mut Context) -> Cont {
    // 0040201f cmp eax,0Fh
    sub(ctx.cpu.regs.eax, 0xfu32, &mut ctx.cpu.flags);
    // 00402022 je near ptr 00403310h
    je(ctx, Cont(x402028), Cont(x403310))
}

pub fn x402028(ctx: &mut Context) -> Cont {
    // 00402028 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00402029 cmp eax,0Dh
    sub(ctx.cpu.regs.eax, 0xdu32, &mut ctx.cpu.flags);
    // 0040202c ja near ptr 00403310h
    ja(ctx, Cont(x402032), Cont(x403310))
}

pub fn x402032(ctx: &mut Context) -> Cont {
    // 00402032 jmp dword ptr [eax*4+40331Ch]
    indirect(
        ctx,
        ctx.memory
            .read((ctx.cpu.regs.eax * 4).wrapping_add(0x40331cu32)),
    )
}

pub fn x403310(ctx: &mut Context) -> Cont {
    // 00403310 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403311 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403312 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00403313 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403314 add esp,240h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x240u32, &mut ctx.cpu.flags);
    // 0040331a ret
    ret(ctx, 0)
}

pub fn x403360(ctx: &mut Context) -> Cont {
    // 00403360 sub esp,3Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x3cu32, &mut ctx.cpu.flags);
    // 00403363 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 00403367 fmul qword ptr ds:[420110h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420110u32));
    // 0040336d push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040336e push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0040336f xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403371 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403372 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403374 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00403375 mov [esp+18h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ebx);
    // 00403379 mov ebp,4296C0h
    ctx.cpu.regs.ebp = 0x4296c0u32;
    // 0040337e fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 00403384 fst dword ptr [esp+48h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x48u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403388 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 0040338c fmul qword ptr ds:[420100h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420100u32));
    // 00403392 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403394 fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 0040339a fstp dword ptr [esp+44h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x44u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040339e fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004033a2 fmul qword ptr ds:[4200F8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200f8u32));
    // 004033a8 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004033aa fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 004033b0 fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004033b4 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004033b8 fmul qword ptr ds:[4200F0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200f0u32));
    // 004033be fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004033c0 fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 004033c6 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004033ca fstp dword ptr [esp+38h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x38u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004033ce fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004033d2 fmul qword ptr ds:[4200E8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200e8u32));
    // 004033d8 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004033da fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004033dc fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 004033e2 fstp dword ptr [esp+34h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x34u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004033e6 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004033ea fmul qword ptr ds:[4200E0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200e0u32));
    // 004033f0 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004033f2 fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 004033f8 fstp dword ptr [esp+40h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x40u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004033fc fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004033fe fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 00403404 fstp dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403408 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 0040340c fmul qword ptr ds:[4200D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200d8u32));
    // 00403412 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403414 fmul qword ptr ds:[420108h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420108u32));
    // 0040341a fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x40341e)
}

pub fn x40341e(ctx: &mut Context) -> Cont {
    // 0040341e xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403420 mov edi,ebp
    ctx.cpu.regs.edi = ctx.cpu.regs.ebp;
    // 00403422 mov [esp+14h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.esi);
    Cont(x403426)
}

pub fn x403426(ctx: &mut Context) -> Cont {
    // 00403426 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00403428 mov edx,edi
    ctx.cpu.regs.edx = ctx.cpu.regs.edi;
    // 0040342a mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    Cont(x40342e)
}

pub fn x40342e(ctx: &mut Context) -> Cont {
    // 0040342e fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 00403432 fcomp dword ptr ds:[4200D0h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4200d0u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403438 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040343a test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040343d je near ptr 00403551h
    je(ctx, Cont(x403443), Cont(x403551))
}

pub fn x403443(ctx: &mut Context) -> Cont {
    // 00403443 lea eax,[ecx-5]
    ctx.cpu.regs.eax = ctx.cpu.regs.ecx.wrapping_add(0xfffffffbu32);
    // 00403446 mov [esp+24h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.eax);
    // 0040344a lea eax,[esi-5]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0xfffffffbu32);
    // 0040344d fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00403451 mov [esp+24h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.eax);
    // 00403455 lea eax,[ebx-5]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx.wrapping_add(0xfffffffbu32);
    // 00403458 fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 0040345e fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00403462 mov [esp+24h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.eax);
    // 00403466 fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 0040346c fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00403470 fmul dword ptr ds:[4200A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4200a0u32) as f64,
    );
    // 00403476 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403478 fsub dword ptr [esp+34h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x34u32)) as f64,
    );
    // 0040347c fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 0040347e fsub dword ptr [esp+38h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as f64,
    );
    // 00403482 fld st(4)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(4));
    // 00403484 fsub dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00403488 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0040348a fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 0040348c fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 0040348e fmul st,st(3)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(3));
    // 00403490 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403492 fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 00403494 fmul st,st(4)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(4));
    // 00403496 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403498 fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 0040349a fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040349e fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034a0 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034a2 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034a4 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004034a8 fsub dword ptr ds:[4200CCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200ccu32) as f64,
    );
    // 004034ae fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004034b2 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004034b4 fsub dword ptr [esp+3Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32)) as f64,
    );
    // 004034b8 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004034ba fsub dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 004034be fld st(4)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(4));
    // 004034c0 fsub dword ptr [esp+40h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 004034c4 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004034c6 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 004034c8 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004034ca fmul st,st(3)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(3));
    // 004034cc faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034ce fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 004034d0 fmul st,st(4)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(4));
    // 004034d2 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034d4 fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 004034d6 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004034da fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034dc fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034de fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004034e0 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004034e4 fsub dword ptr ds:[4200CCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200ccu32) as f64,
    );
    // 004034ea fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004034ee fsub dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 004034f2 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004034f6 fsub dword ptr [esp+44h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 004034fa fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004034fe fsub dword ptr [esp+48h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x48u32)) as f64,
    );
    // 00403502 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403504 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403506 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040350a fmul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040350e faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403510 fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00403514 fmul dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00403518 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040351a fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 0040351c fsub dword ptr ds:[4200CCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200ccu32) as f64,
    );
    // 00403522 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00403526 fcomp
    ctx.cpu.fpu.cmp = ctx.cpu.fpu.get(0).total_cmp(&(ctx.cpu.fpu.get(1)));
    ctx.cpu.fpu.pop();
    // 00403528 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040352a test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040352d je short 00403535h
    je(ctx, Cont(x40352f), Cont(x403535))
}

pub fn x40352f(ctx: &mut Context) -> Cont {
    // 0040352f fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403531 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    Cont(x403535)
}

pub fn x403535(ctx: &mut Context) -> Cont {
    // 00403535 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403539 fcomp
    ctx.cpu.fpu.cmp = ctx.cpu.fpu.get(0).total_cmp(&(ctx.cpu.fpu.get(1)));
    ctx.cpu.fpu.pop();
    // 0040353b fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040353d test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403540 je near ptr 00403605h
    je(ctx, Cont(x403546), Cont(x403605))
}

pub fn x403546(ctx: &mut Context) -> Cont {
    // 00403546 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403548 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 0040354c jmp near ptr 00403605h
    Cont(x403605)
}

pub fn x403551(ctx: &mut Context) -> Cont {
    // 00403551 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 00403555 fcomp dword ptr ds:[4200C8h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4200c8u32) as f64));
    ctx.cpu.fpu.pop();
    // 0040355b fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040355d test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403560 je short 004035A0h
    je(ctx, Cont(x403562), Cont(x4035a0))
}

pub fn x403562(ctx: &mut Context) -> Cont {
    // 00403562 fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00403566 fadd dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 0040356a fmul qword ptr ds:[4200C0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200c0u32));
    // 00403570 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403572 fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00403576 fadd dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 0040357a fmul qword ptr ds:[4200C0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200c0u32));
    // 00403580 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403582 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00403584 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00403586 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403588 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 0040358c fadd dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00403590 fmul qword ptr ds:[4200C0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200c0u32));
    // 00403596 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403598 fst dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040359c faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040359e jmp short 00403605h
    Cont(x403605)
}

pub fn x4035a0(ctx: &mut Context) -> Cont {
    // 004035a0 fld dword ptr [esp+50h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as f64,
    );
    // 004035a4 fcomp dword ptr ds:[4200BCh]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4200bcu32) as f64));
    ctx.cpu.fpu.pop();
    // 004035aa fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 004035ae fsub dword ptr ds:[4200B8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200b8u32) as f64,
    );
    // 004035b4 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 004035b8 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004035ba fsub dword ptr ds:[4200B8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200b8u32) as f64,
    );
    // 004035c0 fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 004035c4 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004035c7 fsub dword ptr ds:[4200B8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200b8u32) as f64,
    );
    // 004035cd fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004035cf fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 004035d1 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004035d3 fmul st,st(3)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(3));
    // 004035d5 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035d7 fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 004035d9 fmul st,st(4)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(4));
    // 004035db faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035dd fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 004035df je short 004035F1h
    je(ctx, Cont(x4035e1), Cont(x4035f1))
}

pub fn x4035e1(ctx: &mut Context) -> Cont {
    // 004035e1 fstp st(3)
    ctx.cpu.fpu.set(3, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035e3 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035e5 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035e7 fmul qword ptr ds:[4200B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200b0u32));
    // 004035ed fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004035ef jmp short 00403605h
    Cont(x403605)
}

pub fn x4035f1(ctx: &mut Context) -> Cont {
    // 004035f1 fstp st(3)
    ctx.cpu.fpu.set(3, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035f3 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035f5 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004035f7 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004035f9 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004035fb fmul qword ptr ds:[4200B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200b0u32));
    // 00403601 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403603 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    Cont(x403605)
}

pub fn x403605(ctx: &mut Context) -> Cont {
    // 00403605 fstp dword ptr [edx]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.edx, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403607 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00403608 add edx,4
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x4u32, &mut ctx.cpu.flags);
    // 0040360b cmp ecx,0Eh
    sub(ctx.cpu.regs.ecx, 0xeu32, &mut ctx.cpu.flags);
    // 0040360e mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00403612 jl near ptr 0040342Eh
    jl(ctx, Cont(x403618), Cont(x40342e))
}

pub fn x403618(ctx: &mut Context) -> Cont {
    // 00403618 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403619 add edi,80h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x80u32, &mut ctx.cpu.flags);
    // 0040361f cmp esi,0Eh
    sub(ctx.cpu.regs.esi, 0xeu32, &mut ctx.cpu.flags);
    // 00403622 mov [esp+14h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.esi);
    // 00403626 jl near ptr 00403426h
    jl(ctx, Cont(x40362c), Cont(x403426))
}

pub fn x40362c(ctx: &mut Context) -> Cont {
    // 0040362c add ebp,1000h
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x1000u32, &mut ctx.cpu.flags);
    // 00403632 inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403633 cmp ebp,4376C0h
    sub(ctx.cpu.regs.ebp, 0x4376c0u32, &mut ctx.cpu.flags);
    // 00403639 mov [esp+18h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ebx);
    // 0040363d jl near ptr 0040341Eh
    jl(ctx, Cont(x403643), Cont(x40341e))
}

pub fn x403643(ctx: &mut Context) -> Cont {
    // 00403643 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403644 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403645 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00403646 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403647 add esp,3Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x3cu32, &mut ctx.cpu.flags);
    // 0040364a ret
    ret(ctx, 0)
}

pub fn x403650(ctx: &mut Context) -> Cont {
    // 00403650 push 40A1A0h
    push(ctx, 0x40a1a0u32);
    // 00403655 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x40365a, dst)
}

pub fn x40365a(ctx: &mut Context) -> Cont {
    // 0040365a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040365d mov ds:[4496FCh],eax
    ctx.memory.write::<u32>(0x4496fcu32, ctx.cpu.regs.eax);
    // 00403662 ret
    ret(ctx, 0)
}

pub fn x403670(ctx: &mut Context) -> Cont {
    // 00403670 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00403671 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403672 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403674 mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    Cont(x403678)
}

pub fn x403678(ctx: &mut Context) -> Cont {
    // 00403678 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 0040367c fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00403682 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403684 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403687 je short 00403693h
    je(ctx, Cont(x403689), Cont(x403693))
}

pub fn x403689(ctx: &mut Context) -> Cont {
    // 00403689 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040368b fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 00403691 jmp short 004036A8h
    Cont(x4036a8)
}

pub fn x403693(ctx: &mut Context) -> Cont {
    // 00403693 fcom dword ptr ds:[420120h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420120u32) as f64));
    // 00403699 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040369b test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 0040369e jne short 004036A8h
    jne(ctx, Cont(x4036a0), Cont(x4036a8))
}

pub fn x4036a0(ctx: &mut Context) -> Cont {
    // 004036a0 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004036a2 fld dword ptr ds:[420120h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420120u32) as f64);
    Cont(x4036a8)
}

pub fn x4036a8(ctx: &mut Context) -> Cont {
    // 004036a8 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004036aa call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4036af, dst)
}

pub fn x4036af(ctx: &mut Context) -> Cont {
    // 004036af mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 004036b3 mov ecx,ds:[425C00h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425c00u32);
    // 004036b9 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004036bd lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 004036c0 shl eax,7
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 004036c3 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 004036c5 mov [ecx+esi*4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.eax,
    );
    // 004036c8 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 004036ce call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4036d3, dst)
}

pub fn x4036d3(ctx: &mut Context) -> Cont {
    // 004036d3 mov edx,ds:[425C04h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425c04u32);
    // 004036d9 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004036da cmp esi,0C8h
    sub(ctx.cpu.regs.esi, 0xc8u32, &mut ctx.cpu.flags);
    // 004036e0 mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    // 004036e4 mov [edx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 004036e8 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004036ea jl short 00403678h
    jl(ctx, Cont(x4036ec), Cont(x403678))
}

pub fn x4036ec(ctx: &mut Context) -> Cont {
    // 004036ec xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004036ee mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    Cont(x4036f2)
}

pub fn x4036f2(ctx: &mut Context) -> Cont {
    // 004036f2 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004036f6 fadd dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 004036fa fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00403700 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403702 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403705 je short 00403711h
    je(ctx, Cont(x403707), Cont(x403711))
}

pub fn x403707(ctx: &mut Context) -> Cont {
    // 00403707 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403709 fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 0040370f jmp short 00403726h
    Cont(x403726)
}

pub fn x403711(ctx: &mut Context) -> Cont {
    // 00403711 fcom dword ptr ds:[420118h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420118u32) as f64));
    // 00403717 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403719 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 0040371c jne short 00403726h
    jne(ctx, Cont(x40371e), Cont(x403726))
}

pub fn x40371e(ctx: &mut Context) -> Cont {
    // 0040371e fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403720 fld dword ptr ds:[420118h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420118u32) as f64);
    Cont(x403726)
}

pub fn x403726(ctx: &mut Context) -> Cont {
    // 00403726 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403728 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40372d, dst)
}

pub fn x40372d(ctx: &mut Context) -> Cont {
    // 0040372d mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00403731 mov ecx,ds:[428CE4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428ce4u32);
    // 00403737 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 0040373b mov [ecx+esi*4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.eax,
    );
    // 0040373e fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00403740 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 00403746 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40374b, dst)
}

pub fn x40374b(ctx: &mut Context) -> Cont {
    // 0040374b mov edx,ds:[428CE8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428ce8u32);
    // 00403751 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403752 cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00403758 mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    // 0040375c mov [edx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 00403760 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403762 jl short 004036F2h
    jl(ctx, Cont(x403764), Cont(x4036f2))
}

pub fn x403764(ctx: &mut Context) -> Cont {
    // 00403764 call 00407620h
    let dst = Cont(x407620);
    call(ctx, 0x403769, dst)
}

pub fn x403769(ctx: &mut Context) -> Cont {
    // 00403769 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040376a pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 0040376b ret
    ret(ctx, 0)
}

pub fn x403770(ctx: &mut Context) -> Cont {
    // 00403770 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00403773 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00403774 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403775 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00403776 mov dword ptr [esp+10h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), 0x0u32);
    // 0040377e mov edi,[esp+10h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00403782 mov dword ptr [esp+14h],3F800000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), 0x3f800000u32);
    // 0040378a mov esi,[esp+14h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0040378e mov dword ptr [esp+18h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), 0x0u32);
    // 00403796 mov ebx,[esp+18h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    Cont(x40379a)
}

pub fn x40379a(ctx: &mut Context) -> Cont {
    // 0040379a call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x40379f, dst)
}

pub fn x40379f(ctx: &mut Context) -> Cont {
    // 0040379f fmul dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 004037a5 push 0
    push(ctx, 0x0u32);
    // 004037a7 fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004037ab call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x4037b0, dst)
}

pub fn x4037b0(ctx: &mut Context) -> Cont {
    // 004037b0 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 004037b4 fmul qword ptr ds:[420180h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420180u32));
    // 004037ba lea eax,[esp+14h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x14u32);
    // 004037be push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004037bf fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004037c1 fmul qword ptr ds:[420178h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420178u32));
    // 004037c7 fstp dword ptr ds:[4496F0h]
    ctx.memory
        .write::<f32>(0x4496f0u32, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004037cd fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004037d1 fmul qword ptr ds:[420170h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420170u32));
    // 004037d7 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004037d9 fmul qword ptr ds:[420168h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420168u32));
    // 004037df fstp dword ptr ds:[4496F4h]
    ctx.memory
        .write::<f32>(0x4496f4u32, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004037e5 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004037e9 fmul qword ptr ds:[420160h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420160u32));
    // 004037ef fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004037f1 fmul qword ptr ds:[420158h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420158u32));
    // 004037f7 fstp dword ptr ds:[4496F8h]
    ctx.memory
        .write::<f32>(0x4496f8u32, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004037fd fld dword ptr ds:[4496F0h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4496f0u32) as f64);
    // 00403803 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00403805 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403809 fld dword ptr ds:[4496F4h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4496f4u32) as f64);
    // 0040380f fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00403811 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403815 fld dword ptr ds:[4496F8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4496f8u32) as f64);
    // 0040381b fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 0040381d fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403821 call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x403826, dst)
}

pub fn x403826(ctx: &mut Context) -> Cont {
    // 00403826 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00403829 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0040382d push 4496C0h
    push(ctx, 0x4496c0u32);
    // 00403832 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00403835 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00403837 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040383a mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 0040383c mov [ecx],edi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edi);
    // 0040383e mov [ecx+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    // 00403841 mov [ecx+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 00403844 mov ecx,[esp+30h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 00403848 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 0040384a mov eax,[esp+34h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 0040384e mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00403851 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00403854 call 00406410h
    let dst = Cont(x406410);
    call(ctx, 0x403859, dst)
}

pub fn x403859(ctx: &mut Context) -> Cont {
    // 00403859 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 0040385d fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 0040385f add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 00403862 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403865 call 00403360h
    let dst = Cont(x403360);
    call(ctx, 0x40386a, dst)
}

pub fn x40386a(ctx: &mut Context) -> Cont {
    // 0040386a fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040386e fcomp dword ptr ds:[420150h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420150u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403874 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00403877 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403879 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040387c je short 00403883h
    je(ctx, Cont(x40387e), Cont(x403883))
}

pub fn x40387e(ctx: &mut Context) -> Cont {
    // 0040387e call 00401AE0h
    let dst = Cont(x401ae0);
    call(ctx, 0x403883, dst)
}

pub fn x403883(ctx: &mut Context) -> Cont {
    // 00403883 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00403887 fmul qword ptr ds:[420148h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420148u32));
    // 0040388d push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040388e fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403890 fmul qword ptr ds:[420140h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420140u32));
    // 00403896 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403899 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040389d fmul qword ptr ds:[420138h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420138u32));
    // 004038a3 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004038a4 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004038a6 fmul qword ptr ds:[420130h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420130u32));
    // 004038ac fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004038af call 00403670h
    let dst = Cont(x403670);
    call(ctx, 0x4038b4, dst)
}

pub fn x4038b4(ctx: &mut Context) -> Cont {
    // 004038b4 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004038b8 fcomp dword ptr ds:[42012Ch]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x42012cu32) as f64));
    ctx.cpu.fpu.pop();
    // 004038be add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 004038c1 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004038c3 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004038c6 je short 00403922h
    je(ctx, Cont(x4038c8), Cont(x403922))
}

pub fn x4038c8(ctx: &mut Context) -> Cont {
    // 004038c8 push 421014h
    push(ctx, 0x421014u32);
    // 004038cd push 0FFh
    push(ctx, 0xffu32);
    // 004038d2 push 19h
    push(ctx, 0x19u32);
    // 004038d4 push 32h
    push(ctx, 0x32u32);
    // 004038d6 push 32h
    push(ctx, 0x32u32);
    // 004038d8 push 32h
    push(ctx, 0x32u32);
    // 004038da push 1E0h
    push(ctx, 0x1e0u32);
    // 004038df call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x4038e4, dst)
}

pub fn x4038e4(ctx: &mut Context) -> Cont {
    // 004038e4 push 42100Ch
    push(ctx, 0x42100cu32);
    // 004038e9 push 0FFh
    push(ctx, 0xffu32);
    // 004038ee push 1Eh
    push(ctx, 0x1eu32);
    // 004038f0 push 1Eh
    push(ctx, 0x1eu32);
    // 004038f2 push 1Eh
    push(ctx, 0x1eu32);
    // 004038f4 push 64h
    push(ctx, 0x64u32);
    // 004038f6 push 1E0h
    push(ctx, 0x1e0u32);
    // 004038fb call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x403900, dst)
}

pub fn x403900(ctx: &mut Context) -> Cont {
    // 00403900 push 421000h
    push(ctx, 0x421000u32);
    // 00403905 push 0FFh
    push(ctx, 0xffu32);
    // 0040390a push 1Eh
    push(ctx, 0x1eu32);
    // 0040390c push 0Fh
    push(ctx, 0xfu32);
    // 0040390e push 0Fh
    push(ctx, 0xfu32);
    // 00403910 push 96h
    push(ctx, 0x96u32);
    // 00403915 push 1E0h
    push(ctx, 0x1e0u32);
    // 0040391a call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x40391f, dst)
}

pub fn x40391f(ctx: &mut Context) -> Cont {
    // 0040391f add esp,54h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x54u32, &mut ctx.cpu.flags);
    Cont(x403922)
}

pub fn x403922(ctx: &mut Context) -> Cont {
    // 00403922 call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x403927, dst)
}

pub fn x403927(ctx: &mut Context) -> Cont {
    // 00403927 call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x40392c, dst)
}

pub fn x40392c(ctx: &mut Context) -> Cont {
    // 0040392c call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x403931, dst)
}

pub fn x403931(ctx: &mut Context) -> Cont {
    // 00403931 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00403933 jne short 00403950h
    jne(ctx, Cont(x403935), Cont(x403950))
}

pub fn x403935(ctx: &mut Context) -> Cont {
    // 00403935 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00403939 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 0040393f fcomp dword ptr ds:[420124h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420124u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403945 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403947 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040394a jne near ptr 0040379Ah
    jne(ctx, Cont(x403950), Cont(x40379a))
}

pub fn x403950(ctx: &mut Context) -> Cont {
    // 00403950 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403951 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403952 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403953 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00403956 ret
    ret(ctx, 0)
}

pub fn x403960(ctx: &mut Context) -> Cont {
    // 00403960 push 40A040h
    push(ctx, 0x40a040u32);
    // 00403965 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x40396a, dst)
}

pub fn x40396a(ctx: &mut Context) -> Cont {
    // 0040396a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040396d mov ds:[4296B0h],eax
    ctx.memory.write::<u32>(0x4296b0u32, ctx.cpu.regs.eax);
    // 00403972 ret
    ret(ctx, 0)
}

pub fn x403980(ctx: &mut Context) -> Cont {
    // 00403980 ret
    ret(ctx, 0)
}

pub fn x403990(ctx: &mut Context) -> Cont {
    // 00403990 sub esp,40h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x40u32, &mut ctx.cpu.flags);
    // 00403993 fld dword ptr [esp+44h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 00403997 fmul dword ptr ds:[420208h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420208u32) as f64,
    );
    // 0040399d push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040399e push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0040399f mov ebp,[esp+4Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 004039a3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004039a4 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004039a6 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004039a7 mov dword ptr [esp+10h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), 0x0u32);
    // 004039af mov dword ptr [esp+14h],425C2Ch
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), 0x425c2cu32);
    // 004039b7 fmul qword ptr ds:[420200h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420200u32));
    // 004039bd fadd qword ptr ds:[4201F8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201f8u32));
    // 004039c3 fstp dword ptr [esp+40h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x40u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004039c7 fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004039cb fmul dword ptr ds:[4201F0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201f0u32) as f64,
    );
    // 004039d1 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004039d3 fmul qword ptr ds:[4201E8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201e8u32));
    // 004039d9 fadd qword ptr ds:[4201E0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201e0u32));
    // 004039df fstp dword ptr [esp+34h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x34u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004039e3 fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004039e7 fmul dword ptr ds:[4201D8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201d8u32) as f64,
    );
    // 004039ed fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004039ef fmul qword ptr ds:[4201D0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201d0u32));
    // 004039f5 fadd qword ptr ds:[4201C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201c8u32));
    // 004039fb fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004039ff fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 00403a03 fmul dword ptr ds:[4201C0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201c0u32) as f64,
    );
    // 00403a09 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403a0b fmul qword ptr ds:[4201D0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201d0u32));
    // 00403a11 fadd qword ptr ds:[4201C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201c8u32));
    // 00403a17 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403a1b fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 00403a1f fmul dword ptr ds:[4201BCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201bcu32) as f64,
    );
    // 00403a25 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403a27 fmul qword ptr ds:[4201D0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201d0u32));
    // 00403a2d fadd qword ptr ds:[4201C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201c8u32));
    // 00403a33 fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 00403a37 fmul dword ptr ds:[4201B8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b8u32) as f64,
    );
    // 00403a3d fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403a3f fmul qword ptr ds:[4201D0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201d0u32));
    // 00403a45 fadd qword ptr ds:[4201C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4201c8u32));
    // 00403a4b fsub dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00403a4f fstp dword ptr [esp+38h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x38u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403a53 fsub dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00403a57 fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x403a5b)
}

pub fn x403a5b(ctx: &mut Context) -> Cont {
    // 00403a5b fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00403a5f mov esi,[esp+14h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00403a63 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403a65 mov [esp+2Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), ctx.cpu.regs.edi);
    // 00403a69 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403a6b fsub dword ptr [esp+34h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x34u32)) as f64,
    );
    // 00403a6f fst dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403a73 fmul dword ptr ds:[4201B4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b4u32) as f64,
    );
    // 00403a79 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403a7d fld dword ptr [esp+38h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as f64,
    );
    // 00403a81 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 00403a83 mov ebx,[esp+28h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00403a87 fmul dword ptr ds:[4201B0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b0u32) as f64,
    );
    // 00403a8d fadd dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00403a91 fstp dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403a95 fld dword ptr [esp+3Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32)) as f64,
    );
    // 00403a99 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 00403a9b fmul dword ptr ds:[4201B0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b0u32) as f64,
    );
    // 00403aa1 fadd dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00403aa5 fsub dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00403aa9 fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403aad fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    Cont(x403aaf)
}

pub fn x403aaf(ctx: &mut Context) -> Cont {
    // 00403aaf fild dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as i32 as f64,
    );
    // 00403ab3 mov eax,[esp+20h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00403ab7 lea ecx,[esp+48h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x48u32);
    // 00403abb push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00403abc lea edx,[esp+48h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x48u32);
    // 00403ac0 fst dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403ac4 fsub dword ptr [esp+44h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 00403ac8 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00403ac9 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00403aca push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00403acb push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00403acc mov [esp+38h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.regs.eax);
    // 00403ad0 fst dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403ad4 fmul dword ptr ds:[4201B4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b4u32) as f64,
    );
    // 00403ada fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403add call 00407E40h
    let dst = Cont(x407e40);
    call(ctx, 0x403ae2, dst)
}

pub fn x403ae2(ctx: &mut Context) -> Cont {
    // 00403ae2 fld dword ptr [esp+58h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x58u32)) as f64,
    );
    // 00403ae6 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00403ae9 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403aee, dst)
}

pub fn x403aee(ctx: &mut Context) -> Cont {
    // 00403aee fld dword ptr [esp+48h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x48u32)) as f64,
    );
    // 00403af2 mov [esi-0Ch],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32),
        ctx.cpu.regs.eax,
    );
    // 00403af5 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403afa, dst)
}

pub fn x403afa(ctx: &mut Context) -> Cont {
    // 00403afa fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403afe fmul dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403b02 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00403b06 fmul dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00403b0a mov [esi-8],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.eax,
    );
    // 00403b0d faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403b0f fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 00403b11 fmul dword ptr ds:[4201ACh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201acu32) as f64,
    );
    // 00403b17 fadd dword ptr [esp+54h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 00403b1b fmul dword ptr ds:[4201A8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201a8u32) as f64,
    );
    // 00403b21 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403b23 fmul qword ptr ds:[4201A0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201a0u32));
    // 00403b29 fadd qword ptr ds:[420198h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x420198u32));
    // 00403b2f fmul dword ptr ds:[420190h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420190u32) as f64,
    );
    // 00403b35 fld dword ptr [esp+4Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32)) as f64,
    );
    // 00403b39 fmul dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00403b3d fmul dword ptr ds:[42018Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42018cu32) as f64,
    );
    // 00403b43 fadd dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00403b47 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403b49 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403b4e, dst)
}

pub fn x403b4e(ctx: &mut Context) -> Cont {
    // 00403b4e mov [esi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, ctx.cpu.regs.eax);
    // 00403b50 mov dword ptr [esi-4],400h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32), 0x400u32);
    // 00403b57 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403b58 add esi,10h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x10u32, &mut ctx.cpu.flags);
    // 00403b5b cmp edi,21h
    sub(ctx.cpu.regs.edi, 0x21u32, &mut ctx.cpu.flags);
    // 00403b5e mov [esp+2Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), ctx.cpu.regs.edi);
    // 00403b62 jl near ptr 00403AAFh
    jl(ctx, Cont(x403b68), Cont(x403aaf))
}

pub fn x403b68(ctx: &mut Context) -> Cont {
    // 00403b68 mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00403b6c mov esi,[esp+10h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00403b70 add eax,400h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x400u32, &mut ctx.cpu.flags);
    // 00403b75 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403b76 cmp eax,42882Ch
    sub(ctx.cpu.regs.eax, 0x42882cu32, &mut ctx.cpu.flags);
    // 00403b7b mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    // 00403b7f mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 00403b83 jl near ptr 00403A5Bh
    jl(ctx, Cont(x403b89), Cont(x403a5b))
}

pub fn x403b89(ctx: &mut Context) -> Cont {
    // 00403b89 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403b8a pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403b8b pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00403b8c pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403b8d add esp,40h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x40u32, &mut ctx.cpu.flags);
    // 00403b90 ret
    ret(ctx, 0)
}

pub fn x403ba0(ctx: &mut Context) -> Cont {
    // 00403ba0 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00403ba1 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00403ba3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403ba4 mov esi,ds:[428CF8h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00403baa add esi,[ebp+8]
    ctx.cpu.regs.esi = add(
        ctx.cpu.regs.esi,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)),
        &mut ctx.cpu.flags,
    );
    // 00403bad mov ecx,[ebp+0Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xcu32));
    // 00403bb0 shr ecx,1
    ctx.cpu.regs.ecx = shr(ctx.cpu.regs.ecx, 0x1u8, &mut ctx.cpu.flags);
    Cont(x403bb2)
}

pub fn x403bb2(ctx: &mut Context) -> Cont {
    // 00403bb2 mov ax,[esi]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u16>(ctx.cpu.regs.esi));
    // 00403bb5 shr al,2
    ctx.cpu
        .regs
        .set_al(shr(ctx.cpu.regs.get_al(), 0x2u8, &mut ctx.cpu.flags));
    // 00403bb8 shr ah,2
    ctx.cpu
        .regs
        .set_ah(shr(ctx.cpu.regs.get_ah(), 0x2u8, &mut ctx.cpu.flags));
    // 00403bbb mov [esi],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esi, ctx.cpu.regs.get_ax());
    // 00403bbe inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403bbf inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00403bc0 loop 00403BB2h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x403bc2)
    } else {
        Cont(x403bb2)
    }
}

pub fn x403bc2(ctx: &mut Context) -> Cont {
    // 00403bc2 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403bc3 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00403bc4 ret
    ret(ctx, 0)
}

pub fn x403bd0(ctx: &mut Context) -> Cont {
    // 00403bd0 sub esp,7Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x7cu32, &mut ctx.cpu.flags);
    // 00403bd3 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00403bd4 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403bd5 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00403bd6 lea esi,[esp+10h]
    ctx.cpu.regs.esi = ctx.cpu.regs.esp.wrapping_add(0x10u32);
    // 00403bda mov edi,1Eh
    ctx.cpu.regs.edi = 0x1eu32;
    Cont(x403bdf)
}

pub fn x403bdf(ctx: &mut Context) -> Cont {
    // 00403bdf call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x403be4, dst)
}

pub fn x403be4(ctx: &mut Context) -> Cont {
    // 00403be4 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00403be5 mov ecx,2710h
    ctx.cpu.regs.ecx = 0x2710u32;
    // 00403bea add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 00403bed idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00403bef dec edi
    ctx.cpu.regs.edi = dec(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403bf0 mov [esi-4],edx
    ctx.memory.write::<u32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.edx,
    );
    // 00403bf3 jne short 00403BDFh
    jne(ctx, Cont(x403bf5), Cont(x403bdf))
}

pub fn x403bf5(ctx: &mut Context) -> Cont {
    // 00403bf5 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x403bfa, dst)
}

pub fn x403bfa(ctx: &mut Context) -> Cont {
    // 00403bfa fmul dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 00403c00 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403c05, dst)
}

pub fn x403c05(ctx: &mut Context) -> Cont {
    // 00403c05 mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00403c07 push 0
    push(ctx, 0x0u32);
    // 00403c09 mov edi,ebx
    ctx.cpu.regs.edi = ctx.cpu.regs.ebx;
    // 00403c0b mov esi,ebx
    ctx.cpu.regs.esi = ctx.cpu.regs.ebx;
    // 00403c0d sar edi,7
    ctx.cpu.regs.edi = sar(ctx.cpu.regs.edi, 0x7u8, &mut ctx.cpu.flags);
    // 00403c10 dec edi
    ctx.cpu.regs.edi = dec(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403c11 and esi,7Fh
    ctx.cpu.regs.esi = and(ctx.cpu.regs.esi, 0x7fu32, &mut ctx.cpu.flags);
    // 00403c14 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x403c19, dst)
}

pub fn x403c19(ctx: &mut Context) -> Cont {
    // 00403c19 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00403c1c cmp ebx,5
    sub(ctx.cpu.regs.ebx, 0x5u32, &mut ctx.cpu.flags);
    // 00403c1f jge short 00403C2Eh
    jge(ctx, Cont(x403c21), Cont(x403c2e))
}

pub fn x403c21(ctx: &mut Context) -> Cont {
    // 00403c21 push 0FFh
    push(ctx, 0xffu32);
    // 00403c26 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x403c2b, dst)
}

pub fn x403c2b(ctx: &mut Context) -> Cont {
    // 00403c2b add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    Cont(x403c2e)
}

pub fn x403c2e(ctx: &mut Context) -> Cont {
    // 00403c2e mov edx,[esp+edi*4+38h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add((ctx.cpu.regs.edi * 4))
            .wrapping_add(0x38u32),
    );
    // 00403c32 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00403c33 add edx,ebx
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403c35 mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 00403c39 fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00403c3d fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403c40 call 00403990h
    let dst = Cont(x403990);
    call(ctx, 0x403c45, dst)
}

pub fn x403c45(ctx: &mut Context) -> Cont {
    // 00403c45 mov eax,ds:[4296B0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x4296b0u32);
    // 00403c4a push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00403c4b call 00406E80h
    let dst = Cont(x406e80);
    call(ctx, 0x403c50, dst)
}

pub fn x403c50(ctx: &mut Context) -> Cont {
    // 00403c50 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00403c53 inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403c54 test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403c56 mov [esp+0Ch],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32), ctx.cpu.regs.ebx);
    // 00403c5a jl near ptr 00403D0Ah
    jl(ctx, Cont(x403c60), Cont(x403d0a))
}

pub fn x403c60(ctx: &mut Context) -> Cont {
    // 00403c60 cmp edi,4
    sub(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 00403c63 jge near ptr 00403D0Ah
    jge(ctx, Cont(x403c69), Cont(x403d0a))
}

pub fn x403c69(ctx: &mut Context) -> Cont {
    // 00403c69 cmp esi,5
    sub(ctx.cpu.regs.esi, 0x5u32, &mut ctx.cpu.flags);
    // 00403c6c jge short 00403C80h
    jge(ctx, Cont(x403c6e), Cont(x403c80))
}

pub fn x403c6e(ctx: &mut Context) -> Cont {
    // 00403c6e push 0FFh
    push(ctx, 0xffu32);
    // 00403c73 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x403c78, dst)
}

pub fn x403c78(ctx: &mut Context) -> Cont {
    // 00403c78 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00403c7b jmp near ptr 00403D0Ah
    Cont(x403d0a)
}

pub fn x403c80(ctx: &mut Context) -> Cont {
    // 00403c80 fild dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as i32 as f64,
    );
    // 00403c84 fmul dword ptr ds:[42020Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42020cu32) as f64,
    );
    // 00403c8a fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403c8c fmul qword ptr ds:[4201C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201c8u32));
    // 00403c92 fadd qword ptr ds:[420178h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x420178u32));
    // 00403c98 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403c9d, dst)
}

pub fn x403c9d(ctx: &mut Context) -> Cont {
    // 00403c9d mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00403c9f call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x403ca4, dst)
}

pub fn x403ca4(ctx: &mut Context) -> Cont {
    // 00403ca4 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x403ca9, dst)
}

pub fn x403ca9(ctx: &mut Context) -> Cont {
    // 00403ca9 push 7D00h
    push(ctx, 0x7d00u32);
    // 00403cae push 6400h
    push(ctx, 0x6400u32);
    // 00403cb3 call 00403BA0h
    let dst = Cont(x403ba0);
    call(ctx, 0x403cb8, dst)
}

pub fn x403cb8(ctx: &mut Context) -> Cont {
    // 00403cb8 push 7D00h
    push(ctx, 0x7d00u32);
    // 00403cbd push 11F80h
    push(ctx, 0x11f80u32);
    // 00403cc2 call 00403BA0h
    let dst = Cont(x403ba0);
    call(ctx, 0x403cc7, dst)
}

pub fn x403cc7(ctx: &mut Context) -> Cont {
    // 00403cc7 mov ecx,[edi*8+42101Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>((ctx.cpu.regs.edi * 8).wrapping_add(0x42101cu32));
    // 00403cce add esi,1Eh
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x1eu32, &mut ctx.cpu.flags);
    // 00403cd1 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00403cd2 push 0FFh
    push(ctx, 0xffu32);
    // 00403cd7 push 1Eh
    push(ctx, 0x1eu32);
    // 00403cd9 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403cda push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403cdb push 41h
    push(ctx, 0x41u32);
    // 00403cdd push 118h
    push(ctx, 0x118u32);
    // 00403ce2 call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x403ce7, dst)
}

pub fn x403ce7(ctx: &mut Context) -> Cont {
    // 00403ce7 mov edx,[edi*8+421020h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>((ctx.cpu.regs.edi * 8).wrapping_add(0x421020u32));
    // 00403cee push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00403cef push 0FFh
    push(ctx, 0xffu32);
    // 00403cf4 push 1Eh
    push(ctx, 0x1eu32);
    // 00403cf6 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403cf7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403cf8 push 8Ch
    push(ctx, 0x8cu32);
    // 00403cfd push 168h
    push(ctx, 0x168u32);
    // 00403d02 call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x403d07, dst)
}

pub fn x403d07(ctx: &mut Context) -> Cont {
    // 00403d07 add esp,48h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x48u32, &mut ctx.cpu.flags);
    Cont(x403d0a)
}

pub fn x403d0a(ctx: &mut Context) -> Cont {
    // 00403d0a call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x403d0f, dst)
}

pub fn x403d0f(ctx: &mut Context) -> Cont {
    // 00403d0f call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x403d14, dst)
}

pub fn x403d14(ctx: &mut Context) -> Cont {
    // 00403d14 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x403d19, dst)
}

pub fn x403d19(ctx: &mut Context) -> Cont {
    // 00403d19 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00403d1b jne short 00403D29h
    jne(ctx, Cont(x403d1d), Cont(x403d29))
}

pub fn x403d1d(ctx: &mut Context) -> Cont {
    // 00403d1d cmp ebx,200h
    sub(ctx.cpu.regs.ebx, 0x200u32, &mut ctx.cpu.flags);
    // 00403d23 jl near ptr 00403BF5h
    jl(ctx, Cont(x403d29), Cont(x403bf5))
}

pub fn x403d29(ctx: &mut Context) -> Cont {
    // 00403d29 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403d2a pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00403d2b pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403d2c add esp,7Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x7cu32, &mut ctx.cpu.flags);
    // 00403d2f ret
    ret(ctx, 0)
}

pub fn x403d30(ctx: &mut Context) -> Cont {
    // 00403d30 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00403d33 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00403d34 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00403d35 mov ebp,[esp+1Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00403d39 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00403d3b xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403d3d push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00403d3e mov cl,[ebp]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.ebp));
    // 00403d41 mov [esp+18h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ebx);
    // 00403d45 cmp cl,al
    sub(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    );
    // 00403d47 je short 00403D56h
    je(ctx, Cont(x403d49), Cont(x403d56))
}

pub fn x403d49(ctx: &mut Context) -> Cont {
    // 00403d49 mov cl,[ebx+ebp+1]
    ctx.cpu.regs.set_cl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .ebx
                .wrapping_add(ctx.cpu.regs.ebp)
                .wrapping_add(0x1u32),
        ),
    );
    // 00403d4d inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403d4e cmp cl,al
    sub(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    );
    // 00403d50 jne short 00403D49h
    jne(ctx, Cont(x403d52), Cont(x403d49))
}

pub fn x403d52(ctx: &mut Context) -> Cont {
    // 00403d52 mov [esp+18h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ebx);
    Cont(x403d56)
}

pub fn x403d56(ctx: &mut Context) -> Cont {
    // 00403d56 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403d58 cmp ebx,eax
    sub(ctx.cpu.regs.ebx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00403d5a jle near ptr 00403E83h
    jle(ctx, Cont(x403d60), Cont(x403e83))
}

pub fn x403d60(ctx: &mut Context) -> Cont {
    // 00403d60 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403d61 mov [esp+18h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.eax);
    // 00403d65 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    Cont(x403d69)
}

pub fn x403d69(ctx: &mut Context) -> Cont {
    // 00403d69 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 00403d6d fsubr dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64
            - ctx.cpu.fpu.get(0),
    );
    // 00403d71 fst dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403d75 fcomp dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403d7b fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403d7d test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403d80 jne near ptr 00403E63h
    jne(ctx, Cont(x403d86), Cont(x403e63))
}

pub fn x403d86(ctx: &mut Context) -> Cont {
    // 00403d86 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403d8a fcomp dword ptr ds:[420230h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420230u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403d90 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403d92 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403d95 je near ptr 00403E63h
    je(ctx, Cont(x403d9b), Cont(x403e63))
}

pub fn x403d9b(ctx: &mut Context) -> Cont {
    // 00403d9b fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403d9f fmul dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403da3 fmul dword ptr ds:[42022Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42022cu32) as f64,
    );
    // 00403da9 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00403daf fdivr dword ptr ds:[420228h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x420228u32) as f64 / ctx.cpu.fpu.get(0),
    );
    // 00403db5 fadd dword ptr ds:[420224h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420224u32) as f64,
    );
    // 00403dbb fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403dbf fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00403dc3 fmul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00403dc7 fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 00403dcb fmul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00403dcf fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403dd1 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00403dd5 fmul dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00403dd9 fmul dword ptr ds:[420220h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420220u32) as f64,
    );
    // 00403ddf fld dword ptr ds:[42021Ch]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x42021cu32) as f64);
    // 00403de5 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403de9 fcomp dword ptr ds:[420218h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420218u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403def fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403df1 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00403df4 jne short 00403E2Eh
    jne(ctx, Cont(x403df6), Cont(x403e2e))
}

pub fn x403df6(ctx: &mut Context) -> Cont {
    // 00403df6 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403df8 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403dfc fsub dword ptr ds:[420218h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420218u32) as f64,
    );
    // 00403e02 fst dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00403e06 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403e08 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403e0a fmul dword ptr ds:[420214h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420214u32) as f64,
    );
    // 00403e10 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00403e16 fdivr dword ptr ds:[42021Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x42021cu32) as f64 / ctx.cpu.fpu.get(0),
    );
    // 00403e1c fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00403e20 fmul dword ptr ds:[420210h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420210u32) as f64,
    );
    // 00403e26 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00403e2c fmulp st(3),st
    ctx.cpu.fpu.set(3, ctx.cpu.fpu.get(3) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    Cont(x403e2e)
}

pub fn x403e2e(ctx: &mut Context) -> Cont {
    // 00403e2e fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00403e32 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403e37, dst)
}

pub fn x403e37(ctx: &mut Context) -> Cont {
    // 00403e37 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00403e39 shl esi,1
    ctx.cpu.regs.esi = shl(ctx.cpu.regs.esi, 0x1u8, &mut ctx.cpu.flags);
    // 00403e3b call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403e40, dst)
}

pub fn x403e40(ctx: &mut Context) -> Cont {
    // 00403e40 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00403e41 mov al,[edi+ebp]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ebp)),
    );
    // 00403e44 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00403e45 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403e46 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00403e47 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403e4c, dst)
}

pub fn x403e4c(ctx: &mut Context) -> Cont {
    // 00403e4c add eax,64h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x64u32, &mut ctx.cpu.flags);
    // 00403e4f push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00403e50 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x403e55, dst)
}

pub fn x403e55(ctx: &mut Context) -> Cont {
    // 00403e55 add eax,140h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x140u32, &mut ctx.cpu.flags);
    // 00403e5a push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00403e5b call 004071B0h
    let dst = Cont(x4071b0);
    call(ctx, 0x403e60, dst)
}

pub fn x403e60(ctx: &mut Context) -> Cont {
    // 00403e60 add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    Cont(x403e63)
}

pub fn x403e63(ctx: &mut Context) -> Cont {
    // 00403e63 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00403e67 mov ecx,[esp+18h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00403e6b inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00403e6c add edx,5
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x5u32, &mut ctx.cpu.flags);
    // 00403e6f add ecx,2
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x2u32, &mut ctx.cpu.flags);
    // 00403e72 cmp edi,ebx
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00403e74 mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00403e78 mov [esp+18h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ecx);
    // 00403e7c jl near ptr 00403D69h
    jl(ctx, Cont(x403e82), Cont(x403d69))
}

pub fn x403e82(ctx: &mut Context) -> Cont {
    // 00403e82 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    Cont(x403e83)
}

pub fn x403e83(ctx: &mut Context) -> Cont {
    // 00403e83 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00403e84 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00403e85 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00403e86 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00403e89 ret
    ret(ctx, 0)
}

pub fn x403e90(ctx: &mut Context) -> Cont {
    // 00403e90 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    Cont(x403e91)
}

pub fn x403e91(ctx: &mut Context) -> Cont {
    // 00403e91 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x403e96, dst)
}

pub fn x403e96(ctx: &mut Context) -> Cont {
    // 00403e96 fmul dword ptr ds:[420228h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420228u32) as f64,
    );
    // 00403e9c push 0
    push(ctx, 0x0u32);
    // 00403e9e fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403ea2 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x403ea7, dst)
}

pub fn x403ea7(ctx: &mut Context) -> Cont {
    // 00403ea7 fld dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as f64,
    );
    // 00403eab fsub dword ptr ds:[420240h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420240u32) as f64,
    );
    // 00403eb1 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403eb4 push 0
    push(ctx, 0x0u32);
    // 00403eb6 push 42100Ch
    push(ctx, 0x42100cu32);
    // 00403ebb call 00403D30h
    let dst = Cont(x403d30);
    call(ctx, 0x403ec0, dst)
}

pub fn x403ec0(ctx: &mut Context) -> Cont {
    // 00403ec0 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00403ec4 fsub dword ptr ds:[42023Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42023cu32) as f64,
    );
    // 00403eca add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00403ecd fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403ed0 push 0C25C0000h
    push(ctx, 0xc25c0000u32);
    // 00403ed5 push 421098h
    push(ctx, 0x421098u32);
    // 00403eda call 00403D30h
    let dst = Cont(x403d30);
    call(ctx, 0x403edf, dst)
}

pub fn x403edf(ctx: &mut Context) -> Cont {
    // 00403edf fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00403ee3 fsub dword ptr ds:[420230h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420230u32) as f64,
    );
    // 00403ee9 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00403eec fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403eef push 425C0000h
    push(ctx, 0x425c0000u32);
    // 00403ef4 push 421088h
    push(ctx, 0x421088u32);
    // 00403ef9 call 00403D30h
    let dst = Cont(x403d30);
    call(ctx, 0x403efe, dst)
}

pub fn x403efe(ctx: &mut Context) -> Cont {
    // 00403efe fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00403f02 fsub dword ptr ds:[420238h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420238u32) as f64,
    );
    // 00403f08 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00403f0b fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403f0e push 0
    push(ctx, 0x0u32);
    // 00403f10 push 421014h
    push(ctx, 0x421014u32);
    // 00403f15 call 00403D30h
    let dst = Cont(x403d30);
    call(ctx, 0x403f1a, dst)
}

pub fn x403f1a(ctx: &mut Context) -> Cont {
    // 00403f1a add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00403f1d call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x403f22, dst)
}

pub fn x403f22(ctx: &mut Context) -> Cont {
    // 00403f22 call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x403f27, dst)
}

pub fn x403f27(ctx: &mut Context) -> Cont {
    // 00403f27 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x403f2c, dst)
}

pub fn x403f2c(ctx: &mut Context) -> Cont {
    // 00403f2c test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00403f2e jne short 00403F4Bh
    jne(ctx, Cont(x403f30), Cont(x403f4b))
}

pub fn x403f30(ctx: &mut Context) -> Cont {
    // 00403f30 fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 00403f34 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00403f3a fcomp dword ptr ds:[420234h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420234u32) as f64));
    ctx.cpu.fpu.pop();
    // 00403f40 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00403f42 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00403f45 jne near ptr 00403E91h
    jne(ctx, Cont(x403f4b), Cont(x403e91))
}

pub fn x403f4b(ctx: &mut Context) -> Cont {
    // 00403f4b pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00403f4c ret
    ret(ctx, 0)
}

pub fn x403f50(ctx: &mut Context) -> Cont {
    // 00403f50 mov eax,[esp+0Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00403f54 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00403f57 cmp eax,1
    sub(ctx.cpu.regs.eax, 0x1u32, &mut ctx.cpu.flags);
    // 00403f5a jne short 00403FA1h
    jne(ctx, Cont(x403f5c), Cont(x403fa1))
}

pub fn x403f5c(ctx: &mut Context) -> Cont {
    // 00403f5c fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00403f60 fmul qword ptr ds:[420268h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420268u32));
    // 00403f66 mov dword ptr [esp+4],0C2DC0000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0xc2dc0000u32);
    // 00403f6e fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403f70 fmul qword ptr ds:[420260h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420260u32));
    // 00403f76 fadd qword ptr ds:[420258h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x420258u32));
    // 00403f7c fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00403f80 fmul qword ptr ds:[420250h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420250u32));
    // 00403f86 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403f88 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403f8a fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00403f8c fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403f8e fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403f92 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403f94 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00403f96 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403f98 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403f9c jmp near ptr 00404078h
    Cont(x404078)
}

pub fn x403fa1(ctx: &mut Context) -> Cont {
    // 00403fa1 cmp eax,2
    sub(ctx.cpu.regs.eax, 0x2u32, &mut ctx.cpu.flags);
    // 00403fa4 jne short 00403FEBh
    jne(ctx, Cont(x403fa6), Cont(x403feb))
}

pub fn x403fa6(ctx: &mut Context) -> Cont {
    // 00403fa6 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00403faa fmul qword ptr ds:[420248h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420248u32));
    // 00403fb0 mov dword ptr [esp+4],42DC0000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0x42dc0000u32);
    // 00403fb8 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403fba fmul qword ptr ds:[420260h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420260u32));
    // 00403fc0 fsubr qword ptr ds:[420258h]
    ctx.cpu
        .fpu
        .set(0, ctx.memory.read::<f64>(0x420258u32) - ctx.cpu.fpu.get(0));
    // 00403fc6 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00403fca fmul qword ptr ds:[420250h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420250u32));
    // 00403fd0 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00403fd2 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00403fd4 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00403fd6 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403fd8 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00403fdc fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00403fde fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00403fe0 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00403fe2 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00403fe6 jmp near ptr 00404078h
    Cont(x404078)
}

pub fn x403feb(ctx: &mut Context) -> Cont {
    // 00403feb cmp eax,3
    sub(ctx.cpu.regs.eax, 0x3u32, &mut ctx.cpu.flags);
    // 00403fee jne short 00404035h
    jne(ctx, Cont(x403ff0), Cont(x404035))
}

pub fn x403ff0(ctx: &mut Context) -> Cont {
    // 00403ff0 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x403ff5, dst)
}

pub fn x403ff5(ctx: &mut Context) -> Cont {
    // 00403ff5 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00403ff6 mov ecx,32h
    ctx.cpu.regs.ecx = 0x32u32;
    // 00403ffb mov dword ptr [esp+4],0C3020000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0xc3020000u32);
    // 00404003 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00404005 mov [esp+18h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.edx);
    // 00404009 fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 0040400d fadd dword ptr ds:[420224h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420224u32) as f64,
    );
    // 00404013 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00404017 fmul qword ptr ds:[420250h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420250u32));
    // 0040401d fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0040401f fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404021 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00404023 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404025 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00404029 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040402b fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 0040402d fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040402f fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404033 jmp short 00404078h
    Cont(x404078)
}

pub fn x404035(ctx: &mut Context) -> Cont {
    // 00404035 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x40403a, dst)
}

pub fn x40403a(ctx: &mut Context) -> Cont {
    // 0040403a cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040403b mov ecx,32h
    ctx.cpu.regs.ecx = 0x32u32;
    // 00404040 mov dword ptr [esp+4],43020000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0x43020000u32);
    // 00404048 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040404a mov [esp+18h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.edx);
    // 0040404e fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00404052 fadd dword ptr ds:[420224h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420224u32) as f64,
    );
    // 00404058 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 0040405c fmul qword ptr ds:[420250h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420250u32));
    // 00404062 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00404064 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404066 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00404068 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040406a fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 0040406e fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404070 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404072 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404074 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x404078)
}

pub fn x404078(ctx: &mut Context) -> Cont {
    // 00404078 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0040407c mov ecx,[esp]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.esp);
    // 00404080 mov edx,eax
    ctx.cpu.regs.edx = ctx.cpu.regs.eax;
    // 00404082 mov [edx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.ecx);
    // 00404084 mov ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00404088 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040408b mov ecx,[esp+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 0040408f mov [edx+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00404092 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404095 ret
    ret(ctx, 0)
}

pub fn x4040a0(ctx: &mut Context) -> Cont {
    // 004040a0 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004040a3 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004040a7 fmul qword ptr ds:[420278h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420278u32));
    // 004040ad fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 004040b1 fmul dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 004040b7 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 004040b9 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004040bb mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 004040bf mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    // 004040c1 fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 004040c3 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004040c5 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004040c9 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004040cd fmul qword ptr ds:[420268h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420268u32));
    // 004040d3 mov edx,[esp]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.esp);
    // 004040d7 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004040d9 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004040db fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 004040df fmul dword ptr ds:[420270h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420270u32) as f64,
    );
    // 004040e5 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004040e7 fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004040eb fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 004040ed fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004040ef mov edx,[esp+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 004040f3 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004040f6 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 004040f8 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004040fa fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004040fe mov edx,[esp+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 00404102 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00404105 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404108 ret
    ret(ctx, 0)
}

pub fn x404110(ctx: &mut Context) -> Cont {
    // 00404110 sub esp,38h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 00404113 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00404114 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404115 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404116 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00404117 mov esi,429580h
    ctx.cpu.regs.esi = 0x429580u32;
    Cont(x40411c)
}

pub fn x40411c(ctx: &mut Context) -> Cont {
    // 0040411c call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x404121, dst)
}

pub fn x404121(ctx: &mut Context) -> Cont {
    // 00404121 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00404122 mov ecx,12Ch
    ctx.cpu.regs.ecx = 0x12cu32;
    // 00404127 add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 0040412a idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040412c cmp esi,429680h
    sub(ctx.cpu.regs.esi, 0x429680u32, &mut ctx.cpu.flags);
    // 00404132 mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00404136 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 0040413a fadd dword ptr ds:[420240h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420240u32) as f64,
    );
    // 00404140 fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404143 jl short 0040411Ch
    jl(ctx, Cont(x404145), Cont(x40411c))
}

pub fn x404145(ctx: &mut Context) -> Cont {
    // 00404145 push 0D98h
    push(ctx, 0xd98u32);
    // 0040414a mov dword ptr ds:[429690h],8Ch
    ctx.memory.write::<u32>(0x429690u32, 0x8cu32);
    // 00404154 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x404159, dst)
}

pub fn x404159(ctx: &mut Context) -> Cont {
    // 00404159 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040415c mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 0040415e xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00404160 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00404162 mov ds:[429694h],eax
    ctx.memory.write::<u32>(0x429694u32, ctx.cpu.regs.eax);
    // 00404167 mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    // 0040416b lea ebp,[ebx+0Ch]
    ctx.cpu.regs.ebp = ctx.cpu.regs.ebx.wrapping_add(0xcu32);
    Cont(x40416e)
}

pub fn x40416e(ctx: &mut Context) -> Cont {
    // 0040416e fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00404172 push 3F800000h
    push(ctx, 0x3f800000u32);
    // 00404177 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404178 lea edx,[esp+20h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x20u32);
    // 0040417c fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 0040417f push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404180 call 004040A0h
    let dst = Cont(x4040a0);
    call(ctx, 0x404185, dst)
}

pub fn x404185(ctx: &mut Context) -> Cont {
    // 00404185 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404187 mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 00404189 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040418c mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 0040418e mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404191 push 40800000h
    push(ctx, 0x40800000u32);
    // 00404196 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404199 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 0040419c lea edx,[esp+28h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    // 004041a0 mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004041a3 lea ecx,[esi+5]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi.wrapping_add(0x5u32);
    // 004041a6 mov [esp+18h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ecx);
    // 004041aa push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004041ab fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 004041af fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004041b2 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004041b3 call 004040A0h
    let dst = Cont(x4040a0);
    call(ctx, 0x4041b8, dst)
}

pub fn x4041b8(ctx: &mut Context) -> Cont {
    // 004041b8 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004041ba mov ecx,ebp
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebp;
    // 004041bc add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004041bf add ebx,18h
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x18u32, &mut ctx.cpu.flags);
    // 004041c2 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004041c4 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004041c7 add ebp,18h
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x18u32, &mut ctx.cpu.flags);
    // 004041ca inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004041cb mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004041ce mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 004041d1 cmp esi,8Ch
    sub(ctx.cpu.regs.esi, 0x8cu32, &mut ctx.cpu.flags);
    // 004041d7 mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    // 004041db mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004041de jl short 0040416Eh
    jl(ctx, Cont(x4041e0), Cont(x40416e))
}

pub fn x4041e0(ctx: &mut Context) -> Cont {
    // 004041e0 mov ebx,32h
    ctx.cpu.regs.ebx = 0x32u32;
    // 004041e5 push 528h
    push(ctx, 0x528u32);
    // 004041ea mov ds:[429698h],ebx
    ctx.memory.write::<u32>(0x429698u32, ctx.cpu.regs.ebx);
    // 004041f0 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x4041f5, dst)
}

pub fn x4041f5(ctx: &mut Context) -> Cont {
    // 004041f5 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004041f8 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004041fa mov ds:[42969Ch],eax
    ctx.memory.write::<u32>(0x42969cu32, ctx.cpu.regs.eax);
    // 004041ff mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00404203 mov esi,0C2C80000h
    ctx.cpu.regs.esi = 0xc2c80000u32;
    // 00404208 mov edx,42C80000h
    ctx.cpu.regs.edx = 0x42c80000u32;
    Cont(x40420d)
}

pub fn x40420d(ctx: &mut Context) -> Cont {
    // 0040420d fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00404211 mov [eax],esi
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.esi);
    // 00404213 mov [eax+8],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.edi);
    // 00404216 mov [eax+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 00404219 mov [eax+14h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x14u32), ctx.cpu.regs.edi);
    // 0040421c fsub dword ptr ds:[420284h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420284u32) as f64,
    );
    // 00404222 add eax,18h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x18u32, &mut ctx.cpu.flags);
    // 00404225 add eax,18h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x18u32, &mut ctx.cpu.flags);
    // 00404228 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00404229 fmul dword ptr ds:[420280h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420280u32) as f64,
    );
    // 0040422f cmp ecx,19h
    sub(ctx.cpu.regs.ecx, 0x19u32, &mut ctx.cpu.flags);
    // 00404232 mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00404236 fst dword ptr [eax-2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffd4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00404239 fst dword ptr [eax-20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffe0u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040423c fst dword ptr [eax-18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffe8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040423f fstp dword ptr [eax-0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0xfffffff4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404242 mov [eax-14h],esi
    ctx.memory.write::<u32>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffecu32),
        ctx.cpu.regs.esi,
    );
    // 00404245 mov [eax-10h],edi
    ctx.memory.write::<u32>(
        ctx.cpu.regs.eax.wrapping_add(0xfffffff0u32),
        ctx.cpu.regs.edi,
    );
    // 00404248 mov [eax-8],edx
    ctx.memory.write::<u32>(
        ctx.cpu.regs.eax.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.edx,
    );
    // 0040424b mov [eax-4],edi
    ctx.memory.write::<u32>(
        ctx.cpu.regs.eax.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.edi,
    );
    // 0040424e jl short 0040420Dh
    jl(ctx, Cont(x404250), Cont(x40420d))
}

pub fn x404250(ctx: &mut Context) -> Cont {
    // 00404250 push 4B0h
    push(ctx, 0x4b0u32);
    // 00404255 mov ds:[429560h],ebx
    ctx.memory.write::<u32>(0x429560u32, ctx.cpu.regs.ebx);
    // 0040425b call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x404260, dst)
}

pub fn x404260(ctx: &mut Context) -> Cont {
    // 00404260 push 4B0h
    push(ctx, 0x4b0u32);
    // 00404265 mov ds:[429564h],eax
    ctx.memory.write::<u32>(0x429564u32, ctx.cpu.regs.eax);
    // 0040426a mov ds:[429568h],ebx
    ctx.memory.write::<u32>(0x429568u32, ctx.cpu.regs.ebx);
    // 00404270 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x404275, dst)
}

pub fn x404275(ctx: &mut Context) -> Cont {
    // 00404275 push 4B0h
    push(ctx, 0x4b0u32);
    // 0040427a mov ds:[42956Ch],eax
    ctx.memory.write::<u32>(0x42956cu32, ctx.cpu.regs.eax);
    // 0040427f mov ds:[4296A0h],ebx
    ctx.memory.write::<u32>(0x4296a0u32, ctx.cpu.regs.ebx);
    // 00404285 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x40428a, dst)
}

pub fn x40428a(ctx: &mut Context) -> Cont {
    // 0040428a add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040428d mov ds:[4296A4h],eax
    ctx.memory.write::<u32>(0x4296a4u32, ctx.cpu.regs.eax);
    // 00404292 mov [esp+10h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edi);
    Cont(x404296)
}

pub fn x404296(ctx: &mut Context) -> Cont {
    // 00404296 fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 0040429a mov ecx,ds:[429564h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x429564u32);
    // 004042a0 mov edx,ds:[42956Ch]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x42956cu32);
    // 004042a6 push 1
    push(ctx, 0x1u32);
    // 004042a8 lea eax,[esp+28h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    // 004042ac fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004042b0 mov esi,[esp+18h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 004042b4 lea ebx,[edi+ecx]
    ctx.cpu.regs.ebx = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx);
    // 004042b7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004042b8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004042b9 lea ebp,[edi+edx]
    ctx.cpu.regs.ebp = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.edx);
    // 004042bc call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x4042c1, dst)
}

pub fn x4042c1(ctx: &mut Context) -> Cont {
    // 004042c1 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004042c3 mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 004042c5 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004042c9 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 004042cf mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004042d1 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004042d4 push 1
    push(ctx, 0x1u32);
    // 004042d6 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004042d9 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 004042dc fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004042e0 mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004042e3 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 004042e7 lea edx,[esp+28h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    // 004042eb push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004042ec push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004042ed call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x4042f2, dst)
}

pub fn x4042f2(ctx: &mut Context) -> Cont {
    // 004042f2 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004042f4 add ebx,0Ch
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0xcu32, &mut ctx.cpu.flags);
    // 004042f7 push 2
    push(ctx, 0x2u32);
    // 004042f9 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004042fa mov [ebx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.ebx, ctx.cpu.regs.ecx);
    // 004042fc mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004042ff lea ecx,[esp+50h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x50u32);
    // 00404303 mov [ebx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404306 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404309 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040430a mov [ebx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040430d call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x404312, dst)
}

pub fn x404312(ctx: &mut Context) -> Cont {
    // 00404312 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404314 mov edx,ebp
    ctx.cpu.regs.edx = ctx.cpu.regs.ebp;
    // 00404316 push 2
    push(ctx, 0x2u32);
    // 00404318 mov [edx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.ecx);
    // 0040431a mov ecx,[eax+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0040431d mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00404320 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404323 mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00404327 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040432a lea edx,[esp+64h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x64u32);
    // 0040432e push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040432f push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404330 call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x404335, dst)
}

pub fn x404335(ctx: &mut Context) -> Cont {
    // 00404335 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404337 add ebp,0Ch
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0xcu32, &mut ctx.cpu.flags);
    // 0040433a add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 0040433d add edi,18h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x18u32, &mut ctx.cpu.flags);
    // 00404340 mov [ebp],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.ebp, ctx.cpu.regs.ecx);
    // 00404343 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404346 mov [ebp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404349 mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0040434d mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404350 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00404351 cmp edi,4B0h
    sub(ctx.cpu.regs.edi, 0x4b0u32, &mut ctx.cpu.flags);
    // 00404357 mov [ebp+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040435a mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 0040435e jl near ptr 00404296h
    jl(ctx, Cont(x404364), Cont(x404296))
}

pub fn x404364(ctx: &mut Context) -> Cont {
    // 00404364 mov esi,32h
    ctx.cpu.regs.esi = 0x32u32;
    // 00404369 push 4B0h
    push(ctx, 0x4b0u32);
    // 0040436e mov ds:[429570h],esi
    ctx.memory.write::<u32>(0x429570u32, ctx.cpu.regs.esi);
    // 00404374 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x404379, dst)
}

pub fn x404379(ctx: &mut Context) -> Cont {
    // 00404379 push 4B0h
    push(ctx, 0x4b0u32);
    // 0040437e mov ds:[429574h],eax
    ctx.memory.write::<u32>(0x429574u32, ctx.cpu.regs.eax);
    // 00404383 mov ds:[429578h],esi
    ctx.memory.write::<u32>(0x429578u32, ctx.cpu.regs.esi);
    // 00404389 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x40438e, dst)
}

pub fn x40438e(ctx: &mut Context) -> Cont {
    // 0040438e push 4B0h
    push(ctx, 0x4b0u32);
    // 00404393 mov ds:[42957Ch],eax
    ctx.memory.write::<u32>(0x42957cu32, ctx.cpu.regs.eax);
    // 00404398 mov ds:[4296A8h],esi
    ctx.memory.write::<u32>(0x4296a8u32, ctx.cpu.regs.esi);
    // 0040439e call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x4043a3, dst)
}

pub fn x4043a3(ctx: &mut Context) -> Cont {
    // 004043a3 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004043a6 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004043a8 mov ds:[4296ACh],eax
    ctx.memory.write::<u32>(0x4296acu32, ctx.cpu.regs.eax);
    // 004043ad mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    Cont(x4043b1)
}

pub fn x4043b1(ctx: &mut Context) -> Cont {
    // 004043b1 fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 004043b5 mov ecx,ds:[429574h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x429574u32);
    // 004043bb mov edx,ds:[42957Ch]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x42957cu32);
    // 004043c1 push 3
    push(ctx, 0x3u32);
    // 004043c3 lea eax,[esp+40h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x40u32);
    // 004043c7 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004043cb mov ebp,[esp+18h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 004043cf lea edi,[esi+ecx]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.ecx);
    // 004043d2 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004043d3 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004043d4 lea ebx,[esi+edx]
    ctx.cpu.regs.ebx = ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.edx);
    // 004043d7 call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x4043dc, dst)
}

pub fn x4043dc(ctx: &mut Context) -> Cont {
    // 004043dc mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004043de mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 004043e0 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004043e4 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 004043ea mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004043ec mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004043ef push 3
    push(ctx, 0x3u32);
    // 004043f1 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004043f4 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 004043f7 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004043fb mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004043fe mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00404402 lea edx,[esp+40h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x40u32);
    // 00404406 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404407 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404408 call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x40440d, dst)
}

pub fn x40440d(ctx: &mut Context) -> Cont {
    // 0040440d mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 0040440f add edi,0Ch
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0xcu32, &mut ctx.cpu.flags);
    // 00404412 push 4
    push(ctx, 0x4u32);
    // 00404414 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404415 mov [edi],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, ctx.cpu.regs.ecx);
    // 00404417 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0040441a lea ecx,[esp+44h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x44u32);
    // 0040441e mov [edi+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404421 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404424 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404425 mov [edi+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edi.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404428 call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x40442d, dst)
}

pub fn x40442d(ctx: &mut Context) -> Cont {
    // 0040442d mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 0040442f mov edx,ebx
    ctx.cpu.regs.edx = ctx.cpu.regs.ebx;
    // 00404431 push 4
    push(ctx, 0x4u32);
    // 00404433 mov [edx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.ecx);
    // 00404435 mov ecx,[eax+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404438 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040443b mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 0040443e mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00404442 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404445 lea edx,[esp+40h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x40u32);
    // 00404449 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040444a push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040444b call 00403F50h
    let dst = Cont(x403f50);
    call(ctx, 0x404450, dst)
}

pub fn x404450(ctx: &mut Context) -> Cont {
    // 00404450 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404452 add ebx,0Ch
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0xcu32, &mut ctx.cpu.flags);
    // 00404455 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00404458 add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 0040445b mov [ebx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.ebx, ctx.cpu.regs.ecx);
    // 0040445d mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404460 mov [ebx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404463 mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00404467 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 0040446a inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040446b cmp esi,4B0h
    sub(ctx.cpu.regs.esi, 0x4b0u32, &mut ctx.cpu.flags);
    // 00404471 mov [ebx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404474 mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 00404478 jl near ptr 004043B1h
    jl(ctx, Cont(x40447e), Cont(x4043b1))
}

pub fn x40447e(ctx: &mut Context) -> Cont {
    // 0040447e push 407EF0h
    push(ctx, 0x407ef0u32);
    // 00404483 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x404488, dst)
}

pub fn x404488(ctx: &mut Context) -> Cont {
    // 00404488 push 408120h
    push(ctx, 0x408120u32);
    // 0040448d mov ds:[429680h],eax
    ctx.memory.write::<u32>(0x429680u32, ctx.cpu.regs.eax);
    // 00404492 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x404497, dst)
}

pub fn x404497(ctx: &mut Context) -> Cont {
    // 00404497 push 408350h
    push(ctx, 0x408350u32);
    // 0040449c mov ds:[429684h],eax
    ctx.memory.write::<u32>(0x429684u32, ctx.cpu.regs.eax);
    // 004044a1 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x4044a6, dst)
}

pub fn x4044a6(ctx: &mut Context) -> Cont {
    // 004044a6 push 408560h
    push(ctx, 0x408560u32);
    // 004044ab mov ds:[429688h],eax
    ctx.memory.write::<u32>(0x429688u32, ctx.cpu.regs.eax);
    // 004044b0 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x4044b5, dst)
}

pub fn x4044b5(ctx: &mut Context) -> Cont {
    // 004044b5 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004044b8 mov ds:[42968Ch],eax
    ctx.memory.write::<u32>(0x42968cu32, ctx.cpu.regs.eax);
    // 004044bd pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 004044be pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004044bf pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 004044c0 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004044c1 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 004044c4 ret
    ret(ctx, 0)
}

pub fn x4044d0(ctx: &mut Context) -> Cont {
    // 004044d0 mov eax,ds:[429694h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x429694u32);
    // 004044d5 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004044d6 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x4044db, dst)
}

pub fn x4044db(ctx: &mut Context) -> Cont {
    // 004044db mov ecx,ds:[42969Ch]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x42969cu32);
    // 004044e1 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004044e2 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x4044e7, dst)
}

pub fn x4044e7(ctx: &mut Context) -> Cont {
    // 004044e7 mov edx,ds:[4296A4h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x4296a4u32);
    // 004044ed push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004044ee call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x4044f3, dst)
}

pub fn x4044f3(ctx: &mut Context) -> Cont {
    // 004044f3 mov eax,ds:[429564h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x429564u32);
    // 004044f8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004044f9 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x4044fe, dst)
}

pub fn x4044fe(ctx: &mut Context) -> Cont {
    // 004044fe mov ecx,ds:[42956Ch]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x42956cu32);
    // 00404504 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404505 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x40450a, dst)
}

pub fn x40450a(ctx: &mut Context) -> Cont {
    // 0040450a add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 0040450d ret
    ret(ctx, 0)
}

pub fn x404630(ctx: &mut Context) -> Cont {
    // 00404630 sub esp,50h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 00404633 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00404634 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404635 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404636 push edi
    push(ctx, ctx.cpu.regs.edi);
    Cont(x404637)
}

pub fn x404637(ctx: &mut Context) -> Cont {
    // 00404637 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x40463c, dst)
}

pub fn x40463c(ctx: &mut Context) -> Cont {
    // 0040463c fmul dword ptr ds:[4202D0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202d0u32) as f64,
    );
    // 00404642 fst dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00404646 fld qword ptr ds:[4201F8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f64>(0x4201f8u32));
    // 0040464c call 0041F114h
    let dst = Cont(x41f114);
    call(ctx, 0x404651, dst)
}

pub fn x404651(ctx: &mut Context) -> Cont {
    // 00404651 fcomp qword ptr ds:[420200h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f64>(0x420200u32)));
    ctx.cpu.fpu.pop();
    // 00404657 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404659 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040465c je short 00404665h
    je(ctx, Cont(x40465e), Cont(x404665))
}

pub fn x40465e(ctx: &mut Context) -> Cont {
    // 0040465e mov ebp,1
    ctx.cpu.regs.ebp = 0x1u32;
    // 00404663 jmp short 00404667h
    Cont(x404667)
}

pub fn x404665(ctx: &mut Context) -> Cont {
    // 00404665 xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    Cont(x404667)
}

pub fn x404667(ctx: &mut Context) -> Cont {
    // 00404667 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040466b call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404670, dst)
}

pub fn x404670(ctx: &mut Context) -> Cont {
    // 00404670 mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    // 00404672 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00404673 and edx,0FFh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0xffu32, &mut ctx.cpu.flags);
    // 00404679 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040467b sar eax,8
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 0040467e mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00404680 and ebx,80000003h
    ctx.cpu.regs.ebx = and(ctx.cpu.regs.ebx, 0x80000003u32, &mut ctx.cpu.flags);
    // 00404686 jns short 0040468Dh
    jns(ctx, Cont(x404688), Cont(x40468d))
}

pub fn x404688(ctx: &mut Context) -> Cont {
    // 00404688 dec ebx
    ctx.cpu.regs.ebx = dec(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00404689 or ebx,0FFFFFFFCh
    ctx.cpu.regs.ebx = or(ctx.cpu.regs.ebx, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0040468c inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    Cont(x40468d)
}

pub fn x40468d(ctx: &mut Context) -> Cont {
    // 0040468d mov eax,ecx
    ctx.cpu.regs.eax = ctx.cpu.regs.ecx;
    // 0040468f push 0
    push(ctx, 0x0u32);
    // 00404691 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00404692 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 00404695 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00404697 mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 00404699 sar edi,6
    ctx.cpu.regs.edi = sar(ctx.cpu.regs.edi, 0x6u8, &mut ctx.cpu.flags);
    // 0040469c and edi,3Fh
    ctx.cpu.regs.edi = and(ctx.cpu.regs.edi, 0x3fu32, &mut ctx.cpu.flags);
    // 0040469f call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x4046a4, dst)
}

pub fn x4046a4(ctx: &mut Context) -> Cont {
    // 004046a4 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004046a7 test bl,1
    and(ctx.cpu.regs.get_bl(), 0x1u8, &mut ctx.cpu.flags);
    // 004046aa je short 004046EEh
    je(ctx, Cont(x4046ac), Cont(x4046ee))
}

pub fn x4046ac(ctx: &mut Context) -> Cont {
    // 004046ac xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x4046ae)
}

pub fn x4046ae(ctx: &mut Context) -> Cont {
    // 004046ae push 28h
    push(ctx, 0x28u32);
    // 004046b0 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046b1 push 140h
    push(ctx, 0x140u32);
    // 004046b6 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046b7 push 0
    push(ctx, 0x0u32);
    // 004046b9 call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x4046be, dst)
}

pub fn x4046be(ctx: &mut Context) -> Cont {
    // 004046be add esi,14h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x14u32, &mut ctx.cpu.flags);
    // 004046c1 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 004046c4 cmp esi,0C8h
    sub(ctx.cpu.regs.esi, 0xc8u32, &mut ctx.cpu.flags);
    // 004046ca jl short 004046AEh
    jl(ctx, Cont(x4046cc), Cont(x4046ae))
}

pub fn x4046cc(ctx: &mut Context) -> Cont {
    // 004046cc xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x4046ce)
}

pub fn x4046ce(ctx: &mut Context) -> Cont {
    // 004046ce push 28h
    push(ctx, 0x28u32);
    // 004046d0 push 0C8h
    push(ctx, 0xc8u32);
    // 004046d5 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046d6 push 0
    push(ctx, 0x0u32);
    // 004046d8 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046d9 call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x4046de, dst)
}

pub fn x4046de(ctx: &mut Context) -> Cont {
    // 004046de add esi,14h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x14u32, &mut ctx.cpu.flags);
    // 004046e1 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 004046e4 cmp esi,140h
    sub(ctx.cpu.regs.esi, 0x140u32, &mut ctx.cpu.flags);
    // 004046ea jl short 004046CEh
    jl(ctx, Cont(x4046ec), Cont(x4046ce))
}

pub fn x4046ec(ctx: &mut Context) -> Cont {
    // 004046ec jmp short 00404734h
    Cont(x404734)
}

pub fn x4046ee(ctx: &mut Context) -> Cont {
    // 004046ee xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x4046f0)
}

pub fn x4046f0(ctx: &mut Context) -> Cont {
    // 004046f0 push 28h
    push(ctx, 0x28u32);
    // 004046f2 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046f3 push 280h
    push(ctx, 0x280u32);
    // 004046f8 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004046f9 push 140h
    push(ctx, 0x140u32);
    // 004046fe call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x404703, dst)
}

pub fn x404703(ctx: &mut Context) -> Cont {
    // 00404703 add esi,14h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x14u32, &mut ctx.cpu.flags);
    // 00404706 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00404709 cmp esi,0C8h
    sub(ctx.cpu.regs.esi, 0xc8u32, &mut ctx.cpu.flags);
    // 0040470f jl short 004046F0h
    jl(ctx, Cont(x404711), Cont(x4046f0))
}

pub fn x404711(ctx: &mut Context) -> Cont {
    // 00404711 mov esi,140h
    ctx.cpu.regs.esi = 0x140u32;
    Cont(x404716)
}

pub fn x404716(ctx: &mut Context) -> Cont {
    // 00404716 push 28h
    push(ctx, 0x28u32);
    // 00404718 push 0C8h
    push(ctx, 0xc8u32);
    // 0040471d push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0040471e push 0
    push(ctx, 0x0u32);
    // 00404720 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404721 call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x404726, dst)
}

pub fn x404726(ctx: &mut Context) -> Cont {
    // 00404726 add esi,14h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x14u32, &mut ctx.cpu.flags);
    // 00404729 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 0040472c cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00404732 jl short 00404716h
    jl(ctx, Cont(x404734), Cont(x404716))
}

pub fn x404734(ctx: &mut Context) -> Cont {
    // 00404734 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00404738 fld qword ptr ds:[4202C8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f64>(0x4202c8u32));
    // 0040473e call 0041F114h
    let dst = Cont(x41f114);
    call(ctx, 0x404743, dst)
}

pub fn x404743(ctx: &mut Context) -> Cont {
    // 00404743 fmul qword ptr ds:[4202C0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202c0u32));
    // 00404749 lea eax,[edi+2]
    ctx.cpu.regs.eax = ctx.cpu.regs.edi.wrapping_add(0x2u32);
    // 0040474c add edi,4
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 0040474f and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 00404752 and edi,3Fh
    ctx.cpu.regs.edi = and(ctx.cpu.regs.edi, 0x3fu32, &mut ctx.cpu.flags);
    // 00404755 fadd qword ptr ds:[4202B8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4202b8u32));
    // 0040475b fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0040475f fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404765 fld dword ptr ds:[4202A8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4202a8u32) as f64);
    // 0040476b fsub dword ptr [eax*4+429580h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>((ctx.cpu.regs.eax * 4).wrapping_add(0x429580u32)) as f64,
    );
    // 00404772 lea eax,[eax*4+429580h]
    ctx.cpu.regs.eax = (ctx.cpu.regs.eax * 4).wrapping_add(0x429580u32);
    // 00404779 lea ecx,[edi*4+429580h]
    ctx.cpu.regs.ecx = (ctx.cpu.regs.edi * 4).wrapping_add(0x429580u32);
    // 00404780 fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00404782 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404784 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 00404786 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404788 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040478c fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00404790 fadd dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 00404792 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404798 fld dword ptr [ecx]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64);
    // 0040479a fadd dword ptr ds:[42023Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42023cu32) as f64,
    );
    // 004047a0 lea ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x24u32);
    // 004047a4 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004047a5 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004047a7 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004047a9 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004047ab fmul dword ptr ds:[4202A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202a4u32) as f64,
    );
    // 004047b1 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004047b3 fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004047b7 fld dword ptr ds:[4202A0h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4202a0u32) as f64);
    // 004047bd fsub dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 004047bf fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 004047c1 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004047c3 fxch st(2)
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(2));
    ctx.cpu.fpu.set(2, t);
    // 004047c5 fmulp st(2),st
    ctx.cpu.fpu.set(2, ctx.cpu.fpu.get(2) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004047c7 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 004047c9 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004047cd fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004047cf fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 004047d3 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004047d5 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004047d9 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004047dd fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004047df fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004047e3 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004047e7 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004047e9 fstp dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004047ed call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x4047f2, dst)
}

pub fn x4047f2(ctx: &mut Context) -> Cont {
    // 004047f2 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004047f6 fmul qword ptr ds:[420298h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420298u32));
    // 004047fc add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004047ff lea edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x3cu32);
    // 00404803 mov dword ptr [esp+30h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), 0x0u32);
    // 0040480b mov ecx,[esp+30h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 0040480f fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404811 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404812 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404815 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00404817 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00404819 fmul qword ptr ds:[4201A0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201a0u32));
    // 0040481f fsubr qword ptr ds:[420198h]
    ctx.cpu
        .fpu
        .set(0, ctx.memory.read::<f64>(0x420198u32) - ctx.cpu.fpu.get(0));
    // 00404825 fstp dword ptr [esp+44h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x44u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404829 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0040482d fmul qword ptr ds:[420290h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420290u32));
    // 00404833 mov edx,[esp+44h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 00404837 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040483a fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040483c fmul qword ptr ds:[4201A0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4201a0u32));
    // 00404842 fstp dword ptr [esp+48h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x48u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404846 mov ecx,[esp+48h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 0040484a mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040484d mov eax,[esp+34h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00404851 mov ecx,[esp+38h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 00404855 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404858 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 0040485a mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 0040485c mov eax,[esp+48h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 00404860 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00404863 mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404866 call 00406410h
    let dst = Cont(x406410);
    call(ctx, 0x40486b, dst)
}

pub fn x40486b(ctx: &mut Context) -> Cont {
    // 0040486b add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    // 0040486e cmp ebx,3
    sub(ctx.cpu.regs.ebx, 0x3u32, &mut ctx.cpu.flags);
    // 00404871 ja near ptr 004049FAh
    ja(ctx, Cont(x404877), Cont(x4049fa))
}

pub fn x404877(ctx: &mut Context) -> Cont {
    // 00404877 jmp dword ptr [ebx*4+404A2Ch]
    indirect(
        ctx,
        ctx.memory
            .read((ctx.cpu.regs.ebx * 4).wrapping_add(0x404a2cu32)),
    )
}

pub fn x4049fa(ctx: &mut Context) -> Cont {
    // 004049fa call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x4049ff, dst)
}

pub fn x4049ff(ctx: &mut Context) -> Cont {
    // 004049ff call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x404a04, dst)
}

pub fn x404a04(ctx: &mut Context) -> Cont {
    // 00404a04 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00404a06 jne short 00404A23h
    jne(ctx, Cont(x404a08), Cont(x404a23))
}

pub fn x404a08(ctx: &mut Context) -> Cont {
    // 00404a08 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00404a0c fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00404a12 fcomp dword ptr ds:[4200C8h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4200c8u32) as f64));
    ctx.cpu.fpu.pop();
    // 00404a18 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404a1a test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00404a1d jne near ptr 00404637h
    jne(ctx, Cont(x404a23), Cont(x404637))
}

pub fn x404a23(ctx: &mut Context) -> Cont {
    // 00404a23 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00404a24 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00404a25 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00404a26 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00404a27 add esp,50h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 00404a2a ret
    ret(ctx, 0)
}

pub fn x404a40(ctx: &mut Context) -> Cont {
    // 00404a40 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00404a43 fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 00404a47 fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00404a4b mov eax,[esp+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00404a4f fstp qword ptr [esp+4]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404a53 lea eax,[eax+eax*2-2Dh]
    ctx.cpu.regs.eax = ctx
        .cpu
        .regs
        .eax
        .wrapping_add((ctx.cpu.regs.eax * 2))
        .wrapping_add(0xffffffd3u32);
    // 00404a57 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00404a59 fsub qword ptr ds:[420330h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) - ctx.memory.read::<f64>(0x420330u32));
    // 00404a5f fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404a61 fsub qword ptr ds:[420328h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) - ctx.memory.read::<f64>(0x420328u32));
    // 00404a67 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404a69 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404a6b fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404a6d fld qword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x4u32)),
    );
    // 00404a71 fsub qword ptr ds:[420320h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) - ctx.memory.read::<f64>(0x420320u32));
    // 00404a77 lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 00404a7a shl eax,1
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 00404a7c fst dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    // 00404a80 fld qword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x4u32)),
    );
    // 00404a84 fsub qword ptr ds:[420318h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) - ctx.memory.read::<f64>(0x420318u32));
    // 00404a8a fst dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    // 00404a8e fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404a90 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404a92 fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 00404a94 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00404a98 fmul dword ptr ds:[4202A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202a4u32) as f64,
    );
    // 00404a9e faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404aa0 fmul qword ptr ds:[420310h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420310u32));
    // 00404aa6 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404aa8 fmul qword ptr ds:[420308h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420308u32));
    // 00404aae fild dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as i32 as f64,
    );
    // 00404ab2 fsub dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00404ab6 fmul qword ptr ds:[420300h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420300u32));
    // 00404abc fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404abe fmul qword ptr ds:[420178h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420178u32));
    // 00404ac4 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404ac6 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404ac8 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404aca fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 00404ace fsub dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00404ad2 fmul qword ptr ds:[4202F8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202f8u32));
    // 00404ad8 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404ada fmul qword ptr ds:[420178h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420178u32));
    // 00404ae0 fst dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00404ae4 mov [esp+20h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.eax);
    // 00404ae8 mov eax,[esp+1Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00404aec faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404aee lea eax,[eax+eax*2-2Dh]
    ctx.cpu.regs.eax = ctx
        .cpu
        .regs
        .eax
        .wrapping_add((ctx.cpu.regs.eax * 2))
        .wrapping_add(0xffffffd3u32);
    // 00404af2 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404af6 fild dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as i32 as f64,
    );
    // 00404afa lea ecx,[eax+eax*4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 00404afd mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00404b01 shl ecx,1
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x1u8, &mut ctx.cpu.flags);
    // 00404b03 fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404b07 mov [esp+20h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ecx);
    // 00404b0b mov ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00404b0f fild dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as i32 as f64,
    );
    // 00404b13 mov edx,eax
    ctx.cpu.regs.edx = ctx.cpu.regs.eax;
    // 00404b15 fstp dword ptr [esp+0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0xcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404b19 mov [edx],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.ecx);
    // 00404b1b mov ecx,[esp+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 00404b1f mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00404b22 mov ecx,[esp+0Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00404b26 mov [edx+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 00404b29 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00404b2c ret
    ret(ctx, 0)
}

pub fn x404b30(ctx: &mut Context) -> Cont {
    // 00404b30 sub esp,38h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 00404b33 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00404b34 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404b35 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404b36 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00404b37 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00404b39 push 11940h
    push(ctx, 0x11940u32);
    // 00404b3e mov ds:[429400h],esi
    ctx.memory.write::<u32>(0x429400u32, ctx.cpu.regs.esi);
    // 00404b44 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x404b49, dst)
}

pub fn x404b49(ctx: &mut Context) -> Cont {
    // 00404b49 mov ebp,[esp+50h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32));
    // 00404b4d add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00404b50 mov ds:[429404h],eax
    ctx.memory.write::<u32>(0x429404u32, ctx.cpu.regs.eax);
    // 00404b55 mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00404b57 mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    Cont(x404b5b)
}

pub fn x404b5b(ctx: &mut Context) -> Cont {
    // 00404b5b mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00404b5f xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00404b61 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00404b62 lea edi,[ebx+0Ch]
    ctx.cpu.regs.edi = ctx.cpu.regs.ebx.wrapping_add(0xcu32);
    // 00404b65 mov [esp+4Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32), ctx.cpu.regs.eax);
    Cont(x404b69)
}

pub fn x404b69(ctx: &mut Context) -> Cont {
    // 00404b69 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00404b6d push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404b6e push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404b6f lea edx,[esp+20h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x20u32);
    // 00404b73 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404b74 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404b75 call 00404A40h
    let dst = Cont(x404a40);
    call(ctx, 0x404b7a, dst)
}

pub fn x404b7a(ctx: &mut Context) -> Cont {
    // 00404b7a mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404b7c mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 00404b7e push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404b7f mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00404b81 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404b84 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404b87 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404b8a lea edx,[esp+38h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x38u32);
    // 00404b8e mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404b91 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00404b95 lea eax,[esi+1]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 00404b98 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404b99 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00404b9a push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404b9b mov [esp+34h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), ctx.cpu.regs.eax);
    // 00404b9f call 00404A40h
    let dst = Cont(x404a40);
    call(ctx, 0x404ba4, dst)
}

pub fn x404ba4(ctx: &mut Context) -> Cont {
    // 00404ba4 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404ba6 mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 00404ba8 add ebx,18h
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x18u32, &mut ctx.cpu.flags);
    // 00404bab add edi,18h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x18u32, &mut ctx.cpu.flags);
    // 00404bae mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00404bb0 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404bb3 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404bb4 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404bb7 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404bba lea edx,[esp+54h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x54u32);
    // 00404bbe mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404bc1 mov ecx,ds:[429400h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x429400u32);
    // 00404bc7 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00404bc8 mov ds:[429400h],ecx
    ctx.memory.write::<u32>(0x429400u32, ctx.cpu.regs.ecx);
    // 00404bce mov ecx,[esp+34h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00404bd2 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404bd3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404bd4 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404bd5 call 00404A40h
    let dst = Cont(x404a40);
    call(ctx, 0x404bda, dst)
}

pub fn x404bda(ctx: &mut Context) -> Cont {
    // 00404bda mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404bdc mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 00404bde push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00404bdf mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00404be1 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404be4 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404be7 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404bea lea edx,[esp+70h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x70u32);
    // 00404bee mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404bf1 mov ecx,[esp+80h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 00404bf8 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404bf9 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404bfa push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404bfb call 00404A40h
    let dst = Cont(x404a40);
    call(ctx, 0x404c00, dst)
}

pub fn x404c00(ctx: &mut Context) -> Cont {
    // 00404c00 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00404c02 mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 00404c04 mov esi,[esp+54h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00404c08 add esp,40h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x40u32, &mut ctx.cpu.flags);
    // 00404c0b mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00404c0d mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00404c10 add ebx,18h
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x18u32, &mut ctx.cpu.flags);
    // 00404c13 add edi,18h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x18u32, &mut ctx.cpu.flags);
    // 00404c16 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404c19 mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00404c1c mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404c1f mov ecx,ds:[429400h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x429400u32);
    // 00404c25 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00404c26 cmp esi,1Eh
    sub(ctx.cpu.regs.esi, 0x1eu32, &mut ctx.cpu.flags);
    // 00404c29 mov ds:[429400h],ecx
    ctx.memory.write::<u32>(0x429400u32, ctx.cpu.regs.ecx);
    // 00404c2f jl near ptr 00404B69h
    jl(ctx, Cont(x404c35), Cont(x404b69))
}

pub fn x404c35(ctx: &mut Context) -> Cont {
    // 00404c35 mov eax,[esp+4Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 00404c39 cmp eax,1Eh
    sub(ctx.cpu.regs.eax, 0x1eu32, &mut ctx.cpu.flags);
    // 00404c3c mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00404c40 jl near ptr 00404B5Bh
    jl(ctx, Cont(x404c46), Cont(x404b5b))
}

pub fn x404c46(ctx: &mut Context) -> Cont {
    // 00404c46 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00404c47 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00404c48 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00404c49 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00404c4a add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 00404c4d ret
    ret(ctx, 0)
}

pub fn x404c50(ctx: &mut Context) -> Cont {
    // 00404c50 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404c51 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404c52 mov esi,429460h
    ctx.cpu.regs.esi = 0x429460u32;
    Cont(x404c57)
}

pub fn x404c57(ctx: &mut Context) -> Cont {
    // 00404c57 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x404c5c, dst)
}

pub fn x404c5c(ctx: &mut Context) -> Cont {
    // 00404c5c cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00404c5d mov ecx,12Ch
    ctx.cpu.regs.ecx = 0x12cu32;
    // 00404c62 add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 00404c65 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00404c67 cmp esi,429560h
    sub(ctx.cpu.regs.esi, 0x429560u32, &mut ctx.cpu.flags);
    // 00404c6d mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404c71 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00404c75 fsub dword ptr ds:[420338h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420338u32) as f64,
    );
    // 00404c7b fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404c7e jl short 00404C57h
    jl(ctx, Cont(x404c80), Cont(x404c57))
}

pub fn x404c80(ctx: &mut Context) -> Cont {
    // 00404c80 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00404c81 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00404c82 ret
    ret(ctx, 0)
}

pub fn x404c90(ctx: &mut Context) -> Cont {
    // 00404c90 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404c91 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404c92 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x404c94)
}

pub fn x404c94(ctx: &mut Context) -> Cont {
    // 00404c94 lea eax,[esi-64h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0xffffff9cu32);
    // 00404c97 mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00404c9b fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00404c9f fmul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00404ca3 fadd dword ptr ds:[420240h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420240u32) as f64,
    );
    // 00404ca9 fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00404caf fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404cb1 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00404cb4 je short 00404CC0h
    je(ctx, Cont(x404cb6), Cont(x404cc0))
}

pub fn x404cb6(ctx: &mut Context) -> Cont {
    // 00404cb6 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404cb8 fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 00404cbe jmp short 00404CD5h
    Cont(x404cd5)
}

pub fn x404cc0(ctx: &mut Context) -> Cont {
    // 00404cc0 fcom dword ptr ds:[420120h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420120u32) as f64));
    // 00404cc6 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404cc8 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00404ccb jne short 00404CD5h
    jne(ctx, Cont(x404ccd), Cont(x404cd5))
}

pub fn x404ccd(ctx: &mut Context) -> Cont {
    // 00404ccd fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404ccf fld dword ptr ds:[420120h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420120u32) as f64);
    Cont(x404cd5)
}

pub fn x404cd5(ctx: &mut Context) -> Cont {
    // 00404cd5 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00404cd7 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404cdc, dst)
}

pub fn x404cdc(ctx: &mut Context) -> Cont {
    // 00404cdc mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00404ce0 mov edx,ds:[425C00h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425c00u32);
    // 00404ce6 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00404cea lea ecx,[eax+eax*4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 00404ced shl ecx,7
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x7u8, &mut ctx.cpu.flags);
    // 00404cf0 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00404cf2 mov [edx+esi*4],ecx
    ctx.memory.write::<u32>(
        ctx.cpu.regs.edx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.ecx,
    );
    // 00404cf5 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 00404cfb call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404d00, dst)
}

pub fn x404d00(ctx: &mut Context) -> Cont {
    // 00404d00 mov ecx,ds:[425C04h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425c04u32);
    // 00404d06 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00404d07 cmp esi,0C8h
    sub(ctx.cpu.regs.esi, 0xc8u32, &mut ctx.cpu.flags);
    // 00404d0d mov [ecx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .ecx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 00404d11 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404d13 jl near ptr 00404C94h
    jl(ctx, Cont(x404d19), Cont(x404c94))
}

pub fn x404d19(ctx: &mut Context) -> Cont {
    // 00404d19 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x404d1b)
}

pub fn x404d1b(ctx: &mut Context) -> Cont {
    // 00404d1b lea edx,[esi-140h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esi.wrapping_add(0xfffffec0u32);
    // 00404d21 mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 00404d25 fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 00404d29 fmul dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00404d2d fadd dword ptr ds:[42033Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42033cu32) as f64,
    );
    // 00404d33 fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00404d39 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404d3b test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00404d3e je short 00404D4Ah
    je(ctx, Cont(x404d40), Cont(x404d4a))
}

pub fn x404d40(ctx: &mut Context) -> Cont {
    // 00404d40 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404d42 fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 00404d48 jmp short 00404D5Fh
    Cont(x404d5f)
}

pub fn x404d4a(ctx: &mut Context) -> Cont {
    // 00404d4a fcom dword ptr ds:[420118h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420118u32) as f64));
    // 00404d50 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404d52 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00404d55 jne short 00404D5Fh
    jne(ctx, Cont(x404d57), Cont(x404d5f))
}

pub fn x404d57(ctx: &mut Context) -> Cont {
    // 00404d57 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404d59 fld dword ptr ds:[420118h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420118u32) as f64);
    Cont(x404d5f)
}

pub fn x404d5f(ctx: &mut Context) -> Cont {
    // 00404d5f fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00404d61 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404d66, dst)
}

pub fn x404d66(ctx: &mut Context) -> Cont {
    // 00404d66 mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00404d6a mov ecx,ds:[428CE4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428ce4u32);
    // 00404d70 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00404d74 mov [ecx+esi*4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.eax,
    );
    // 00404d77 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00404d79 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 00404d7f call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404d84, dst)
}

pub fn x404d84(ctx: &mut Context) -> Cont {
    // 00404d84 mov edx,ds:[428CE8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428ce8u32);
    // 00404d8a inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00404d8b cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00404d91 mov [edx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 00404d95 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404d97 jl short 00404D1Bh
    jl(ctx, Cont(x404d99), Cont(x404d1b))
}

pub fn x404d99(ctx: &mut Context) -> Cont {
    // 00404d99 call 00407620h
    let dst = Cont(x407620);
    call(ctx, 0x404d9e, dst)
}

pub fn x404d9e(ctx: &mut Context) -> Cont {
    // 00404d9e pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00404d9f pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00404da0 ret
    ret(ctx, 0)
}

pub fn x404db0(ctx: &mut Context) -> Cont {
    // 00404db0 sub esp,50h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 00404db3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00404db4 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00404db5 mov dword ptr [esp+28h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), 0x0u32);
    // 00404dbd mov edi,[esp+28h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    Cont(x404dc1)
}

pub fn x404dc1(ctx: &mut Context) -> Cont {
    // 00404dc1 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x404dc6, dst)
}

pub fn x404dc6(ctx: &mut Context) -> Cont {
    // 00404dc6 fmul dword ptr ds:[420378h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420378u32) as f64,
    );
    // 00404dcc fst dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00404dd0 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404dd5, dst)
}

pub fn x404dd5(ctx: &mut Context) -> Cont {
    // 00404dd5 fld dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00404dd9 fld qword ptr ds:[4202C8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f64>(0x4202c8u32));
    // 00404ddf mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00404de1 sar esi,6
    ctx.cpu.regs.esi = sar(ctx.cpu.regs.esi, 0x6u8, &mut ctx.cpu.flags);
    // 00404de4 and esi,3Fh
    ctx.cpu.regs.esi = and(ctx.cpu.regs.esi, 0x3fu32, &mut ctx.cpu.flags);
    // 00404de7 call 0041F114h
    let dst = Cont(x41f114);
    call(ctx, 0x404dec, dst)
}

pub fn x404dec(ctx: &mut Context) -> Cont {
    // 00404dec fmul qword ptr ds:[420370h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420370u32));
    // 00404df2 push 0
    push(ctx, 0x0u32);
    // 00404df4 fadd qword ptr ds:[4202B8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4202b8u32));
    // 00404dfa fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404dfe call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x404e03, dst)
}

pub fn x404e03(ctx: &mut Context) -> Cont {
    // 00404e03 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00404e07 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404e0d fld dword ptr ds:[420368h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420368u32) as f64);
    // 00404e13 lea eax,[esi+2]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x2u32);
    // 00404e16 add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 00404e19 and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 00404e1c and esi,3Fh
    ctx.cpu.regs.esi = and(ctx.cpu.regs.esi, 0x3fu32, &mut ctx.cpu.flags);
    // 00404e1f mov ecx,[esp+0Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00404e23 fsub dword ptr [eax*4+429460h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>((ctx.cpu.regs.eax * 4).wrapping_add(0x429460u32)) as f64,
    );
    // 00404e2a lea eax,[esi*4+429460h]
    ctx.cpu.regs.eax = (ctx.cpu.regs.esi * 4).wrapping_add(0x429460u32);
    // 00404e31 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404e32 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404e34 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00404e38 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00404e3a fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00404e3c fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404e3e fld st(1)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(1));
    // 00404e40 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404e42 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404e46 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00404e4a fadd dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 00404e4c fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404e52 fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 00404e54 fadd dword ptr ds:[42023Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42023cu32) as f64,
    );
    // 00404e5a fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404e5c fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404e5e fmul dword ptr [esp+14h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00404e62 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00404e66 fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 00404e68 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404e6a fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404e6e fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404e70 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404e72 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 00404e74 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00404e76 fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404e7a call 00404B30h
    let dst = Cont(x404b30);
    call(ctx, 0x404e7f, dst)
}

pub fn x404e7f(ctx: &mut Context) -> Cont {
    // 00404e7f fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00404e83 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00404e85 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404e89 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00404e8d fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00404e8f fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404e93 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00404e97 lea edx,[esp+18h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x18u32);
    // 00404e9b fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00404e9d fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404ea1 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00404ea2 call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x404ea7, dst)
}

pub fn x404ea7(ctx: &mut Context) -> Cont {
    // 00404ea7 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00404eab fmul qword ptr ds:[420298h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420298u32));
    // 00404eb1 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404eb4 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00404eb6 fmul qword ptr ds:[420360h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420360u32));
    // 00404ebc fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404ec0 fld dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00404ec4 fmul qword ptr ds:[420290h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420290u32));
    // 00404eca lea eax,[esp+34h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x34u32);
    // 00404ece mov edx,[esp+2Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 00404ed2 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00404ed3 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404ed5 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404ed8 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00404eda sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00404edd mov [ecx],edi
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edi);
    // 00404edf mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00404ee2 mov edx,[esp+2Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 00404ee6 fmul qword ptr ds:[420358h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420358u32));
    // 00404eec fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404ef0 mov eax,[esp+4Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 00404ef4 mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00404ef7 mov eax,[esp+30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 00404efb mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 00404efd mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00404eff mov edx,[esp+34h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00404f03 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00404f06 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00404f09 call 00406410h
    let dst = Cont(x406410);
    call(ctx, 0x404f0e, dst)
}

pub fn x404f0e(ctx: &mut Context) -> Cont {
    // 00404f0e push 42C80000h
    push(ctx, 0x42c80000u32);
    // 00404f13 lea eax,[esp+3Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x3cu32);
    // 00404f17 push 43A00000h
    push(ctx, 0x43a00000u32);
    // 00404f1c lea ecx,[esp+58h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x58u32);
    // 00404f20 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00404f21 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00404f22 push 429400h
    push(ctx, 0x429400u32);
    // 00404f27 call 00406670h
    let dst = Cont(x406670);
    call(ctx, 0x404f2c, dst)
}

pub fn x404f2c(ctx: &mut Context) -> Cont {
    // 00404f2c fld dword ptr [esp+38h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as f64,
    );
    // 00404f30 fcomp dword ptr ds:[420350h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420350u32) as f64));
    ctx.cpu.fpu.pop();
    // 00404f36 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00404f39 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404f3b test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00404f3e je short 00404F5Eh
    je(ctx, Cont(x404f40), Cont(x404f5e))
}

pub fn x404f40(ctx: &mut Context) -> Cont {
    // 00404f40 fld dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00404f44 fmul qword ptr ds:[420348h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420348u32));
    // 00404f4a fsubr qword ptr ds:[420340h]
    ctx.cpu
        .fpu
        .set(0, ctx.memory.read::<f64>(0x420340u32) - ctx.cpu.fpu.get(0));
    // 00404f50 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x404f55, dst)
}

pub fn x404f55(ctx: &mut Context) -> Cont {
    // 00404f55 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00404f56 call 004057C0h
    let dst = Cont(x4057c0);
    call(ctx, 0x404f5b, dst)
}

pub fn x404f5b(ctx: &mut Context) -> Cont {
    // 00404f5b add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    Cont(x404f5e)
}

pub fn x404f5e(ctx: &mut Context) -> Cont {
    // 00404f5e call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x404f63, dst)
}

pub fn x404f63(ctx: &mut Context) -> Cont {
    // 00404f63 call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x404f68, dst)
}

pub fn x404f68(ctx: &mut Context) -> Cont {
    // 00404f68 push 3F7851ECh
    push(ctx, 0x3f7851ecu32);
    // 00404f6d push 3F7851ECh
    push(ctx, 0x3f7851ecu32);
    // 00404f72 call 00404C90h
    let dst = Cont(x404c90);
    call(ctx, 0x404f77, dst)
}

pub fn x404f77(ctx: &mut Context) -> Cont {
    // 00404f77 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00404f7a call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x404f7f, dst)
}

pub fn x404f7f(ctx: &mut Context) -> Cont {
    // 00404f7f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00404f81 jne short 00404F9Eh
    jne(ctx, Cont(x404f83), Cont(x404f9e))
}

pub fn x404f83(ctx: &mut Context) -> Cont {
    // 00404f83 fld dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00404f87 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00404f8d fcomp dword ptr ds:[4200D0h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4200d0u32) as f64));
    ctx.cpu.fpu.pop();
    // 00404f93 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00404f95 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00404f98 jne near ptr 00404DC1h
    jne(ctx, Cont(x404f9e), Cont(x404dc1))
}

pub fn x404f9e(ctx: &mut Context) -> Cont {
    // 00404f9e pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00404f9f pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00404fa0 add esp,50h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 00404fa3 ret
    ret(ctx, 0)
}

pub fn x404fb0(ctx: &mut Context) -> Cont {
    // 00404fb0 sub esp,14h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00404fb3 fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00404fb7 fadd dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00404fbb fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404fc1 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00404fc5 fadd dword ptr ds:[4201ACh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4201acu32) as f64,
    );
    // 00404fcb fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00404fcd fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404fcf fmul qword ptr ds:[4202F0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202f0u32));
    // 00404fd5 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00404fd9 fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00404fdd fadd dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00404fe1 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00404fe7 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00404feb fadd dword ptr ds:[420224h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420224u32) as f64,
    );
    // 00404ff1 fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00404ff3 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00404ff5 fmul qword ptr ds:[4202E8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202e8u32));
    // 00404ffb fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00404fff fadd dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405003 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00405009 fld dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 0040500d fadd dword ptr ds:[4200A8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4200a8u32) as f64,
    );
    // 00405013 fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00405015 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405017 fmul qword ptr ds:[4202F0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202f0u32));
    // 0040501d fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405021 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405025 fadd dword ptr ds:[4202E0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4202e0u32) as f64,
    );
    // 0040502b fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040502d fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040502f fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 00405035 fadd dword ptr [esp+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00405039 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040503d fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00405041 fadd dword ptr ds:[4202D4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4202d4u32) as f64,
    );
    // 00405047 fst dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040504b fdivr st,st(3)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(3) / ctx.cpu.fpu.get(0));
    // 0040504d fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040504f fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 00405055 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00405057 fstp dword ptr [esp+0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0xcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040505b fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040505d fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405061 fadd dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00405065 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 0040506b fstp qword ptr [esp]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040506f fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405073 fadd dword ptr ds:[4200A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4200a4u32) as f64,
    );
    // 00405079 fdivr qword ptr [esp]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f64>(ctx.cpu.regs.esp) / ctx.cpu.fpu.get(0),
    );
    // 0040507d fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 0040507f fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 00405085 fadd dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405089 fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040508d fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405091 fadd dword ptr ds:[4202E0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4202e0u32) as f64,
    );
    // 00405097 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405099 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040509b fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 004050a1 fadd dword ptr [esp+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 004050a5 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004050a9 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 004050ad fdivr qword ptr [esp]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f64>(ctx.cpu.regs.esp) / ctx.cpu.fpu.get(0),
    );
    // 004050b1 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004050b3 fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 004050b9 fadd dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 004050bd mov eax,[esp+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 004050c1 mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    // 004050c3 fstp dword ptr [esp+0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0xcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004050c7 fld dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 004050cb fadd dword ptr ds:[4200A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x4200a4u32) as f64,
    );
    // 004050d1 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004050d3 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004050d5 fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 004050db fsubr dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64
            - ctx.cpu.fpu.get(0),
    );
    // 004050df fld dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 004050e3 fmul dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004050e7 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004050eb fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 004050ef fmul dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 004050f3 mov edx,[esp+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 004050f7 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 004050f9 fstp dword ptr [esp+0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0xcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004050fd mov edx,[esp+0Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00405101 fmul dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00405105 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00405108 fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040510c mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00405110 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00405113 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00405116 ret
    ret(ctx, 0)
}

pub fn x405120(ctx: &mut Context) -> Cont {
    // 00405120 sub esp,30h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00405123 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405124 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00405125 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00405126 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00405127 push 408C10h
    push(ctx, 0x408c10u32);
    // 0040512c call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x405131, dst)
}

pub fn x405131(ctx: &mut Context) -> Cont {
    // 00405131 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00405134 mov ds:[42938Ch],eax
    ctx.memory.write::<u32>(0x42938cu32, ctx.cpu.regs.eax);
    // 00405139 mov esi,429460h
    ctx.cpu.regs.esi = 0x429460u32;
    Cont(x40513e)
}

pub fn x40513e(ctx: &mut Context) -> Cont {
    // 0040513e call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x405143, dst)
}

pub fn x405143(ctx: &mut Context) -> Cont {
    // 00405143 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405144 mov ecx,12Ch
    ctx.cpu.regs.ecx = 0x12cu32;
    // 00405149 add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 0040514c idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040514e cmp esi,429560h
    sub(ctx.cpu.regs.esi, 0x429560u32, &mut ctx.cpu.flags);
    // 00405154 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 00405158 fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 0040515c fsub dword ptr ds:[420338h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420338u32) as f64,
    );
    // 00405162 fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405165 jl short 0040513Eh
    jl(ctx, Cont(x405167), Cont(x40513e))
}

pub fn x405167(ctx: &mut Context) -> Cont {
    // 00405167 mov esi,4293A4h
    ctx.cpu.regs.esi = 0x4293a4u32;
    // 0040516c mov [esp+10h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.esi);
    // 00405170 jmp short 00405176h
    Cont(x405176)
}

pub fn x405172(ctx: &mut Context) -> Cont {
    // 00405172 mov esi,[esp+10h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    Cont(x405176)
}

pub fn x405176(ctx: &mut Context) -> Cont {
    // 00405176 push 0D98h
    push(ctx, 0xd98u32);
    // 0040517b mov dword ptr [esi-4],8Ch
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32), 0x8cu32);
    // 00405182 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x405187, dst)
}

pub fn x405187(ctx: &mut Context) -> Cont {
    // 00405187 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040518a mov [esi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, ctx.cpu.regs.eax);
    // 0040518c mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 0040518e call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x405193, dst)
}

pub fn x405193(ctx: &mut Context) -> Cont {
    // 00405193 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405194 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    // 00405199 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040519b mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 0040519f fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 004051a3 fmul dword ptr ds:[420384h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420384u32) as f64,
    );
    // 004051a9 fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004051ad call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4051b2, dst)
}

pub fn x4051b2(ctx: &mut Context) -> Cont {
    // 004051b2 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004051b3 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    // 004051b8 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004051ba mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 004051be fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 004051c2 fmul dword ptr ds:[420384h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420384u32) as f64,
    );
    // 004051c8 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004051cc call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4051d1, dst)
}

pub fn x4051d1(ctx: &mut Context) -> Cont {
    // 004051d1 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004051d2 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    // 004051d7 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004051d9 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 004051dd fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 004051e1 fmul dword ptr ds:[420384h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420384u32) as f64,
    );
    // 004051e7 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004051eb call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4051f0, dst)
}

pub fn x4051f0(ctx: &mut Context) -> Cont {
    // 004051f0 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004051f1 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    // 004051f6 lea edi,[esi+0Ch]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0xcu32);
    // 004051f9 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004051fb xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004051fd mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 00405201 fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00405205 mov [esp+24h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.ebx);
    // 00405209 fmul dword ptr ds:[420384h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420384u32) as f64,
    );
    // 0040520f fstp dword ptr [esp+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x405213)
}

pub fn x405213(ctx: &mut Context) -> Cont {
    // 00405213 fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00405217 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0040521b mov eax,[esp+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 0040521f mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00405223 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405224 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405228 mov edx,[esp+24h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0040522c mov ebp,[esp+28h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00405230 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405231 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405232 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405233 push 3F000000h
    push(ctx, 0x3f000000u32);
    // 00405238 lea eax,[esp+3Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x3cu32);
    // 0040523c push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0040523d push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040523e call 00404FB0h
    let dst = Cont(x404fb0);
    call(ctx, 0x405243, dst)
}

pub fn x405243(ctx: &mut Context) -> Cont {
    // 00405243 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00405245 mov ecx,esi
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi;
    // 00405247 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00405249 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0040524c mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040524f mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00405252 mov edx,[esp+34h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00405256 mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00405259 mov ecx,[esp+30h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 0040525d mov eax,[esp+38h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 00405261 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405262 mov ecx,[esp+40h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00405266 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405267 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405268 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405269 push 3F800000h
    push(ctx, 0x3f800000u32);
    // 0040526e lea edx,[esp+64h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x64u32);
    // 00405272 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00405273 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405274 call 00404FB0h
    let dst = Cont(x404fb0);
    call(ctx, 0x405279, dst)
}

pub fn x405279(ctx: &mut Context) -> Cont {
    // 00405279 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 0040527b mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 0040527d add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 00405280 add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 00405283 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00405285 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00405288 add edi,18h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x18u32, &mut ctx.cpu.flags);
    // 0040528b inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040528c mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040528f mov eax,[eax+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00405292 cmp ebx,8Ch
    sub(ctx.cpu.regs.ebx, 0x8cu32, &mut ctx.cpu.flags);
    // 00405298 mov [esp+24h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.ebx);
    // 0040529c mov [ecx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040529f jl near ptr 00405213h
    jl(ctx, Cont(x4052a5), Cont(x405213))
}

pub fn x4052a5(ctx: &mut Context) -> Cont {
    // 004052a5 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 004052a9 add eax,8
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x8u32, &mut ctx.cpu.flags);
    // 004052ac cmp eax,4293F4h
    sub(ctx.cpu.regs.eax, 0x4293f4u32, &mut ctx.cpu.flags);
    // 004052b1 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 004052b5 jl near ptr 00405172h
    jl(ctx, Cont(x4052bb), Cont(x405172))
}

pub fn x4052bb(ctx: &mut Context) -> Cont {
    // 004052bb push 1338h
    push(ctx, 0x1338u32);
    // 004052c0 mov dword ptr ds:[4293F0h],0C8h
    ctx.memory.write::<u32>(0x4293f0u32, 0xc8u32);
    // 004052ca call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x4052cf, dst)
}

pub fn x4052cf(ctx: &mut Context) -> Cont {
    // 004052cf add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004052d2 mov ds:[4293F4h],eax
    ctx.memory.write::<u32>(0x4293f4u32, ctx.cpu.regs.eax);
    // 004052d7 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 004052d9 mov ebx,0C8h
    ctx.cpu.regs.ebx = 0xc8u32;
    Cont(x4052de)
}

pub fn x4052de(ctx: &mut Context) -> Cont {
    // 004052de call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4052e3, dst)
}

pub fn x4052e3(ctx: &mut Context) -> Cont {
    // 004052e3 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004052e4 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    // 004052e9 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004052eb add edx,12Ch
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x12cu32, &mut ctx.cpu.flags);
    // 004052f1 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 004052f5 fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 004052f9 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004052fb fmul dword ptr ds:[4201B0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4201b0u32) as f64,
    );
    // 00405301 fsub dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 00405307 fstp dword ptr [esi+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040530a fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0040530c call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405311, dst)
}

pub fn x405311(ctx: &mut Context) -> Cont {
    // 00405311 fmul dword ptr ds:[4202A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202a4u32) as f64,
    );
    // 00405317 mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 00405319 fstp dword ptr [esp+24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x24u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040531d call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x405322, dst)
}

pub fn x405322(ctx: &mut Context) -> Cont {
    // 00405322 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405323 idiv edi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.edi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405325 mov [esp+20h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.edx);
    // 00405329 fild dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as i32 as f64,
    );
    // 0040532d fsub dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405331 fstp dword ptr [esi]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esi, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00405333 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x405338, dst)
}

pub fn x405338(ctx: &mut Context) -> Cont {
    // 00405338 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405339 idiv edi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.edi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040533b mov [esp+20h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.edx);
    // 0040533f mov edx,esi
    ctx.cpu.regs.edx = ctx.cpu.regs.esi;
    // 00405341 fild dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as i32 as f64,
    );
    // 00405345 fsub dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405349 fmul dword ptr ds:[420380h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420380u32) as f64,
    );
    // 0040534f fstp dword ptr [esi+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405352 fld dword ptr [esi]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esi) as f64);
    // 00405354 fmul dword ptr ds:[42037Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42037cu32) as f64,
    );
    // 0040535a fstp dword ptr [esi]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esi, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 0040535c mov eax,[edx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edx);
    // 0040535e fld dword ptr [esi+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0x8u32)) as f64,
    );
    // 00405361 mov ecx,[edx+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32));
    // 00405364 mov [esi+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 00405367 fsub dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 0040536d mov edx,[edx+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32));
    // 00405370 mov [esi+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00405373 mov [esi+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00405376 fstp dword ptr [esi+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405379 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x40537e, dst)
}

pub fn x40537e(ctx: &mut Context) -> Cont {
    // 0040537e cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040537f mov ecx,64h
    ctx.cpu.regs.ecx = 0x64u32;
    // 00405384 add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 00405387 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405389 dec ebx
    ctx.cpu.regs.ebx = dec(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040538a mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 0040538e fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00405392 fadd dword ptr [esi-4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32)) as f64,
    );
    // 00405395 fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405398 jne near ptr 004052DEh
    jne(ctx, Cont(x40539e), Cont(x4052de))
}

pub fn x40539e(ctx: &mut Context) -> Cont {
    // 0040539e pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040539f pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004053a0 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 004053a1 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004053a2 add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 004053a5 ret
    ret(ctx, 0)
}

pub fn x4053b0(ctx: &mut Context) -> Cont {
    // 004053b0 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004053b1 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004053b2 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x4053b4)
}

pub fn x4053b4(ctx: &mut Context) -> Cont {
    // 004053b4 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4053b9, dst)
}

pub fn x4053b9(ctx: &mut Context) -> Cont {
    // 004053b9 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004053ba mov ecx,3Ch
    ctx.cpu.regs.ecx = 0x3cu32;
    // 004053bf idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004053c1 lea edx,[edx+esi-1Eh]
    ctx.cpu.regs.edx = ctx
        .cpu
        .regs
        .edx
        .wrapping_add(ctx.cpu.regs.esi)
        .wrapping_add(0xffffffe2u32);
    // 004053c5 mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004053c9 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004053cd fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 004053d3 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004053d5 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004053d8 je short 004053E4h
    je(ctx, Cont(x4053da), Cont(x4053e4))
}

pub fn x4053da(ctx: &mut Context) -> Cont {
    // 004053da fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004053dc fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 004053e2 jmp short 004053F9h
    Cont(x4053f9)
}

pub fn x4053e4(ctx: &mut Context) -> Cont {
    // 004053e4 fcom dword ptr ds:[420120h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420120u32) as f64));
    // 004053ea fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004053ec test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 004053ef jne short 004053F9h
    jne(ctx, Cont(x4053f1), Cont(x4053f9))
}

pub fn x4053f1(ctx: &mut Context) -> Cont {
    // 004053f1 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004053f3 fld dword ptr ds:[420120h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420120u32) as f64);
    Cont(x4053f9)
}

pub fn x4053f9(ctx: &mut Context) -> Cont {
    // 004053f9 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004053fb call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405400, dst)
}

pub fn x405400(ctx: &mut Context) -> Cont {
    // 00405400 mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00405404 mov ecx,ds:[425C00h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425c00u32);
    // 0040540a fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 0040540e lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 00405411 shl eax,7
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 00405414 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00405416 mov [ecx+esi*4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.eax,
    );
    // 00405419 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 0040541f call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405424, dst)
}

pub fn x405424(ctx: &mut Context) -> Cont {
    // 00405424 mov edx,ds:[425C04h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425c04u32);
    // 0040542a inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040542b cmp esi,0C8h
    sub(ctx.cpu.regs.esi, 0xc8u32, &mut ctx.cpu.flags);
    // 00405431 mov [edx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 00405435 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405437 jl near ptr 004053B4h
    jl(ctx, Cont(x40543d), Cont(x4053b4))
}

pub fn x40543d(ctx: &mut Context) -> Cont {
    // 0040543d xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040543f mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    Cont(x405443)
}

pub fn x405443(ctx: &mut Context) -> Cont {
    // 00405443 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00405447 fsub dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 0040544b fmul dword ptr [esp+14h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 0040544f fadd dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00405453 fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00405459 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040545b test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0040545e je short 0040546Ah
    je(ctx, Cont(x405460), Cont(x40546a))
}

pub fn x405460(ctx: &mut Context) -> Cont {
    // 00405460 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405462 fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 00405468 jmp short 0040547Fh
    Cont(x40547f)
}

pub fn x40546a(ctx: &mut Context) -> Cont {
    // 0040546a fcom dword ptr ds:[420118h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420118u32) as f64));
    // 00405470 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405472 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405475 jne short 0040547Fh
    jne(ctx, Cont(x405477), Cont(x40547f))
}

pub fn x405477(ctx: &mut Context) -> Cont {
    // 00405477 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405479 fld dword ptr ds:[420118h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420118u32) as f64);
    Cont(x40547f)
}

pub fn x40547f(ctx: &mut Context) -> Cont {
    // 0040547f fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00405481 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405486, dst)
}

pub fn x405486(ctx: &mut Context) -> Cont {
    // 00405486 mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 0040548a mov ecx,ds:[428CE4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428ce4u32);
    // 00405490 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00405494 mov [ecx+esi*4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)),
        ctx.cpu.regs.eax,
    );
    // 00405497 fsubr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    // 00405499 fmul dword ptr ds:[42011Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42011cu32) as f64,
    );
    // 0040549f call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4054a4, dst)
}

pub fn x4054a4(ctx: &mut Context) -> Cont {
    // 004054a4 mov edx,ds:[428CE8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428ce8u32);
    // 004054aa inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004054ab cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 004054b1 mov [esp+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    // 004054b5 mov [edx+esi-1],al
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.esi)
            .wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_al(),
    );
    // 004054b9 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004054bb jl short 00405443h
    jl(ctx, Cont(x4054bd), Cont(x405443))
}

pub fn x4054bd(ctx: &mut Context) -> Cont {
    // 004054bd call 00407620h
    let dst = Cont(x407620);
    call(ctx, 0x4054c2, dst)
}

pub fn x4054c2(ctx: &mut Context) -> Cont {
    // 004054c2 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004054c3 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 004054c4 ret
    ret(ctx, 0)
}

pub fn x4054d0(ctx: &mut Context) -> Cont {
    // 004054d0 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004054d1 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 004054d3 sub esp,58h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x58u32, &mut ctx.cpu.flags);
    // 004054d6 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004054d7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004054d8 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004054d9 mov dword ptr [ebp-34h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffccu32), 0x0u32);
    Cont(x4054e0)
}

pub fn x4054e0(ctx: &mut Context) -> Cont {
    // 004054e0 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x4054e5, dst)
}

pub fn x4054e5(ctx: &mut Context) -> Cont {
    // 004054e5 fmul dword ptr ds:[4203E8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4203e8u32) as f64,
    );
    // 004054eb call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4054f0, dst)
}

pub fn x4054f0(ctx: &mut Context) -> Cont {
    // 004054f0 mov [ebp-8],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.eax,
    );
    // 004054f3 mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 004054f5 fild dword ptr [ebp-8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32)) as i32 as f64,
    );
    // 004054f8 sar ebx,6
    ctx.cpu.regs.ebx = sar(ctx.cpu.regs.ebx, 0x6u8, &mut ctx.cpu.flags);
    // 004054fb and ebx,3Fh
    ctx.cpu.regs.ebx = and(ctx.cpu.regs.ebx, 0x3fu32, &mut ctx.cpu.flags);
    // 004054fe fst qword ptr [ebp-10h]
    ctx.memory.write::<f64>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32),
        ctx.cpu.fpu.get(0),
    );
    // 00405501 fld qword ptr ds:[4202C8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f64>(0x4202c8u32));
    // 00405507 call 0041F114h
    let dst = Cont(x41f114);
    call(ctx, 0x40550c, dst)
}

pub fn x40550c(ctx: &mut Context) -> Cont {
    // 0040550c fmul qword ptr ds:[420358h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420358u32));
    // 00405512 push 0
    push(ctx, 0x0u32);
    // 00405514 fadd qword ptr ds:[4202B8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x4202b8u32));
    // 0040551a fstp dword ptr [ebp-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040551d call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x405522, dst)
}

pub fn x405522(ctx: &mut Context) -> Cont {
    // 00405522 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00405525 mov esi,ds:[42938Ch]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x42938cu32);
    // 0040552b mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00405531 mov ecx,64h
    ctx.cpu.regs.ecx = 0x64u32;
    Cont(x405536)
}

pub fn x405536(ctx: &mut Context) -> Cont {
    // 00405536 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405537 add edi,140h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x140u32, &mut ctx.cpu.flags);
    // 0040553d mov ecx,0A0h
    ctx.cpu.regs.ecx = 0xa0u32;
    Cont(x405542)
}

pub fn x405542(ctx: &mut Context) -> Cont {
    // 00405542 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 00405544 mov ah,al
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    // 00405546 mov [edi],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.edi, ctx.cpu.regs.get_ax());
    // 00405549 mov [edi+280h],ax
    ctx.memory.write::<u16>(
        ctx.cpu.regs.edi.wrapping_add(0x280u32),
        ctx.cpu.regs.get_ax(),
    );
    // 00405550 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00405551 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00405552 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00405553 loop 00405542h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x405555)
    } else {
        Cont(x405542)
    }
}

pub fn x405555(ctx: &mut Context) -> Cont {
    // 00405555 add edi,280h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x280u32, &mut ctx.cpu.flags);
    // 0040555b pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 0040555c loop 00405536h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x40555e)
    } else {
        Cont(x405536)
    }
}

pub fn x40555e(ctx: &mut Context) -> Cont {
    // 0040555e fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 00405561 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00405567 fld dword ptr ds:[4202A8h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4202a8u32) as f64);
    // 0040556d lea eax,[ebx+2]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx.wrapping_add(0x2u32);
    // 00405570 lea ecx,[ebx+4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx.wrapping_add(0x4u32);
    // 00405573 and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 00405576 and ecx,3Fh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3fu32, &mut ctx.cpu.flags);
    // 00405579 lea edx,[ebp-1Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32);
    // 0040557c fsub dword ptr [eax*4+429460h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>((ctx.cpu.regs.eax * 4).wrapping_add(0x429460u32)) as f64,
    );
    // 00405583 lea eax,[eax*4+429460h]
    ctx.cpu.regs.eax = (ctx.cpu.regs.eax * 4).wrapping_add(0x429460u32);
    // 0040558a lea ecx,[ecx*4+429460h]
    ctx.cpu.regs.ecx = (ctx.cpu.regs.ecx * 4).wrapping_add(0x429460u32);
    // 00405591 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405592 fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00405594 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405596 fmul dword ptr [ebp-4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32)) as f64,
    );
    // 00405599 fstp dword ptr [ebp-28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffd8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040559c fild dword ptr [ebp-8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32)) as i32 as f64,
    );
    // 0040559f fadd dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 004055a1 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 004055a7 fld dword ptr [ecx]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64);
    // 004055a9 fadd dword ptr ds:[42023Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42023cu32) as f64,
    );
    // 004055af fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004055b1 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004055b3 fld dword ptr [ebp-4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32)) as f64,
    );
    // 004055b6 fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 004055b8 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004055ba fstp dword ptr [ebp-24h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffdcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004055bd fld dword ptr ds:[4202A0h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4202a0u32) as f64);
    // 004055c3 fsub dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 004055c5 fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 004055c7 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004055c9 fmul dword ptr [ebp-4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32)) as f64,
    );
    // 004055cc fsub qword ptr ds:[420258h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) - ctx.memory.read::<f64>(0x420258u32));
    // 004055d2 fstp dword ptr [ebp-20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe0u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004055d5 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004055d7 fld dword ptr [ebp-28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffd8u32)) as f64,
    );
    // 004055da fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004055dc fstp dword ptr [ebp-1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004055df fld dword ptr [ebp-24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffdcu32)) as f64,
    );
    // 004055e2 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004055e4 fstp dword ptr [ebp-18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004055e7 fld dword ptr [ebp-20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe0u32)) as f64,
    );
    // 004055ea fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 004055ec fstp dword ptr [ebp-14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffecu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004055ef call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x4055f4, dst)
}

pub fn x4055f4(ctx: &mut Context) -> Cont {
    // 004055f4 fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 004055f7 fmul qword ptr ds:[420298h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420298u32));
    // 004055fd add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00405600 lea eax,[ebp-58h]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp.wrapping_add(0xffffffa8u32);
    // 00405603 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405605 fmul qword ptr ds:[4203E0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203e0u32));
    // 0040560b fstp dword ptr [ebp-30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffd0u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040560e fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 00405611 fmul qword ptr ds:[420290h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420290u32));
    // 00405617 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00405619 fmul qword ptr ds:[4203D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203d8u32));
    // 0040561f fstp dword ptr [ebp-2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffd4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405622 mov edx,[ebp-34h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffccu32));
    // 00405625 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405626 mov eax,[ebp-30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffd0u32));
    // 00405629 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040562c mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 0040562e sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00405631 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00405633 mov edx,[ebp-2Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffd4u32));
    // 00405636 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00405639 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040563b mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040563e mov ecx,[ebp-1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32));
    // 00405641 mov edx,[ebp-18h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe8u32));
    // 00405644 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00405646 mov ecx,[ebp-14h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffecu32));
    // 00405649 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040564c mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040564f call 00406410h
    let dst = Cont(x406410);
    call(ctx, 0x405654, dst)
}

pub fn x405654(ctx: &mut Context) -> Cont {
    // 00405654 mov edi,[ebp-8]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32));
    // 00405657 mov ecx,9
    ctx.cpu.regs.ecx = 0x9u32;
    // 0040565c mov eax,edi
    ctx.cpu.regs.eax = ctx.cpu.regs.edi;
    // 0040565e cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040565f fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 00405662 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 00405665 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00405667 sar eax,6
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x6u8, &mut ctx.cpu.flags);
    // 0040566a cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040566b fld qword ptr ds:[4203D0h]
    ctx.cpu.fpu.push(ctx.memory.read::<f64>(0x4203d0u32));
    // 00405671 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405673 mov esi,edx
    ctx.cpu.regs.esi = ctx.cpu.regs.edx;
    // 00405675 call 0041F114h
    let dst = Cont(x41f114);
    call(ctx, 0x40567a, dst)
}

pub fn x40567a(ctx: &mut Context) -> Cont {
    // 0040567a lea eax,[esi+1]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x1u32);
    // 0040567d mov ecx,9
    ctx.cpu.regs.ecx = 0x9u32;
    // 00405682 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405683 fmul qword ptr ds:[4203C8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203c8u32));
    // 00405689 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040568b add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 0040568e lea eax,[esi*8+4293A8h]
    ctx.cpu.regs.eax = (ctx.cpu.regs.esi * 8).wrapping_add(0x4293a8u32);
    // 00405695 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00405698 push 4293A0h
    push(ctx, 0x4293a0u32);
    // 0040569d lea edx,[edx*8+4293A8h]
    ctx.cpu.regs.edx = (ctx.cpu.regs.edx * 8).wrapping_add(0x4293a8u32);
    // 004056a4 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004056a5 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004056a6 call 00406830h
    let dst = Cont(x406830);
    call(ctx, 0x4056ab, dst)
}

pub fn x4056ab(ctx: &mut Context) -> Cont {
    // 004056ab fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 004056ae fmul qword ptr ds:[4203C0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203c0u32));
    // 004056b4 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004056b7 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004056b9 fmul qword ptr ds:[4203B8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203b8u32));
    // 004056bf fsubr qword ptr ds:[4203B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.memory.read::<f64>(0x4203b0u32) - ctx.cpu.fpu.get(0));
    // 004056c5 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004056c8 fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 004056cb fmul qword ptr ds:[4203A8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203a8u32));
    // 004056d1 push 3F4CCCCDh
    push(ctx, 0x3f4ccccdu32);
    // 004056d6 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004056d7 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004056d9 fmul qword ptr ds:[4203A0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203a0u32));
    // 004056df fadd qword ptr ds:[420398h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x420398u32));
    // 004056e5 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004056e8 fld qword ptr [ebp-10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32)),
    );
    // 004056eb fmul qword ptr ds:[420390h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420390u32));
    // 004056f1 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004056f2 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004056f4 fmul qword ptr ds:[4203A0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203a0u32));
    // 004056fa fadd qword ptr ds:[420388h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) + ctx.memory.read::<f64>(0x420388u32));
    // 00405700 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00405703 call 004053B0h
    let dst = Cont(x4053b0);
    call(ctx, 0x405708, dst)
}

pub fn x405708(ctx: &mut Context) -> Cont {
    // 00405708 push 42C80000h
    push(ctx, 0x42c80000u32);
    // 0040570d lea ecx,[ebp-28h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebp.wrapping_add(0xffffffd8u32);
    // 00405710 push 43200000h
    push(ctx, 0x43200000u32);
    // 00405715 lea edx,[ebp-58h]
    ctx.cpu.regs.edx = ctx.cpu.regs.ebp.wrapping_add(0xffffffa8u32);
    // 00405718 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405719 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040571a push 4293A0h
    push(ctx, 0x4293a0u32);
    // 0040571f call 00406670h
    let dst = Cont(x406670);
    call(ctx, 0x405724, dst)
}

pub fn x405724(ctx: &mut Context) -> Cont {
    // 00405724 add esp,24h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x24u32, &mut ctx.cpu.flags);
    // 00405727 sar ebx,1
    ctx.cpu.regs.ebx = sar(ctx.cpu.regs.ebx, 0x1u8, &mut ctx.cpu.flags);
    // 00405729 dec ebx
    ctx.cpu.regs.ebx = dec(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040572a sub ebx,0
    ctx.cpu.regs.ebx = sub(ctx.cpu.regs.ebx, 0x0u32, &mut ctx.cpu.flags);
    // 0040572d je short 00405739h
    je(ctx, Cont(x40572f), Cont(x405739))
}

pub fn x40572f(ctx: &mut Context) -> Cont {
    // 0040572f dec ebx
    ctx.cpu.regs.ebx = dec(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00405730 jne short 00405776h
    jne(ctx, Cont(x405732), Cont(x405776))
}

pub fn x405732(ctx: &mut Context) -> Cont {
    // 00405732 push 4210B0h
    push(ctx, 0x4210b0u32);
    // 00405737 jmp short 0040573Eh
    Cont(x40573e)
}

pub fn x405739(ctx: &mut Context) -> Cont {
    // 00405739 push 4210A4h
    push(ctx, 0x4210a4u32);
    Cont(x40573e)
}

pub fn x40573e(ctx: &mut Context) -> Cont {
    // 0040573e push 0FFh
    push(ctx, 0xffu32);
    // 00405743 push 1Eh
    push(ctx, 0x1eu32);
    // 00405745 push 28h
    push(ctx, 0x28u32);
    // 00405747 push 28h
    push(ctx, 0x28u32);
    // 00405749 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x40574e, dst)
}

pub fn x40574e(ctx: &mut Context) -> Cont {
    // 0040574e cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040574f mov ecx,5
    ctx.cpu.regs.ecx = 0x5u32;
    // 00405754 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405756 add edx,64h
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x64u32, &mut ctx.cpu.flags);
    // 00405759 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040575a call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x40575f, dst)
}

pub fn x40575f(ctx: &mut Context) -> Cont {
    // 0040575f cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405760 mov ecx,5
    ctx.cpu.regs.ecx = 0x5u32;
    // 00405765 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405767 add edx,1F4h
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x1f4u32, &mut ctx.cpu.flags);
    // 0040576d push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040576e call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x405773, dst)
}

pub fn x405773(ctx: &mut Context) -> Cont {
    // 00405773 add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    Cont(x405776)
}

pub fn x405776(ctx: &mut Context) -> Cont {
    // 00405776 call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x40577b, dst)
}

pub fn x40577b(ctx: &mut Context) -> Cont {
    // 0040577b call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x405780, dst)
}

pub fn x405780(ctx: &mut Context) -> Cont {
    // 00405780 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x405785, dst)
}

pub fn x405785(ctx: &mut Context) -> Cont {
    // 00405785 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405787 jne short 00405796h
    jne(ctx, Cont(x405789), Cont(x405796))
}

pub fn x405789(ctx: &mut Context) -> Cont {
    // 00405789 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040578a cmp edi,1A4h
    sub(ctx.cpu.regs.edi, 0x1a4u32, &mut ctx.cpu.flags);
    // 00405790 jl near ptr 004054E0h
    jl(ctx, Cont(x405796), Cont(x4054e0))
}

pub fn x405796(ctx: &mut Context) -> Cont {
    // 00405796 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00405797 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00405798 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00405799 mov esp,ebp
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    // 0040579b pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040579c ret
    ret(ctx, 0)
}

pub fn x4057a0(ctx: &mut Context) -> Cont {
    // 004057a0 push 40B520h
    push(ctx, 0x40b520u32);
    // 004057a5 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x4057aa, dst)
}

pub fn x4057aa(ctx: &mut Context) -> Cont {
    // 004057aa add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004057ad mov ds:[429388h],eax
    ctx.memory.write::<u32>(0x429388u32, ctx.cpu.regs.eax);
    // 004057b2 ret
    ret(ctx, 0)
}

pub fn x4057c0(ctx: &mut Context) -> Cont {
    // 004057c0 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004057c1 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 004057c3 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004057c4 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004057c5 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004057c6 mov esi,ds:[429388h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x429388u32);
    // 004057cc add esi,0C80h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0xc80u32, &mut ctx.cpu.flags);
    // 004057d2 mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 004057d8 add edi,0A0h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0xa0u32, &mut ctx.cpu.flags);
    // 004057de mov dl,[ebp+8]
    ctx.cpu
        .regs
        .set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)));
    // 004057e1 mov dh,0FFh
    ctx.cpu.regs.set_dh(0xffu8);
    // 004057e3 sub dh,[ebp+8]
    ctx.cpu.regs.set_dh(sub(
        ctx.cpu.regs.get_dh(),
        ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)),
        &mut ctx.cpu.flags,
    ));
    // 004057e6 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    Cont(x4057eb)
}

pub fn x4057eb(ctx: &mut Context) -> Cont {
    // 004057eb push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004057ec mov ecx,140h
    ctx.cpu.regs.ecx = 0x140u32;
    Cont(x4057f1)
}

pub fn x4057f1(ctx: &mut Context) -> Cont {
    // 004057f1 mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004057f3 mov bl,al
    ctx.cpu.regs.set_bl(ctx.cpu.regs.get_al());
    // 004057f5 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 004057f7 mul dl
    todo!();
    // 004057f9 shr ax,8
    ctx.cpu
        .regs
        .set_ax(shr(ctx.cpu.regs.get_ax(), 0x8u8, &mut ctx.cpu.flags));
    // 004057fd add al,bl
    ctx.cpu.regs.set_al(add(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_bl(),
        &mut ctx.cpu.flags,
    ));
    // 004057ff jae short 00405803h
    jae(ctx, Cont(x405801), Cont(x405803))
}

pub fn x405801(ctx: &mut Context) -> Cont {
    // 00405801 mov al,0FFh
    ctx.cpu.regs.set_al(0xffu8);
    Cont(x405803)
}

pub fn x405803(ctx: &mut Context) -> Cont {
    // 00405803 mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 00405805 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00405806 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00405807 loop 004057F1h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x405809)
    } else {
        Cont(x4057f1)
    }
}

pub fn x405809(ctx: &mut Context) -> Cont {
    // 00405809 add edi,140h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x140u32, &mut ctx.cpu.flags);
    // 0040580f pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00405810 loop 004057EBh
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x405812)
    } else {
        Cont(x4057eb)
    }
}

pub fn x405812(ctx: &mut Context) -> Cont {
    // 00405812 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00405813 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00405814 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00405815 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00405816 ret
    ret(ctx, 0)
}

pub fn x405820(ctx: &mut Context) -> Cont {
    // 00405820 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00405821 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00405823 mov eax,[ebp+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32));
    // 00405826 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00405827 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00405828 lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0040582b shl eax,6
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x6u8, &mut ctx.cpu.flags);
    // 0040582e mov [ebp+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 00405831 mov esi,ds:[429388h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x429388u32);
    // 00405837 add esi,[ebp+8]
    ctx.cpu.regs.esi = add(
        ctx.cpu.regs.esi,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)),
        &mut ctx.cpu.flags,
    );
    // 0040583a mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00405840 mov ecx,64h
    ctx.cpu.regs.ecx = 0x64u32;
    // 00405845 mov dl,[ebp+0Ch]
    ctx.cpu
        .regs
        .set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0xcu32)));
    Cont(x405848)
}

pub fn x405848(ctx: &mut Context) -> Cont {
    // 00405848 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405849 mov ecx,140h
    ctx.cpu.regs.ecx = 0x140u32;
    Cont(x40584e)
}

pub fn x40584e(ctx: &mut Context) -> Cont {
    // 0040584e mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 00405850 mul dl
    todo!();
    // 00405852 shr ax,8
    ctx.cpu
        .regs
        .set_ax(shr(ctx.cpu.regs.get_ax(), 0x8u8, &mut ctx.cpu.flags));
    // 00405856 mov ah,al
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    // 00405858 mov [edi],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.edi, ctx.cpu.regs.get_ax());
    // 0040585b mov [edi+280h],ax
    ctx.memory.write::<u16>(
        ctx.cpu.regs.edi.wrapping_add(0x280u32),
        ctx.cpu.regs.get_ax(),
    );
    // 00405862 add esi,1
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x1u32, &mut ctx.cpu.flags);
    // 00405865 add edi,2
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x2u32, &mut ctx.cpu.flags);
    // 00405868 loop 0040584Eh
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x40586a)
    } else {
        Cont(x40584e)
    }
}

pub fn x40586a(ctx: &mut Context) -> Cont {
    // 0040586a add edi,280h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x280u32, &mut ctx.cpu.flags);
    // 00405870 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00405871 loop 00405848h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x405873)
    } else {
        Cont(x405848)
    }
}

pub fn x405873(ctx: &mut Context) -> Cont {
    // 00405873 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00405874 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00405875 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00405876 ret
    ret(ctx, 0)
}

pub fn x405880(ctx: &mut Context) -> Cont {
    // 00405880 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    Cont(x405883)
}

pub fn x405883(ctx: &mut Context) -> Cont {
    // 00405883 push 0
    push(ctx, 0x0u32);
    // 00405885 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x40588a, dst)
}

pub fn x40588a(ctx: &mut Context) -> Cont {
    // 0040588a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040588d call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x405892, dst)
}

pub fn x405892(ctx: &mut Context) -> Cont {
    // 00405892 fmul dword ptr ds:[420400h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420400u32) as f64,
    );
    // 00405898 fst dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    // 0040589c fcomp dword ptr ds:[420238h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420238u32) as f64));
    ctx.cpu.fpu.pop();
    // 004058a2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004058a4 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004058a7 je near ptr 00405939h
    je(ctx, Cont(x4058ad), Cont(x405939))
}

pub fn x4058ad(ctx: &mut Context) -> Cont {
    // 004058ad fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 004058b1 fcomp dword ptr ds:[42023Ch]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x42023cu32) as f64));
    ctx.cpu.fpu.pop();
    // 004058b7 fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 004058bb fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 004058bd test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 004058c0 je short 004058F0h
    je(ctx, Cont(x4058c2), Cont(x4058f0))
}

pub fn x4058c2(ctx: &mut Context) -> Cont {
    // 004058c2 fmul dword ptr ds:[4202A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202a4u32) as f64,
    );
    // 004058c8 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4058cd, dst)
}

pub fn x4058cd(ctx: &mut Context) -> Cont {
    // 004058cd fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 004058d1 fmul qword ptr ds:[4203F8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203f8u32));
    // 004058d7 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004058d8 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004058da fmul qword ptr ds:[4203F0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203f0u32));
    // 004058e0 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4058e5, dst)
}

pub fn x4058e5(ctx: &mut Context) -> Cont {
    // 004058e5 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004058e6 call 00405820h
    let dst = Cont(x405820);
    call(ctx, 0x4058eb, dst)
}

pub fn x4058eb(ctx: &mut Context) -> Cont {
    // 004058eb add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 004058ee jmp short 00405952h
    Cont(x405952)
}

pub fn x4058f0(ctx: &mut Context) -> Cont {
    // 004058f0 fsub dword ptr ds:[42023Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x42023cu32) as f64,
    );
    // 004058f6 fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004058fa fld dword ptr ds:[42023Ch]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x42023cu32) as f64);
    // 00405900 fsub dword ptr [esp+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as f64,
    );
    // 00405904 fmul dword ptr ds:[4202A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4202a4u32) as f64,
    );
    // 0040590a call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40590f, dst)
}

pub fn x40590f(ctx: &mut Context) -> Cont {
    // 0040590f fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 00405913 fmul qword ptr ds:[4203F8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203f8u32));
    // 00405919 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040591a fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 0040591c fmul qword ptr ds:[4203F0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4203f0u32));
    // 00405922 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405927, dst)
}

pub fn x405927(ctx: &mut Context) -> Cont {
    // 00405927 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405928 call 00405820h
    let dst = Cont(x405820);
    call(ctx, 0x40592d, dst)
}

pub fn x40592d(ctx: &mut Context) -> Cont {
    // 0040592d fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00405931 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405936, dst)
}

pub fn x405936(ctx: &mut Context) -> Cont {
    // 00405936 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405937 jmp short 0040594Ah
    Cont(x40594a)
}

pub fn x405939(ctx: &mut Context) -> Cont {
    // 00405939 push 0
    push(ctx, 0x0u32);
    // 0040593b push 8Ch
    push(ctx, 0x8cu32);
    // 00405940 call 00405820h
    let dst = Cont(x405820);
    call(ctx, 0x405945, dst)
}

pub fn x405945(ctx: &mut Context) -> Cont {
    // 00405945 push 0FFh
    push(ctx, 0xffu32);
    Cont(x40594a)
}

pub fn x40594a(ctx: &mut Context) -> Cont {
    // 0040594a call 004057C0h
    let dst = Cont(x4057c0);
    call(ctx, 0x40594f, dst)
}

pub fn x40594f(ctx: &mut Context) -> Cont {
    // 0040594f add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    Cont(x405952)
}

pub fn x405952(ctx: &mut Context) -> Cont {
    // 00405952 call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x405957, dst)
}

pub fn x405957(ctx: &mut Context) -> Cont {
    // 00405957 call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x40595c, dst)
}

pub fn x40595c(ctx: &mut Context) -> Cont {
    // 0040595c call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x405961, dst)
}

pub fn x405961(ctx: &mut Context) -> Cont {
    // 00405961 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405963 jne short 0040597Ah
    jne(ctx, Cont(x405965), Cont(x40597a))
}

pub fn x405965(ctx: &mut Context) -> Cont {
    // 00405965 fld dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.esp) as f64);
    // 00405969 fcomp dword ptr ds:[4203ECh]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4203ecu32) as f64));
    ctx.cpu.fpu.pop();
    // 0040596f fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405971 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00405974 jne near ptr 00405883h
    jne(ctx, Cont(x40597a), Cont(x405883))
}

pub fn x40597a(ctx: &mut Context) -> Cont {
    // 0040597a add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0040597d ret
    ret(ctx, 0)
}

pub fn x405980(ctx: &mut Context) -> Cont {
    // 00405980 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405981 mov eax,ds:[4210E4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x4210e4u32);
    // 00405986 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00405988 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040598a je short 0040599Ch
    je(ctx, Cont(x40598c), Cont(x40599c))
}

pub fn x40598c(ctx: &mut Context) -> Cont {
    // 0040598c mov eax,4210E4h
    ctx.cpu.regs.eax = 0x4210e4u32;
    Cont(x405991)
}

pub fn x405991(ctx: &mut Context) -> Cont {
    // 00405991 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 00405994 add eax,4
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x4u32, &mut ctx.cpu.flags);
    // 00405997 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00405998 test edx,edx
    and(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040599a jne short 00405991h
    jne(ctx, Cont(x40599c), Cont(x405991))
}

pub fn x40599c(ctx: &mut Context) -> Cont {
    // 0040599c mov eax,55555556h
    ctx.cpu.regs.eax = 0x55555556u32;
    // 004059a1 push 40B3C0h
    push(ctx, 0x40b3c0u32);
    // 004059a6 imul ecx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.ecx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 004059a8 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 004059aa shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 004059ad add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004059af mov ds:[429380h],edx
    ctx.memory.write::<u32>(0x429380u32, ctx.cpu.regs.edx);
    // 004059b5 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x4059ba, dst)
}

pub fn x4059ba(ctx: &mut Context) -> Cont {
    // 004059ba add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004059bd mov ds:[429384h],eax
    ctx.memory.write::<u32>(0x429384u32, ctx.cpu.regs.eax);
    // 004059c2 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004059c4 mov eax,428E80h
    ctx.cpu.regs.eax = 0x428e80u32;
    Cont(x4059c9)
}

pub fn x4059c9(ctx: &mut Context) -> Cont {
    // 004059c9 mov edx,ecx
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx;
    // 004059cb and edx,8000001Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x8000001fu32, &mut ctx.cpu.flags);
    // 004059d1 jns short 004059D8h
    jns(ctx, Cont(x4059d3), Cont(x4059d8))
}

pub fn x4059d3(ctx: &mut Context) -> Cont {
    // 004059d3 dec edx
    ctx.cpu.regs.edx = dec(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004059d4 or edx,0FFFFFFE0h
    ctx.cpu.regs.edx = or(ctx.cpu.regs.edx, 0xffffffe0u32, &mut ctx.cpu.flags);
    // 004059d7 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    Cont(x4059d8)
}

pub fn x4059d8(ctx: &mut Context) -> Cont {
    // 004059d8 mov [esp],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.edx);
    // 004059dc add eax,4
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x4u32, &mut ctx.cpu.flags);
    // 004059df fild dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<u32>(ctx.cpu.regs.esp) as i32 as f64);
    // 004059e3 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004059e4 cmp eax,429380h
    sub(ctx.cpu.regs.eax, 0x429380u32, &mut ctx.cpu.flags);
    // 004059e9 fmul dword ptr ds:[42018Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42018cu32) as f64,
    );
    // 004059ef fsubr dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x420128u32) as f64 - ctx.cpu.fpu.get(0),
    );
    // 004059f5 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004059f7 fmulp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) * ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004059f9 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004059fb fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 004059fd fstp dword ptr [eax-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405a00 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405a02 jl short 004059C9h
    jl(ctx, Cont(x405a04), Cont(x4059c9))
}

pub fn x405a04(ctx: &mut Context) -> Cont {
    // 00405a04 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00405a05 ret
    ret(ctx, 0)
}

pub fn x405a10(ctx: &mut Context) -> Cont {
    // 00405a10 sub esp,30h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00405a13 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405a14 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00405a15 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00405a16 push edi
    push(ctx, ctx.cpu.regs.edi);
    Cont(x405a17)
}

pub fn x405a17(ctx: &mut Context) -> Cont {
    // 00405a17 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x405a1c, dst)
}

pub fn x405a1c(ctx: &mut Context) -> Cont {
    // 00405a1c fmul dword ptr ds:[420228h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420228u32) as f64,
    );
    // 00405a22 push 0
    push(ctx, 0x0u32);
    // 00405a24 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405a28 call 00406C60h
    let dst = Cont(x406c60);
    call(ctx, 0x405a2d, dst)
}

pub fn x405a2d(ctx: &mut Context) -> Cont {
    // 00405a2d mov ebp,0FFFFFFE2h
    ctx.cpu.regs.ebp = 0xffffffe2u32;
    // 00405a32 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00405a35 mov dword ptr [esp+28h],0FFFFFFFAh
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), 0xfffffffau32);
    // 00405a3d mov [esp+1Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.ebp);
    Cont(x405a41)
}

pub fn x405a41(ctx: &mut Context) -> Cont {
    // 00405a41 fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 00405a45 mov edi,0FFFFFFE2h
    ctx.cpu.regs.edi = 0xffffffe2u32;
    // 00405a4a mov dword ptr [esp+20h],0FFFFFFFAh
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0xfffffffau32);
    // 00405a52 mov [esp+1Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.edi);
    // 00405a56 fadd dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00405a5a fmul qword ptr ds:[4200D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4200d8u32));
    // 00405a60 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405a62 fmul qword ptr ds:[420168h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420168u32));
    // 00405a68 fstp qword ptr [esp+38h]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405a6c fild dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as i32 as f64,
    );
    // 00405a70 fmul dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 00405a76 fsub dword ptr ds:[4200A8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200a8u32) as f64,
    );
    // 00405a7c fstp dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405a80 mov ebx,[esp+30h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    Cont(x405a84)
}

pub fn x405a84(ctx: &mut Context) -> Cont {
    // 00405a84 fild dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as i32 as f64,
    );
    // 00405a88 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405a89 lea eax,[esp+14h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x14u32);
    // 00405a8d lea ecx,[esp+18h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x18u32);
    // 00405a91 fsub dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405a95 fmul qword ptr ds:[420438h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420438u32));
    // 00405a9b fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00405a9d fmul qword ptr ds:[420168h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420168u32));
    // 00405aa3 fadd dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405aa7 fadd qword ptr [esp+3Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x3cu32)),
    );
    // 00405aab fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00405aae fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00405ab2 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405ab3 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405ab4 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405ab5 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405ab6 fmul dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 00405abc fsub dword ptr ds:[4200A8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200a8u32) as f64,
    );
    // 00405ac2 fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00405ac5 call 00407E40h
    let dst = Cont(x407e40);
    call(ctx, 0x405aca, dst)
}

pub fn x405aca(ctx: &mut Context) -> Cont {
    // 00405aca fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405ace fadd dword ptr ds:[420430h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420430u32) as f64,
    );
    // 00405ad4 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00405ad7 fstp dword ptr [esp+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405adb fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405adf fadd dword ptr ds:[420240h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420240u32) as f64,
    );
    // 00405ae5 fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405ae9 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00405aed fmul dword ptr ds:[42042Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42042cu32) as f64,
    );
    // 00405af3 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405af8, dst)
}

pub fn x405af8(ctx: &mut Context) -> Cont {
    // 00405af8 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00405afa test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00405afc jge short 00405B02h
    jge(ctx, Cont(x405afe), Cont(x405b02))
}

pub fn x405afe(ctx: &mut Context) -> Cont {
    // 00405afe xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00405b00 jmp short 00405B0Ch
    Cont(x405b0c)
}

pub fn x405b02(ctx: &mut Context) -> Cont {
    // 00405b02 cmp esi,1Fh
    sub(ctx.cpu.regs.esi, 0x1fu32, &mut ctx.cpu.flags);
    // 00405b05 jle short 00405B0Ch
    jle(ctx, Cont(x405b07), Cont(x405b0c))
}

pub fn x405b07(ctx: &mut Context) -> Cont {
    // 00405b07 mov esi,1Fh
    ctx.cpu.regs.esi = 0x1fu32;
    Cont(x405b0c)
}

pub fn x405b0c(ctx: &mut Context) -> Cont {
    // 00405b0c fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405b10 fmul dword ptr ds:[42042Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42042cu32) as f64,
    );
    // 00405b16 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405b1b, dst)
}

pub fn x405b1b(ctx: &mut Context) -> Cont {
    // 00405b1b test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405b1d jge short 00405B23h
    jge(ctx, Cont(x405b1f), Cont(x405b23))
}

pub fn x405b1f(ctx: &mut Context) -> Cont {
    // 00405b1f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405b21 jmp short 00405B2Dh
    Cont(x405b2d)
}

pub fn x405b23(ctx: &mut Context) -> Cont {
    // 00405b23 cmp eax,9
    sub(ctx.cpu.regs.eax, 0x9u32, &mut ctx.cpu.flags);
    // 00405b26 jle short 00405B2Dh
    jle(ctx, Cont(x405b28), Cont(x405b2d))
}

pub fn x405b28(ctx: &mut Context) -> Cont {
    // 00405b28 mov eax,9
    ctx.cpu.regs.eax = 0x9u32;
    Cont(x405b2d)
}

pub fn x405b2d(ctx: &mut Context) -> Cont {
    // 00405b2d shl eax,5
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x5u8, &mut ctx.cpu.flags);
    // 00405b30 add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00405b32 fld dword ptr [eax*4+428E80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.eax * 4).wrapping_add(0x428e80u32)) as f64,
    );
    // 00405b39 fmul dword ptr ds:[420428h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420428u32) as f64,
    );
    // 00405b3f fst dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00405b43 fcomp dword ptr ds:[420224h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420224u32) as f64));
    ctx.cpu.fpu.pop();
    // 00405b49 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405b4b test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405b4e jne short 00405B5Ah
    jne(ctx, Cont(x405b50), Cont(x405b5a))
}

pub fn x405b50(ctx: &mut Context) -> Cont {
    // 00405b50 mov dword ptr [esp+1Ch],41A00000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), 0x41a00000u32);
    // 00405b58 jmp short 00405B6Bh
    Cont(x405b6b)
}

pub fn x405b5a(ctx: &mut Context) -> Cont {
    // 00405b5a fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405b5e fcomp dword ptr ds:[420128h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420128u32) as f64));
    ctx.cpu.fpu.pop();
    // 00405b64 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405b66 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405b69 jne short 00405BE2h
    jne(ctx, Cont(x405b6b), Cont(x405be2))
}

pub fn x405b6b(ctx: &mut Context) -> Cont {
    // 00405b6b fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00405b6f fcomp dword ptr ds:[420424h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420424u32) as f64));
    ctx.cpu.fpu.pop();
    // 00405b75 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405b77 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00405b7a jne short 00405BE2h
    jne(ctx, Cont(x405b7c), Cont(x405be2))
}

pub fn x405b7c(ctx: &mut Context) -> Cont {
    // 00405b7c fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405b80 fcomp dword ptr ds:[420424h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420424u32) as f64));
    ctx.cpu.fpu.pop();
    // 00405b86 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405b88 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00405b8b jne short 00405BE2h
    jne(ctx, Cont(x405b8d), Cont(x405be2))
}

pub fn x405b8d(ctx: &mut Context) -> Cont {
    // 00405b8d fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 00405b91 fcomp dword ptr ds:[420420h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420420u32) as f64));
    ctx.cpu.fpu.pop();
    // 00405b97 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405b99 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405b9c je short 00405BE2h
    je(ctx, Cont(x405b9e), Cont(x405be2))
}

pub fn x405b9e(ctx: &mut Context) -> Cont {
    // 00405b9e fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405ba2 fcomp dword ptr ds:[42041Ch]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x42041cu32) as f64));
    ctx.cpu.fpu.pop();
    // 00405ba8 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405baa test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405bad je short 00405BE2h
    je(ctx, Cont(x405baf), Cont(x405be2))
}

pub fn x405baf(ctx: &mut Context) -> Cont {
    // 00405baf fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405bb3 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405bb8, dst)
}

pub fn x405bb8(ctx: &mut Context) -> Cont {
    // 00405bb8 mov edx,ds:[429384h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x429384u32);
    // 00405bbe fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405bc2 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405bc3 push 0C8h
    push(ctx, 0xc8u32);
    // 00405bc8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405bc9 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405bca call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405bcf, dst)
}

pub fn x405bcf(ctx: &mut Context) -> Cont {
    // 00405bcf fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405bd3 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405bd4 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405bd9, dst)
}

pub fn x405bd9(ctx: &mut Context) -> Cont {
    // 00405bd9 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405bda call 00406D70h
    let dst = Cont(x406d70);
    call(ctx, 0x405bdf, dst)
}

pub fn x405bdf(ctx: &mut Context) -> Cont {
    // 00405bdf add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    Cont(x405be2)
}

pub fn x405be2(ctx: &mut Context) -> Cont {
    // 00405be2 mov edx,[esp+20h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00405be6 add edi,5
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x5u32, &mut ctx.cpu.flags);
    // 00405be9 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00405bea cmp edi,1Eh
    sub(ctx.cpu.regs.edi, 0x1eu32, &mut ctx.cpu.flags);
    // 00405bed mov [esp+20h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.edx);
    // 00405bf1 mov [esp+1Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.edi);
    // 00405bf5 jle near ptr 00405A84h
    jle(ctx, Cont(x405bfb), Cont(x405a84))
}

pub fn x405bfb(ctx: &mut Context) -> Cont {
    // 00405bfb mov edx,[esp+28h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00405bff add ebp,5
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x5u32, &mut ctx.cpu.flags);
    // 00405c02 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00405c03 cmp ebp,1Eh
    sub(ctx.cpu.regs.ebp, 0x1eu32, &mut ctx.cpu.flags);
    // 00405c06 mov [esp+28h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.edx);
    // 00405c0a mov [esp+1Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.ebp);
    // 00405c0e jle near ptr 00405A41h
    jle(ctx, Cont(x405c14), Cont(x405a41))
}

pub fn x405c14(ctx: &mut Context) -> Cont {
    // 00405c14 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00405c18 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405c1d, dst)
}

pub fn x405c1d(ctx: &mut Context) -> Cont {
    // 00405c1d mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00405c1f sar esi,7
    ctx.cpu.regs.esi = sar(ctx.cpu.regs.esi, 0x7u8, &mut ctx.cpu.flags);
    // 00405c22 sub esi,2
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0x2u32, &mut ctx.cpu.flags);
    // 00405c25 js near ptr 00405DE7h
    js(ctx, Cont(x405c2b), Cont(x405de7))
}

pub fn x405c2b(ctx: &mut Context) -> Cont {
    // 00405c2b cmp esi,ds:[429380h]
    sub(
        ctx.cpu.regs.esi,
        ctx.memory.read::<u32>(0x429380u32),
        &mut ctx.cpu.flags,
    );
    // 00405c31 jge near ptr 00405DE7h
    jge(ctx, Cont(x405c37), Cont(x405de7))
}

pub fn x405c37(ctx: &mut Context) -> Cont {
    // 00405c37 and eax,8000007Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x8000007fu32, &mut ctx.cpu.flags);
    // 00405c3c jns short 00405C43h
    jns(ctx, Cont(x405c3e), Cont(x405c43))
}

pub fn x405c3e(ctx: &mut Context) -> Cont {
    // 00405c3e dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405c3f or eax,0FFFFFF80h
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, 0xffffff80u32, &mut ctx.cpu.flags);
    // 00405c42 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x405c43)
}

pub fn x405c43(ctx: &mut Context) -> Cont {
    // 00405c43 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405c44 mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 00405c48 fild dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as i32 as f64,
    );
    // 00405c4c fdivr dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x420188u32) as f64 / ctx.cpu.fpu.get(0),
    );
    // 00405c52 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00405c58 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00405c5a fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 00405c5c fadd dword ptr ds:[420418h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420418u32) as f64,
    );
    // 00405c62 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405c67, dst)
}

pub fn x405c67(ctx: &mut Context) -> Cont {
    // 00405c67 fld dword ptr ds:[420418h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420418u32) as f64);
    // 00405c6d fsub st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) - ctx.cpu.fpu.get(1));
    // 00405c6f mov [esp+28h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.eax);
    // 00405c73 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405c78, dst)
}

pub fn x405c78(ctx: &mut Context) -> Cont {
    // 00405c78 fadd dword ptr ds:[420188h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420188u32) as f64,
    );
    // 00405c7e lea edi,[esi+esi*2]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add((ctx.cpu.regs.esi * 2));
    // 00405c81 mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 00405c85 shl edi,2
    ctx.cpu.regs.edi = shl(ctx.cpu.regs.edi, 0x2u8, &mut ctx.cpu.flags);
    // 00405c88 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405c8d, dst)
}

pub fn x405c8d(ctx: &mut Context) -> Cont {
    // 00405c8d mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00405c8f mov eax,[edi+4210E4h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4210e4u32));
    // 00405c95 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405c96 lea eax,[esi+2]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x2u32);
    // 00405c99 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405c9a fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405c9e mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405ca3 push 0FFh
    push(ctx, 0xffu32);
    // 00405ca8 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405caa fmul qword ptr ds:[420410h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420410u32));
    // 00405cb0 fst qword ptr [esp+28h]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.fpu.get(0));
    // 00405cb4 fild dword ptr [esp+38h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as i32 as f64,
    );
    // 00405cb8 fstp qword ptr [esp+38h]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405cbc fild dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as i32 as f64,
    );
    // 00405cc0 fstp qword ptr [esp+30h]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405cc4 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405cc5 fld dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405ccc fdivr st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    // 00405cce fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00405cd0 fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405cd6 fadd qword ptr [esp+3Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x3cu32)),
    );
    // 00405cda call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405cdf, dst)
}

pub fn x405cdf(ctx: &mut Context) -> Cont {
    // 00405cdf push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405ce0 mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 00405ce2 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405ce3 mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405ce8 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405cea fdiv dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            / ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405cf1 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405cf3 fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405cf9 fadd qword ptr [esp+38h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x38u32)),
    );
    // 00405cfd call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405d02, dst)
}

pub fn x405d02(ctx: &mut Context) -> Cont {
    // 00405d02 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405d03 push 32h
    push(ctx, 0x32u32);
    // 00405d05 push 190h
    push(ctx, 0x190u32);
    // 00405d0a call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x405d0f, dst)
}

pub fn x405d0f(ctx: &mut Context) -> Cont {
    // 00405d0f mov edx,[edi+4210E8h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4210e8u32));
    // 00405d15 lea eax,[esi+6]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x6u32);
    // 00405d18 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405d19 mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405d1e cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405d1f idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405d21 push 0FFh
    push(ctx, 0xffu32);
    // 00405d26 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405d27 fld dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405d2e fdivr qword ptr [esp+48h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x48u32))
            / ctx.cpu.fpu.get(0),
    );
    // 00405d32 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405d34 fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405d3a fadd qword ptr [esp+58h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x58u32)),
    );
    // 00405d3e call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405d43, dst)
}

pub fn x405d43(ctx: &mut Context) -> Cont {
    // 00405d43 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405d44 lea eax,[esi+3]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x3u32);
    // 00405d47 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405d48 mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405d4d idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405d4f fld dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405d56 fdivr qword ptr [esp+4Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x4cu32))
            / ctx.cpu.fpu.get(0),
    );
    // 00405d5a fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00405d5c fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405d62 fadd qword ptr [esp+54h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x54u32)),
    );
    // 00405d66 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405d6b, dst)
}

pub fn x405d6b(ctx: &mut Context) -> Cont {
    // 00405d6b push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405d6c push 64h
    push(ctx, 0x64u32);
    // 00405d6e push 17Ch
    push(ctx, 0x17cu32);
    // 00405d73 call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x405d78, dst)
}

pub fn x405d78(ctx: &mut Context) -> Cont {
    // 00405d78 mov edx,[edi+4210ECh]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4210ecu32));
    // 00405d7e lea eax,[esi+4]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x4u32);
    // 00405d81 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405d82 mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405d87 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405d88 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405d8a push 0FFh
    push(ctx, 0xffu32);
    // 00405d8f push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405d90 fld dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405d97 fdivr qword ptr [esp+64h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x64u32))
            / ctx.cpu.fpu.get(0),
    );
    // 00405d9b fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00405d9d fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405da3 fadd qword ptr [esp+74h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x74u32)),
    );
    // 00405da7 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405dac, dst)
}

pub fn x405dac(ctx: &mut Context) -> Cont {
    // 00405dac push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405dad lea eax,[esi+5]
    ctx.cpu.regs.eax = ctx.cpu.regs.esi.wrapping_add(0x5u32);
    // 00405db0 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00405db1 mov ecx,0Ah
    ctx.cpu.regs.ecx = 0xau32;
    // 00405db6 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00405db8 fld dword ptr [edx*4+4210BCh]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x4210bcu32)) as f64,
    );
    // 00405dbf fdivr qword ptr [esp+68h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x68u32))
            / ctx.cpu.fpu.get(0),
    );
    // 00405dc3 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00405dc5 fmul qword ptr ds:[420408h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420408u32));
    // 00405dcb fadd qword ptr [esp+70h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x70u32)),
    );
    // 00405dcf call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x405dd4, dst)
}

pub fn x405dd4(ctx: &mut Context) -> Cont {
    // 00405dd4 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405dd5 push 96h
    push(ctx, 0x96u32);
    // 00405dda push 168h
    push(ctx, 0x168u32);
    // 00405ddf call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x405de4, dst)
}

pub fn x405de4(ctx: &mut Context) -> Cont {
    // 00405de4 add esp,54h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x54u32, &mut ctx.cpu.flags);
    Cont(x405de7)
}

pub fn x405de7(ctx: &mut Context) -> Cont {
    // 00405de7 call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x405dec, dst)
}

pub fn x405dec(ctx: &mut Context) -> Cont {
    // 00405dec call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x405df1, dst)
}

pub fn x405df1(ctx: &mut Context) -> Cont {
    // 00405df1 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x405df6, dst)
}

pub fn x405df6(ctx: &mut Context) -> Cont {
    // 00405df6 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00405df8 jne short 00405E09h
    jne(ctx, Cont(x405dfa), Cont(x405e09))
}

pub fn x405dfa(ctx: &mut Context) -> Cont {
    // 00405dfa mov edx,ds:[429380h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x429380u32);
    // 00405e00 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00405e01 cmp esi,edx
    sub(ctx.cpu.regs.esi, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00405e03 jl near ptr 00405A17h
    jl(ctx, Cont(x405e09), Cont(x405a17))
}

pub fn x405e09(ctx: &mut Context) -> Cont {
    // 00405e09 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00405e0a pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00405e0b pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00405e0c pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00405e0d add esp,30h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x30u32, &mut ctx.cpu.flags);
    // 00405e10 ret
    ret(ctx, 0)
}

pub fn x405e20(ctx: &mut Context) -> Cont {
    // 00405e20 sub esp,44h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x44u32, &mut ctx.cpu.flags);
    // 00405e23 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00405e24 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00405e25 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00405e26 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00405e27 mov dword ptr [esp+14h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), 0x0u32);
    // 00405e2f mov ebx,425C28h
    ctx.cpu.regs.ebx = 0x425c28u32;
    // 00405e34 mov ebp,400h
    ctx.cpu.regs.ebp = 0x400u32;
    Cont(x405e39)
}

pub fn x405e39(ctx: &mut Context) -> Cont {
    // 00405e39 fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 00405e3d xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00405e3f mov esi,ebx
    ctx.cpu.regs.esi = ctx.cpu.regs.ebx;
    // 00405e41 mov [esp+1Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.edi);
    // 00405e45 fsubr dword ptr ds:[420468h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x420468u32) as f64 - ctx.cpu.fpu.get(0),
    );
    // 00405e4b fmul dword ptr ds:[42042Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42042cu32) as f64,
    );
    // 00405e51 fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    Cont(x405e55)
}

pub fn x405e55(ctx: &mut Context) -> Cont {
    // 00405e55 mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 00405e59 lea ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x24u32);
    // 00405e5d push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405e5e push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405e5f mov dword ptr [esp+2Ch],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), 0x0u32);
    // 00405e67 mov dword ptr [esp+30h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), 0x0u32);
    // 00405e6f mov dword ptr [esp+34h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), 0x0u32);
    // 00405e77 call 004065B0h
    let dst = Cont(x4065b0);
    call(ctx, 0x405e7c, dst)
}

pub fn x405e7c(ctx: &mut Context) -> Cont {
    // 00405e7c fild dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as i32 as f64,
    );
    // 00405e80 mov eax,[esp+60h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 00405e84 mov edx,[esp+28h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00405e88 lea ecx,[esp+44h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x44u32);
    // 00405e8c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00405e8d fsub dword ptr ds:[4200A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x4200a4u32) as f64,
    );
    // 00405e93 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00405e94 mov [esp+50h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32), ctx.cpu.regs.edx);
    // 00405e98 mov dword ptr [esp+54h],3F800000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32), 0x3f800000u32);
    // 00405ea0 fmul dword ptr ds:[42042Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42042cu32) as f64,
    );
    // 00405ea6 fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405eaa call 00406550h
    let dst = Cont(x406550);
    call(ctx, 0x405eaf, dst)
}

pub fn x405eaf(ctx: &mut Context) -> Cont {
    // 00405eaf mov edx,[esp+4Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 00405eb3 mov eax,[esp+50h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32));
    // 00405eb7 mov ecx,[esp+54h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 00405ebb mov [esp+40h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32), ctx.cpu.regs.edx);
    // 00405ebf lea edx,[esp+40h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x40u32);
    // 00405ec3 mov [esp+44h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32), ctx.cpu.regs.eax);
    // 00405ec7 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00405ec8 mov [esp+4Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32), ctx.cpu.regs.ecx);
    // 00405ecc call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x405ed1, dst)
}

pub fn x405ed1(ctx: &mut Context) -> Cont {
    // 00405ed1 fld dword ptr [esp+48h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x48u32)) as f64,
    );
    // 00405ed5 fmul dword ptr [esp+48h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x48u32)) as f64,
    );
    // 00405ed9 fld dword ptr [esp+44h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 00405edd fmul dword ptr [esp+44h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 00405ee1 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 00405ee4 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405ee6 fld dword ptr [esp+34h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x34u32)) as f64,
    );
    // 00405eea fmul dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405eee fld dword ptr [esp+30h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00405ef2 fmul dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405ef6 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405ef8 fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 00405efa fst dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00405efe fmul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405f02 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405f06 fmul dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405f0a fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405f0e fmul dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405f12 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405f14 fsub dword ptr ds:[420464h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(0x420464u32) as f64,
    );
    // 00405f1a fmul st,st(2)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(2));
    // 00405f1c fmul dword ptr ds:[420270h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420270u32) as f64,
    );
    // 00405f22 fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405f24 fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 00405f26 fstp dword ptr [esp+18h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x18u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405f2a fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 00405f2c fadd dword ptr ds:[420460h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420460u32) as f64,
    );
    // 00405f32 fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405f36 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00405f3a fsub dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405f3e fdiv dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            / ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405f42 fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00405f48 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405f4a test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 00405f4d je short 00405F5Fh
    je(ctx, Cont(x405f4f), Cont(x405f5f))
}

pub fn x405f4f(ctx: &mut Context) -> Cont {
    // 00405f4f fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405f51 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00405f55 fchs
    ctx.cpu.fpu.set(0, -ctx.cpu.fpu.get(0));
    // 00405f57 fsub dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00405f5b fdiv dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            / ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    Cont(x405f5f)
}

pub fn x405f5f(ctx: &mut Context) -> Cont {
    // 00405f5f fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00405f61 fmul dword ptr [esp+30h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x30u32)) as f64,
    );
    // 00405f65 fadd dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00405f69 fstp dword ptr [esp+48h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x48u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405f6d fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00405f6f fmul dword ptr [esp+34h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x34u32)) as f64,
    );
    // 00405f73 fadd dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00405f77 fstp dword ptr [esp+4Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00405f7b fmul dword ptr [esp+38h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as f64,
    );
    // 00405f7f fadd dword ptr [esp+2Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00405f83 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00405f85 fmul dword ptr ds:[42045Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x42045cu32) as f64,
    );
    // 00405f8b fadd dword ptr ds:[420458h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420458u32) as f64,
    );
    // 00405f91 fabs
    // Fabs not implemented
    todo!();
}

pub fn x405fd2(ctx: &mut Context) -> Cont {
    // 00405fd2 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405fd4 fld dword ptr ds:[420444h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420444u32) as f64);
    // 00405fda jmp short 00405FF1h
    Cont(x405ff1)
}

pub fn x405fdc(ctx: &mut Context) -> Cont {
    // 00405fdc fcom dword ptr ds:[420098h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x420098u32) as f64));
    // 00405fe2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00405fe4 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00405fe7 je short 00405FF1h
    je(ctx, Cont(x405fe9), Cont(x405ff1))
}

pub fn x405fe9(ctx: &mut Context) -> Cont {
    // 00405fe9 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00405feb fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    Cont(x405ff1)
}

pub fn x405ff1(ctx: &mut Context) -> Cont {
    // 00405ff1 fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00405ff5 fmul dword ptr ds:[420440h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420440u32) as f64,
    );
    // 00405ffb call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406000, dst)
}

pub fn x406000(ctx: &mut Context) -> Cont {
    // 00406000 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00406004 fmul dword ptr ds:[420440h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420440u32) as f64,
    );
    // 0040600a mov [esi-8],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.eax,
    );
    // 0040600d call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406012, dst)
}

pub fn x406012(ctx: &mut Context) -> Cont {
    // 00406012 fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 00406014 mov [esi-4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.eax,
    );
    // 00406017 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40601c, dst)
}

pub fn x40601c(ctx: &mut Context) -> Cont {
    // 0040601c mov [esi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, ctx.cpu.regs.eax);
    // 0040601e mov [esi+4],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x4u32), ctx.cpu.regs.ebp);
    // 00406021 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406022 add esi,10h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x10u32, &mut ctx.cpu.flags);
    // 00406025 cmp edi,20h
    sub(ctx.cpu.regs.edi, 0x20u32, &mut ctx.cpu.flags);
    // 00406028 mov [esp+1Ch],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.edi);
    // 0040602c jle near ptr 00405E55h
    jle(ctx, Cont(x406032), Cont(x405e55))
}

pub fn x406032(ctx: &mut Context) -> Cont {
    // 00406032 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00406036 add ebx,ebp
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00406038 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00406039 cmp ebx,428428h
    sub(ctx.cpu.regs.ebx, 0x428428u32, &mut ctx.cpu.flags);
    // 0040603f mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00406043 jle near ptr 00405E39h
    jle(ctx, Cont(x406049), Cont(x405e39))
}

pub fn x406049(ctx: &mut Context) -> Cont {
    // 00406049 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040604a pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040604b pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040604c pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040604d add esp,44h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x44u32, &mut ctx.cpu.flags);
    // 00406050 ret
    ret(ctx, 0)
}

pub fn x406060(ctx: &mut Context) -> Cont {
    // 00406060 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406061 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406062 mov esi,428D80h
    ctx.cpu.regs.esi = 0x428d80u32;
    Cont(x406067)
}

pub fn x406067(ctx: &mut Context) -> Cont {
    // 00406067 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x40606c, dst)
}

pub fn x40606c(ctx: &mut Context) -> Cont {
    // 0040606c cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040606d mov ecx,320h
    ctx.cpu.regs.ecx = 0x320u32;
    // 00406072 add esi,4
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x4u32, &mut ctx.cpu.flags);
    // 00406075 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00406077 sub edx,96h
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, 0x96u32, &mut ctx.cpu.flags);
    // 0040607d cmp esi,428E80h
    sub(ctx.cpu.regs.esi, 0x428e80u32, &mut ctx.cpu.flags);
    // 00406083 mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00406087 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 0040608b fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040608e jl short 00406067h
    jl(ctx, Cont(x406090), Cont(x406067))
}

pub fn x406090(ctx: &mut Context) -> Cont {
    // 00406090 push 40A040h
    push(ctx, 0x40a040u32);
    // 00406095 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x40609a, dst)
}

pub fn x40609a(ctx: &mut Context) -> Cont {
    // 0040609a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040609d mov ds:[428D78h],eax
    ctx.memory.write::<u32>(0x428d78u32, ctx.cpu.regs.eax);
    // 004060a2 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004060a3 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 004060a4 ret
    ret(ctx, 0)
}

pub fn x4060b0(ctx: &mut Context) -> Cont {
    // 004060b0 sub esp,50h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 004060b3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004060b4 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004060b5 mov dword ptr [esp+1Ch],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), 0x0u32);
    // 004060bd mov edi,[esp+1Ch]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    Cont(x4060c1)
}

pub fn x4060c1(ctx: &mut Context) -> Cont {
    // 004060c1 call 00407EC0h
    let dst = Cont(x407ec0);
    call(ctx, 0x4060c6, dst)
}

pub fn x4060c6(ctx: &mut Context) -> Cont {
    // 004060c6 fmul dword ptr ds:[4203E8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4203e8u32) as f64,
    );
    // 004060cc call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4060d1, dst)
}

pub fn x4060d1(ctx: &mut Context) -> Cont {
    // 004060d1 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 004060d3 sar eax,7
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 004060d6 mov [esp+8],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32), ctx.cpu.regs.esi);
    // 004060da and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 004060dd fild dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as i32 as f64,
    );
    // 004060e1 lea ecx,[eax+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 004060e4 lea edx,[eax+5]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x5u32);
    // 004060e7 and ecx,3Fh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3fu32, &mut ctx.cpu.flags);
    // 004060ea and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 004060ed fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004060ef fadd dword ptr [ecx*4+428D80h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>((ctx.cpu.regs.ecx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 004060f6 lea ecx,[eax+8]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x8u32);
    // 004060f9 and ecx,3Fh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3fu32, &mut ctx.cpu.flags);
    // 004060fc fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00406102 fld dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 00406109 fadd dword ptr ds:[420498h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420498u32) as f64,
    );
    // 0040610f lea edx,[eax+3]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x3u32);
    // 00406112 lea ecx,[ecx*4+428D80h]
    ctx.cpu.regs.ecx = (ctx.cpu.regs.ecx * 4).wrapping_add(0x428d80u32);
    // 00406119 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 0040611c fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040611e fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00406120 fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 00406126 fstp dword ptr [esp+28h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x28u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040612a fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0040612c fadd dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 00406133 lea edx,[eax+4]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x4u32);
    // 00406136 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 00406139 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 0040613f fld dword ptr [ecx]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64);
    // 00406141 fadd dword ptr ds:[420124h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420124u32) as f64,
    );
    // 00406147 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406149 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 0040614b fmul qword ptr ds:[4202D8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202d8u32));
    // 00406151 fstp dword ptr [esp+2Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x2cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406155 fild dword ptr [esp+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as i32 as f64,
    );
    // 00406159 fstp qword ptr [esp+8]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp.wrapping_add(0x8u32), ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040615d fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0040615f fadd dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 00406166 lea edx,[eax+0Ah]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0xau32);
    // 00406169 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 0040616c fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00406172 fld dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 00406179 fadd dword ptr ds:[420494h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420494u32) as f64,
    );
    // 0040617f lea edx,[eax+7]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x7u32);
    // 00406182 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 00406185 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406187 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00406189 fmul qword ptr ds:[420258h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420258u32));
    // 0040618f fadd qword ptr [esp+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x8u32)),
    );
    // 00406193 fstp dword ptr [esp+30h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x30u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406197 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00406199 fadd dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 004061a0 lea edx,[eax+0Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0xcu32);
    // 004061a3 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 004061a6 fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 004061ac fld dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 004061b3 fadd dword ptr ds:[420490h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420490u32) as f64,
    );
    // 004061b9 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004061bb fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 004061bd fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 004061bf fstp dword ptr [esp+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004061c3 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004061c5 fadd dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 004061c7 lea ecx,[eax+0Bh]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0xbu32);
    // 004061ca and ecx,3Fh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3fu32, &mut ctx.cpu.flags);
    // 004061cd fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 004061d3 fld dword ptr [ecx*4+428D80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.ecx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 004061da fadd dword ptr ds:[42048Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x42048cu32) as f64,
    );
    // 004061e0 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004061e2 lea edx,[eax+2]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add(0x2u32);
    // 004061e5 add eax,0Dh
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0xdu32, &mut ctx.cpu.flags);
    // 004061e8 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 004061eb and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 004061ee fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 004061f0 fadd st(0),st
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) + ctx.cpu.fpu.get(0));
    // 004061f2 fstp dword ptr [esp+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004061f6 fadd dword ptr [edx*4+428D80h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 004061fd fmul qword ptr ds:[4202B0h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4202b0u32));
    // 00406203 fld dword ptr [eax*4+428D80h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>((ctx.cpu.regs.eax * 4).wrapping_add(0x428d80u32)) as f64,
    );
    // 0040620a fadd dword ptr ds:[420488h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420488u32) as f64,
    );
    // 00406210 lea eax,[esp+10h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x10u32);
    // 00406214 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406215 fdivp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) / ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406217 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00406219 fmul qword ptr ds:[420480h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420480u32));
    // 0040621f fstp dword ptr [esp+1Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x1cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406223 call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x406228, dst)
}

pub fn x406228(ctx: &mut Context) -> Cont {
    // 00406228 fld qword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0xcu32)),
    );
    // 0040622c fmul qword ptr ds:[420478h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420478u32));
    // 00406232 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00406235 lea ecx,[esp+34h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x34u32);
    // 00406239 fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 0040623b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040623c sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040623f mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00406241 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00406244 mov [edx],edi
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.edi);
    // 00406246 fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040624a fld qword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f64>(ctx.cpu.regs.esp.wrapping_add(0x24u32)),
    );
    // 0040624e fmul qword ptr ds:[420470h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x420470u32));
    // 00406254 mov eax,[esp+3Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00406258 mov [edx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 0040625b mov eax,[esp+2Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 0040625f fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00406261 fstp dword ptr [esp+40h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x40u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406265 mov ecx,[esp+40h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00406269 mov [edx+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040626c mov ecx,[esp+30h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 00406270 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 00406272 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 00406274 mov eax,[esp+34h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00406278 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040627b mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0040627e call 00406410h
    let dst = Cont(x406410);
    call(ctx, 0x406283, dst)
}

pub fn x406283(ctx: &mut Context) -> Cont {
    // 00406283 lea ecx,[esp+44h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x44u32);
    // 00406287 lea edx,[esp+50h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x50u32);
    // 0040628b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040628c push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040628d call 00405E20h
    let dst = Cont(x405e20);
    call(ctx, 0x406292, dst)
}

pub fn x406292(ctx: &mut Context) -> Cont {
    // 00406292 mov eax,ds:[428D78h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428d78u32);
    // 00406297 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406298 call 00406E80h
    let dst = Cont(x406e80);
    call(ctx, 0x40629d, dst)
}

pub fn x40629d(ctx: &mut Context) -> Cont {
    // 0040629d add esp,28h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x28u32, &mut ctx.cpu.flags);
    // 004062a0 cmp esi,40h
    sub(ctx.cpu.regs.esi, 0x40u32, &mut ctx.cpu.flags);
    // 004062a3 jl short 004062B4h
    jl(ctx, Cont(x4062a5), Cont(x4062b4))
}

pub fn x4062a5(ctx: &mut Context) -> Cont {
    // 004062a5 cmp esi,80h
    sub(ctx.cpu.regs.esi, 0x80u32, &mut ctx.cpu.flags);
    // 004062ab jg short 004062BCh
    jg(ctx, Cont(x4062ad), Cont(x4062bc))
}

pub fn x4062ad(ctx: &mut Context) -> Cont {
    // 004062ad push 421290h
    push(ctx, 0x421290u32);
    // 004062b2 jmp short 004062F7h
    Cont(x4062f7)
}

pub fn x4062b4(ctx: &mut Context) -> Cont {
    // 004062b4 cmp esi,80h
    sub(ctx.cpu.regs.esi, 0x80u32, &mut ctx.cpu.flags);
    // 004062ba jl short 004062CBh
    jl(ctx, Cont(x4062bc), Cont(x4062cb))
}

pub fn x4062bc(ctx: &mut Context) -> Cont {
    // 004062bc cmp esi,100h
    sub(ctx.cpu.regs.esi, 0x100u32, &mut ctx.cpu.flags);
    // 004062c2 jge short 004062D3h
    jge(ctx, Cont(x4062c4), Cont(x4062d3))
}

pub fn x4062c4(ctx: &mut Context) -> Cont {
    // 004062c4 push 421278h
    push(ctx, 0x421278u32);
    // 004062c9 jmp short 004062F7h
    Cont(x4062f7)
}

pub fn x4062cb(ctx: &mut Context) -> Cont {
    // 004062cb cmp esi,100h
    sub(ctx.cpu.regs.esi, 0x100u32, &mut ctx.cpu.flags);
    // 004062d1 jl short 004062E2h
    jl(ctx, Cont(x4062d3), Cont(x4062e2))
}

pub fn x4062d3(ctx: &mut Context) -> Cont {
    // 004062d3 cmp esi,180h
    sub(ctx.cpu.regs.esi, 0x180u32, &mut ctx.cpu.flags);
    // 004062d9 jge short 004062EAh
    jge(ctx, Cont(x4062db), Cont(x4062ea))
}

pub fn x4062db(ctx: &mut Context) -> Cont {
    // 004062db push 421260h
    push(ctx, 0x421260u32);
    // 004062e0 jmp short 004062F7h
    Cont(x4062f7)
}

pub fn x4062e2(ctx: &mut Context) -> Cont {
    // 004062e2 cmp esi,180h
    sub(ctx.cpu.regs.esi, 0x180u32, &mut ctx.cpu.flags);
    // 004062e8 jl short 00406311h
    jl(ctx, Cont(x4062ea), Cont(x406311))
}

pub fn x4062ea(ctx: &mut Context) -> Cont {
    // 004062ea cmp esi,1E0h
    sub(ctx.cpu.regs.esi, 0x1e0u32, &mut ctx.cpu.flags);
    // 004062f0 jge short 00406311h
    jge(ctx, Cont(x4062f2), Cont(x406311))
}

pub fn x4062f2(ctx: &mut Context) -> Cont {
    // 004062f2 push 42124Ch
    push(ctx, 0x42124cu32);
    Cont(x4062f7)
}

pub fn x4062f7(ctx: &mut Context) -> Cont {
    // 004062f7 push 0C8h
    push(ctx, 0xc8u32);
    // 004062fc push 19h
    push(ctx, 0x19u32);
    // 004062fe push 1Eh
    push(ctx, 0x1eu32);
    // 00406300 push 1Eh
    push(ctx, 0x1eu32);
    // 00406302 push 64h
    push(ctx, 0x64u32);
    // 00406304 push 140h
    push(ctx, 0x140u32);
    // 00406309 call 00407330h
    let dst = Cont(x407330);
    call(ctx, 0x40630e, dst)
}

pub fn x40630e(ctx: &mut Context) -> Cont {
    // 0040630e add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    Cont(x406311)
}

pub fn x406311(ctx: &mut Context) -> Cont {
    // 00406311 call 00407390h
    let dst = Cont(x407390);
    call(ctx, 0x406316, dst)
}

pub fn x406316(ctx: &mut Context) -> Cont {
    // 00406316 call 00406C90h
    let dst = Cont(x406c90);
    call(ctx, 0x40631b, dst)
}

pub fn x40631b(ctx: &mut Context) -> Cont {
    // 0040631b call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x406320, dst)
}

pub fn x406320(ctx: &mut Context) -> Cont {
    // 00406320 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406322 jne short 00406331h
    jne(ctx, Cont(x406324), Cont(x406331))
}

pub fn x406324(ctx: &mut Context) -> Cont {
    // 00406324 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406325 cmp esi,200h
    sub(ctx.cpu.regs.esi, 0x200u32, &mut ctx.cpu.flags);
    // 0040632b jl near ptr 004060C1h
    jl(ctx, Cont(x406331), Cont(x4060c1))
}

pub fn x406331(ctx: &mut Context) -> Cont {
    // 00406331 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406332 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406333 add esp,50h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    // 00406336 ret
    ret(ctx, 0)
}

pub fn x406340(ctx: &mut Context) -> Cont {
    // 00406340 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00406343 fld dword ptr [esp+28h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 00406347 fmul dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 0040634b fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 0040634f fmul dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00406353 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00406357 fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406359 mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    // 0040635b fstp dword ptr [esp]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 0040635f fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00406363 fmul dword ptr [esp+1Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00406367 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 0040636b fmul dword ptr [esp+28h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as f64,
    );
    // 0040636f mov edx,[esp]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.esp);
    // 00406373 fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406375 mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 00406377 fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040637b fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 0040637f fmul dword ptr [esp+24h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00406383 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 00406387 fmul dword ptr [esp+18h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 0040638b mov edx,[esp+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 0040638f fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406391 mov [ecx+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00406394 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406398 mov edx,[esp+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 0040639c mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040639f add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004063a2 ret
    ret(ctx, 0)
}

pub fn x4063b0(ctx: &mut Context) -> Cont {
    // 004063b0 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 004063b4 fmul dword ptr [esp+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 004063b8 fld dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 004063bc fmul dword ptr [esp+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 004063c0 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063c2 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 004063c6 fmul dword ptr [esp+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as f64,
    );
    // 004063ca faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063cc ret
    ret(ctx, 0)
}

pub fn x4063d0(ctx: &mut Context) -> Cont {
    // 004063d0 mov eax,[esp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 004063d4 fld dword ptr [eax+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 004063d7 fld dword ptr [eax+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 004063da fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 004063dc fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 004063de fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 004063e0 fld st(2)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(2));
    // 004063e2 fmul st,st(3)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(3));
    // 004063e4 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063e6 fld st(3)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(3));
    // 004063e8 fmul st,st(4)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(4));
    // 004063ea faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063ec fsqrt
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sqrt());
    // 004063ee fstp st(3)
    ctx.cpu.fpu.set(3, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063f0 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063f2 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004063f4 fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 004063f6 fdiv st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) / ctx.cpu.fpu.get(1));
    // 004063f8 fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004063fa fld dword ptr [eax+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 004063fd fdiv st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) / ctx.cpu.fpu.get(1));
    // 004063ff fstp dword ptr [eax+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406402 fld dword ptr [eax+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 00406405 fdiv st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) / ctx.cpu.fpu.get(1));
    // 00406407 fstp dword ptr [eax+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040640a fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040640c ret
    ret(ctx, 0)
}

pub fn x406410(ctx: &mut Context) -> Cont {
    // 00406410 sub esp,18h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 00406413 mov eax,[esp+1Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00406417 mov ecx,[esp+20h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 0040641b mov edx,[esp+24h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0040641f mov [esp],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.eax);
    // 00406423 mov eax,[esp+28h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00406427 mov [esp+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0040642b mov ecx,[esp+2Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 0040642f mov [esp+1Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.eax);
    // 00406433 mov [esp+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 00406437 mov edx,[esp+30h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 0040643b lea eax,[esp]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0040643f mov [esp+20h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ecx);
    // 00406443 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406444 mov [esp+28h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.edx);
    // 00406448 call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x40644d, dst)
}

pub fn x40644d(ctx: &mut Context) -> Cont {
    // 0040644d mov edx,[esp+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00406451 mov eax,[esp+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 00406455 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00406458 mov ecx,esp
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp;
    // 0040645a sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0040645d mov [ecx],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.ecx, ctx.cpu.regs.edx);
    // 0040645f mov edx,[esp+20h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00406463 mov [ecx+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 00406466 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 00406468 mov [ecx+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040646b mov ecx,[esp+34h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 0040646f mov edx,[esp+38h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 00406473 mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 00406475 mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00406479 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 0040647c mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040647f call 004063B0h
    let dst = Cont(x4063b0);
    call(ctx, 0x406484, dst)
}

pub fn x406484(ctx: &mut Context) -> Cont {
    // 00406484 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00406488 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 0040648a lea edx,[esp+34h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x34u32);
    // 0040648e push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040648f fsubr dword ptr [esp+38h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x38u32)) as f64
            - ctx.cpu.fpu.get(0),
    );
    // 00406493 fstp dword ptr [esp+38h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x38u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406497 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0040649b fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 0040649d fsubr dword ptr [esp+3Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32)) as f64
            - ctx.cpu.fpu.get(0),
    );
    // 004064a1 fstp dword ptr [esp+3Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004064a5 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004064a9 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 004064ab fsubr dword ptr [esp+40h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64
            - ctx.cpu.fpu.get(0),
    );
    // 004064af fstp dword ptr [esp+40h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x40u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004064b3 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004064b5 call 004063D0h
    let dst = Cont(x4063d0);
    call(ctx, 0x4064ba, dst)
}

pub fn x4064ba(ctx: &mut Context) -> Cont {
    // 004064ba mov ecx,[esp+38h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 004064be mov edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 004064c2 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004064c5 mov eax,esp
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 004064c7 sub esp,0Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 004064ca mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 004064cc mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 004064d0 mov [eax+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004064d3 mov edx,esp
    ctx.cpu.regs.edx = ctx.cpu.regs.esp;
    // 004064d5 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 004064d8 mov eax,[esp+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 004064dc mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 004064e0 mov [edx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edx, ctx.cpu.regs.eax);
    // 004064e2 mov eax,[esp+20h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 004064e6 mov [edx+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 004064e9 lea ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x24u32);
    // 004064ed push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004064ee mov [edx+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 004064f1 call 00406340h
    let dst = Cont(x406340);
    call(ctx, 0x4064f6, dst)
}

pub fn x4064f6(ctx: &mut Context) -> Cont {
    // 004064f6 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004064f8 mov [esp+44h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32), ctx.cpu.regs.edx);
    // 004064fc mov ecx,[eax+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004064ff fld dword ptr [esp+44h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x44u32)) as f64,
    );
    // 00406503 mov edx,[eax+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 00406506 mov eax,[esp+50h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32));
    // 0040650a mov [esp+48h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32), ctx.cpu.regs.ecx);
    // 0040650e mov [esp+4Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32), ctx.cpu.regs.edx);
    // 00406512 fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00406514 mov [eax+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 00406517 mov ecx,[esp+38h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 0040651b mov [eax+8],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.edx);
    // 0040651e mov edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00406522 mov [eax+0Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32), ctx.cpu.regs.ecx);
    // 00406525 mov ecx,[esp+40h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00406529 mov [eax+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 0040652c mov edx,[esp+1Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00406530 mov [eax+14h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x14u32), ctx.cpu.regs.ecx);
    // 00406533 mov ecx,[esp+20h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00406537 mov [eax+18h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x18u32), ctx.cpu.regs.edx);
    // 0040653a mov edx,[esp+24h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0040653e mov [eax+1Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x1cu32), ctx.cpu.regs.ecx);
    // 00406541 mov [eax+20h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x20u32), ctx.cpu.regs.edx);
    // 00406544 add esp,34h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x34u32, &mut ctx.cpu.flags);
    // 00406547 ret
    ret(ctx, 0)
}

pub fn x406550(ctx: &mut Context) -> Cont {
    // 00406550 mov ecx,[esp+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 00406554 mov eax,[esp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00406558 fld dword ptr [ecx+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x10u32)) as f64,
    );
    // 0040655b fmul dword ptr [eax+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 0040655e fld dword ptr [ecx+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32)) as f64,
    );
    // 00406561 fmul dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 00406563 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406565 fld dword ptr [ecx+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x14u32)) as f64,
    );
    // 00406568 fmul dword ptr [eax+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 0040656b faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040656d fld dword ptr [ecx+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x1cu32)) as f64,
    );
    // 00406570 fmul dword ptr [eax+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 00406573 fld dword ptr [ecx+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x18u32)) as f64,
    );
    // 00406576 fmul dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 00406578 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040657a fld dword ptr [ecx+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x20u32)) as f64,
    );
    // 0040657d fmul dword ptr [eax+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 00406580 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406582 fstp dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406586 fld dword ptr [ecx+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)) as f64,
    );
    // 00406589 fmul dword ptr [eax+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 0040658c fld dword ptr [ecx+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 0040658f fmul dword ptr [eax+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 00406592 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406594 fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 00406596 fmul dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 00406598 mov ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 0040659c mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    // 0040659f faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004065a1 fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004065a3 fstp dword ptr [eax+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004065a6 ret
    ret(ctx, 0)
}

pub fn x4065b0(ctx: &mut Context) -> Cont {
    // 004065b0 mov eax,[esp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 004065b4 mov ecx,[esp+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 004065b8 fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 004065ba fadd dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 004065bc fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004065be fld dword ptr [ecx+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)) as f64,
    );
    // 004065c1 fadd dword ptr [eax+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 004065c4 fstp dword ptr [eax+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004065c7 fld dword ptr [ecx+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 004065ca fadd dword ptr [eax+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 004065cd fstp dword ptr [eax+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004065d0 ret
    ret(ctx, 0)
}

pub fn x4065e0(ctx: &mut Context) -> Cont {
    // 004065e0 mov eax,[esp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 004065e4 mov ecx,[esp+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 004065e8 fld dword ptr [eax]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64);
    // 004065ea fsub dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) - ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 004065ec fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 004065ee fld dword ptr [eax+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 004065f1 fsub dword ptr [ecx+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)) as f64,
    );
    // 004065f4 fstp dword ptr [eax+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004065f7 fld dword ptr [eax+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 004065fa fsub dword ptr [ecx+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 004065fd fstp dword ptr [eax+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.eax.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406600 ret
    ret(ctx, 0)
}

pub fn x406610(ctx: &mut Context) -> Cont {
    // 00406610 mov ecx,[esp+0Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00406614 fld dword ptr [ecx+8]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 00406617 fadd dword ptr ds:[420240h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420240u32) as f64,
    );
    // 0040661d fmul dword ptr ds:[4204A0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204a0u32) as f64,
    );
    // 00406623 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00406629 fst dword ptr [ecx+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ecx.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 0040662c fcomp dword ptr ds:[42049Ch]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x42049cu32) as f64));
    ctx.cpu.fpu.pop();
    // 00406632 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 00406634 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 00406637 je short 00406640h
    je(ctx, Cont(x406639), Cont(x406640))
}

pub fn x406639(ctx: &mut Context) -> Cont {
    // 00406639 mov dword ptr [ecx+8],3C23D70Ah
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32), 0x3c23d70au32);
    Cont(x406640)
}

pub fn x406640(ctx: &mut Context) -> Cont {
    // 00406640 fld dword ptr [ecx]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64);
    // 00406642 fdiv dword ptr [ecx+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            / ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 00406645 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0040664a fmul dword ptr ds:[420270h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420270u32) as f64,
    );
    // 00406650 fadd dword ptr [esp+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as f64,
    );
    // 00406654 fstp dword ptr [ecx]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.ecx, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00406656 fld dword ptr [ecx+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)) as f64,
    );
    // 00406659 fdiv dword ptr [ecx+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            / ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 0040665c fmul dword ptr ds:[420270h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x420270u32) as f64,
    );
    // 00406662 fadd dword ptr [esp+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)) as f64,
    );
    // 00406666 fstp dword ptr [ecx+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.ecx.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406669 ret
    ret(ctx, 0)
}

pub fn x406670(ctx: &mut Context) -> Cont {
    // 00406670 sub esp,1Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    // 00406673 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406674 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406675 mov ebp,[esp+28h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00406679 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040667b mov [esp+8],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32), ctx.cpu.regs.ebx);
    // 0040667f cmp [ebp],ebx
    sub(
        ctx.memory.read::<u32>(ctx.cpu.regs.ebp),
        ctx.cpu.regs.ebx,
        &mut ctx.cpu.flags,
    );
    // 00406682 jle near ptr 0040681Ch
    jle(ctx, Cont(x406688), Cont(x40681c))
}

pub fn x406688(ctx: &mut Context) -> Cont {
    // 00406688 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406689 mov esi,[esp+34h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 0040668d push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0040668e mov edi,[esp+34h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    Cont(x406692)
}

pub fn x406692(ctx: &mut Context) -> Cont {
    // 00406692 mov ecx,[ebp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x4u32));
    // 00406695 mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 00406697 add eax,ecx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406699 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0040669a mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    // 0040669c add eax,0Ch
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0xcu32, &mut ctx.cpu.flags);
    // 0040669f mov edx,[ecx]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.ecx);
    // 004066a1 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 004066a5 mov edx,[ecx+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32));
    // 004066a8 mov [esp+28h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.edx);
    // 004066ac mov ecx,[ecx+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32));
    // 004066af mov [esp+2Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), ctx.cpu.regs.ecx);
    // 004066b3 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 004066b5 mov [esp+18h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.edx);
    // 004066b9 mov ecx,[eax+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 004066bc mov [esp+1Ch],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.ecx);
    // 004066c0 mov edx,[eax+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 004066c3 lea eax,[esp+24h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x24u32);
    // 004066c7 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004066c8 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 004066cc call 004065E0h
    let dst = Cont(x4065e0);
    call(ctx, 0x4066d1, dst)
}

pub fn x4066d1(ctx: &mut Context) -> Cont {
    // 004066d1 lea ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x1cu32);
    // 004066d5 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004066d6 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004066d7 call 004065E0h
    let dst = Cont(x4065e0);
    call(ctx, 0x4066dc, dst)
}

pub fn x4066dc(ctx: &mut Context) -> Cont {
    // 004066dc lea edx,[esp+30h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x30u32);
    // 004066e0 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004066e1 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004066e2 call 00406550h
    let dst = Cont(x406550);
    call(ctx, 0x4066e7, dst)
}

pub fn x4066e7(ctx: &mut Context) -> Cont {
    // 004066e7 lea eax,[esp+2Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x2cu32);
    // 004066eb push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004066ec push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004066ed call 00406550h
    let dst = Cont(x406550);
    call(ctx, 0x4066f2, dst)
}

pub fn x4066f2(ctx: &mut Context) -> Cont {
    // 004066f2 mov edx,[esp+60h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 004066f6 mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 004066fa lea ecx,[esp+40h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x40u32);
    // 004066fe push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 004066ff push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406700 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406701 call 00406610h
    let dst = Cont(x406610);
    call(ctx, 0x406706, dst)
}

pub fn x406706(ctx: &mut Context) -> Cont {
    // 00406706 add esp,2Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x2cu32, &mut ctx.cpu.flags);
    // 00406709 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040670b je near ptr 00406803h
    je(ctx, Cont(x406711), Cont(x406803))
}

pub fn x406711(ctx: &mut Context) -> Cont {
    // 00406711 mov edx,[esp+40h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00406715 mov eax,[esp+3Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00406719 lea ecx,[esp+14h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x14u32);
    // 0040671d push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040671e push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040671f push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406720 call 00406610h
    let dst = Cont(x406610);
    call(ctx, 0x406725, dst)
}

pub fn x406725(ctx: &mut Context) -> Cont {
    // 00406725 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00406728 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040672a je near ptr 00406803h
    je(ctx, Cont(x406730), Cont(x406803))
}

pub fn x406730(ctx: &mut Context) -> Cont {
    // 00406730 fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00406734 push 0FEh
    push(ctx, 0xfeu32);
    // 00406739 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40673e, dst)
}

pub fn x40673e(ctx: &mut Context) -> Cont {
    // 0040673e fld dword ptr [esp+18h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x18u32)) as f64,
    );
    // 00406742 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406743 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406748, dst)
}

pub fn x406748(ctx: &mut Context) -> Cont {
    // 00406748 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 0040674c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040674d call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406752, dst)
}

pub fn x406752(ctx: &mut Context) -> Cont {
    // 00406752 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00406756 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406757 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40675c, dst)
}

pub fn x40675c(ctx: &mut Context) -> Cont {
    // 0040675c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040675d call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x406762, dst)
}

pub fn x406762(ctx: &mut Context) -> Cont {
    // 00406762 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00406766 push 0FEh
    push(ctx, 0xfeu32);
    // 0040676b call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406770, dst)
}

pub fn x406770(ctx: &mut Context) -> Cont {
    // 00406770 fld dword ptr [esp+2Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32)) as f64,
    );
    // 00406774 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406775 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40677a, dst)
}

pub fn x40677a(ctx: &mut Context) -> Cont {
    // 0040677a fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 0040677e inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040677f push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406780 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x406785, dst)
}

pub fn x406785(ctx: &mut Context) -> Cont {
    // 00406785 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 00406789 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040678a call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40678f, dst)
}

pub fn x40678f(ctx: &mut Context) -> Cont {
    // 0040678f inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406790 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406791 call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x406796, dst)
}

pub fn x406796(ctx: &mut Context) -> Cont {
    // 00406796 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 0040679a push 0FEh
    push(ctx, 0xfeu32);
    // 0040679f call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067a4, dst)
}

pub fn x4067a4(ctx: &mut Context) -> Cont {
    // 004067a4 fld dword ptr [esp+40h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x40u32)) as f64,
    );
    // 004067a8 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067a9 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067aa call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067af, dst)
}

pub fn x4067af(ctx: &mut Context) -> Cont {
    // 004067af fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004067b3 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067b4 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067b9, dst)
}

pub fn x4067b9(ctx: &mut Context) -> Cont {
    // 004067b9 fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004067bd inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067be push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067bf call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067c4, dst)
}

pub fn x4067c4(ctx: &mut Context) -> Cont {
    // 004067c4 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067c5 call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x4067ca, dst)
}

pub fn x4067ca(ctx: &mut Context) -> Cont {
    // 004067ca fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004067ce push 0FEh
    push(ctx, 0xfeu32);
    // 004067d3 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067d8, dst)
}

pub fn x4067d8(ctx: &mut Context) -> Cont {
    // 004067d8 fld dword ptr [esp+54h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as f64,
    );
    // 004067dc inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067dd push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067de call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067e3, dst)
}

pub fn x4067e3(ctx: &mut Context) -> Cont {
    // 004067e3 fld dword ptr [esp+68h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x68u32)) as f64,
    );
    // 004067e7 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067e8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067e9 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067ee, dst)
}

pub fn x4067ee(ctx: &mut Context) -> Cont {
    // 004067ee fld dword ptr [esp+68h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x68u32)) as f64,
    );
    // 004067f2 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067f3 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067f4 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4067f9, dst)
}

pub fn x4067f9(ctx: &mut Context) -> Cont {
    // 004067f9 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004067fa push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004067fb call 00407470h
    let dst = Cont(x407470);
    call(ctx, 0x406800, dst)
}

pub fn x406800(ctx: &mut Context) -> Cont {
    // 00406800 add esp,50h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x50u32, &mut ctx.cpu.flags);
    Cont(x406803)
}

pub fn x406803(ctx: &mut Context) -> Cont {
    // 00406803 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00406807 mov ecx,[ebp]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.ebp);
    // 0040680a inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040680b add ebx,18h
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x18u32, &mut ctx.cpu.flags);
    // 0040680e cmp eax,ecx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406810 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00406814 jl near ptr 00406692h
    jl(ctx, Cont(x40681a), Cont(x406692))
}

pub fn x40681a(ctx: &mut Context) -> Cont {
    // 0040681a pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040681b pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    Cont(x40681c)
}

pub fn x40681c(ctx: &mut Context) -> Cont {
    // 0040681c pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040681d pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040681e add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    // 00406821 ret
    ret(ctx, 0)
}

pub fn x406830(ctx: &mut Context) -> Cont {
    // 00406830 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406831 fld dword ptr ds:[420128h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420128u32) as f64);
    // 00406837 fsub dword ptr [esp+14h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            - ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as f64,
    );
    // 0040683b push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040683c mov ebx,[esp+14h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00406840 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406841 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406843 mov eax,[ebx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.ebx);
    // 00406845 fstp dword ptr [esp+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406849 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040684b jle near ptr 004068E9h
    jle(ctx, Cont(x406851), Cont(x4068e9))
}

pub fn x406851(ctx: &mut Context) -> Cont {
    // 00406851 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406852 mov ebp,[esp+14h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00406856 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406857 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x406859)
}

pub fn x406859(ctx: &mut Context) -> Cont {
    // 00406859 mov eax,[ebp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x4u32));
    // 0040685c mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00406860 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00406864 mov ecx,[ecx+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32));
    // 00406867 add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406869 mov edx,[ebx+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebx.wrapping_add(0x4u32));
    // 0040686c add ecx,esi
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040686e fmul dword ptr [eax]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(ctx.cpu.regs.eax) as f64,
    );
    // 00406870 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 00406874 fmul dword ptr [ecx]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(ctx.cpu.regs.ecx) as f64,
    );
    // 00406876 add edx,esi
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406878 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406879 add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 0040687c faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0040687e fstp dword ptr [edx]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.edx, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00406880 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00406884 fmul dword ptr [eax+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x4u32)) as f64,
    );
    // 00406887 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 0040688b fmul dword ptr [ecx+4]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32)) as f64,
    );
    // 0040688e faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00406890 fstp dword ptr [edx+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.edx.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 00406893 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00406897 fmul dword ptr [eax+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x8u32)) as f64,
    );
    // 0040689a fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 0040689e fmul dword ptr [ecx+8]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32)) as f64,
    );
    // 004068a1 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004068a3 fstp dword ptr [edx+8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.edx.wrapping_add(0x8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004068a6 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 004068aa fmul dword ptr [eax+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0xcu32)) as f64,
    );
    // 004068ad fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004068b1 fmul dword ptr [ecx+0Ch]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32)) as f64,
    );
    // 004068b4 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004068b6 fstp dword ptr [edx+0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.edx.wrapping_add(0xcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004068b9 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 004068bd fmul dword ptr [eax+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x10u32)) as f64,
    );
    // 004068c0 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004068c4 fmul dword ptr [ecx+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x10u32)) as f64,
    );
    // 004068c7 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004068c9 fstp dword ptr [edx+10h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.edx.wrapping_add(0x10u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004068cc fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 004068d0 fmul dword ptr [eax+14h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.eax.wrapping_add(0x14u32)) as f64,
    );
    // 004068d3 fld dword ptr [esp+24h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x24u32)) as f64,
    );
    // 004068d7 fmul dword ptr [ecx+14h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>(ctx.cpu.regs.ecx.wrapping_add(0x14u32)) as f64,
    );
    // 004068da faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 004068dc fstp dword ptr [edx+14h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.edx.wrapping_add(0x14u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004068df cmp edi,[ebx]
    sub(
        ctx.cpu.regs.edi,
        ctx.memory.read::<u32>(ctx.cpu.regs.ebx),
        &mut ctx.cpu.flags,
    );
    // 004068e1 jl near ptr 00406859h
    jl(ctx, Cont(x4068e7), Cont(x406859))
}

pub fn x4068e7(ctx: &mut Context) -> Cont {
    // 004068e7 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004068e8 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    Cont(x4068e9)
}

pub fn x4068e9(ctx: &mut Context) -> Cont {
    // 004068e9 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 004068ea pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004068eb pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 004068ec ret
    ret(ctx, 0)
}

pub fn x406960(ctx: &mut Context) -> Cont {
    // 00406960 sub esp,62Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x62cu32, &mut ctx.cpu.flags);
    // 00406966 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406967 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406968 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040696a push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0040696b push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0040696c call dword ptr ds:[420028h]
    let dst = Cont(kernel32::GetModuleHandleA_stdcall);
    call(ctx, 0x406972, dst)
}

pub fn x406972(ctx: &mut Context) -> Cont {
    // 00406972 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00406974 mov ecx,40h
    ctx.cpu.regs.ecx = 0x40u32;
    // 00406979 or eax,0FFFFFFFFh
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, 0xffffffffu32, &mut ctx.cpu.flags);
    // 0040697c lea edi,[esp+138h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esp.wrapping_add(0x138u32);
    // 00406983 rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00406985 mov ecx,40h
    ctx.cpu.regs.ecx = 0x40u32;
    // 0040698a xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040698c lea edi,[esp+38h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esp.wrapping_add(0x38u32);
    // 00406990 rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00406992 lea eax,[esp+38h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x38u32);
    // 00406996 lea ecx,[esp+138h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x138u32);
    // 0040699d push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0040699e push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040699f push 20h
    push(ctx, 0x20u32);
    // 004069a1 push 20h
    push(ctx, 0x20u32);
    // 004069a3 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004069a4 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004069a5 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004069a6 call dword ptr ds:[42004Ch]
    let dst = Cont(user32::CreateCursor_stdcall);
    call(ctx, 0x4069ac, dst)
}

pub fn x4069ac(ctx: &mut Context) -> Cont {
    // 004069ac lea edx,[esp+10h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x10u32);
    // 004069b0 mov edi,421710h
    ctx.cpu.regs.edi = 0x421710u32;
    // 004069b5 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 004069b6 mov dword ptr [esp+14h],3
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), 0x3u32);
    // 004069be mov dword ptr [esp+18h],4068F0h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), 0x4068f0u32);
    // 004069c6 mov [esp+1Ch],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.ebx);
    // 004069ca mov [esp+20h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ebx);
    // 004069ce mov [esp+24h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.esi);
    // 004069d2 mov [esp+28h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.ebx);
    // 004069d6 mov [esp+2Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), ctx.cpu.regs.eax);
    // 004069da mov dword ptr [esp+30h],6
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), 0x6u32);
    // 004069e2 mov [esp+34h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), ctx.cpu.regs.edi);
    // 004069e6 mov [esp+38h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.regs.edi);
    // 004069ea call dword ptr ds:[420068h]
    let dst = Cont(user32::RegisterClassA_stdcall);
    call(ctx, 0x4069f0, dst)
}

pub fn x4069f0(ctx: &mut Context) -> Cont {
    // 004069f0 test ax,ax
    and(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    );
    // 004069f3 jne short 00406A10h
    jne(ctx, Cont(x4069f5), Cont(x406a10))
}

pub fn x4069f5(ctx: &mut Context) -> Cont {
    // 004069f5 push 10h
    push(ctx, 0x10u32);
    // 004069f7 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004069f8 push 4216F0h
    push(ctx, 0x4216f0u32);
    // 004069fd push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004069fe call dword ptr ds:[420040h]
    let dst = Cont(user32::MessageBoxA_stdcall);
    call(ctx, 0x406a04, dst)
}

pub fn x406a04(ctx: &mut Context) -> Cont {
    // 00406a04 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406a05 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406a06 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406a08 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00406a09 add esp,62Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x62cu32, &mut ctx.cpu.flags);
    // 00406a0f ret
    ret(ctx, 0)
}

pub fn x406a10(ctx: &mut Context) -> Cont {
    // 00406a10 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a11 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406a12 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a13 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a14 push 0C8h
    push(ctx, 0xc8u32);
    // 00406a19 push 0C8h
    push(ctx, 0xc8u32);
    // 00406a1e push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a1f push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a20 push 80000000h
    push(ctx, 0x80000000u32);
    // 00406a25 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406a26 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406a27 push 8
    push(ctx, 0x8u32);
    // 00406a29 call dword ptr ds:[42003Ch]
    let dst = Cont(user32::CreateWindowExA_stdcall);
    call(ctx, 0x406a2f, dst)
}

pub fn x406a2f(ctx: &mut Context) -> Cont {
    // 00406a2f push 0Ah
    push(ctx, 0xau32);
    // 00406a31 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406a32 mov ds:[428D70h],eax
    ctx.memory.write::<u32>(0x428d70u32, ctx.cpu.regs.eax);
    // 00406a37 call dword ptr ds:[420044h]
    let dst = Cont(user32::ShowWindow_stdcall);
    call(ctx, 0x406a3d, dst)
}

pub fn x406a3d(ctx: &mut Context) -> Cont {
    // 00406a3d mov eax,ds:[428D70h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428d70u32);
    // 00406a42 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406a43 call dword ptr ds:[420060h]
    let dst = Cont(user32::UpdateWindow_stdcall);
    call(ctx, 0x406a49, dst)
}

pub fn x406a49(ctx: &mut Context) -> Cont {
    // 00406a49 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a4a push 428CECh
    push(ctx, 0x428cecu32);
    // 00406a4f push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a50 call 0041F070h
    let dst = Cont(x41f070);
    call(ctx, 0x406a55, dst)
}

pub fn x406a55(ctx: &mut Context) -> Cont {
    // 00406a55 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406a57 je short 00406A74h
    je(ctx, Cont(x406a59), Cont(x406a74))
}

pub fn x406a59(ctx: &mut Context) -> Cont {
    // 00406a59 push 10h
    push(ctx, 0x10u32);
    // 00406a5b push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406a5c push 4216CCh
    push(ctx, 0x4216ccu32);
    // 00406a61 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406a62 call dword ptr ds:[420040h]
    let dst = Cont(user32::MessageBoxA_stdcall);
    call(ctx, 0x406a68, dst)
}

pub fn x406a68(ctx: &mut Context) -> Cont {
    // 00406a68 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406a69 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406a6a xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406a6c pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00406a6d add esp,62Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x62cu32, &mut ctx.cpu.flags);
    // 00406a73 ret
    ret(ctx, 0)
}

pub fn x406a74(ctx: &mut Context) -> Cont {
    // 00406a74 mov eax,ds:[428CECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cecu32);
    // 00406a79 mov edx,ds:[428D70h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428d70u32);
    // 00406a7f push 11h
    push(ctx, 0x11u32);
    // 00406a81 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406a82 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406a84 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406a85 call dword ptr [ecx+50h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x50u32)));
    call(ctx, 0x406a88, dst)
}

pub fn x406a88(ctx: &mut Context) -> Cont {
    // 00406a88 mov eax,ds:[428CECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cecu32);
    // 00406a8d push 8
    push(ctx, 0x8u32);
    // 00406a8f push 1E0h
    push(ctx, 0x1e0u32);
    // 00406a94 push 280h
    push(ctx, 0x280u32);
    // 00406a99 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406a9b push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406a9c call dword ptr [ecx+54h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x54u32)));
    call(ctx, 0x406a9f, dst)
}

pub fn x406a9f(ctx: &mut Context) -> Cont {
    // 00406a9f mov eax,ds:[428CECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cecu32);
    // 00406aa4 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406aa5 mov dword ptr ds:[428D00h],6Ch
    ctx.memory.write::<u32>(0x428d00u32, 0x6cu32);
    // 00406aaf mov dword ptr ds:[428D04h],21h
    ctx.memory.write::<u32>(0x428d04u32, 0x21u32);
    // 00406ab9 mov dword ptr ds:[428D68h],4218h
    ctx.memory.write::<u32>(0x428d68u32, 0x4218u32);
    // 00406ac3 mov dword ptr ds:[428D14h],1
    ctx.memory.write::<u32>(0x428d14u32, 0x1u32);
    // 00406acd mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406acf push 428CFCh
    push(ctx, 0x428cfcu32);
    // 00406ad4 push 428D00h
    push(ctx, 0x428d00u32);
    // 00406ad9 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406ada call dword ptr [edx+18h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.edx.wrapping_add(0x18u32)));
    call(ctx, 0x406add, dst)
}

pub fn x406add(ctx: &mut Context) -> Cont {
    // 00406add test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406adf je short 00406AFCh
    je(ctx, Cont(x406ae1), Cont(x406afc))
}

pub fn x406ae1(ctx: &mut Context) -> Cont {
    // 00406ae1 push 10h
    push(ctx, 0x10u32);
    // 00406ae3 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406ae4 push 4216A4h
    push(ctx, 0x4216a4u32);
    // 00406ae9 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406aea call dword ptr ds:[420040h]
    let dst = Cont(user32::MessageBoxA_stdcall);
    call(ctx, 0x406af0, dst)
}

pub fn x406af0(ctx: &mut Context) -> Cont {
    // 00406af0 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406af1 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406af2 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406af4 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00406af5 add esp,62Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x62cu32, &mut ctx.cpu.flags);
    // 00406afb ret
    ret(ctx, 0)
}

pub fn x406afc(ctx: &mut Context) -> Cont {
    // 00406afc mov eax,ds:[428CFCh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cfcu32);
    // 00406b01 lea edx,[esp+0Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0xcu32);
    // 00406b05 mov dword ptr [esp+0Ch],4
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32), 0x4u32);
    // 00406b0d push 428CF0h
    push(ctx, 0x428cf0u32);
    // 00406b12 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406b14 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406b15 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406b16 call dword ptr [ecx+30h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x30u32)));
    call(ctx, 0x406b19, dst)
}

pub fn x406b19(ctx: &mut Context) -> Cont {
    // 00406b19 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x406b1b)
}

pub fn x406b1b(ctx: &mut Context) -> Cont {
    // 00406b1b mov ecx,[eax+4212A4h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4212a4u32));
    // 00406b21 add eax,4
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x4u32, &mut ctx.cpu.flags);
    // 00406b24 mov edx,ecx
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx;
    // 00406b26 sar edx,10h
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x10u8, &mut ctx.cpu.flags);
    // 00406b29 sar ecx,8
    ctx.cpu.regs.ecx = sar(ctx.cpu.regs.ecx, 0x8u8, &mut ctx.cpu.flags);
    // 00406b2c mov [esp+eax+234h],dl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x234u32),
        ctx.cpu.regs.get_dl(),
    );
    // 00406b33 mov [esp+eax+235h],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x235u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00406b3a mov cl,[eax+4212A0h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(0x4212a0u32)),
    );
    // 00406b40 cmp eax,400h
    sub(ctx.cpu.regs.eax, 0x400u32, &mut ctx.cpu.flags);
    // 00406b45 mov [esp+eax+236h],cl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x236u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00406b4c mov [esp+eax+237h],bl
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .esp
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0x237u32),
        ctx.cpu.regs.get_bl(),
    );
    // 00406b53 jl short 00406B1Bh
    jl(ctx, Cont(x406b55), Cont(x406b1b))
}

pub fn x406b55(ctx: &mut Context) -> Cont {
    // 00406b55 mov eax,ds:[428CECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cecu32);
    // 00406b5a push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406b5b lea ecx,[esp+23Ch]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x23cu32);
    // 00406b62 push 428CF4h
    push(ctx, 0x428cf4u32);
    // 00406b67 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406b69 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406b6a push 4Ch
    push(ctx, 0x4cu32);
    // 00406b6c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406b6d call dword ptr [edx+14h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.edx.wrapping_add(0x14u32)));
    call(ctx, 0x406b70, dst)
}

pub fn x406b70(ctx: &mut Context) -> Cont {
    // 00406b70 mov eax,ds:[428CFCh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cfcu32);
    // 00406b75 mov ecx,ds:[428CF4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428cf4u32);
    // 00406b7b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406b7c push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406b7d mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406b7f call dword ptr [edx+7Ch]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.edx.wrapping_add(0x7cu32)));
    call(ctx, 0x406b82, dst)
}

pub fn x406b82(ctx: &mut Context) -> Cont {
    // 00406b82 push 1F400h
    push(ctx, 0x1f400u32);
    // 00406b87 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406b8c, dst)
}

pub fn x406b8c(ctx: &mut Context) -> Cont {
    // 00406b8c push 1F400h
    push(ctx, 0x1f400u32);
    // 00406b91 mov ds:[428CF8h],eax
    ctx.memory.write::<u32>(0x428cf8u32, ctx.cpu.regs.eax);
    // 00406b96 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406b9b, dst)
}

pub fn x406b9b(ctx: &mut Context) -> Cont {
    // 00406b9b push 1F400h
    push(ctx, 0x1f400u32);
    // 00406ba0 mov ds:[428D74h],eax
    ctx.memory.write::<u32>(0x428d74u32, ctx.cpu.regs.eax);
    // 00406ba5 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406baa, dst)
}

pub fn x406baa(ctx: &mut Context) -> Cont {
    // 00406baa push 0A00h
    push(ctx, 0xa00u32);
    // 00406baf mov ds:[428D6Ch],eax
    ctx.memory.write::<u32>(0x428d6cu32, ctx.cpu.regs.eax);
    // 00406bb4 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406bb9, dst)
}

pub fn x406bb9(ctx: &mut Context) -> Cont {
    // 00406bb9 push 280h
    push(ctx, 0x280u32);
    // 00406bbe mov ds:[428CE4h],eax
    ctx.memory.write::<u32>(0x428ce4u32, ctx.cpu.regs.eax);
    // 00406bc3 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406bc8, dst)
}

pub fn x406bc8(ctx: &mut Context) -> Cont {
    // 00406bc8 push 320h
    push(ctx, 0x320u32);
    // 00406bcd mov ds:[428CE8h],eax
    ctx.memory.write::<u32>(0x428ce8u32, ctx.cpu.regs.eax);
    // 00406bd2 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406bd7, dst)
}

pub fn x406bd7(ctx: &mut Context) -> Cont {
    // 00406bd7 push 0C8h
    push(ctx, 0xc8u32);
    // 00406bdc mov ds:[425C00h],eax
    ctx.memory.write::<u32>(0x425c00u32, ctx.cpu.regs.eax);
    // 00406be1 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x406be6, dst)
}

pub fn x406be6(ctx: &mut Context) -> Cont {
    // 00406be6 add esp,1Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x1cu32, &mut ctx.cpu.flags);
    // 00406be9 mov ds:[425C04h],eax
    ctx.memory.write::<u32>(0x425c04u32, ctx.cpu.regs.eax);
    // 00406bee mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 00406bf3 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406bf4 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406bf5 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00406bf6 add esp,62Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x62cu32, &mut ctx.cpu.flags);
    // 00406bfc ret
    ret(ctx, 0)
}

pub fn x406c00(ctx: &mut Context) -> Cont {
    // 00406c00 mov eax,ds:[428CF4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cf4u32);
    // 00406c05 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406c06 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406c08 call dword ptr [ecx+8]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x8u32)));
    call(ctx, 0x406c0b, dst)
}

pub fn x406c0b(ctx: &mut Context) -> Cont {
    // 00406c0b mov eax,ds:[428CFCh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cfcu32);
    // 00406c10 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406c11 mov edx,[eax]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406c13 call dword ptr [edx+8]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.edx.wrapping_add(0x8u32)));
    call(ctx, 0x406c16, dst)
}

pub fn x406c16(ctx: &mut Context) -> Cont {
    // 00406c16 mov eax,ds:[428CECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cecu32);
    // 00406c1b push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406c1c mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406c1e call dword ptr [ecx+8]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x8u32)));
    call(ctx, 0x406c21, dst)
}

pub fn x406c21(ctx: &mut Context) -> Cont {
    // 00406c21 mov edx,ds:[428D70h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428d70u32);
    // 00406c27 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406c28 call dword ptr ds:[420048h]
    let dst = Cont(user32::DestroyWindow_stdcall);
    call(ctx, 0x406c2e, dst)
}

pub fn x406c2e(ctx: &mut Context) -> Cont {
    // 00406c2e mov eax,ds:[428CF8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cf8u32);
    // 00406c33 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406c34 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x406c39, dst)
}

pub fn x406c39(ctx: &mut Context) -> Cont {
    // 00406c39 mov ecx,ds:[428D74h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428d74u32);
    // 00406c3f push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406c40 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x406c45, dst)
}

pub fn x406c45(ctx: &mut Context) -> Cont {
    // 00406c45 mov edx,ds:[428D6Ch]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428d6cu32);
    // 00406c4b push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406c4c call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x406c51, dst)
}

pub fn x406c51(ctx: &mut Context) -> Cont {
    // 00406c51 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00406c54 ret
    ret(ctx, 0)
}

pub fn x406c60(ctx: &mut Context) -> Cont {
    // 00406c60 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406c61 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00406c63 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406c64 mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00406c6a mov ecx,7D00h
    ctx.cpu.regs.ecx = 0x7d00u32;
    // 00406c6f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406c71 mov al,[ebp+8]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)));
    // 00406c74 shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 00406c77 mov al,[ebp+8]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)));
    // 00406c7a shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 00406c7d mov al,[ebp+8]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)));
    // 00406c80 shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 00406c83 mov al,[ebp+8]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp.wrapping_add(0x8u32)));
    // 00406c86 rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00406c88 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406c89 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00406c8a ret
    ret(ctx, 0)
}

pub fn x406c90(ctx: &mut Context) -> Cont {
    // 00406c90 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406c91 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00406c93 sub esp,88h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x88u32, &mut ctx.cpu.flags);
    // 00406c99 mov eax,ds:[428CF0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cf0u32);
    // 00406c9e push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406c9f push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406ca0 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406ca1 push 0
    push(ctx, 0x0u32);
    // 00406ca3 lea edx,[ebp-88h]
    ctx.cpu.regs.edx = ctx.cpu.regs.ebp.wrapping_add(0xffffff78u32);
    // 00406ca9 push 1
    push(ctx, 0x1u32);
    // 00406cab mov dword ptr [ebp-88h],6Ch
    ctx.memory
        .write::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffff78u32), 0x6cu32);
    // 00406cb5 mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406cb7 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406cb8 push 0
    push(ctx, 0x0u32);
    // 00406cba push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406cbb call dword ptr [ecx+64h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x64u32)));
    call(ctx, 0x406cbe, dst)
}

pub fn x406cbe(ctx: &mut Context) -> Cont {
    // 00406cbe xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00406cc0 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406cc2 mov edi,[ebp-64h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffff9cu32));
    // 00406cc5 mov ecx,5780h
    ctx.cpu.regs.ecx = 0x5780u32;
    // 00406cca rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00406ccc mov esi,ds:[428CF8h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00406cd2 mov edx,ds:[428D74h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428d74u32);
    // 00406cd8 mov ecx,1F400h
    ctx.cpu.regs.ecx = 0x1f400u32;
    Cont(x406cdd)
}

pub fn x406cdd(ctx: &mut Context) -> Cont {
    // 00406cdd mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 00406cdf mov bl,[edx]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edx));
    // 00406ce1 add ax,bx
    ctx.cpu.regs.set_ax(add(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_bx(),
        &mut ctx.cpu.flags,
    ));
    // 00406ce4 add ax,bx
    ctx.cpu.regs.set_ax(add(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_bx(),
        &mut ctx.cpu.flags,
    ));
    // 00406ce7 add ax,bx
    ctx.cpu.regs.set_ax(add(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_bx(),
        &mut ctx.cpu.flags,
    ));
    // 00406cea shr ax,2
    ctx.cpu
        .regs
        .set_ax(shr(ctx.cpu.regs.get_ax(), 0x2u8, &mut ctx.cpu.flags));
    // 00406cee mov [edx],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edx, ctx.cpu.regs.get_al());
    // 00406cf0 mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 00406cf2 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406cf3 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406cf4 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00406cf5 loop 00406CDDh
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x406cf7)
    } else {
        Cont(x406cdd)
    }
}

pub fn x406cf7(ctx: &mut Context) -> Cont {
    // 00406cf7 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406cf9 mov ecx,5780h
    ctx.cpu.regs.ecx = 0x5780u32;
    // 00406cfe rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00406d00 mov eax,ds:[428CF0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cf0u32);
    // 00406d05 mov edx,[ebp-64h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffff9cu32));
    // 00406d08 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406d09 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406d0a mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406d0c call dword ptr [ecx+80h]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x80u32)));
    call(ctx, 0x406d12, dst)
}

pub fn x406d12(ctx: &mut Context) -> Cont {
    // 00406d12 mov eax,ds:[428CFCh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cfcu32);
    // 00406d17 push 1
    push(ctx, 0x1u32);
    // 00406d19 push 0
    push(ctx, 0x0u32);
    // 00406d1b push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406d1c mov ecx,[eax]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.eax);
    // 00406d1e call dword ptr [ecx+2Ch]
    let dst = indirect(ctx, ctx.memory.read(ctx.cpu.regs.ecx.wrapping_add(0x2cu32)));
    call(ctx, 0x406d21, dst)
}

pub fn x406d21(ctx: &mut Context) -> Cont {
    // 00406d21 push 0
    push(ctx, 0x0u32);
    // 00406d23 push 0
    push(ctx, 0x0u32);
    // 00406d25 push 0
    push(ctx, 0x0u32);
    // 00406d27 lea edx,[ebp-1Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32);
    // 00406d2a push 0
    push(ctx, 0x0u32);
    // 00406d2c push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00406d2d call dword ptr ds:[420058h]
    let dst = Cont(user32::PeekMessageA_stdcall);
    call(ctx, 0x406d33, dst)
}

pub fn x406d33(ctx: &mut Context) -> Cont {
    // 00406d33 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406d34 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406d35 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406d37 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00406d38 je short 00406D54h
    je(ctx, Cont(x406d3a), Cont(x406d54))
}

pub fn x406d3a(ctx: &mut Context) -> Cont {
    // 00406d3a push 0
    push(ctx, 0x0u32);
    // 00406d3c push 0
    push(ctx, 0x0u32);
    // 00406d3e lea eax,[ebp-1Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32);
    // 00406d41 push 0
    push(ctx, 0x0u32);
    // 00406d43 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00406d44 call dword ptr ds:[420054h]
    let dst = Cont(user32::GetMessageA_stdcall);
    call(ctx, 0x406d4a, dst)
}

pub fn x406d4a(ctx: &mut Context) -> Cont {
    // 00406d4a lea ecx,[ebp-1Ch]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32);
    // 00406d4d push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00406d4e call dword ptr ds:[420050h]
    let dst = Cont(user32::DispatchMessageA_stdcall);
    call(ctx, 0x406d54, dst)
}

pub fn x406d54(ctx: &mut Context) -> Cont {
    // 00406d54 mov esp,ebp
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    // 00406d56 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00406d57 ret
    ret(ctx, 0)
}

pub fn x406d60(ctx: &mut Context) -> Cont {
    // 00406d60 mov eax,ds:[421A64h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x421a64u32);
    // 00406d65 ret
    ret(ctx, 0)
}

pub fn x406d70(ctx: &mut Context) -> Cont {
    // 00406d70 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00406d73 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406d74 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406d75 mov esi,[esp+1Ch]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00406d79 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406d7a mov edi,[esp+28h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00406d7e mov ecx,[esp+2Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 00406d82 mov eax,edi
    ctx.cpu.regs.eax = ctx.cpu.regs.edi;
    // 00406d84 mov ebp,[esp+24h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00406d88 sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 00406d8a sub esi,eax
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406d8c mov eax,20000h
    ctx.cpu.regs.eax = 0x20000u32;
    // 00406d91 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00406d92 idiv edi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.edi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00406d94 mov [esp+14h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.esi);
    // 00406d98 mov dword ptr [esp+20h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0x0u32);
    // 00406da0 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00406da4 mov eax,20000h
    ctx.cpu.regs.eax = 0x20000u32;
    // 00406da9 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00406daa idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00406dac mov edx,ecx
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx;
    // 00406dae sar edx,1
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x1u8, &mut ctx.cpu.flags);
    // 00406db0 sub ebp,edx
    ctx.cpu.regs.ebp = sub(ctx.cpu.regs.ebp, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00406db2 test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406db4 mov [esp+18h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.eax);
    // 00406db8 jle near ptr 00406E76h
    jle(ctx, Cont(x406dbe), Cont(x406e76))
}

pub fn x406dbe(ctx: &mut Context) -> Cont {
    // 00406dbe lea ebp,[ebp+ebp*4]
    ctx.cpu.regs.ebp = ctx.cpu.regs.ebp.wrapping_add((ctx.cpu.regs.ebp * 4));
    // 00406dc2 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406dc3 shl ebp,7
    ctx.cpu.regs.ebp = shl(ctx.cpu.regs.ebp, 0x7u8, &mut ctx.cpu.flags);
    // 00406dc6 mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    Cont(x406dca)
}

pub fn x406dca(ctx: &mut Context) -> Cont {
    // 00406dca xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406dcc test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406dce mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 00406dd2 jle short 00406E52h
    jle(ctx, Cont(x406dd4), Cont(x406e52))
}

pub fn x406dd4(ctx: &mut Context) -> Cont {
    // 00406dd4 mov [esp+28h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.edi);
    Cont(x406dd8)
}

pub fn x406dd8(ctx: &mut Context) -> Cont {
    // 00406dd8 test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406dda jl short 00406E34h
    jl(ctx, Cont(x406ddc), Cont(x406e34))
}

pub fn x406ddc(ctx: &mut Context) -> Cont {
    // 00406ddc cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00406de2 jge short 00406E34h
    jge(ctx, Cont(x406de4), Cont(x406e34))
}

pub fn x406de4(ctx: &mut Context) -> Cont {
    // 00406de4 test ebp,ebp
    and(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00406de6 jl short 00406E34h
    jl(ctx, Cont(x406de8), Cont(x406e34))
}

pub fn x406de8(ctx: &mut Context) -> Cont {
    // 00406de8 cmp ebp,1F400h
    sub(ctx.cpu.regs.ebp, 0x1f400u32, &mut ctx.cpu.flags);
    // 00406dee jge short 00406E34h
    jge(ctx, Cont(x406df0), Cont(x406e34))
}

pub fn x406df0(ctx: &mut Context) -> Cont {
    // 00406df0 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00406df4 mov ebx,[esp+38h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 00406df8 sar ecx,5
    ctx.cpu.regs.ecx = sar(ctx.cpu.regs.ecx, 0x5u8, &mut ctx.cpu.flags);
    // 00406dfb and ecx,0FFFFFFC0h
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0xffffffc0u32, &mut ctx.cpu.flags);
    // 00406dfe mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00406e04 sar eax,0Bh
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0xbu8, &mut ctx.cpu.flags);
    // 00406e07 add ebx,ecx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406e09 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406e0b lea edx,[esi+ebp]
    ctx.cpu.regs.edx = ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.ebp);
    // 00406e0e mov cl,[eax+ebx]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.ebx)),
    );
    // 00406e11 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406e13 imul ecx,[esp+34h]
    let x = ctx.cpu.regs.ecx as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ecx = res as u32;
    // 00406e18 mov al,[edi+edx]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.edx)),
    );
    // 00406e1b sar ecx,8
    ctx.cpu.regs.ecx = sar(ctx.cpu.regs.ecx, 0x8u8, &mut ctx.cpu.flags);
    // 00406e1e add ecx,eax
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406e20 cmp ecx,0FFh
    sub(ctx.cpu.regs.ecx, 0xffu32, &mut ctx.cpu.flags);
    // 00406e26 jle short 00406E2Dh
    jle(ctx, Cont(x406e28), Cont(x406e2d))
}

pub fn x406e28(ctx: &mut Context) -> Cont {
    // 00406e28 mov ecx,0FFh
    ctx.cpu.regs.ecx = 0xffu32;
    Cont(x406e2d)
}

pub fn x406e2d(ctx: &mut Context) -> Cont {
    // 00406e2d mov [edi+edx],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.edx),
        ctx.cpu.regs.get_cl(),
    );
    // 00406e30 mov edi,[esp+2Ch]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    Cont(x406e34)
}

pub fn x406e34(ctx: &mut Context) -> Cont {
    // 00406e34 mov eax,[esp+30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 00406e38 mov ebx,[esp+14h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00406e3c mov ecx,[esp+28h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00406e40 add eax,ebx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00406e42 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406e43 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406e44 mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 00406e48 mov [esp+28h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.ecx);
    // 00406e4c jne short 00406DD8h
    jne(ctx, Cont(x406e4e), Cont(x406dd8))
}

pub fn x406e4e(ctx: &mut Context) -> Cont {
    // 00406e4e mov esi,[esp+18h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    Cont(x406e52)
}

pub fn x406e52(ctx: &mut Context) -> Cont {
    // 00406e52 mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00406e56 mov edx,[esp+24h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00406e5a mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00406e5e add edx,ecx
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00406e60 add ebp,280h
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x280u32, &mut ctx.cpu.flags);
    // 00406e66 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406e67 mov [esp+24h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.edx);
    // 00406e6b mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00406e6f jne near ptr 00406DCAh
    jne(ctx, Cont(x406e75), Cont(x406dca))
}

pub fn x406e75(ctx: &mut Context) -> Cont {
    // 00406e75 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    Cont(x406e76)
}

pub fn x406e76(ctx: &mut Context) -> Cont {
    // 00406e76 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00406e77 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00406e78 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00406e79 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00406e7c ret
    ret(ctx, 0)
}

pub fn x406e80(ctx: &mut Context) -> Cont {
    // 00406e80 sub esp,7Ch
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x7cu32, &mut ctx.cpu.flags);
    // 00406e83 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00406e84 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00406e85 mov ecx,425C20h
    ctx.cpu.regs.ecx = 0x425c20u32;
    // 00406e8a push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00406e8b push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00406e8c mov dword ptr [esp+18h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), 0x0u32);
    // 00406e94 mov [esp+28h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.ecx);
    Cont(x406e98)
}

pub fn x406e98(ctx: &mut Context) -> Cont {
    // 00406e98 mov eax,[esp+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00406e9c mov dword ptr [esp+24h],20h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), 0x20u32);
    // 00406ea4 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    Cont(x406ea8)
}

pub fn x406ea8(ctx: &mut Context) -> Cont {
    // 00406ea8 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00406eac mov esi,[ecx]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(ctx.cpu.regs.ecx);
    // 00406eae mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 00406eb2 mov edx,[ecx+400h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x400u32));
    // 00406eb8 sub edx,esi
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406eba mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406ebf imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406ec1 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406ec4 mov edi,[ecx+4]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x4u32));
    // 00406ec7 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406ec9 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406ecc add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406ece mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406ed3 mov [esp+5Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32), ctx.cpu.regs.edx);
    // 00406ed7 mov edx,[ecx+404h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x404u32));
    // 00406edd sub edx,edi
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406edf mov ebx,[ecx+8]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x8u32));
    // 00406ee2 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406ee4 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406ee7 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406ee9 mov ebp,[ecx+0Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0xcu32));
    // 00406eec shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406eef add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406ef1 mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406ef6 mov [esp+60h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32), ctx.cpu.regs.edx);
    // 00406efa mov edx,[ecx+408h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x408u32));
    // 00406f00 sub edx,ebx
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00406f02 mov [esp+38h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.regs.ebp);
    // 00406f06 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406f08 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406f0b mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406f0d shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406f10 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f12 mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406f17 mov [esp+64h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32), ctx.cpu.regs.edx);
    // 00406f1b mov edx,[ecx+40Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x40cu32));
    // 00406f21 sub edx,ebp
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00406f23 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406f25 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406f28 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406f2a shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406f2d add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f2f mov eax,[ecx+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x10u32));
    // 00406f32 mov [esp+68h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32), ctx.cpu.regs.edx);
    // 00406f36 mov edx,[ecx+410h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x410u32));
    // 00406f3c mov [esp+3Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32), ctx.cpu.regs.eax);
    // 00406f40 sub edx,eax
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f42 mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406f47 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406f49 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406f4c mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406f4e shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406f51 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f53 mov eax,[ecx+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x14u32));
    // 00406f56 mov [esp+6Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32), ctx.cpu.regs.edx);
    // 00406f5a mov edx,[ecx+414h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x414u32));
    // 00406f60 mov [esp+40h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32), ctx.cpu.regs.eax);
    // 00406f64 sub edx,eax
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f66 mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406f6b imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406f6d sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406f70 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406f72 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406f75 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f77 mov eax,[ecx+18h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x18u32));
    // 00406f7a mov [esp+70h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32), ctx.cpu.regs.edx);
    // 00406f7e mov edx,[ecx+418h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x418u32));
    // 00406f84 mov [esp+44h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32), ctx.cpu.regs.eax);
    // 00406f88 sub edx,eax
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f8a mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406f8f imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406f91 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406f94 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406f96 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406f99 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406f9b mov eax,[ecx+1Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x1cu32));
    // 00406f9e mov [esp+74h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32), ctx.cpu.regs.edx);
    // 00406fa2 mov edx,[ecx+41Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add(0x41cu32));
    // 00406fa8 mov [esp+48h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32), ctx.cpu.regs.eax);
    // 00406fac sub edx,eax
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406fae mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406fb3 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406fb5 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406fb8 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406fba mov dword ptr [esp+20h],14h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0x14u32);
    // 00406fc2 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406fc5 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406fc7 mov [esp+78h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32), ctx.cpu.regs.edx);
    Cont(x406fcb)
}

pub fn x406fcb(ctx: &mut Context) -> Cont {
    // 00406fcb mov edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 00406fcf mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406fd4 sub edx,esi
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00406fd6 mov [esp+4Ch],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32), ctx.cpu.regs.esi);
    // 00406fda imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406fdc sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00406fdf mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00406fe1 mov [esp+50h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32), ctx.cpu.regs.edi);
    // 00406fe5 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00406fe8 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00406fea mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00406fef mov [esp+7Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32), ctx.cpu.regs.edx);
    // 00406ff3 mov edx,[esp+40h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 00406ff7 sub edx,edi
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00406ff9 mov [esp+54h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32), ctx.cpu.regs.ebx);
    // 00406ffd imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00406fff sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 00407002 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00407004 mov [esp+58h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32), ctx.cpu.regs.ebp);
    // 00407008 shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 0040700b add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040700d mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00407012 mov [esp+80h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32), ctx.cpu.regs.edx);
    // 00407019 mov edx,[esp+44h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 0040701d sub edx,ebx
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040701f mov dword ptr [esp+1Ch],14h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), 0x14u32);
    // 00407027 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00407029 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 0040702c mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 0040702e shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00407031 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407033 mov eax,66666667h
    ctx.cpu.regs.eax = 0x66666667u32;
    // 00407038 mov [esp+84h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x84u32), ctx.cpu.regs.edx);
    // 0040703f mov edx,[esp+48h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 00407043 sub edx,ebp
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407045 imul edx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.edx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00407047 sar edx,3
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x3u8, &mut ctx.cpu.flags);
    // 0040704a mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 0040704c shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 0040704f add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407051 mov eax,ebp
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp;
    // 00407053 mov [esp+88h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32), ctx.cpu.regs.edx);
    Cont(x40705a)
}

pub fn x40705a(ctx: &mut Context) -> Cont {
    // 0040705a mov edx,eax
    ctx.cpu.regs.edx = ctx.cpu.regs.eax;
    // 0040705c imul eax,[esp+4Ch]
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 00407061 imul edx,[esp+50h]
    let x = ctx.cpu.regs.edx as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.edx = res as u32;
    // 00407066 mov ebp,[esp+90h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x90u32));
    // 0040706d sar edx,17h
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x17u8, &mut ctx.cpu.flags);
    // 00407070 and edx,3Fh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x3fu32, &mut ctx.cpu.flags);
    // 00407073 shl edx,6
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, 0x6u8, &mut ctx.cpu.flags);
    // 00407076 sar eax,17h
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x17u8, &mut ctx.cpu.flags);
    // 00407079 and eax,3Fh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0x3fu32, &mut ctx.cpu.flags);
    // 0040707c add ebp,edx
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040707e xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407080 mov dl,[eax+ebp]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.ebp)),
    );
    // 00407083 mov eax,ds:[428CF8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428cf8u32);
    // 00407088 imul edx,[esp+54h]
    let x = ctx.cpu.regs.edx as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.edx = res as u32;
    // 0040708d mov ebp,[esp+10h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407091 sar edx,0Ah
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0xau8, &mut ctx.cpu.flags);
    // 00407094 mov [eax+ebp],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.ebp),
        ctx.cpu.regs.get_dl(),
    );
    // 00407097 mov edx,[esp+7Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x7cu32));
    // 0040709b mov eax,[esp+80h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x80u32));
    // 004070a2 inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004070a3 mov [esp+10h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ebp);
    // 004070a7 mov ebp,[esp+4Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32));
    // 004070ab add ebp,edx
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004070ad mov edx,[esp+50h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32));
    // 004070b1 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004070b3 mov eax,[esp+54h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32));
    // 004070b7 mov [esp+50h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x50u32), ctx.cpu.regs.edx);
    // 004070bb mov edx,[esp+84h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x84u32));
    // 004070c2 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004070c4 mov edx,[esp+1Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 004070c8 mov [esp+4Ch],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4cu32), ctx.cpu.regs.ebp);
    // 004070cc mov ebp,[esp+88h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x88u32));
    // 004070d3 mov [esp+54h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x54u32), ctx.cpu.regs.eax);
    // 004070d7 mov eax,[esp+58h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32));
    // 004070db add eax,ebp
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004070dd dec edx
    ctx.cpu.regs.edx = dec(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004070de mov [esp+58h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x58u32), ctx.cpu.regs.eax);
    // 004070e2 mov [esp+1Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.edx);
    // 004070e6 jne near ptr 0040705Ah
    jne(ctx, Cont(x4070ec), Cont(x40705a))
}

pub fn x4070ec(ctx: &mut Context) -> Cont {
    // 004070ec mov eax,[esp+5Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x5cu32));
    // 004070f0 mov ebp,[esp+60h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x60u32));
    // 004070f4 mov edx,[esp+64h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x64u32));
    // 004070f8 add esi,eax
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004070fa mov eax,[esp+68h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x68u32));
    // 004070fe add edi,ebp
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407100 mov ebp,[esp+38h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 00407104 add ebx,edx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407106 mov edx,[esp+3Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 0040710a add ebp,eax
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040710c mov eax,[esp+6Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x6cu32));
    // 00407110 mov [esp+38h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32), ctx.cpu.regs.ebp);
    // 00407114 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407116 mov eax,[esp+40h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32));
    // 0040711a mov [esp+3Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32), ctx.cpu.regs.edx);
    // 0040711e mov edx,[esp+70h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x70u32));
    // 00407122 add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407124 mov edx,[esp+44h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32));
    // 00407128 mov [esp+40h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x40u32), ctx.cpu.regs.eax);
    // 0040712c mov eax,[esp+74h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x74u32));
    // 00407130 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407132 mov eax,[esp+48h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32));
    // 00407136 mov [esp+44h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44u32), ctx.cpu.regs.edx);
    // 0040713a mov edx,[esp+78h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x78u32));
    // 0040713e add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407140 mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407144 mov [esp+48h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x48u32), ctx.cpu.regs.eax);
    // 00407148 mov eax,[esp+20h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 0040714c add edx,26Ch
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x26cu32, &mut ctx.cpu.flags);
    // 00407152 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407153 mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 00407157 mov [esp+20h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.eax);
    // 0040715b jne near ptr 00406FCBh
    jne(ctx, Cont(x407161), Cont(x406fcb))
}

pub fn x407161(ctx: &mut Context) -> Cont {
    // 00407161 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407165 mov eax,[esp+24h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00407169 add edx,14h
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x14u32, &mut ctx.cpu.flags);
    // 0040716c add ecx,10h
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x10u32, &mut ctx.cpu.flags);
    // 0040716f dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407170 mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 00407174 mov [esp+24h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.eax);
    // 00407178 jne near ptr 00406EA8h
    jne(ctx, Cont(x40717e), Cont(x406ea8))
}

pub fn x40717e(ctx: &mut Context) -> Cont {
    // 0040717e mov ecx,[esp+28h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00407182 mov esi,[esp+18h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00407186 add ecx,400h
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x400u32, &mut ctx.cpu.flags);
    // 0040718c add esi,3200h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x3200u32, &mut ctx.cpu.flags);
    // 00407192 cmp ecx,428420h
    sub(ctx.cpu.regs.ecx, 0x428420u32, &mut ctx.cpu.flags);
    // 00407198 mov [esp+18h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.esi);
    // 0040719c mov [esp+28h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.ecx);
    // 004071a0 jl near ptr 00406E98h
    jl(ctx, Cont(x4071a6), Cont(x406e98))
}

pub fn x4071a6(ctx: &mut Context) -> Cont {
    // 004071a6 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 004071a7 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004071a8 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 004071a9 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004071aa add esp,7Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x7cu32, &mut ctx.cpu.flags);
    // 004071ad ret
    ret(ctx, 0)
}

pub fn x4071b0(ctx: &mut Context) -> Cont {
    // 004071b0 mov eax,ds:[421714h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x421714u32);
    // 004071b5 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004071b8 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004071b9 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004071ba push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004071bb push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004071bc push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004071bd call 00407E90h
    let dst = Cont(x407e90);
    call(ctx, 0x4071c2, dst)
}

pub fn x4071c2(ctx: &mut Context) -> Cont {
    // 004071c2 mov esi,[esp+34h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 004071c6 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 004071c9 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004071cb mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 004071cd test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004071cf je near ptr 00407322h
    je(ctx, Cont(x4071d5), Cont(x407322))
}

pub fn x4071d5(ctx: &mut Context) -> Cont {
    // 004071d5 mov ebx,[esp+2Ch]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 004071d9 test ebx,ebx
    and(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004071db je near ptr 00407322h
    je(ctx, Cont(x4071e1), Cont(x407322))
}

pub fn x4071e1(ctx: &mut Context) -> Cont {
    // 004071e1 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004071e3 test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004071e5 jle near ptr 00407322h
    jle(ctx, Cont(x4071eb), Cont(x407322))
}

pub fn x4071eb(ctx: &mut Context) -> Cont {
    // 004071eb mov al,[esp+34h]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x34u32)),
    );
    // 004071ef mov ebp,ds:[421714h]
    ctx.cpu.regs.ebp = ctx.memory.read::<u32>(0x421714u32);
    Cont(x4071f5)
}

pub fn x4071f5(ctx: &mut Context) -> Cont {
    // 004071f5 cmp [edx+ebp],al
    sub(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.ebp)),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    );
    // 004071f8 jne short 004071FCh
    jne(ctx, Cont(x4071fa), Cont(x4071fc))
}

pub fn x4071fa(ctx: &mut Context) -> Cont {
    // 004071fa mov ecx,edx
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx;
    Cont(x4071fc)
}

pub fn x4071fc(ctx: &mut Context) -> Cont {
    // 004071fc inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004071fd cmp edx,edi
    sub(ctx.cpu.regs.edx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004071ff jl short 004071F5h
    jl(ctx, Cont(x407201), Cont(x4071f5))
}

pub fn x407201(ctx: &mut Context) -> Cont {
    // 00407201 test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407203 je near ptr 00407322h
    je(ctx, Cont(x407209), Cont(x407322))
}

pub fn x407209(ctx: &mut Context) -> Cont {
    // 00407209 mov edi,[esp+24h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0040720d mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 0040720f cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00407210 sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407212 mov ebp,[esp+28h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00407216 sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 00407218 sub edi,eax
    ctx.cpu.regs.edi = sub(ctx.cpu.regs.edi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040721a mov eax,8000h
    ctx.cpu.regs.eax = 0x8000u32;
    // 0040721f cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00407220 idiv ebx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ebx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 00407222 shl ecx,8
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x8u8, &mut ctx.cpu.flags);
    // 00407225 mov [esp+18h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.edi);
    // 00407229 mov dword ptr [esp+34h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), 0x0u32);
    // 00407231 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 00407235 mov eax,8000h
    ctx.cpu.regs.eax = 0x8000u32;
    // 0040723a cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040723b idiv esi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.esi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0040723d mov [esp+1Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.eax);
    // 00407241 mov eax,ds:[428CE0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x428ce0u32);
    // 00407246 add ecx,eax
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407248 mov eax,esi
    ctx.cpu.regs.eax = ctx.cpu.regs.esi;
    // 0040724a cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040724b sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040724d mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00407251 sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 00407253 sub ebp,eax
    ctx.cpu.regs.ebp = sub(ctx.cpu.regs.ebp, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407255 test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407257 jle near ptr 00407322h
    jle(ctx, Cont(x40725d), Cont(x407322))
}

pub fn x40725d(ctx: &mut Context) -> Cont {
    // 0040725d lea ebp,[ebp+ebp*4]
    ctx.cpu.regs.ebp = ctx.cpu.regs.ebp.wrapping_add((ctx.cpu.regs.ebp * 4));
    // 00407261 mov [esp+28h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.esi);
    // 00407265 shl ebp,7
    ctx.cpu.regs.ebp = shl(ctx.cpu.regs.ebp, 0x7u8, &mut ctx.cpu.flags);
    Cont(x407268)
}

pub fn x407268(ctx: &mut Context) -> Cont {
    // 00407268 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0040726a mov esi,edi
    ctx.cpu.regs.esi = ctx.cpu.regs.edi;
    // 0040726c test ebx,ebx
    and(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0040726e mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 00407272 jle near ptr 004072FFh
    jle(ctx, Cont(x407278), Cont(x4072ff))
}

pub fn x407278(ctx: &mut Context) -> Cont {
    // 00407278 mov [esp+24h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.ebx);
    Cont(x40727c)
}

pub fn x40727c(ctx: &mut Context) -> Cont {
    // 0040727c test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040727e jl short 004072E1h
    jl(ctx, Cont(x407280), Cont(x4072e1))
}

pub fn x407280(ctx: &mut Context) -> Cont {
    // 00407280 cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00407286 jge short 004072E1h
    jge(ctx, Cont(x407288), Cont(x4072e1))
}

pub fn x407288(ctx: &mut Context) -> Cont {
    // 00407288 test ebp,ebp
    and(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0040728a jl short 004072E1h
    jl(ctx, Cont(x40728c), Cont(x4072e1))
}

pub fn x40728c(ctx: &mut Context) -> Cont {
    // 0040728c cmp ebp,1F400h
    sub(ctx.cpu.regs.ebp, 0x1f400u32, &mut ctx.cpu.flags);
    // 00407292 jge short 004072E1h
    jge(ctx, Cont(x407294), Cont(x4072e1))
}

pub fn x407294(ctx: &mut Context) -> Cont {
    // 00407294 mov edx,[esp+34h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00407298 mov ebx,[esp+10h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0040729c sar edx,7
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0x7u8, &mut ctx.cpu.flags);
    // 0040729f and edx,0FFFFFFF0h
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0xfffffff0u32, &mut ctx.cpu.flags);
    // 004072a2 mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 004072a8 sar eax,0Bh
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0xbu8, &mut ctx.cpu.flags);
    // 004072ab add ebx,edx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004072ad xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004072af lea ecx,[esi+ebp]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.ebp);
    // 004072b2 mov dl,[eax+ebx]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.ebx)),
    );
    // 004072b5 mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 004072b7 mov edx,[esp+38h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x38u32));
    // 004072bb and edx,0FFh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0xffu32, &mut ctx.cpu.flags);
    // 004072c1 imul eax,edx
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 004072c4 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004072c6 mov dl,[edi+ecx]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx)),
    );
    // 004072c9 sar eax,8
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 004072cc add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004072ce cmp eax,0FFh
    sub(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 004072d3 jle short 004072DAh
    jle(ctx, Cont(x4072d5), Cont(x4072da))
}

pub fn x4072d5(ctx: &mut Context) -> Cont {
    // 004072d5 mov eax,0FFh
    ctx.cpu.regs.eax = 0xffu32;
    Cont(x4072da)
}

pub fn x4072da(ctx: &mut Context) -> Cont {
    // 004072da mov ebx,[esp+2Ch]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 004072de mov [edi+ecx],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx),
        ctx.cpu.regs.get_al(),
    );
    Cont(x4072e1)
}

pub fn x4072e1(ctx: &mut Context) -> Cont {
    // 004072e1 mov eax,[esp+30h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32));
    // 004072e5 mov edi,[esp+14h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 004072e9 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 004072ed add eax,edi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004072ef inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004072f0 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004072f1 mov [esp+30h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), ctx.cpu.regs.eax);
    // 004072f5 mov [esp+24h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.ecx);
    // 004072f9 jne short 0040727Ch
    jne(ctx, Cont(x4072fb), Cont(x40727c))
}

pub fn x4072fb(ctx: &mut Context) -> Cont {
    // 004072fb mov edi,[esp+18h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    Cont(x4072ff)
}

pub fn x4072ff(ctx: &mut Context) -> Cont {
    // 004072ff mov eax,[esp+1Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407303 mov edx,[esp+34h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32));
    // 00407307 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407309 mov eax,[esp+28h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 0040730d add ebp,280h
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x280u32, &mut ctx.cpu.flags);
    // 00407313 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407314 mov [esp+34h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), ctx.cpu.regs.edx);
    // 00407318 mov [esp+28h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.eax);
    // 0040731c jne near ptr 00407268h
    jne(ctx, Cont(x407322), Cont(x407268))
}

pub fn x407322(ctx: &mut Context) -> Cont {
    // 00407322 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407323 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407324 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407325 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407326 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00407329 ret
    ret(ctx, 0)
}

pub fn x407330(ctx: &mut Context) -> Cont {
    // 00407330 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407331 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407332 mov ebp,[esp+24h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00407336 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00407337 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00407338 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407339 call 00407E90h
    let dst = Cont(x407e90);
    call(ctx, 0x40733e, dst)
}

pub fn x40733e(ctx: &mut Context) -> Cont {
    // 0040733e mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00407340 imul eax,[esp+28h]
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 00407345 mov edi,[esp+18h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00407349 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0040734c cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0040734d sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040734f xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407351 sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 00407353 sub edi,eax
    ctx.cpu.regs.edi = sub(ctx.cpu.regs.edi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407355 test ebx,ebx
    and(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407357 jle short 00407385h
    jle(ctx, Cont(x407359), Cont(x407385))
}

pub fn x407359(ctx: &mut Context) -> Cont {
    // 00407359 mov eax,[esp+28h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 0040735d mov cl,[esi+ebp]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.ebp)),
    );
    // 00407360 mov edx,[esp+20h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00407364 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00407365 mov eax,[esp+20h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 00407369 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0040736a mov ecx,[esp+20h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 0040736e push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0040736f push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00407370 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00407371 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00407372 call 004071B0h
    let dst = Cont(x4071b0);
    call(ctx, 0x407377, dst)
}

pub fn x407377(ctx: &mut Context) -> Cont {
    // 00407377 mov ecx,[esp+3Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 0040737b add esp,18h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x18u32, &mut ctx.cpu.flags);
    // 0040737e inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040737f add edi,ecx
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407381 cmp esi,ebx
    sub(ctx.cpu.regs.esi, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407383 jl short 00407359h
    jl(ctx, Cont(x407385), Cont(x407359))
}

pub fn x407385(ctx: &mut Context) -> Cont {
    // 00407385 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407386 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407387 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407388 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407389 ret
    ret(ctx, 0)
}

pub fn x407390(ctx: &mut Context) -> Cont {
    // 00407390 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00407391 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00407392 mov esi,421724h
    ctx.cpu.regs.esi = 0x421724u32;
    Cont(x407397)
}

pub fn x407397(ctx: &mut Context) -> Cont {
    // 00407397 fld dword ptr [esi-0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32)) as f64,
    );
    // 0040739a fcomp dword ptr [esi-8]
    ctx.cpu.fpu.cmp = ctx.cpu.fpu.get(0).total_cmp(
        &(ctx
            .memory
            .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0xfffffff8u32)) as f64),
    );
    ctx.cpu.fpu.pop();
    // 0040739d fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0040739f test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 004073a2 jne short 0040740Eh
    jne(ctx, Cont(x4073a4), Cont(x40740e))
}

pub fn x4073a4(ctx: &mut Context) -> Cont {
    // 004073a4 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4073a9, dst)
}

pub fn x4073a9(ctx: &mut Context) -> Cont {
    // 004073a9 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004073aa mov ecx,32h
    ctx.cpu.regs.ecx = 0x32u32;
    // 004073af idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004073b1 neg edx
    ctx.cpu.regs.edx = neg(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004073b3 mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004073b7 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004073bb fstp dword ptr [esi-0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004073be call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4073c3, dst)
}

pub fn x4073c3(ctx: &mut Context) -> Cont {
    // 004073c3 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004073c4 mov ecx,32h
    ctx.cpu.regs.ecx = 0x32u32;
    // 004073c9 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004073cb mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004073cf fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004073d3 fstp dword ptr [esi-8]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff8u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004073d6 call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4073db, dst)
}

pub fn x4073db(ctx: &mut Context) -> Cont {
    // 004073db cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004073dc mov ecx,280h
    ctx.cpu.regs.ecx = 0x280u32;
    // 004073e1 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004073e3 mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 004073e7 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 004073eb fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 004073ee call 00407E10h
    let dst = Cont(x407e10);
    call(ctx, 0x4073f3, dst)
}

pub fn x4073f3(ctx: &mut Context) -> Cont {
    // 004073f3 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004073f4 mov ecx,3Ch
    ctx.cpu.regs.ecx = 0x3cu32;
    // 004073f9 idiv ecx
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.ecx as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004073fb sub edx,1Eh
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, 0x1eu32, &mut ctx.cpu.flags);
    // 004073fe mov [esp+4],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.edx);
    // 00407402 fild dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as i32 as f64,
    );
    // 00407406 fmul dword ptr ds:[4204A4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204a4u32) as f64,
    );
    // 0040740c fstp dword ptr [esi]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.esi, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    Cont(x40740e)
}

pub fn x40740e(ctx: &mut Context) -> Cont {
    // 0040740e fld dword ptr [esi-0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32)) as f64,
    );
    // 00407411 fadd dword ptr ds:[420128h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(0x420128u32) as f64,
    );
    // 00407417 lea edx,[esi-0Ch]
    ctx.cpu.regs.edx = ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32);
    // 0040741a test edx,edx
    and(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0040741c fstp dword ptr [esi-0Ch]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffff4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040741f fld dword ptr [esi-4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32)) as f64,
    );
    // 00407422 fadd dword ptr [esi]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) + ctx.memory.read::<f32>(ctx.cpu.regs.esi) as f64,
    );
    // 00407424 fst dword ptr [esp+4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x4u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    // 00407428 fstp dword ptr [esi-4]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0040742b jbe short 0040745Ch
    jbe(ctx, Cont(x40742d), Cont(x40745c))
}

pub fn x40742d(ctx: &mut Context) -> Cont {
    // 0040742d fld dword ptr [esp+4]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x4u32)) as f64,
    );
    // 00407431 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x407436, dst)
}

pub fn x407436(ctx: &mut Context) -> Cont {
    // 00407436 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407438 jl short 0040745Ch
    jl(ctx, Cont(x40743a), Cont(x40745c))
}

pub fn x40743a(ctx: &mut Context) -> Cont {
    // 0040743a cmp eax,280h
    sub(ctx.cpu.regs.eax, 0x280u32, &mut ctx.cpu.flags);
    // 0040743f jge short 0040745Ch
    jge(ctx, Cont(x407441), Cont(x40745c))
}

pub fn x407441(ctx: &mut Context) -> Cont {
    // 00407441 mov ecx,0C8h
    ctx.cpu.regs.ecx = 0xc8u32;
    Cont(x407446)
}

pub fn x407446(ctx: &mut Context) -> Cont {
    // 00407446 mov edx,ds:[428CF8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428cf8u32);
    // 0040744c add eax,280h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x280u32, &mut ctx.cpu.flags);
    // 00407451 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407452 mov byte ptr [edx+eax-280h],0FAh
    ctx.memory.write::<u8>(
        ctx.cpu
            .regs
            .edx
            .wrapping_add(ctx.cpu.regs.eax)
            .wrapping_add(0xfffffd80u32),
        0xfau8,
    );
    // 0040745a jne short 00407446h
    jne(ctx, Cont(x40745c), Cont(x407446))
}

pub fn x40745c(ctx: &mut Context) -> Cont {
    // 0040745c add esi,10h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x10u32, &mut ctx.cpu.flags);
    // 0040745f cmp esi,421764h
    sub(ctx.cpu.regs.esi, 0x421764u32, &mut ctx.cpu.flags);
    // 00407465 jl near ptr 00407397h
    jl(ctx, Cont(x40746b), Cont(x407397))
}

pub fn x40746b(ctx: &mut Context) -> Cont {
    // 0040746b pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040746c pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 0040746d ret
    ret(ctx, 0)
}

pub fn x407470(ctx: &mut Context) -> Cont {
    // 00407470 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407471 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407472 mov ebp,[esp+18h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00407476 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00407477 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00407478 mov edi,[esp+18h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 0040747c test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040747e jge short 00407488h
    jge(ctx, Cont(x407480), Cont(x407488))
}

pub fn x407480(ctx: &mut Context) -> Cont {
    // 00407480 test ebp,ebp
    and(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407482 jl near ptr 00407610h
    jl(ctx, Cont(x407488), Cont(x407610))
}

pub fn x407488(ctx: &mut Context) -> Cont {
    // 00407488 mov ebx,[esp+14h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0040748c mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407490 test ebx,ebx
    and(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407492 jge short 0040749Ch
    jge(ctx, Cont(x407494), Cont(x40749c))
}

pub fn x407494(ctx: &mut Context) -> Cont {
    // 00407494 test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407496 jl near ptr 00407610h
    jl(ctx, Cont(x40749c), Cont(x407610))
}

pub fn x40749c(ctx: &mut Context) -> Cont {
    // 0040749c cmp edi,0C7h
    sub(ctx.cpu.regs.edi, 0xc7u32, &mut ctx.cpu.flags);
    // 004074a2 jle short 004074B0h
    jle(ctx, Cont(x4074a4), Cont(x4074b0))
}

pub fn x4074a4(ctx: &mut Context) -> Cont {
    // 004074a4 cmp ebp,0C7h
    sub(ctx.cpu.regs.ebp, 0xc7u32, &mut ctx.cpu.flags);
    // 004074aa jg near ptr 00407610h
    jg(ctx, Cont(x4074b0), Cont(x407610))
}

pub fn x4074b0(ctx: &mut Context) -> Cont {
    // 004074b0 cmp ebx,27Fh
    sub(ctx.cpu.regs.ebx, 0x27fu32, &mut ctx.cpu.flags);
    // 004074b6 jle short 004074C4h
    jle(ctx, Cont(x4074b8), Cont(x4074c4))
}

pub fn x4074b8(ctx: &mut Context) -> Cont {
    // 004074b8 cmp ecx,27Fh
    sub(ctx.cpu.regs.ecx, 0x27fu32, &mut ctx.cpu.flags);
    // 004074be jg near ptr 00407610h
    jg(ctx, Cont(x4074c4), Cont(x407610))
}

pub fn x4074c4(ctx: &mut Context) -> Cont {
    // 004074c4 cmp edi,ebp
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 004074c6 jne short 00407522h
    jne(ctx, Cont(x4074c8), Cont(x407522))
}

pub fn x4074c8(ctx: &mut Context) -> Cont {
    // 004074c8 cmp ecx,ebx
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004074ca jge short 004074D2h
    jge(ctx, Cont(x4074cc), Cont(x4074d2))
}

pub fn x4074cc(ctx: &mut Context) -> Cont {
    // 004074cc mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 004074ce mov ebx,ecx
    ctx.cpu.regs.ebx = ctx.cpu.regs.ecx;
    // 004074d0 mov ecx,eax
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax;
    Cont(x4074d2)
}

pub fn x4074d2(ctx: &mut Context) -> Cont {
    // 004074d2 test ebx,ebx
    and(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004074d4 jge short 004074D8h
    jge(ctx, Cont(x4074d6), Cont(x4074d8))
}

pub fn x4074d6(ctx: &mut Context) -> Cont {
    // 004074d6 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    Cont(x4074d8)
}

pub fn x4074d8(ctx: &mut Context) -> Cont {
    // 004074d8 cmp ecx,27Fh
    sub(ctx.cpu.regs.ecx, 0x27fu32, &mut ctx.cpu.flags);
    // 004074de jle short 004074E5h
    jle(ctx, Cont(x4074e0), Cont(x4074e5))
}

pub fn x4074e0(ctx: &mut Context) -> Cont {
    // 004074e0 mov ecx,27Fh
    ctx.cpu.regs.ecx = 0x27fu32;
    Cont(x4074e5)
}

pub fn x4074e5(ctx: &mut Context) -> Cont {
    // 004074e5 mov esi,ds:[428CF8h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428cf8u32);
    // 004074eb lea edi,[edi+edi*4]
    ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add((ctx.cpu.regs.edi * 4));
    // 004074ee shl edi,7
    ctx.cpu.regs.edi = shl(ctx.cpu.regs.edi, 0x7u8, &mut ctx.cpu.flags);
    // 004074f1 add edi,esi
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004074f3 add edi,ebx
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004074f5 cmp ebx,ecx
    sub(ctx.cpu.regs.ebx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004074f7 jge near ptr 00407610h
    jge(ctx, Cont(x4074fd), Cont(x407610))
}

pub fn x4074fd(ctx: &mut Context) -> Cont {
    // 004074fd mov al,[esp+24h]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x24u32)),
    );
    // 00407501 sub ecx,ebx
    ctx.cpu.regs.ecx = sub(ctx.cpu.regs.ecx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407503 mov bl,al
    ctx.cpu.regs.set_bl(ctx.cpu.regs.get_al());
    // 00407505 mov edx,ecx
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx;
    // 00407507 mov bh,bl
    ctx.cpu.regs.set_bh(ctx.cpu.regs.get_bl());
    // 00407509 mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 0040750b shl eax,10h
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 0040750e mov ax,bx
    ctx.cpu.regs.set_ax(ctx.cpu.regs.get_bx());
    // 00407511 shr ecx,2
    ctx.cpu.regs.ecx = shr(ctx.cpu.regs.ecx, 0x2u8, &mut ctx.cpu.flags);
    // 00407514 rep stosd
    rep(ctx, Rep::REP, stosd);
    // 00407516 mov ecx,edx
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx;
    // 00407518 and ecx,3
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3u32, &mut ctx.cpu.flags);
    // 0040751b rep stosb
    rep(ctx, Rep::REP, stosb);
    // 0040751d pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040751e pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040751f pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407520 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407521 ret
    ret(ctx, 0)
}

pub fn x407522(ctx: &mut Context) -> Cont {
    // 00407522 cmp ebx,ecx
    sub(ctx.cpu.regs.ebx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407524 jne short 00407570h
    jne(ctx, Cont(x407526), Cont(x407570))
}

pub fn x407526(ctx: &mut Context) -> Cont {
    // 00407526 cmp ebp,edi
    sub(ctx.cpu.regs.ebp, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407528 jge short 00407530h
    jge(ctx, Cont(x40752a), Cont(x407530))
}

pub fn x40752a(ctx: &mut Context) -> Cont {
    // 0040752a mov eax,edi
    ctx.cpu.regs.eax = ctx.cpu.regs.edi;
    // 0040752c mov edi,ebp
    ctx.cpu.regs.edi = ctx.cpu.regs.ebp;
    // 0040752e mov ebp,eax
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    Cont(x407530)
}

pub fn x407530(ctx: &mut Context) -> Cont {
    // 00407530 test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407532 jge short 00407536h
    jge(ctx, Cont(x407534), Cont(x407536))
}

pub fn x407534(ctx: &mut Context) -> Cont {
    // 00407534 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    Cont(x407536)
}

pub fn x407536(ctx: &mut Context) -> Cont {
    // 00407536 cmp ebp,0C7h
    sub(ctx.cpu.regs.ebp, 0xc7u32, &mut ctx.cpu.flags);
    // 0040753c jle short 00407543h
    jle(ctx, Cont(x40753e), Cont(x407543))
}

pub fn x40753e(ctx: &mut Context) -> Cont {
    // 0040753e mov ebp,0C7h
    ctx.cpu.regs.ebp = 0xc7u32;
    Cont(x407543)
}

pub fn x407543(ctx: &mut Context) -> Cont {
    // 00407543 mov esi,ds:[428CF8h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00407549 lea eax,[edi+edi*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.edi.wrapping_add((ctx.cpu.regs.edi * 4));
    // 0040754c shl eax,7
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x7u8, &mut ctx.cpu.flags);
    // 0040754f add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407551 add eax,ebx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407553 cmp edi,ebp
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407555 jge near ptr 00407610h
    jge(ctx, Cont(x40755b), Cont(x407610))
}

pub fn x40755b(ctx: &mut Context) -> Cont {
    // 0040755b mov cl,[esp+24h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x24u32)),
    );
    // 0040755f sub ebp,edi
    ctx.cpu.regs.ebp = sub(ctx.cpu.regs.ebp, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    Cont(x407561)
}

pub fn x407561(ctx: &mut Context) -> Cont {
    // 00407561 mov [eax],cl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.eax, ctx.cpu.regs.get_cl());
    // 00407563 add eax,280h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x280u32, &mut ctx.cpu.flags);
    // 00407568 dec ebp
    ctx.cpu.regs.ebp = dec(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407569 jne short 00407561h
    jne(ctx, Cont(x40756b), Cont(x407561))
}

pub fn x40756b(ctx: &mut Context) -> Cont {
    // 0040756b pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040756c pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040756d pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040756e pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0040756f ret
    ret(ctx, 0)
}

pub fn x407570(ctx: &mut Context) -> Cont {
    // 00407570 sub ecx,ebx
    ctx.cpu.regs.ecx = sub(ctx.cpu.regs.ecx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407572 mov [esp+20h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.ecx);
    // 00407576 fild dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as i32 as f64,
    );
    // 0040757a call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x40757f, dst)
}

pub fn x40757f(ctx: &mut Context) -> Cont {
    // 0040757f cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 00407580 sub ebp,edi
    ctx.cpu.regs.ebp = sub(ctx.cpu.regs.ebp, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407582 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00407584 shl ebx,0Ah
    ctx.cpu.regs.ebx = shl(ctx.cpu.regs.ebx, 0xau8, &mut ctx.cpu.flags);
    // 00407587 mov [esp+14h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.ebp);
    // 0040758b mov [esp+18h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ebx);
    // 0040758f fild dword ptr [esp+14h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32)) as i32 as f64,
    );
    // 00407593 xor esi,edx
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407595 mov ebx,edi
    ctx.cpu.regs.ebx = ctx.cpu.regs.edi;
    // 00407597 sub esi,edx
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407599 shl ebx,0Ah
    ctx.cpu.regs.ebx = shl(ctx.cpu.regs.ebx, 0xau8, &mut ctx.cpu.flags);
    // 0040759c call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x4075a1, dst)
}

pub fn x4075a1(ctx: &mut Context) -> Cont {
    // 004075a1 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004075a2 xor eax,edx
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004075a4 sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004075a6 cmp eax,esi
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004075a8 jle short 004075ACh
    jle(ctx, Cont(x4075aa), Cont(x4075ac))
}

pub fn x4075aa(ctx: &mut Context) -> Cont {
    // 004075aa mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    Cont(x4075ac)
}

pub fn x4075ac(ctx: &mut Context) -> Cont {
    // 004075ac test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004075ae jle short 00407610h
    jle(ctx, Cont(x4075b0), Cont(x407610))
}

pub fn x4075b0(ctx: &mut Context) -> Cont {
    // 004075b0 mov eax,[esp+20h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 004075b4 shl eax,0Ah
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0xau8, &mut ctx.cpu.flags);
    // 004075b7 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004075b8 idiv esi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.esi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004075ba mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 004075bc mov eax,ebp
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp;
    // 004075be shl eax,0Ah
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0xau8, &mut ctx.cpu.flags);
    // 004075c1 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 004075c2 idiv esi
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.cpu.regs.esi as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 004075c4 test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004075c6 mov ebp,eax
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    // 004075c8 jle short 00407610h
    jle(ctx, Cont(x4075ca), Cont(x407610))
}

pub fn x4075ca(ctx: &mut Context) -> Cont {
    // 004075ca mov al,[esp+24h]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x24u32)),
    );
    Cont(x4075ce)
}

pub fn x4075ce(ctx: &mut Context) -> Cont {
    // 004075ce mov edx,[esp+18h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 004075d2 mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 004075d4 sar edx,0Ah
    ctx.cpu.regs.edx = sar(ctx.cpu.regs.edx, 0xau8, &mut ctx.cpu.flags);
    // 004075d7 sar ecx,0Ah
    ctx.cpu.regs.ecx = sar(ctx.cpu.regs.ecx, 0xau8, &mut ctx.cpu.flags);
    // 004075da test edx,edx
    and(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004075dc jl short 00407601h
    jl(ctx, Cont(x4075de), Cont(x407601))
}

pub fn x4075de(ctx: &mut Context) -> Cont {
    // 004075de test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004075e0 jl short 00407601h
    jl(ctx, Cont(x4075e2), Cont(x407601))
}

pub fn x4075e2(ctx: &mut Context) -> Cont {
    // 004075e2 cmp edx,27Fh
    sub(ctx.cpu.regs.edx, 0x27fu32, &mut ctx.cpu.flags);
    // 004075e8 jg short 00407601h
    jg(ctx, Cont(x4075ea), Cont(x407601))
}

pub fn x4075ea(ctx: &mut Context) -> Cont {
    // 004075ea cmp ecx,0C7h
    sub(ctx.cpu.regs.ecx, 0xc7u32, &mut ctx.cpu.flags);
    // 004075f0 jg short 00407601h
    jg(ctx, Cont(x4075f2), Cont(x407601))
}

pub fn x4075f2(ctx: &mut Context) -> Cont {
    // 004075f2 lea ecx,[ecx+ecx*4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.ecx * 4));
    // 004075f5 shl ecx,7
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x7u8, &mut ctx.cpu.flags);
    // 004075f8 add ecx,ds:[428CF8h]
    ctx.cpu.regs.ecx = add(
        ctx.cpu.regs.ecx,
        ctx.memory.read::<u32>(0x428cf8u32),
        &mut ctx.cpu.flags,
    );
    // 004075fe mov [ecx+edx],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.edx),
        ctx.cpu.regs.get_al(),
    );
    Cont(x407601)
}

pub fn x407601(ctx: &mut Context) -> Cont {
    // 00407601 mov ecx,[esp+18h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00407605 add ebx,ebp
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407607 add ecx,edi
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407609 dec esi
    ctx.cpu.regs.esi = dec(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040760a mov [esp+18h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ecx);
    // 0040760e jne short 004075CEh
    jne(ctx, Cont(x407610), Cont(x4075ce))
}

pub fn x407610(ctx: &mut Context) -> Cont {
    // 00407610 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407611 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407612 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407613 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407614 ret
    ret(ctx, 0)
}

pub fn x407620(ctx: &mut Context) -> Cont {
    // 00407620 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407621 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 00407623 sub esp,24h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x24u32, &mut ctx.cpu.flags);
    // 00407626 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407627 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407629 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0040762a push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0040762b mov [ebp-8],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.eax,
    );
    // 0040762e mov [ebp-4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.eax,
    );
    Cont(x407631)
}

pub fn x407631(ctx: &mut Context) -> Cont {
    // 00407631 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x407633)
}

pub fn x407633(ctx: &mut Context) -> Cont {
    // 00407633 mov edx,ds:[425C00h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425c00u32);
    // 00407639 mov ecx,ds:[428CE4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x428ce4u32);
    // 0040763f mov edi,ds:[425C04h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425c04u32);
    // 00407645 mov ebx,[edx+eax*4]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add((ctx.cpu.regs.eax * 4)));
    // 00407648 mov ecx,[ecx+esi*4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.esi * 4)));
    // 0040764b mov edx,ds:[428CE8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x428ce8u32);
    // 00407651 mov al,[edi+eax]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.eax)),
    );
    // 00407654 add ecx,ebx
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407656 or bl,0FFh
    ctx.cpu
        .regs
        .set_bl(or(ctx.cpu.regs.get_bl(), 0xffu8, &mut ctx.cpu.flags));
    // 00407659 mov dl,[edx+esi]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.esi)),
    );
    // 0040765c mov [ebp-10h],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32),
        ctx.cpu.regs.get_al(),
    );
    // 0040765f mov [ebp-14h],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffecu32),
        ctx.cpu.regs.get_dl(),
    );
    // 00407662 sub bl,dl
    ctx.cpu.regs.set_bl(sub(
        ctx.cpu.regs.get_bl(),
        ctx.cpu.regs.get_dl(),
        &mut ctx.cpu.flags,
    ));
    // 00407664 or dl,0FFh
    ctx.cpu
        .regs
        .set_dl(or(ctx.cpu.regs.get_dl(), 0xffu8, &mut ctx.cpu.flags));
    // 00407667 mov [ebp-0Ch],bl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff4u32),
        ctx.cpu.regs.get_bl(),
    );
    // 0040766a sub dl,al
    ctx.cpu.regs.set_dl(sub(
        ctx.cpu.regs.get_dl(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    ));
    // 0040766c mov eax,[ebp-0Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff4u32));
    // 0040766f mov edi,ds:[428D6Ch]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428d6cu32);
    // 00407675 and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0040767a mov [ebp-20h],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe0u32),
        ctx.cpu.regs.eax,
    );
    // 0040767d mov eax,[ebp-10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff0u32));
    // 00407680 and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 00407685 mov [ebp-18h],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe8u32),
        ctx.cpu.regs.get_dl(),
    );
    // 00407688 mov ebx,[ebp-18h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe8u32));
    // 0040768b mov edx,[ebp-14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffecu32));
    // 0040768e mov [ebp-1Ch],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32),
        ctx.cpu.regs.eax,
    );
    // 00407691 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407693 mov al,[edi+ecx+1]
    ctx.cpu.regs.set_al(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .edi
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x1u32),
        ),
    );
    // 00407697 and ebx,0FFh
    ctx.cpu.regs.ebx = and(ctx.cpu.regs.ebx, 0xffu32, &mut ctx.cpu.flags);
    // 0040769d mov [ebp-24h],ebx
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xffffffdcu32),
        ctx.cpu.regs.ebx,
    );
    // 004076a0 imul eax,ebx
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.ebx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 004076a3 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076a5 and edx,0FFh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0xffu32, &mut ctx.cpu.flags);
    // 004076ab mov bl,[edi+ecx+281h]
    ctx.cpu.regs.set_bl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .edi
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x281u32),
        ),
    );
    // 004076b2 imul eax,edx
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 004076b5 imul ebx,edx
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076b8 mov edx,[ebp-1Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe4u32));
    // 004076bb imul ebx,edx
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076be add eax,ebx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076c0 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076c2 mov bl,[edi+ecx+280h]
    ctx.cpu.regs.set_bl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .edi
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x280u32),
        ),
    );
    // 004076c9 imul ebx,edx
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076cc mov edx,[ebp-20h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffe0u32));
    // 004076cf imul ebx,edx
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076d2 add eax,ebx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076d4 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076d6 mov bl,[edi+ecx]
    ctx.cpu.regs.set_bl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx)),
    );
    // 004076d9 mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 004076df imul ebx,[ebp-24h]
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xffffffdcu32)) as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076e3 imul ebx,edx
    let x = ctx.cpu.regs.ebx as i32;
    let y = ctx.cpu.regs.edx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.ebx = res as u32;
    // 004076e6 add eax,ebx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004076e8 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004076ea sar eax,10h
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 004076ed and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 004076f2 lea ecx,[eax+eax*2]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 2));
    // 004076f5 mov eax,[ebp-8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32));
    // 004076f8 mov dl,[edi+eax]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.eax)),
    );
    // 004076fb add ecx,edx
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004076fd sar ecx,2
    ctx.cpu.regs.ecx = sar(ctx.cpu.regs.ecx, 0x2u8, &mut ctx.cpu.flags);
    // 00407700 mov [edi+eax],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.eax),
        ctx.cpu.regs.get_cl(),
    );
    // 00407703 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407704 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407705 mov [ebp-8],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffff8u32),
        ctx.cpu.regs.eax,
    );
    // 00407708 mov eax,[ebp-4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32));
    // 0040770b cmp esi,280h
    sub(ctx.cpu.regs.esi, 0x280u32, &mut ctx.cpu.flags);
    // 00407711 jl near ptr 00407633h
    jl(ctx, Cont(x407717), Cont(x407633))
}

pub fn x407717(ctx: &mut Context) -> Cont {
    // 00407717 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407718 cmp eax,0C8h
    sub(ctx.cpu.regs.eax, 0xc8u32, &mut ctx.cpu.flags);
    // 0040771d mov [ebp-4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.eax,
    );
    // 00407720 jl near ptr 00407631h
    jl(ctx, Cont(x407726), Cont(x407631))
}

pub fn x407726(ctx: &mut Context) -> Cont {
    // 00407726 mov esi,ds:[428D6Ch]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428d6cu32);
    // 0040772c mov edi,ds:[428CF8h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00407732 mov ecx,1F400h
    ctx.cpu.regs.ecx = 0x1f400u32;
    // 00407737 xor ax,ax
    ctx.cpu.regs.set_ax(xor(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    ));
    Cont(x40773a)
}

pub fn x40773a(ctx: &mut Context) -> Cont {
    // 0040773a xor bx,bx
    ctx.cpu.regs.set_bx(xor(
        ctx.cpu.regs.get_bx(),
        ctx.cpu.regs.get_bx(),
        &mut ctx.cpu.flags,
    ));
    // 0040773d mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 0040773f shr al,1
    ctx.cpu
        .regs
        .set_al(shr(ctx.cpu.regs.get_al(), 0x1u8, &mut ctx.cpu.flags));
    // 00407741 mov bl,[edi]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 00407743 shr bl,1
    ctx.cpu
        .regs
        .set_bl(shr(ctx.cpu.regs.get_bl(), 0x1u8, &mut ctx.cpu.flags));
    // 00407745 add al,bl
    ctx.cpu.regs.set_al(add(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_bl(),
        &mut ctx.cpu.flags,
    ));
    // 00407747 jae short 0040774Bh
    jae(ctx, Cont(x407749), Cont(x40774b))
}

pub fn x407749(ctx: &mut Context) -> Cont {
    // 00407749 mov al,0FFh
    ctx.cpu.regs.set_al(0xffu8);
    Cont(x40774b)
}

pub fn x40774b(ctx: &mut Context) -> Cont {
    // 0040774b mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 0040774d inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0040774e inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040774f loop 0040773Ah
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x407751)
    } else {
        Cont(x40773a)
    }
}

pub fn x407751(ctx: &mut Context) -> Cont {
    // 00407751 mov esi,ds:[428CF8h]
    ctx.cpu.regs.esi = ctx.memory.read::<u32>(0x428cf8u32);
    // 00407757 mov edi,ds:[428D6Ch]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x428d6cu32);
    // 0040775d mov ecx,7D00h
    ctx.cpu.regs.ecx = 0x7d00u32;
    // 00407762 rep movsd
    rep(ctx, Rep::REP, movsd);
    // 00407764 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407765 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407766 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407767 mov esp,ebp
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    // 00407769 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040776a ret
    ret(ctx, 0)
}

pub fn x407770(ctx: &mut Context) -> Cont {
    // 00407770 mov ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00407774 mov eax,2
    ctx.cpu.regs.eax = 0x2u32;
    // 00407779 test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040777b jne short 00407783h
    jne(ctx, Cont(x40777d), Cont(x407783))
}

pub fn x40777d(ctx: &mut Context) -> Cont {
    // 0040777d mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 00407782 ret
    ret(ctx, 0)
}

pub fn x407783(ctx: &mut Context) -> Cont {
    // 00407783 cmp ecx,1
    sub(ctx.cpu.regs.ecx, 0x1u32, &mut ctx.cpu.flags);
    // 00407786 jne short 0040778Eh
    jne(ctx, Cont(x407788), Cont(x40778e))
}

pub fn x407788(ctx: &mut Context) -> Cont {
    // 00407788 mov eax,2
    ctx.cpu.regs.eax = 0x2u32;
    // 0040778d ret
    ret(ctx, 0)
}

pub fn x40778e(ctx: &mut Context) -> Cont {
    // 0040778e dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040778f test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407791 jle short 00407798h
    jle(ctx, Cont(x407793), Cont(x407798))
}

pub fn x407793(ctx: &mut Context) -> Cont {
    // 00407793 add eax,eax
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407795 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407796 jne short 00407793h
    jne(ctx, Cont(x407798), Cont(x407793))
}

pub fn x407798(ctx: &mut Context) -> Cont {
    // 00407798 ret
    ret(ctx, 0)
}

pub fn x4077a0(ctx: &mut Context) -> Cont {
    // 004077a0 mov eax,[esp+4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 004077a4 mov edx,1
    ctx.cpu.regs.edx = 0x1u32;
    // 004077a9 lea ecx,[eax+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add(0x1u32);
    // 004077ac shl edx,cl
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, ctx.cpu.regs.get_cl(), &mut ctx.cpu.flags);
    // 004077ae mov ds:[425B86h],cx
    ctx.memory.write::<u16>(0x425b86u32, ctx.cpu.regs.get_cx());
    // 004077b5 mov cl,al
    ctx.cpu.regs.set_cl(ctx.cpu.regs.get_al());
    // 004077b7 mov ds:[425B80h],dx
    ctx.memory.write::<u16>(0x425b80u32, ctx.cpu.regs.get_dx());
    // 004077be mov edx,1
    ctx.cpu.regs.edx = 0x1u32;
    // 004077c3 shl edx,cl
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, ctx.cpu.regs.get_cl(), &mut ctx.cpu.flags);
    // 004077c5 mov ds:[425B84h],dx
    ctx.memory.write::<u16>(0x425b84u32, ctx.cpu.regs.get_dx());
    // 004077cc mov eax,ds:[425B84h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b84u32);
    // 004077d1 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004077d2 mov ds:[421A70h],ax
    ctx.memory.write::<u16>(0x421a70u32, ctx.cpu.regs.get_ax());
    // 004077d8 mov ecx,ds:[421A70h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x421a70u32);
    // 004077de lea eax,[ecx+1]
    ctx.cpu.regs.eax = ctx.cpu.regs.ecx.wrapping_add(0x1u32);
    // 004077e1 mov ds:[425A74h],ax
    ctx.memory.write::<u16>(0x425a74u32, ctx.cpu.regs.get_ax());
    // 004077e7 mov ds:[425B8Ch],ax
    ctx.memory.write::<u16>(0x425b8cu32, ctx.cpu.regs.get_ax());
    // 004077ed xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004077ef mov ds:[425B90h],ax
    ctx.memory.write::<u16>(0x425b90u32, ctx.cpu.regs.get_ax());
    // 004077f5 mov ds:[425B8Eh],ax
    ctx.memory.write::<u16>(0x425b8eu32, ctx.cpu.regs.get_ax());
    // 004077fb xor ax,ax
    ctx.cpu.regs.set_ax(xor(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    ));
    // 004077fe ret
    ret(ctx, 0)
}

pub fn x407800(ctx: &mut Context) -> Cont {
    // 00407800 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00407801 mov dx,ds:[425B90h]
    ctx.cpu.regs.set_dx(ctx.memory.read::<u16>(0x425b90u32));
    // 00407808 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407809 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0040780a push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0040780b mov si,ds:[425B8Eh]
    ctx.cpu.regs.set_si(ctx.memory.read::<u16>(0x425b8eu32));
    // 00407812 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00407813 test dx,dx
    and(
        ctx.cpu.regs.get_dx(),
        ctx.cpu.regs.get_dx(),
        &mut ctx.cpu.flags,
    );
    // 00407816 jne near ptr 004078B1h
    jne(ctx, Cont(x40781c), Cont(x4078b1))
}

pub fn x40781c(ctx: &mut Context) -> Cont {
    // 0040781c test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 0040781f jg short 00407880h
    jg(ctx, Cont(x407821), Cont(x407880))
}

pub fn x407821(ctx: &mut Context) -> Cont {
    // 00407821 mov edi,ds:[425B88h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425b88u32);
    // 00407827 mov ebp,425A78h
    ctx.cpu.regs.ebp = 0x425a78u32;
    // 0040782c mov ds:[425B7Ch],ebp
    ctx.memory.write::<u32>(0x425b7cu32, ctx.cpu.regs.ebp);
    // 00407832 movzx si,byte ptr [edi]
    ctx.cpu
        .regs
        .set_si(ctx.memory.read::<u8>(ctx.cpu.regs.edi) as _);
    // 00407836 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407837 mov ds:[425B8Eh],si
    ctx.memory.write::<u16>(0x425b8eu32, ctx.cpu.regs.get_si());
    // 0040783e test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 00407841 mov ds:[425B88h],edi
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.edi);
    // 00407847 jge short 00407852h
    jge(ctx, Cont(x407849), Cont(x407852))
}

pub fn x407849(ctx: &mut Context) -> Cont {
    // 00407849 mov ax,si
    ctx.cpu.regs.set_ax(ctx.cpu.regs.get_si());
    // 0040784c pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040784d pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040784e pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040784f pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407850 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 00407851 ret
    ret(ctx, 0)
}

pub fn x407852(ctx: &mut Context) -> Cont {
    // 00407852 je short 0040788Ch
    je(ctx, Cont(x407854), Cont(x40788c))
}

pub fn x407854(ctx: &mut Context) -> Cont {
    // 00407854 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407856 test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 00407859 jle short 0040788Ch
    jle(ctx, Cont(x40785b), Cont(x40788c))
}

pub fn x40785b(ctx: &mut Context) -> Cont {
    // 0040785b movzx ax,byte ptr [edi]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u8>(ctx.cpu.regs.edi) as _);
    // 0040785f inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407860 test ax,ax
    and(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    );
    // 00407863 mov ds:[425B88h],edi
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.edi);
    // 00407869 jl near ptr 0040799Ch
    jl(ctx, Cont(x40786f), Cont(x40799c))
}

pub fn x40786f(ctx: &mut Context) -> Cont {
    // 0040786f movsx edx,cx
    ctx.cpu.regs.edx = ctx.cpu.regs.get_cx() as i16 as i32 as u32;
    // 00407872 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407873 cmp cx,si
    sub(
        ctx.cpu.regs.get_cx(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 00407876 mov [edx+425A78h],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.edx.wrapping_add(0x425a78u32),
        ctx.cpu.regs.get_al(),
    );
    // 0040787c jl short 0040785Bh
    jl(ctx, Cont(x40787e), Cont(x40785b))
}

pub fn x40787e(ctx: &mut Context) -> Cont {
    // 0040787e jmp short 0040788Ch
    Cont(x40788c)
}

pub fn x407880(ctx: &mut Context) -> Cont {
    // 00407880 mov ebp,ds:[425B7Ch]
    ctx.cpu.regs.ebp = ctx.memory.read::<u32>(0x425b7cu32);
    // 00407886 mov edi,ds:[425B88h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425b88u32);
    Cont(x40788c)
}

pub fn x40788c(ctx: &mut Context) -> Cont {
    // 0040788c mov al,[ebp]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp));
    // 0040788f inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407890 mov dx,8
    ctx.cpu.regs.set_dx(0x8u16);
    // 00407894 dec si
    ctx.cpu
        .regs
        .set_si(dec(ctx.cpu.regs.get_si(), &mut ctx.cpu.flags));
    // 00407896 mov ds:[425B82h],al
    ctx.memory.write::<u8>(0x425b82u32, ctx.cpu.regs.get_al());
    // 0040789b mov ds:[425B7Ch],ebp
    ctx.memory.write::<u32>(0x425b7cu32, ctx.cpu.regs.ebp);
    // 004078a1 mov ds:[425B90h],dx
    ctx.memory.write::<u16>(0x425b90u32, ctx.cpu.regs.get_dx());
    // 004078a8 mov ds:[425B8Eh],si
    ctx.memory.write::<u16>(0x425b8eu32, ctx.cpu.regs.get_si());
    // 004078af jmp short 004078BDh
    Cont(x4078bd)
}

pub fn x4078b1(ctx: &mut Context) -> Cont {
    // 004078b1 mov ebp,ds:[425B7Ch]
    ctx.cpu.regs.ebp = ctx.memory.read::<u32>(0x425b7cu32);
    // 004078b7 mov edi,ds:[425B88h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425b88u32);
    Cont(x4078bd)
}

pub fn x4078bd(ctx: &mut Context) -> Cont {
    // 004078bd mov bl,ds:[425B90h]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(0x425b90u32));
    // 004078c3 mov cl,8
    ctx.cpu.regs.set_cl(0x8u8);
    // 004078c5 mov ax,ds:[425B86h]
    ctx.cpu.regs.set_ax(ctx.memory.read::<u16>(0x425b86u32));
    // 004078cb sub cl,bl
    ctx.cpu.regs.set_cl(sub(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_bl(),
        &mut ctx.cpu.flags,
    ));
    // 004078cd xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004078cf mov bl,ds:[425B82h]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(0x425b82u32));
    // 004078d5 shr ebx,cl
    ctx.cpu.regs.ebx = shr(ctx.cpu.regs.ebx, ctx.cpu.regs.get_cl(), &mut ctx.cpu.flags);
    // 004078d7 cmp ax,dx
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_dx(),
        &mut ctx.cpu.flags,
    );
    // 004078da mov [esp+10h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ebx);
    // 004078de jle near ptr 00407986h
    jle(ctx, Cont(x4078e4), Cont(x407986))
}

pub fn x4078e4(ctx: &mut Context) -> Cont {
    // 004078e4 test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 004078e7 jg short 0040793Ch
    jg(ctx, Cont(x4078e9), Cont(x40793c))
}

pub fn x4078e9(ctx: &mut Context) -> Cont {
    // 004078e9 mov ebp,425A78h
    ctx.cpu.regs.ebp = 0x425a78u32;
    // 004078ee inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004078ef mov ds:[425B7Ch],ebp
    ctx.memory.write::<u32>(0x425b7cu32, ctx.cpu.regs.ebp);
    // 004078f5 movzx si,byte ptr [edi-1]
    ctx.cpu.regs.set_si(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(0xffffffffu32)) as _,
    );
    // 004078fa test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 004078fd mov ds:[425B8Eh],si
    ctx.memory.write::<u16>(0x425b8eu32, ctx.cpu.regs.get_si());
    // 00407904 mov ds:[425B88h],edi
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.edi);
    // 0040790a jl near ptr 00407849h
    jl(ctx, Cont(x407910), Cont(x407849))
}

pub fn x407910(ctx: &mut Context) -> Cont {
    // 00407910 je short 0040793Ch
    je(ctx, Cont(x407912), Cont(x40793c))
}

pub fn x407912(ctx: &mut Context) -> Cont {
    // 00407912 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407914 test si,si
    and(
        ctx.cpu.regs.get_si(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 00407917 jle short 0040793Ch
    jle(ctx, Cont(x407919), Cont(x40793c))
}

pub fn x407919(ctx: &mut Context) -> Cont {
    // 00407919 movzx ax,byte ptr [edi]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u8>(ctx.cpu.regs.edi) as _);
    // 0040791d inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0040791e test ax,ax
    and(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    );
    // 00407921 mov ds:[425B88h],edi
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.edi);
    // 00407927 jl short 0040799Ch
    jl(ctx, Cont(x407929), Cont(x40799c))
}

pub fn x407929(ctx: &mut Context) -> Cont {
    // 00407929 movsx ebx,cx
    ctx.cpu.regs.ebx = ctx.cpu.regs.get_cx() as i16 as i32 as u32;
    // 0040792c inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0040792d cmp cx,si
    sub(
        ctx.cpu.regs.get_cx(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 00407930 mov [ebx+425A78h],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebx.wrapping_add(0x425a78u32),
        ctx.cpu.regs.get_al(),
    );
    // 00407936 jl short 00407919h
    jl(ctx, Cont(x407938), Cont(x407919))
}

pub fn x407938(ctx: &mut Context) -> Cont {
    // 00407938 mov ebx,[esp+10h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    Cont(x40793c)
}

pub fn x40793c(ctx: &mut Context) -> Cont {
    // 0040793c mov al,[ebp]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.ebp));
    // 0040793f xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407941 mov cl,al
    ctx.cpu.regs.set_cl(ctx.cpu.regs.get_al());
    // 00407943 mov ds:[425B82h],al
    ctx.memory.write::<u8>(0x425b82u32, ctx.cpu.regs.get_al());
    // 00407948 mov eax,ecx
    ctx.cpu.regs.eax = ctx.cpu.regs.ecx;
    // 0040794a mov cl,ds:[425B90h]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(0x425b90u32));
    // 00407950 shl eax,cl
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, ctx.cpu.regs.get_cl(), &mut ctx.cpu.flags);
    // 00407952 inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407953 add dx,8
    ctx.cpu
        .regs
        .set_dx(add(ctx.cpu.regs.get_dx(), 0x8u16, &mut ctx.cpu.flags));
    // 00407957 mov ds:[425B7Ch],ebp
    ctx.memory.write::<u32>(0x425b7cu32, ctx.cpu.regs.ebp);
    // 0040795d mov ds:[425B90h],dx
    ctx.memory.write::<u16>(0x425b90u32, ctx.cpu.regs.get_dx());
    // 00407964 or ebx,eax
    ctx.cpu.regs.ebx = or(ctx.cpu.regs.ebx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407966 dec si
    ctx.cpu
        .regs
        .set_si(dec(ctx.cpu.regs.get_si(), &mut ctx.cpu.flags));
    // 00407968 cmp ds:[425B86h],dx
    sub(
        ctx.memory.read::<u16>(0x425b86u32),
        ctx.cpu.regs.get_dx(),
        &mut ctx.cpu.flags,
    );
    // 0040796f mov [esp+10h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ebx);
    // 00407973 mov ds:[425B8Eh],si
    ctx.memory.write::<u16>(0x425b8eu32, ctx.cpu.regs.get_si());
    // 0040797a jg near ptr 004078E4h
    jg(ctx, Cont(x407980), Cont(x4078e4))
}

pub fn x407980(ctx: &mut Context) -> Cont {
    // 00407980 mov ax,ds:[425B86h]
    ctx.cpu.regs.set_ax(ctx.memory.read::<u16>(0x425b86u32));
    Cont(x407986)
}

pub fn x407986(ctx: &mut Context) -> Cont {
    // 00407986 movsx ecx,ax
    ctx.cpu.regs.ecx = ctx.cpu.regs.get_ax() as i16 as i32 as u32;
    // 00407989 sub dx,ax
    ctx.cpu.regs.set_dx(sub(
        ctx.cpu.regs.get_dx(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    ));
    // 0040798c mov eax,[ecx*4+421790h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>((ctx.cpu.regs.ecx * 4).wrapping_add(0x421790u32));
    // 00407993 mov ds:[425B90h],dx
    ctx.memory.write::<u16>(0x425b90u32, ctx.cpu.regs.get_dx());
    // 0040799a and eax,ebx
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    Cont(x40799c)
}

pub fn x40799c(ctx: &mut Context) -> Cont {
    // 0040799c pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0040799d pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0040799e pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0040799f pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 004079a0 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 004079a1 ret
    ret(ctx, 0)
}

pub fn x4079b0(ctx: &mut Context) -> Cont {
    // 004079b0 sub esp,10h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 004079b3 mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 004079b7 mov ecx,ds:[425B88h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425b88u32);
    // 004079bd push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 004079be mov [esp+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 004079c2 movzx ax,byte ptr [ecx]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u8>(ctx.cpu.regs.ecx) as _);
    // 004079c6 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004079c7 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 004079c8 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004079ca inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004079cb cmp ax,si
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_si(),
        &mut ctx.cpu.flags,
    );
    // 004079ce push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004079cf mov [esp+1Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.eax);
    // 004079d3 mov ds:[425B88h],ecx
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.ecx);
    // 004079d9 jl near ptr 00407BDFh
    jl(ctx, Cont(x4079df), Cont(x407bdf))
}

pub fn x4079df(ctx: &mut Context) -> Cont {
    // 004079df cmp ax,2
    sub(ctx.cpu.regs.get_ax(), 0x2u16, &mut ctx.cpu.flags);
    // 004079e3 jl near ptr 00407BDBh
    jl(ctx, Cont(x4079e9), Cont(x407bdb))
}

pub fn x4079e9(ctx: &mut Context) -> Cont {
    // 004079e9 cmp ax,9
    sub(ctx.cpu.regs.get_ax(), 0x9u16, &mut ctx.cpu.flags);
    // 004079ed jg near ptr 00407BDBh
    jg(ctx, Cont(x4079f3), Cont(x407bdb))
}

pub fn x4079f3(ctx: &mut Context) -> Cont {
    // 004079f3 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004079f4 call 004077A0h
    let dst = Cont(x4077a0);
    call(ctx, 0x4079f9, dst)
}

pub fn x4079f9(ctx: &mut Context) -> Cont {
    // 004079f9 mov ebp,[esp+2Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x2cu32));
    // 004079fd mov [esp+1Ch],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.esi);
    // 00407a01 movsx ecx,bp
    ctx.cpu.regs.ecx = ctx.cpu.regs.get_bp() as i16 as i32 as u32;
    // 00407a04 add ecx,0Ah
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0xau32, &mut ctx.cpu.flags);
    // 00407a07 mov [esp+28h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32), ctx.cpu.regs.esi);
    // 00407a0b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00407a0c call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x407a11, dst)
}

pub fn x407a11(ctx: &mut Context) -> Cont {
    // 00407a11 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407a14 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 00407a18 mov esi,424A74h
    ctx.cpu.regs.esi = 0x424a74u32;
    // 00407a1d mov edi,eax
    ctx.cpu.regs.edi = ctx.cpu.regs.eax;
    // 00407a1f call 00407800h
    let dst = Cont(x407800);
    call(ctx, 0x407a24, dst)
}

pub fn x407a24(ctx: &mut Context) -> Cont {
    // 00407a24 mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00407a26 cmp bx,ds:[421A70h]
    sub(
        ctx.cpu.regs.get_bx(),
        ctx.memory.read::<u16>(0x421a70u32),
        &mut ctx.cpu.flags,
    );
    // 00407a2d je near ptr 00407BABh
    je(ctx, Cont(x407a33), Cont(x407bab))
}

pub fn x407a33(ctx: &mut Context) -> Cont {
    // 00407a33 test bx,bx
    and(
        ctx.cpu.regs.get_bx(),
        ctx.cpu.regs.get_bx(),
        &mut ctx.cpu.flags,
    );
    // 00407a36 jl near ptr 00407BC3h
    jl(ctx, Cont(x407a3c), Cont(x407bc3))
}

pub fn x407a3c(ctx: &mut Context) -> Cont {
    // 00407a3c cmp bx,ds:[425B84h]
    sub(
        ctx.cpu.regs.get_bx(),
        ctx.memory.read::<u16>(0x425b84u32),
        &mut ctx.cpu.flags,
    );
    // 00407a43 jne near ptr 00407AD2h
    jne(ctx, Cont(x407a49), Cont(x407ad2))
}

pub fn x407a49(ctx: &mut Context) -> Cont {
    // 00407a49 mov edx,[esp+1Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407a4d mov ax,ds:[425A74h]
    ctx.cpu.regs.set_ax(ctx.memory.read::<u16>(0x425a74u32));
    // 00407a53 mov ds:[425B8Ch],ax
    ctx.memory.write::<u16>(0x425b8cu32, ctx.cpu.regs.get_ax());
    // 00407a59 lea ecx,[edx+1]
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx.wrapping_add(0x1u32);
    // 00407a5c mov edx,1
    ctx.cpu.regs.edx = 0x1u32;
    // 00407a61 shl edx,cl
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, ctx.cpu.regs.get_cl(), &mut ctx.cpu.flags);
    // 00407a63 mov ds:[425B86h],cx
    ctx.memory.write::<u16>(0x425b86u32, ctx.cpu.regs.get_cx());
    // 00407a6a mov ds:[425B80h],dx
    ctx.memory.write::<u16>(0x425b80u32, ctx.cpu.regs.get_dx());
    // 00407a71 call 00407800h
    let dst = Cont(x407800);
    call(ctx, 0x407a76, dst)
}

pub fn x407a76(ctx: &mut Context) -> Cont {
    // 00407a76 cmp ax,ds:[425B84h]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x425b84u32),
        &mut ctx.cpu.flags,
    );
    // 00407a7d jne short 00407A8Dh
    jne(ctx, Cont(x407a7f), Cont(x407a8d))
}

pub fn x407a7f(ctx: &mut Context) -> Cont {
    // 00407a7f call 00407800h
    let dst = Cont(x407800);
    call(ctx, 0x407a84, dst)
}

pub fn x407a84(ctx: &mut Context) -> Cont {
    // 00407a84 cmp ax,ds:[425B84h]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x425b84u32),
        &mut ctx.cpu.flags,
    );
    // 00407a8b je short 00407A7Fh
    je(ctx, Cont(x407a8d), Cont(x407a7f))
}

pub fn x407a8d(ctx: &mut Context) -> Cont {
    // 00407a8d cmp ax,ds:[421A70h]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x421a70u32),
        &mut ctx.cpu.flags,
    );
    // 00407a94 je near ptr 00407BABh
    je(ctx, Cont(x407a9a), Cont(x407bab))
}

pub fn x407a9a(ctx: &mut Context) -> Cont {
    // 00407a9a cmp ax,ds:[425B8Ch]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x425b8cu32),
        &mut ctx.cpu.flags,
    );
    // 00407aa1 jl short 00407AA5h
    jl(ctx, Cont(x407aa3), Cont(x407aa5))
}

pub fn x407aa3(ctx: &mut Context) -> Cont {
    // 00407aa3 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x407aa5)
}

pub fn x407aa5(ctx: &mut Context) -> Cont {
    // 00407aa5 mov ecx,[esp+10h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407aa9 mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 00407aab inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407aac mov [esp+18h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.eax);
    // 00407ab0 mov [ecx],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.ecx, ctx.cpu.regs.get_al());
    // 00407ab2 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407ab3 dec ebp
    ctx.cpu.regs.ebp = dec(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407ab4 mov [esp+24h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.eax);
    // 00407ab8 test bp,bp
    and(
        ctx.cpu.regs.get_bp(),
        ctx.cpu.regs.get_bp(),
        &mut ctx.cpu.flags,
    );
    // 00407abb mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 00407abf jne near ptr 00407B97h
    jne(ctx, Cont(x407ac5), Cont(x407b97))
}

pub fn x407ac5(ctx: &mut Context) -> Cont {
    // 00407ac5 mov edi,[esp+14h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407ac9 mov ebp,[esp+28h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    // 00407acd jmp near ptr 00407B97h
    Cont(x407b97)
}

pub fn x407ad2(ctx: &mut Context) -> Cont {
    // 00407ad2 mov ax,ds:[425B8Ch]
    ctx.cpu.regs.set_ax(ctx.memory.read::<u16>(0x425b8cu32));
    // 00407ad8 mov ecx,ebx
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx;
    // 00407ada cmp bx,ax
    sub(
        ctx.cpu.regs.get_bx(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    );
    // 00407add jl short 00407AEAh
    jl(ctx, Cont(x407adf), Cont(x407aea))
}

pub fn x407adf(ctx: &mut Context) -> Cont {
    // 00407adf mov dl,[esp+18h]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x18u32)),
    );
    // 00407ae3 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 00407ae7 mov [esi],dl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi, ctx.cpu.regs.get_dl());
    // 00407ae9 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x407aea)
}

pub fn x407aea(ctx: &mut Context) -> Cont {
    // 00407aea cmp cx,ds:[425A74h]
    sub(
        ctx.cpu.regs.get_cx(),
        ctx.memory.read::<u16>(0x425a74u32),
        &mut ctx.cpu.flags,
    );
    // 00407af1 jl short 00407B11h
    jl(ctx, Cont(x407af3), Cont(x407b11))
}

pub fn x407af3(ctx: &mut Context) -> Cont {
    // 00407af3 movsx ecx,cx
    ctx.cpu.regs.ecx = ctx.cpu.regs.get_cx() as i16 as i32 as u32;
    // 00407af6 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407af7 mov dl,[ecx+423A74h]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(0x423a74u32)),
    );
    // 00407afd mov cx,[ecx*2+421A74h]
    ctx.cpu.regs.set_cx(
        ctx.memory
            .read::<u16>((ctx.cpu.regs.ecx * 2).wrapping_add(0x421a74u32)),
    );
    // 00407b05 cmp cx,ds:[425A74h]
    sub(
        ctx.cpu.regs.get_cx(),
        ctx.memory.read::<u16>(0x425a74u32),
        &mut ctx.cpu.flags,
    );
    // 00407b0c mov [esi-1],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_dl(),
    );
    // 00407b0f jge short 00407AF3h
    jge(ctx, Cont(x407b11), Cont(x407af3))
}

pub fn x407b11(ctx: &mut Context) -> Cont {
    // 00407b11 mov [esi],cl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi, ctx.cpu.regs.get_cl());
    // 00407b13 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407b14 cmp ax,ds:[425B80h]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x425b80u32),
        &mut ctx.cpu.flags,
    );
    // 00407b1b jge short 00407B4Ch
    jge(ctx, Cont(x407b1d), Cont(x407b4c))
}

pub fn x407b1d(ctx: &mut Context) -> Cont {
    // 00407b1d movsx edx,ax
    ctx.cpu.regs.edx = ctx.cpu.regs.get_ax() as i16 as i32 as u32;
    // 00407b20 mov [esp+18h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.ecx);
    // 00407b24 inc ax
    ctx.cpu
        .regs
        .set_ax(inc(ctx.cpu.regs.get_ax(), &mut ctx.cpu.flags));
    // 00407b26 cmp ax,ds:[425B80h]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory.read::<u16>(0x425b80u32),
        &mut ctx.cpu.flags,
    );
    // 00407b2d mov [edx+423A74h],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.edx.wrapping_add(0x423a74u32),
        ctx.cpu.regs.get_cl(),
    );
    // 00407b33 mov cx,[esp+24h]
    ctx.cpu.regs.set_cx(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.esp.wrapping_add(0x24u32)),
    );
    // 00407b38 mov ds:[425B8Ch],ax
    ctx.memory.write::<u16>(0x425b8cu32, ctx.cpu.regs.get_ax());
    // 00407b3e mov [edx*2+421A74h],cx
    ctx.memory.write::<u16>(
        (ctx.cpu.regs.edx * 2).wrapping_add(0x421a74u32),
        ctx.cpu.regs.get_cx(),
    );
    // 00407b46 mov [esp+24h],ebx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32), ctx.cpu.regs.ebx);
    // 00407b4a jl short 00407B64h
    jl(ctx, Cont(x407b4c), Cont(x407b64))
}

pub fn x407b4c(ctx: &mut Context) -> Cont {
    // 00407b4c cmp word ptr ds:[425B86h],0Ch
    sub(
        ctx.memory.read::<u16>(0x425b86u32),
        0xcu16,
        &mut ctx.cpu.flags,
    );
    // 00407b54 jge short 00407B64h
    jge(ctx, Cont(x407b56), Cont(x407b64))
}

pub fn x407b56(ctx: &mut Context) -> Cont {
    // 00407b56 shl word ptr ds:[425B80h],1
    ctx.memory.write::<u16>(
        0x425b80u32,
        shl(
            ctx.memory.read::<u16>(0x425b80u32),
            0x1u8,
            &mut ctx.cpu.flags,
        ),
    );
    // 00407b5d inc word ptr ds:[425B86h]
    ctx.memory.write::<u16>(
        0x425b86u32,
        inc(ctx.memory.read::<u16>(0x425b86u32), &mut ctx.cpu.flags),
    );
    Cont(x407b64)
}

pub fn x407b64(ctx: &mut Context) -> Cont {
    // 00407b64 cmp esi,424A74h
    sub(ctx.cpu.regs.esi, 0x424a74u32, &mut ctx.cpu.flags);
    // 00407b6a jbe short 00407B97h
    jbe(ctx, Cont(x407b6c), Cont(x407b97))
}

pub fn x407b6c(ctx: &mut Context) -> Cont {
    // 00407b6c mov dl,[esi-1]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(0xffffffffu32)),
    );
    // 00407b6f mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407b73 dec esi
    ctx.cpu.regs.esi = dec(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407b74 mov [edi],dl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_dl());
    // 00407b76 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407b77 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407b78 mov cl,[esi]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 00407b7a dec ebp
    ctx.cpu.regs.ebp = dec(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 00407b7b mov [eax-1],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffffu32),
        ctx.cpu.regs.get_cl(),
    );
    // 00407b7e mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00407b82 test bp,bp
    and(
        ctx.cpu.regs.get_bp(),
        ctx.cpu.regs.get_bp(),
        &mut ctx.cpu.flags,
    );
    // 00407b85 jne short 00407B8Fh
    jne(ctx, Cont(x407b87), Cont(x407b8f))
}

pub fn x407b87(ctx: &mut Context) -> Cont {
    // 00407b87 mov edi,[esp+14h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407b8b mov ebp,[esp+28h]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x28u32));
    Cont(x407b8f)
}

pub fn x407b8f(ctx: &mut Context) -> Cont {
    // 00407b8f cmp esi,424A74h
    sub(ctx.cpu.regs.esi, 0x424a74u32, &mut ctx.cpu.flags);
    // 00407b95 ja short 00407B6Ch
    ja(ctx, Cont(x407b97), Cont(x407b6c))
}

pub fn x407b97(ctx: &mut Context) -> Cont {
    // 00407b97 call 00407800h
    let dst = Cont(x407800);
    call(ctx, 0x407b9c, dst)
}

pub fn x407b9c(ctx: &mut Context) -> Cont {
    // 00407b9c mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 00407b9e cmp bx,ds:[421A70h]
    sub(
        ctx.cpu.regs.get_bx(),
        ctx.memory.read::<u16>(0x421a70u32),
        &mut ctx.cpu.flags,
    );
    // 00407ba5 jne near ptr 00407A33h
    jne(ctx, Cont(x407bab), Cont(x407a33))
}

pub fn x407bab(ctx: &mut Context) -> Cont {
    // 00407bab mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407baf push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00407bb0 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x407bb5, dst)
}

pub fn x407bb5(ctx: &mut Context) -> Cont {
    // 00407bb5 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407bb8 xor ax,ax
    ctx.cpu.regs.set_ax(xor(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    ));
    // 00407bbb pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407bbc pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407bbd pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407bbe pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407bbf add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00407bc2 ret
    ret(ctx, 0)
}

pub fn x407bc3(ctx: &mut Context) -> Cont {
    // 00407bc3 mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407bc7 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00407bc8 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x407bcd, dst)
}

pub fn x407bcd(ctx: &mut Context) -> Cont {
    // 00407bcd add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407bd0 xor ax,ax
    ctx.cpu.regs.set_ax(xor(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    ));
    // 00407bd3 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407bd4 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407bd5 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407bd6 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407bd7 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00407bda ret
    ret(ctx, 0)
}

pub fn x407bdb(ctx: &mut Context) -> Cont {
    // 00407bdb or ax,0FFFFh
    ctx.cpu
        .regs
        .set_ax(or(ctx.cpu.regs.get_ax(), 0xffffu16, &mut ctx.cpu.flags));
    Cont(x407bdf)
}

pub fn x407bdf(ctx: &mut Context) -> Cont {
    // 00407bdf pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407be0 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407be1 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407be2 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407be3 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 00407be6 ret
    ret(ctx, 0)
}

pub fn x407bf0(ctx: &mut Context) -> Cont {
    // 00407bf0 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407bf3 mov eax,[esp+0Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 00407bf7 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407bf8 add eax,0Ah
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0xau32, &mut ctx.cpu.flags);
    // 00407bfb xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407bfd push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407bfe mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407c03 mov bl,[eax]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407c05 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00407c06 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 00407c07 add eax,3
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x3u32, &mut ctx.cpu.flags);
    // 00407c0a push 400h
    push(ctx, 0x400u32);
    // 00407c0f mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407c14 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x407c19, dst)
}

pub fn x407c19(ctx: &mut Context) -> Cont {
    // 00407c19 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407c1c mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 00407c1e test bl,80h
    and(ctx.cpu.regs.get_bl(), 0x80u8, &mut ctx.cpu.flags);
    // 00407c21 mov [esp+1Ch],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.esi);
    // 00407c25 je short 00407C86h
    je(ctx, Cont(x407c27), Cont(x407c86))
}

pub fn x407c27(ctx: &mut Context) -> Cont {
    // 00407c27 and ebx,7
    ctx.cpu.regs.ebx = and(ctx.cpu.regs.ebx, 0x7u32, &mut ctx.cpu.flags);
    // 00407c2a xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407c2c inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407c2d push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407c2e call 00407770h
    let dst = Cont(x407770);
    call(ctx, 0x407c33, dst)
}

pub fn x407c33(ctx: &mut Context) -> Cont {
    // 00407c33 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407c36 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407c38 jle short 00407C86h
    jle(ctx, Cont(x407c3a), Cont(x407c86))
}

pub fn x407c3a(ctx: &mut Context) -> Cont {
    // 00407c3a add esi,2
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x2u32, &mut ctx.cpu.flags);
    Cont(x407c3d)
}

pub fn x407c3d(ctx: &mut Context) -> Cont {
    // 00407c3d mov ecx,ds:[425B88h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425b88u32);
    // 00407c43 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407c44 add esi,3
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x3u32, &mut ctx.cpu.flags);
    // 00407c47 mov dl,[ecx]
    ctx.cpu.regs.set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.ecx));
    // 00407c49 mov [esi-5],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffbu32),
        ctx.cpu.regs.get_dl(),
    );
    // 00407c4c mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    // 00407c51 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407c52 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407c57 mov al,[eax]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407c59 mov [esi-4],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.get_al(),
    );
    // 00407c5c mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    // 00407c61 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407c62 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407c67 mov cl,[eax]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407c69 mov [esi-3],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffdu32),
        ctx.cpu.regs.get_cl(),
    );
    // 00407c6c mov edx,ds:[425B88h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425b88u32);
    // 00407c72 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407c73 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407c74 mov ds:[425B88h],edx
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.edx);
    // 00407c7a call 00407770h
    let dst = Cont(x407770);
    call(ctx, 0x407c7f, dst)
}

pub fn x407c7f(ctx: &mut Context) -> Cont {
    // 00407c7f add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407c82 cmp edi,eax
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407c84 jl short 00407C3Dh
    jl(ctx, Cont(x407c86), Cont(x407c3d))
}

pub fn x407c86(ctx: &mut Context) -> Cont {
    // 00407c86 mov ebp,[esp+1Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    Cont(x407c8a)
}

pub fn x407c8a(ctx: &mut Context) -> Cont {
    // 00407c8a mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    Cont(x407c8f)
}

pub fn x407c8f(ctx: &mut Context) -> Cont {
    // 00407c8f mov cl,[eax]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407c91 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407c92 cmp cl,21h
    sub(ctx.cpu.regs.get_cl(), 0x21u8, &mut ctx.cpu.flags);
    // 00407c95 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407c9a je near ptr 00407D79h
    je(ctx, Cont(x407ca0), Cont(x407d79))
}

pub fn x407ca0(ctx: &mut Context) -> Cont {
    // 00407ca0 cmp cl,2Ch
    sub(ctx.cpu.regs.get_cl(), 0x2cu8, &mut ctx.cpu.flags);
    // 00407ca3 je short 00407CB0h
    je(ctx, Cont(x407ca5), Cont(x407cb0))
}

pub fn x407ca5(ctx: &mut Context) -> Cont {
    // 00407ca5 cmp cl,3Bh
    sub(ctx.cpu.regs.get_cl(), 0x3bu8, &mut ctx.cpu.flags);
    // 00407ca8 je near ptr 00407D98h
    je(ctx, Cont(x407cae), Cont(x407d98))
}

pub fn x407cae(ctx: &mut Context) -> Cont {
    // 00407cae jmp short 00407C8Fh
    Cont(x407c8f)
}

pub fn x407cb0(ctx: &mut Context) -> Cont {
    // 00407cb0 add eax,4
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x4u32, &mut ctx.cpu.flags);
    // 00407cb3 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407cb5 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407cba mov bp,[eax]
    ctx.cpu
        .regs
        .set_bp(ctx.memory.read::<u16>(ctx.cpu.regs.eax));
    // 00407cbd add eax,2
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x2u32, &mut ctx.cpu.flags);
    // 00407cc0 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407cc5 mov si,[eax]
    ctx.cpu
        .regs
        .set_si(ctx.memory.read::<u16>(ctx.cpu.regs.eax));
    // 00407cc8 add eax,2
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x2u32, &mut ctx.cpu.flags);
    // 00407ccb mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407cd0 mov edx,esi
    ctx.cpu.regs.edx = ctx.cpu.regs.esi;
    // 00407cd2 mov bl,[eax]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407cd4 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407cd5 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407cda mov eax,ebp
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp;
    // 00407cdc and edx,0FFFFh
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0xffffu32, &mut ctx.cpu.flags);
    // 00407ce2 and eax,0FFFFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffffu32, &mut ctx.cpu.flags);
    // 00407ce7 imul edx,eax
    let x = ctx.cpu.regs.edx as i32;
    let y = ctx.cpu.regs.eax as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.edx = res as u32;
    // 00407cea push edx
    push(ctx, ctx.cpu.regs.edx);
    // 00407ceb mov [esp+18h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32), ctx.cpu.regs.esi);
    // 00407cef call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x407cf4, dst)
}

pub fn x407cf4(ctx: &mut Context) -> Cont {
    // 00407cf4 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407cf7 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 00407cfb test bl,80h
    and(ctx.cpu.regs.get_bl(), 0x80u8, &mut ctx.cpu.flags);
    // 00407cfe je short 00407D65h
    je(ctx, Cont(x407d00), Cont(x407d65))
}

pub fn x407d00(ctx: &mut Context) -> Cont {
    // 00407d00 and ebx,7
    ctx.cpu.regs.ebx = and(ctx.cpu.regs.ebx, 0x7u32, &mut ctx.cpu.flags);
    // 00407d03 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407d05 inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 00407d06 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407d07 call 00407770h
    let dst = Cont(x407770);
    call(ctx, 0x407d0c, dst)
}

pub fn x407d0c(ctx: &mut Context) -> Cont {
    // 00407d0c add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407d0f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d11 jle short 00407D65h
    jle(ctx, Cont(x407d13), Cont(x407d65))
}

pub fn x407d13(ctx: &mut Context) -> Cont {
    // 00407d13 mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407d17 lea esi,[ecx+2]
    ctx.cpu.regs.esi = ctx.cpu.regs.ecx.wrapping_add(0x2u32);
    Cont(x407d1a)
}

pub fn x407d1a(ctx: &mut Context) -> Cont {
    // 00407d1a mov edx,ds:[425B88h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425b88u32);
    // 00407d20 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 00407d21 add esi,3
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x3u32, &mut ctx.cpu.flags);
    // 00407d24 mov al,[edx]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edx));
    // 00407d26 mov [esi-5],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffbu32),
        ctx.cpu.regs.get_al(),
    );
    // 00407d29 mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    // 00407d2e inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d2f mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407d34 mov cl,[eax]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407d36 mov [esi-4],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.get_cl(),
    );
    // 00407d39 mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    // 00407d3e inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d3f mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407d44 mov dl,[eax]
    ctx.cpu.regs.set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 00407d46 mov [esi-3],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0xfffffffdu32),
        ctx.cpu.regs.get_dl(),
    );
    // 00407d49 mov eax,ds:[425B88h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b88u32);
    // 00407d4e inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d4f inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407d50 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407d55 call 00407770h
    let dst = Cont(x407770);
    call(ctx, 0x407d5a, dst)
}

pub fn x407d5a(ctx: &mut Context) -> Cont {
    // 00407d5a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407d5d cmp edi,eax
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d5f jl short 00407D1Ah
    jl(ctx, Cont(x407d61), Cont(x407d1a))
}

pub fn x407d61(ctx: &mut Context) -> Cont {
    // 00407d61 mov esi,[esp+14h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    Cont(x407d65)
}

pub fn x407d65(ctx: &mut Context) -> Cont {
    // 00407d65 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407d69 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 00407d6a push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 00407d6b push eax
    push(ctx, ctx.cpu.regs.eax);
    // 00407d6c call 004079B0h
    let dst = Cont(x4079b0);
    call(ctx, 0x407d71, dst)
}

pub fn x407d71(ctx: &mut Context) -> Cont {
    // 00407d71 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 00407d74 jmp near ptr 00407C8Ah
    Cont(x407c8a)
}

pub fn x407d79(ctx: &mut Context) -> Cont {
    // 00407d79 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d7a mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407d7f cmp byte ptr [eax],0
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.eax),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 00407d82 je near ptr 00407C8Fh
    je(ctx, Cont(x407d88), Cont(x407c8f))
}

pub fn x407d88(ctx: &mut Context) -> Cont {
    // 00407d88 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407d89 mov ds:[425B88h],eax
    ctx.memory.write::<u32>(0x425b88u32, ctx.cpu.regs.eax);
    // 00407d8e cmp byte ptr [eax],0
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.eax),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 00407d91 jne short 00407D88h
    jne(ctx, Cont(x407d93), Cont(x407d88))
}

pub fn x407d93(ctx: &mut Context) -> Cont {
    // 00407d93 jmp near ptr 00407C8Fh
    Cont(x407c8f)
}

pub fn x407d98(ctx: &mut Context) -> Cont {
    // 00407d98 mov edi,[esp+14h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407d9c and ebp,0FFFFh
    ctx.cpu.regs.ebp = and(ctx.cpu.regs.ebp, 0xffffu32, &mut ctx.cpu.flags);
    // 00407da2 and edi,0FFFFh
    ctx.cpu.regs.edi = and(ctx.cpu.regs.edi, 0xffffu32, &mut ctx.cpu.flags);
    // 00407da8 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407daa imul edi,ebp
    let x = ctx.cpu.regs.edi as i32;
    let y = ctx.cpu.regs.ebp as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.edi = res as u32;
    // 00407dad test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407daf jle short 00407DECh
    jle(ctx, Cont(x407db1), Cont(x407dec))
}

pub fn x407db1(ctx: &mut Context) -> Cont {
    // 00407db1 mov ebx,[esp+10h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 00407db5 mov ebp,[esp+1Ch]
    ctx.cpu.regs.ebp = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407db9 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407dbb xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407dbd mov al,[ebx+esi]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ebx.wrapping_add(ctx.cpu.regs.esi)),
    );
    // 00407dc0 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407dc2 lea eax,[eax+eax*2]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 2));
    // 00407dc5 mov cl,[eax+ebp+1]
    ctx.cpu.regs.set_cl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.ebp)
                .wrapping_add(0x1u32),
        ),
    );
    // 00407dc9 mov dl,[eax+ebp+2]
    ctx.cpu.regs.set_dl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.ebp)
                .wrapping_add(0x2u32),
        ),
    );
    // 00407dcd add ecx,edx
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407dcf xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407dd1 mov dl,[eax+ebp]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.ebp)),
    );
    // 00407dd4 mov eax,55555556h
    ctx.cpu.regs.eax = 0x55555556u32;
    // 00407dd9 add ecx,edx
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 00407ddb imul ecx
    let x = ctx.cpu.regs.eax as u32 as i32;
    let y = ctx.cpu.regs.ecx as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    ctx.cpu.flags.set(Flags::CF, flag);
    ctx.cpu.flags.set(Flags::OF, flag);
    ctx.cpu.regs.edx = (res >> 32) as u32;
    ctx.cpu.regs.eax = res as u32;
    // 00407ddd mov eax,edx
    ctx.cpu.regs.eax = ctx.cpu.regs.edx;
    // 00407ddf shr eax,1Fh
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1fu8, &mut ctx.cpu.flags);
    // 00407de2 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407de4 mov [ebx+esi],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.ebx.wrapping_add(ctx.cpu.regs.esi),
        ctx.cpu.regs.get_dl(),
    );
    // 00407de7 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 00407de8 cmp esi,edi
    sub(ctx.cpu.regs.esi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 00407dea jl short 00407DB1h
    jl(ctx, Cont(x407dec), Cont(x407db1))
}

pub fn x407dec(ctx: &mut Context) -> Cont {
    // 00407dec mov ecx,[esp+1Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 00407df0 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 00407df1 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x407df6, dst)
}

pub fn x407df6(ctx: &mut Context) -> Cont {
    // 00407df6 mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407dfa add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 00407dfd pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 00407dfe pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 00407dff pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 00407e00 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 00407e01 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407e04 ret
    ret(ctx, 0)
}

pub fn x407e10(ctx: &mut Context) -> Cont {
    // 00407e10 mov eax,ds:[4217C4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x4217c4u32);
    // 00407e15 mov ecx,ds:[4217C8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x4217c8u32);
    // 00407e1b add eax,ecx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407e1d add ecx,91h
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x91u32, &mut ctx.cpu.flags);
    // 00407e23 add eax,15h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x15u32, &mut ctx.cpu.flags);
    // 00407e26 mov ds:[4217C8h],ecx
    ctx.memory.write::<u32>(0x4217c8u32, ctx.cpu.regs.ecx);
    // 00407e2c mov ds:[4217C4h],eax
    ctx.memory.write::<u32>(0x4217c4u32, ctx.cpu.regs.eax);
    // 00407e31 add eax,ecx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407e33 ret
    ret(ctx, 0)
}

pub fn x407e40(ctx: &mut Context) -> Cont {
    // 00407e40 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407e43 fld dword ptr [esp+1Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32)) as f64,
    );
    // 00407e47 fmul qword ptr ds:[4204A8h]
    ctx.cpu
        .fpu
        .set(0, ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(0x4204a8u32));
    // 00407e4d mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 00407e51 mov ecx,[esp+18h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 00407e55 fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 00407e57 fcos
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).cos());
    // 00407e59 fstp qword ptr [esp]
    ctx.memory
        .write::<f64>(ctx.cpu.regs.esp, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00407e5d fsin
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0).sin());
    // 00407e5f fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00407e63 fmul st,st(1)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(1));
    // 00407e65 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00407e69 fmul qword ptr [esp]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(ctx.cpu.regs.esp),
    );
    // 00407e6d faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00407e6f fstp dword ptr [eax]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.eax, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00407e71 fld dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 00407e75 fmul qword ptr [esp]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f64>(ctx.cpu.regs.esp),
    );
    // 00407e79 fld dword ptr [esp+0Ch]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0xcu32)) as f64,
    );
    // 00407e7d fmul st,st(2)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) * ctx.cpu.fpu.get(2));
    // 00407e7f fsubp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) - ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00407e81 fstp dword ptr [ecx]
    ctx.memory
        .write::<f32>(ctx.cpu.regs.ecx, ctx.cpu.fpu.get(0) as f32);
    ctx.cpu.fpu.pop();
    // 00407e83 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 00407e85 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407e88 ret
    ret(ctx, 0)
}

pub fn x407e90(ctx: &mut Context) -> Cont {
    // 00407e90 mov ecx,[esp+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32));
    // 00407e94 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407e96 cmp byte ptr [ecx],0
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.ecx),
        0x0u8,
        &mut ctx.cpu.flags,
    );
    // 00407e99 je short 00407EA4h
    je(ctx, Cont(x407e9b), Cont(x407ea4))
}

pub fn x407e9b(ctx: &mut Context) -> Cont {
    // 00407e9b mov dl,[eax+ecx+1]
    ctx.cpu.regs.set_dl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x1u32),
        ),
    );
    // 00407e9f inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 00407ea0 test dl,dl
    and(
        ctx.cpu.regs.get_dl(),
        ctx.cpu.regs.get_dl(),
        &mut ctx.cpu.flags,
    );
    // 00407ea2 jne short 00407E9Bh
    jne(ctx, Cont(x407ea4), Cont(x407e9b))
}

pub fn x407ea4(ctx: &mut Context) -> Cont {
    // 00407ea4 ret
    ret(ctx, 0)
}

pub fn x407eb0(ctx: &mut Context) -> Cont {
    // 00407eb0 call dword ptr ds:[420008h]
    let dst = Cont(kernel32::GetTickCount_stdcall);
    call(ctx, 0x407eb6, dst)
}

pub fn x407eb6(ctx: &mut Context) -> Cont {
    // 00407eb6 mov ds:[425BFCh],eax
    ctx.memory.write::<u32>(0x425bfcu32, ctx.cpu.regs.eax);
    // 00407ebb ret
    ret(ctx, 0)
}

pub fn x407ec0(ctx: &mut Context) -> Cont {
    // 00407ec0 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407ec3 call dword ptr ds:[420008h]
    let dst = Cont(kernel32::GetTickCount_stdcall);
    call(ctx, 0x407ec9, dst)
}

pub fn x407ec9(ctx: &mut Context) -> Cont {
    // 00407ec9 mov ecx,ds:[425BFCh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bfcu32);
    // 00407ecf mov dword ptr [esp+4],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0x0u32);
    // 00407ed7 sub eax,ecx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 00407ed9 mov [esp],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.eax);
    // 00407edd fild qword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<u64>(ctx.cpu.regs.esp) as i64 as f64);
    // 00407ee1 fmul dword ptr ds:[4204B0h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204b0u32) as f64,
    );
    // 00407ee7 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 00407eea ret
    ret(ctx, 0)
}

pub fn x41df90(ctx: &mut Context) -> Cont {
    // 0041df90 call 0041E9C0h
    let dst = Cont(x41e9c0);
    call(ctx, 0x41df95, dst)
}

pub fn x41df95(ctx: &mut Context) -> Cont {
    // 0041df95 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0041df9a ret
    ret(ctx, 0)
}

pub fn x41dfa0(ctx: &mut Context) -> Cont {
    // 0041dfa0 jmp near ptr 00403980h
    Cont(x403980)
}

pub fn x41dfb0(ctx: &mut Context) -> Cont {
    // 0041dfb0 jmp near ptr 0041EEA0h
    Cont(x41eea0)
}

pub fn x41dfc0(ctx: &mut Context) -> Cont {
    // 0041dfc0 jmp near ptr 0041EED0h
    Cont(x41eed0)
}

pub fn x41dfd0(ctx: &mut Context) -> Cont {
    // 0041dfd0 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041dfd1 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041dfd2 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041dfd3 mov esi,[esp+14h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041dfd7 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041dfd8 mov edi,[esp+14h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041dfdc mov dword ptr [esi],40h
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, 0x40u32);
    // 0041dfe2 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041dfe7 lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0041dfea shl eax,6
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x6u8, &mut ctx.cpu.flags);
    // 0041dfed push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041dfee call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41dff3, dst)
}

pub fn x41dff3(ctx: &mut Context) -> Cont {
    // 0041dff3 mov [esi+4],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x4u32), ctx.cpu.regs.eax);
    // 0041dff6 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 0041dff8 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041dffd add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e000 mov dword ptr [esp+14h],40h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), 0x40u32);
    Cont(x41e008)
}

pub fn x41e008(ctx: &mut Context) -> Cont {
    // 0041e008 xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e00a test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e00c jle short 0041E075h
    jle(ctx, Cont(x41e00e), Cont(x41e075))
}

pub fn x41e00e(ctx: &mut Context) -> Cont {
    // 0041e00e mov dl,[edi]
    ctx.cpu.regs.set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 0041e010 mov bl,[edi+2]
    ctx.cpu
        .regs
        .set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x2u32)));
    // 0041e013 mov cl,dl
    ctx.cpu.regs.set_cl(ctx.cpu.regs.get_dl());
    // 0041e015 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e017 mov al,[edi+1]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x1u32)));
    // 0041e01a and ecx,0Fh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0xfu32, &mut ctx.cpu.flags);
    // 0041e01d shl ecx,8
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x8u8, &mut ctx.cpu.flags);
    // 0041e020 or ecx,eax
    ctx.cpu.regs.ecx = or(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e022 mov al,bl
    ctx.cpu.regs.set_al(ctx.cpu.regs.get_bl());
    // 0041e024 shr al,4
    ctx.cpu
        .regs
        .set_al(shr(ctx.cpu.regs.get_al(), 0x4u8, &mut ctx.cpu.flags));
    // 0041e027 and dl,0F0h
    ctx.cpu
        .regs
        .set_dl(and(ctx.cpu.regs.get_dl(), 0xf0u8, &mut ctx.cpu.flags));
    // 0041e02a and bl,0Fh
    ctx.cpu
        .regs
        .set_bl(and(ctx.cpu.regs.get_bl(), 0xfu8, &mut ctx.cpu.flags));
    // 0041e02d or al,dl
    ctx.cpu.regs.set_al(or(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_dl(),
        &mut ctx.cpu.flags,
    ));
    // 0041e02f mov dl,[edi+3]
    ctx.cpu
        .regs
        .set_dl(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x3u32)));
    // 0041e032 add edi,4
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 0041e035 mov [esi+4],dl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x4u32), ctx.cpu.regs.get_dl());
    // 0041e038 dec al
    ctx.cpu
        .regs
        .set_al(dec(ctx.cpu.regs.get_al(), &mut ctx.cpu.flags));
    // 0041e03a xor dl,dl
    ctx.cpu.regs.set_dl(xor(
        ctx.cpu.regs.get_dl(),
        ctx.cpu.regs.get_dl(),
        &mut ctx.cpu.flags,
    ));
    // 0041e03c test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e03e mov [esi],cl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi, ctx.cpu.regs.get_cl());
    // 0041e040 mov [esi+1],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x1u32), ctx.cpu.regs.get_al());
    // 0041e043 mov byte ptr [esi+2],0FFh
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x2u32), 0xffu8);
    // 0041e047 mov [esi+3],bl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x3u32), ctx.cpu.regs.get_bl());
    // 0041e04a je short 0041E066h
    je(ctx, Cont(x41e04c), Cont(x41e066))
}

pub fn x41e04c(ctx: &mut Context) -> Cont {
    // 0041e04c xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0041e04e mov eax,4217CCh
    ctx.cpu.regs.eax = 0x4217ccu32;
    Cont(x41e053)
}

pub fn x41e053(ctx: &mut Context) -> Cont {
    // 0041e053 cmp ecx,[eax]
    sub(
        ctx.cpu.regs.ecx,
        ctx.memory.read::<u32>(ctx.cpu.regs.eax),
        &mut ctx.cpu.flags,
    );
    // 0041e055 jae short 0041E064h
    jae(ctx, Cont(x41e057), Cont(x41e064))
}

pub fn x41e057(ctx: &mut Context) -> Cont {
    // 0041e057 add eax,4
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x4u32, &mut ctx.cpu.flags);
    // 0041e05a inc ebx
    ctx.cpu.regs.ebx = inc(ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0041e05b cmp eax,4218ECh
    sub(ctx.cpu.regs.eax, 0x4218ecu32, &mut ctx.cpu.flags);
    // 0041e060 jl short 0041E053h
    jl(ctx, Cont(x41e062), Cont(x41e053))
}

pub fn x41e062(ctx: &mut Context) -> Cont {
    // 0041e062 jmp short 0041E066h
    Cont(x41e066)
}

pub fn x41e064(ctx: &mut Context) -> Cont {
    // 0041e064 mov dl,bl
    ctx.cpu.regs.set_dl(ctx.cpu.regs.get_bl());
    Cont(x41e066)
}

pub fn x41e066(ctx: &mut Context) -> Cont {
    // 0041e066 mov [esi],dl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi, ctx.cpu.regs.get_dl());
    // 0041e068 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e06d add esi,5
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x5u32, &mut ctx.cpu.flags);
    // 0041e070 inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e071 cmp ebp,eax
    sub(ctx.cpu.regs.ebp, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e073 jl short 0041E00Eh
    jl(ctx, Cont(x41e075), Cont(x41e00e))
}

pub fn x41e075(ctx: &mut Context) -> Cont {
    // 0041e075 mov ecx,[esp+14h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041e079 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e07a mov [esp+14h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.ecx);
    // 0041e07e jne short 0041E008h
    jne(ctx, Cont(x41e080), Cont(x41e008))
}

pub fn x41e080(ctx: &mut Context) -> Cont {
    // 0041e080 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041e081 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e082 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041e083 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041e084 ret
    ret(ctx, 0)
}

pub fn x41e090(ctx: &mut Context) -> Cont {
    // 0041e090 sub esp,440h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x440u32, &mut ctx.cpu.flags);
    // 0041e096 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e097 mov ebx,[esp+448h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x448u32));
    // 0041e09e push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041e09f push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e0a0 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041e0a1 xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e0a3 mov ecx,10Fh
    ctx.cpu.regs.ecx = 0x10fu32;
    // 0041e0a8 mov esi,ebx
    ctx.cpu.regs.esi = ctx.cpu.regs.ebx;
    // 0041e0aa lea edi,[esp+14h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esp.wrapping_add(0x14u32);
    // 0041e0ae mov ds:[425BC0h],ebp
    ctx.memory.write::<u32>(0x425bc0u32, ctx.cpu.regs.ebp);
    // 0041e0b4 mov ds:[425BC4h],ebp
    ctx.memory.write::<u32>(0x425bc4u32, ctx.cpu.regs.ebp);
    // 0041e0ba mov dword ptr ds:[425BC8h],1
    ctx.memory.write::<u32>(0x425bc8u32, 0x1u32);
    // 0041e0c4 mov dword ptr ds:[425BCCh],10000h
    ctx.memory.write::<u32>(0x425bccu32, 0x10000u32);
    // 0041e0ce mov dword ptr [esp+10h],43Ch
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), 0x43cu32);
    // 0041e0d6 rep movsd
    rep(ctx, Rep::REP, movsd);
    // 0041e0d8 lea eax,[esp+42h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x42u32);
    // 0041e0dc mov esi,20h
    ctx.cpu.regs.esi = 0x20u32;
    Cont(x41e0e1)
}

pub fn x41e0e1(ctx: &mut Context) -> Cont {
    // 0041e0e1 mov cx,[eax-4]
    ctx.cpu.regs.set_cx(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.eax.wrapping_add(0xfffffffcu32)),
    );
    // 0041e0e5 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e0e7 mov dl,ch
    ctx.cpu.regs.set_dl(ctx.cpu.regs.get_ch());
    // 0041e0e9 add eax,1Eh
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x1eu32, &mut ctx.cpu.flags);
    // 0041e0ec mov dh,cl
    ctx.cpu.regs.set_dh(ctx.cpu.regs.get_cl());
    // 0041e0ee mov [eax-22h],dx
    ctx.memory.write::<u16>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffdeu32),
        ctx.cpu.regs.get_dx(),
    );
    // 0041e0f2 mov cx,[eax-1Eh]
    ctx.cpu.regs.set_cx(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.eax.wrapping_add(0xffffffe2u32)),
    );
    // 0041e0f6 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e0f8 mov dl,ch
    ctx.cpu.regs.set_dl(ctx.cpu.regs.get_ch());
    // 0041e0fa mov dh,cl
    ctx.cpu.regs.set_dh(ctx.cpu.regs.get_cl());
    // 0041e0fc mov [eax-1Eh],dx
    ctx.memory.write::<u16>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffe2u32),
        ctx.cpu.regs.get_dx(),
    );
    // 0041e100 mov cx,[eax-1Ch]
    ctx.cpu.regs.set_cx(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.eax.wrapping_add(0xffffffe4u32)),
    );
    // 0041e104 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e106 dec esi
    ctx.cpu.regs.esi = dec(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e107 mov dl,ch
    ctx.cpu.regs.set_dl(ctx.cpu.regs.get_ch());
    // 0041e109 mov dh,cl
    ctx.cpu.regs.set_dh(ctx.cpu.regs.get_cl());
    // 0041e10b mov [eax-1Ch],dx
    ctx.memory.write::<u16>(
        ctx.cpu.regs.eax.wrapping_add(0xffffffe4u32),
        ctx.cpu.regs.get_dx(),
    );
    // 0041e10f jne short 0041E0E1h
    jne(ctx, Cont(x41e111), Cont(x41e0e1))
}

pub fn x41e111(ctx: &mut Context) -> Cont {
    // 0041e111 mov esi,4
    ctx.cpu.regs.esi = 0x4u32;
    // 0041e116 lea eax,[esp+44Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x44cu32);
    // 0041e11d push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e11e push 42190Ch
    push(ctx, 0x42190cu32);
    // 0041e123 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e124 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e129, dst)
}

pub fn x41e129(ctx: &mut Context) -> Cont {
    // 0041e129 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e12c test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e12e jne short 0041E136h
    jne(ctx, Cont(x41e130), Cont(x41e136))
}

pub fn x41e130(ctx: &mut Context) -> Cont {
    // 0041e130 mov ds:[425BD8h],esi
    ctx.memory.write::<u32>(0x425bd8u32, ctx.cpu.regs.esi);
    Cont(x41e136)
}

pub fn x41e136(ctx: &mut Context) -> Cont {
    // 0041e136 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e137 lea ecx,[esp+450h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x450u32);
    // 0041e13e push 421904h
    push(ctx, 0x421904u32);
    // 0041e143 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e144 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e149, dst)
}

pub fn x41e149(ctx: &mut Context) -> Cont {
    // 0041e149 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e14c test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e14e jne short 0041E156h
    jne(ctx, Cont(x41e150), Cont(x41e156))
}

pub fn x41e150(ctx: &mut Context) -> Cont {
    // 0041e150 mov ds:[425BD8h],esi
    ctx.memory.write::<u32>(0x425bd8u32, ctx.cpu.regs.esi);
    Cont(x41e156)
}

pub fn x41e156(ctx: &mut Context) -> Cont {
    // 0041e156 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e157 lea edx,[esp+450h]
    ctx.cpu.regs.edx = ctx.cpu.regs.esp.wrapping_add(0x450u32);
    // 0041e15e push 4218FCh
    push(ctx, 0x4218fcu32);
    // 0041e163 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041e164 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e169, dst)
}

pub fn x41e169(ctx: &mut Context) -> Cont {
    // 0041e169 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e16c test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e16e jne short 0041E176h
    jne(ctx, Cont(x41e170), Cont(x41e176))
}

pub fn x41e170(ctx: &mut Context) -> Cont {
    // 0041e170 mov ds:[425BD8h],esi
    ctx.memory.write::<u32>(0x425bd8u32, ctx.cpu.regs.esi);
    Cont(x41e176)
}

pub fn x41e176(ctx: &mut Context) -> Cont {
    // 0041e176 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e177 lea eax,[esp+450h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x450u32);
    // 0041e17e push 4218F4h
    push(ctx, 0x4218f4u32);
    // 0041e183 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e184 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e189, dst)
}

pub fn x41e189(ctx: &mut Context) -> Cont {
    // 0041e189 add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e18c test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e18e jne short 0041E19Ah
    jne(ctx, Cont(x41e190), Cont(x41e19a))
}

pub fn x41e190(ctx: &mut Context) -> Cont {
    // 0041e190 mov dword ptr ds:[425BD8h],8
    ctx.memory.write::<u32>(0x425bd8u32, 0x8u32);
    Cont(x41e19a)
}

pub fn x41e19a(ctx: &mut Context) -> Cont {
    // 0041e19a push 3
    push(ctx, 0x3u32);
    // 0041e19c lea ecx,[esp+451h]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0x451u32);
    // 0041e1a3 push 4218F0h
    push(ctx, 0x4218f0u32);
    // 0041e1a8 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e1a9 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e1ae, dst)
}

pub fn x41e1ae(ctx: &mut Context) -> Cont {
    // 0041e1ae add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e1b1 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e1b3 jne short 0041E1C6h
    jne(ctx, Cont(x41e1b5), Cont(x41e1c6))
}

pub fn x41e1b5(ctx: &mut Context) -> Cont {
    // 0041e1b5 movsx edx,byte ptr [esp+44Ch]
    ctx.cpu.regs.edx =
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esp.wrapping_add(0x44cu32)) as i8 as i32 as u32;
    // 0041e1bd sub edx,30h
    ctx.cpu.regs.edx = sub(ctx.cpu.regs.edx, 0x30u32, &mut ctx.cpu.flags);
    // 0041e1c0 mov ds:[425BD8h],edx
    ctx.memory.write::<u32>(0x425bd8u32, ctx.cpu.regs.edx);
    Cont(x41e1c6)
}

pub fn x41e1c6(ctx: &mut Context) -> Cont {
    // 0041e1c6 push 2
    push(ctx, 0x2u32);
    // 0041e1c8 lea eax,[esp+452h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x452u32);
    // 0041e1cf push 4218ECh
    push(ctx, 0x4218ecu32);
    // 0041e1d4 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e1d5 call 0041E4A0h
    let dst = Cont(x41e4a0);
    call(ctx, 0x41e1da, dst)
}

pub fn x41e1da(ctx: &mut Context) -> Cont {
    // 0041e1da add esp,0Ch
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0xcu32, &mut ctx.cpu.flags);
    // 0041e1dd test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e1df jne short 0041E1FFh
    jne(ctx, Cont(x41e1e1), Cont(x41e1ff))
}

pub fn x41e1e1(ctx: &mut Context) -> Cont {
    // 0041e1e1 mov ecx,[esp+44Ch]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x44cu32));
    // 0041e1e8 movsx eax,cl
    ctx.cpu.regs.eax = ctx.cpu.regs.get_cl() as i8 as i32 as u32;
    // 0041e1eb lea edx,[eax+eax*4]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0041e1ee movsx eax,ch
    ctx.cpu.regs.eax = ctx.cpu.regs.get_ch() as i8 as i32 as u32;
    // 0041e1f1 lea eax,[eax+edx*2-210h]
    ctx.cpu.regs.eax = ctx
        .cpu
        .regs
        .eax
        .wrapping_add((ctx.cpu.regs.edx * 2))
        .wrapping_add(0xfffffdf0u32);
    // 0041e1f8 mov ds:[425BD8h],eax
    ctx.memory.write::<u32>(0x425bd8u32, ctx.cpu.regs.eax);
    // 0041e1fd jmp short 0041E204h
    Cont(x41e204)
}

pub fn x41e1ff(ctx: &mut Context) -> Cont {
    // 0041e1ff mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    Cont(x41e204)
}

pub fn x41e204(ctx: &mut Context) -> Cont {
    // 0041e204 lea ecx,[eax+eax*2]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 2));
    // 0041e207 shl ecx,3
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x3u8, &mut ctx.cpu.flags);
    // 0041e20a push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e20b call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e210, dst)
}

pub fn x41e210(ctx: &mut Context) -> Cont {
    // 0041e210 mov ds:[425BDCh],eax
    ctx.memory.write::<u32>(0x425bdcu32, ctx.cpu.regs.eax);
    // 0041e215 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e21a add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e21d xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e21f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e221 jle short 0041E243h
    jle(ctx, Cont(x41e223), Cont(x41e243))
}

pub fn x41e223(ctx: &mut Context) -> Cont {
    // 0041e223 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x41e225)
}

pub fn x41e225(ctx: &mut Context) -> Cont {
    // 0041e225 mov edi,ds:[425BDCh]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425bdcu32);
    // 0041e22b mov ecx,6
    ctx.cpu.regs.ecx = 0x6u32;
    // 0041e230 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e232 add edi,esi
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e234 rep stosd
    rep(ctx, Rep::REP, stosd);
    // 0041e236 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e23b inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e23c add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 0041e23f cmp edx,eax
    sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e241 jl short 0041E225h
    jl(ctx, Cont(x41e243), Cont(x41e225))
}

pub fn x41e243(ctx: &mut Context) -> Cont {
    // 0041e243 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    Cont(x41e245)
}

pub fn x41e245(ctx: &mut Context) -> Cont {
    // 0041e245 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e247 mov al,[esp+ecx+3CCh]
    ctx.cpu.regs.set_al(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .esp
                .wrapping_add(ctx.cpu.regs.ecx)
                .wrapping_add(0x3ccu32),
        ),
    );
    // 0041e24e cmp eax,ebp
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e250 jle short 0041E254h
    jle(ctx, Cont(x41e252), Cont(x41e254))
}

pub fn x41e252(ctx: &mut Context) -> Cont {
    // 0041e252 mov ebp,eax
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    Cont(x41e254)
}

pub fn x41e254(ctx: &mut Context) -> Cont {
    // 0041e254 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e255 cmp ecx,80h
    sub(ctx.cpu.regs.ecx, 0x80u32, &mut ctx.cpu.flags);
    // 0041e25b jl short 0041E245h
    jl(ctx, Cont(x41e25d), Cont(x41e245))
}

pub fn x41e25d(ctx: &mut Context) -> Cont {
    // 0041e25d lea eax,[ebp+1]
    ctx.cpu.regs.eax = ctx.cpu.regs.ebp.wrapping_add(0x1u32);
    // 0041e260 mov ds:[425BF4h],eax
    ctx.memory.write::<u32>(0x425bf4u32, ctx.cpu.regs.eax);
    // 0041e265 lea edx,[eax*8]
    ctx.cpu.regs.edx = (ctx.cpu.regs.eax * 8);
    // 0041e26c push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041e26d call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e272, dst)
}

pub fn x41e272(ctx: &mut Context) -> Cont {
    // 0041e272 mov ds:[425BF8h],eax
    ctx.memory.write::<u32>(0x425bf8u32, ctx.cpu.regs.eax);
    // 0041e277 mov eax,ds:[425BF4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bf4u32);
    // 0041e27c add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e27f xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e281 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e283 jle near ptr 0041E30Ch
    jle(ctx, Cont(x41e289), Cont(x41e30c))
}

pub fn x41e289(ctx: &mut Context) -> Cont {
    // 0041e289 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e28e shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 0041e291 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e292 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e297, dst)
}

pub fn x41e297(ctx: &mut Context) -> Cont {
    // 0041e297 mov edx,ds:[425BF8h]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bf8u32);
    // 0041e29d mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 0041e29f xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e2a1 mov edi,ebx
    ctx.cpu.regs.edi = ctx.cpu.regs.ebx;
    // 0041e2a3 lea eax,[edx+ebp*8]
    ctx.cpu.regs.eax = ctx.cpu.regs.edx.wrapping_add((ctx.cpu.regs.ebp * 8));
    // 0041e2a6 mov edx,[esp+458h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x458u32));
    // 0041e2ad mov [eax],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.eax, ctx.cpu.regs.ecx);
    // 0041e2af mov [eax+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    // 0041e2b2 mov ecx,ds:[425BD8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e2b8 mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041e2bc shl ecx,8
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x8u8, &mut ctx.cpu.flags);
    // 0041e2bf lea esi,[eax+edx]
    ctx.cpu.regs.esi = ctx.cpu.regs.eax.wrapping_add(ctx.cpu.regs.edx);
    // 0041e2c2 mov edx,ecx
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx;
    // 0041e2c4 shr ecx,2
    ctx.cpu.regs.ecx = shr(ctx.cpu.regs.ecx, 0x2u8, &mut ctx.cpu.flags);
    // 0041e2c7 rep movsd
    rep(ctx, Rep::REP, movsd);
    // 0041e2c9 mov ecx,edx
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx;
    // 0041e2cb mov edx,[esp+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041e2cf and ecx,3
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0x3u32, &mut ctx.cpu.flags);
    // 0041e2d2 rep movsb
    rep(ctx, Rep::REP, movsb);
    // 0041e2d4 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e2d9 mov ecx,ds:[425BF8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bf8u32);
    // 0041e2df shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 0041e2e2 add edx,eax
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e2e4 mov [esp+14h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.edx);
    // 0041e2e8 lea edx,[ecx+ebp*8]
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.ebp * 8));
    // 0041e2eb push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041e2ec push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e2ed call 0041DFD0h
    let dst = Cont(x41dfd0);
    call(ctx, 0x41e2f2, dst)
}

pub fn x41e2f2(ctx: &mut Context) -> Cont {
    // 0041e2f2 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e2f3 call 0041F0D0h
    let dst = Cont(x41f0d0);
    call(ctx, 0x41e2f8, dst)
}

pub fn x41e2f8(ctx: &mut Context) -> Cont {
    // 0041e2f8 mov eax,ds:[425BF4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bf4u32);
    // 0041e2fd add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041e300 inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e301 cmp ebp,eax
    sub(ctx.cpu.regs.ebp, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e303 jl short 0041E289h
    jl(ctx, Cont(x41e305), Cont(x41e289))
}

pub fn x41e305(ctx: &mut Context) -> Cont {
    // 0041e305 mov ebx,[esp+454h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x454u32));
    Cont(x41e30c)
}

pub fn x41e30c(ctx: &mut Context) -> Cont {
    // 0041e30c mov dword ptr ds:[425BD4h],6
    ctx.memory.write::<u32>(0x425bd4u32, 0x6u32);
    // 0041e316 mov dword ptr ds:[425BD0h],7Dh
    ctx.memory.write::<u32>(0x425bd0u32, 0x7du32);
    // 0041e320 call 0041E510h
    let dst = Cont(x41e510);
    call(ctx, 0x41e325, dst)
}

pub fn x41e325(ctx: &mut Context) -> Cont {
    // 0041e325 push 300h
    push(ctx, 0x300u32);
    // 0041e32a mov dword ptr ds:[425BECh],20h
    ctx.memory.write::<u32>(0x425becu32, 0x20u32);
    // 0041e334 call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e339, dst)
}

pub fn x41e339(ctx: &mut Context) -> Cont {
    // 0041e339 mov ds:[425BF0h],eax
    ctx.memory.write::<u32>(0x425bf0u32, ctx.cpu.regs.eax);
    // 0041e33e mov eax,ds:[425BECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425becu32);
    // 0041e343 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e346 xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e348 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e34a jle short 0041E36Ch
    jle(ctx, Cont(x41e34c), Cont(x41e36c))
}

pub fn x41e34c(ctx: &mut Context) -> Cont {
    // 0041e34c xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    Cont(x41e34e)
}

pub fn x41e34e(ctx: &mut Context) -> Cont {
    // 0041e34e mov edi,ds:[425BF0h]
    ctx.cpu.regs.edi = ctx.memory.read::<u32>(0x425bf0u32);
    // 0041e354 mov ecx,6
    ctx.cpu.regs.ecx = 0x6u32;
    // 0041e359 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e35b add edi,esi
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e35d rep stosd
    rep(ctx, Rep::REP, stosd);
    // 0041e35f mov eax,ds:[425BECh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425becu32);
    // 0041e364 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e365 add esi,18h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x18u32, &mut ctx.cpu.flags);
    // 0041e368 cmp edx,eax
    sub(ctx.cpu.regs.edx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e36a jl short 0041E34Eh
    jl(ctx, Cont(x41e36c), Cont(x41e34e))
}

pub fn x41e36c(ctx: &mut Context) -> Cont {
    // 0041e36c xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e36e lea edi,[esp+44h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esp.wrapping_add(0x44u32);
    Cont(x41e372)
}

pub fn x41e372(ctx: &mut Context) -> Cont {
    // 0041e372 mov ax,[edi-6]
    ctx.cpu.regs.set_ax(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.edi.wrapping_add(0xfffffffau32)),
    );
    // 0041e376 test ax,ax
    and(
        ctx.cpu.regs.get_ax(),
        ctx.cpu.regs.get_ax(),
        &mut ctx.cpu.flags,
    );
    // 0041e379 je near ptr 0041E410h
    je(ctx, Cont(x41e37f), Cont(x41e410))
}

pub fn x41e37f(ctx: &mut Context) -> Cont {
    // 0041e37f mov ecx,ds:[425BF0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bf0u32);
    // 0041e385 and eax,0FFFFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffffu32, &mut ctx.cpu.flags);
    // 0041e38a shl eax,1
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041e38c lea esi,[ecx+ebp]
    ctx.cpu.regs.esi = ctx.cpu.regs.ecx.wrapping_add(ctx.cpu.regs.ebp);
    // 0041e38f xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e391 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e393 mov [esi+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0041e396 mov dx,[edi-2]
    ctx.cpu.regs.set_dx(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.edi.wrapping_add(0xfffffffeu32)),
    );
    // 0041e39a shl edx,1
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, 0x1u8, &mut ctx.cpu.flags);
    // 0041e39c xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e39e mov [esi+0Ch],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xcu32), ctx.cpu.regs.edx);
    // 0041e3a1 mov ax,[edi-2]
    ctx.cpu.regs.set_ax(
        ctx.memory
            .read::<u16>(ctx.cpu.regs.edi.wrapping_add(0xfffffffeu32)),
    );
    // 0041e3a5 mov cx,[edi]
    ctx.cpu
        .regs
        .set_cx(ctx.memory.read::<u16>(ctx.cpu.regs.edi));
    // 0041e3a8 add eax,ecx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e3aa mov dword ptr [esi],0
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, 0x0u32);
    // 0041e3b0 shl eax,1
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041e3b2 mov [esi+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 0041e3b5 mov ax,[edi]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u16>(ctx.cpu.regs.edi));
    // 0041e3b8 cmp ax,2
    sub(ctx.cpu.regs.get_ax(), 0x2u16, &mut ctx.cpu.flags);
    // 0041e3bc jbe short 0041E3CAh
    jbe(ctx, Cont(x41e3be), Cont(x41e3ca))
}

pub fn x41e3be(ctx: &mut Context) -> Cont {
    // 0041e3be cmp ax,[edi-2]
    sub(
        ctx.cpu.regs.get_ax(),
        ctx.memory
            .read::<u16>(ctx.cpu.regs.edi.wrapping_add(0xfffffffeu32)),
        &mut ctx.cpu.flags,
    );
    // 0041e3c2 je short 0041E3CAh
    je(ctx, Cont(x41e3c4), Cont(x41e3ca))
}

pub fn x41e3c4(ctx: &mut Context) -> Cont {
    // 0041e3c4 mov dword ptr [esi],1
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, 0x1u32);
    Cont(x41e3ca)
}

pub fn x41e3ca(ctx: &mut Context) -> Cont {
    // 0041e3ca mov eax,[esi+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32));
    // 0041e3cd mov dl,[edi-3]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(0xfffffffdu32)),
    );
    // 0041e3d0 shl eax,1
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041e3d2 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e3d3 mov [esi+5],dl
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x5u32), ctx.cpu.regs.get_dl());
    // 0041e3d6 mov byte ptr [esi+4],0
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x4u32), 0x0u8);
    // 0041e3da call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e3df, dst)
}

pub fn x41e3df(ctx: &mut Context) -> Cont {
    // 0041e3df mov ecx,[esi+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32));
    // 0041e3e2 mov [esi+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 0041e3e5 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e3e8 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e3ea test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e3ec jle short 0041E410h
    jle(ctx, Cont(x41e3ee), Cont(x41e410))
}

pub fn x41e3ee(ctx: &mut Context) -> Cont {
    // 0041e3ee mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041e3f2 mov cl,[edx+ebx]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edx.wrapping_add(ctx.cpu.regs.ebx)),
    );
    // 0041e3f5 inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e3f6 movsx cx,cl
    ctx.cpu
        .regs
        .set_cx(ctx.cpu.regs.get_cl() as i8 as i16 as u16);
    // 0041e3fa mov [esp+10h],edx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edx);
    // 0041e3fe mov edx,[esi+14h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x14u32));
    // 0041e401 shl ecx,6
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x6u8, &mut ctx.cpu.flags);
    // 0041e404 mov [edx+eax*2],cx
    ctx.memory.write::<u16>(
        ctx.cpu.regs.edx.wrapping_add((ctx.cpu.regs.eax * 2)),
        ctx.cpu.regs.get_cx(),
    );
    // 0041e408 mov ecx,[esi+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32));
    // 0041e40b inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e40c cmp eax,ecx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e40e jl short 0041E3EEh
    jl(ctx, Cont(x41e410), Cont(x41e3ee))
}

pub fn x41e410(ctx: &mut Context) -> Cont {
    // 0041e410 add ebp,18h
    ctx.cpu.regs.ebp = add(ctx.cpu.regs.ebp, 0x18u32, &mut ctx.cpu.flags);
    // 0041e413 add edi,1Eh
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x1eu32, &mut ctx.cpu.flags);
    // 0041e416 cmp ebp,2E8h
    sub(ctx.cpu.regs.ebp, 0x2e8u32, &mut ctx.cpu.flags);
    // 0041e41c jl near ptr 0041E372h
    jl(ctx, Cont(x41e422), Cont(x41e372))
}

pub fn x41e422(ctx: &mut Context) -> Cont {
    // 0041e422 mov eax,[esp+3CBh]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cbu32));
    // 0041e429 and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0041e42e mov ds:[425BE4h],eax
    ctx.memory.write::<u32>(0x425be4u32, ctx.cpu.regs.eax);
    // 0041e433 mov eax,[esp+3CAh]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cau32));
    // 0041e43a and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0041e43f mov ds:[425BE0h],eax
    ctx.memory.write::<u32>(0x425be0u32, ctx.cpu.regs.eax);
    // 0041e444 lea ecx,[eax*4]
    ctx.cpu.regs.ecx = (ctx.cpu.regs.eax * 4);
    // 0041e44b push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e44c call 0041F0B0h
    let dst = Cont(x41f0b0);
    call(ctx, 0x41e451, dst)
}

pub fn x41e451(ctx: &mut Context) -> Cont {
    // 0041e451 mov ecx,ds:[425BE0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425be0u32);
    // 0041e457 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e45a mov ds:[425BE8h],eax
    ctx.memory.write::<u32>(0x425be8u32, ctx.cpu.regs.eax);
    // 0041e45f xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e461 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041e462 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e463 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041e464 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041e465 test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e467 jle short 0041E486h
    jle(ctx, Cont(x41e469), Cont(x41e486))
}

pub fn x41e469(ctx: &mut Context) -> Cont {
    // 0041e469 mov ecx,ds:[425BE8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425be8u32);
    // 0041e46f xor edx,edx
    ctx.cpu.regs.edx = xor(ctx.cpu.regs.edx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e471 mov dl,[esp+eax+3BCh]
    ctx.cpu.regs.set_dl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .esp
                .wrapping_add(ctx.cpu.regs.eax)
                .wrapping_add(0x3bcu32),
        ),
    );
    // 0041e478 mov [ecx+eax*4],edx
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.eax * 4)),
        ctx.cpu.regs.edx,
    );
    // 0041e47b mov ecx,ds:[425BE0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425be0u32);
    // 0041e481 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e482 cmp eax,ecx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e484 jl short 0041E469h
    jl(ctx, Cont(x41e486), Cont(x41e469))
}

pub fn x41e486(ctx: &mut Context) -> Cont {
    // 0041e486 mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0041e48b add esp,440h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x440u32, &mut ctx.cpu.flags);
    // 0041e491 ret
    ret(ctx, 0)
}

pub fn x41e4a0(ctx: &mut Context) -> Cont {
    // 0041e4a0 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e4a1 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e4a2 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041e4a3 mov edi,[esp+18h]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x18u32));
    // 0041e4a7 xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e4a9 test edi,edi
    and(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041e4ab jle short 0041E4C6h
    jle(ctx, Cont(x41e4ad), Cont(x41e4c6))
}

pub fn x41e4ad(ctx: &mut Context) -> Cont {
    // 0041e4ad mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041e4b1 mov esi,[esp+10h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041e4b5 sub esi,eax
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    Cont(x41e4b7)
}

pub fn x41e4b7(ctx: &mut Context) -> Cont {
    // 0041e4b7 mov dl,[esi+eax]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(ctx.cpu.regs.eax)),
    );
    // 0041e4ba mov bl,[eax]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.eax));
    // 0041e4bc cmp dl,bl
    sub(
        ctx.cpu.regs.get_dl(),
        ctx.cpu.regs.get_bl(),
        &mut ctx.cpu.flags,
    );
    // 0041e4be jne short 0041E4CCh
    jne(ctx, Cont(x41e4c0), Cont(x41e4cc))
}

pub fn x41e4c0(ctx: &mut Context) -> Cont {
    // 0041e4c0 inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e4c1 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e4c2 cmp ecx,edi
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041e4c4 jl short 0041E4B7h
    jl(ctx, Cont(x41e4c6), Cont(x41e4b7))
}

pub fn x41e4c6(ctx: &mut Context) -> Cont {
    // 0041e4c6 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041e4c7 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e4c8 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e4ca pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041e4cb ret
    ret(ctx, 0)
}

pub fn x41e4cc(ctx: &mut Context) -> Cont {
    // 0041e4cc pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041e4cd pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e4ce mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0041e4d3 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041e4d4 ret
    ret(ctx, 0)
}

pub fn x41e4e0(ctx: &mut Context) -> Cont {
    // 0041e4e0 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041e4e3 mov eax,[esp+0Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 0041e4e7 mov dword ptr [esp+4],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x4u32), 0x0u32);
    // 0041e4ef mov ecx,[eax*4+421914h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>((ctx.cpu.regs.eax * 4).wrapping_add(0x421914u32));
    // 0041e4f6 mov [esp],ecx
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.ecx);
    // 0041e4fa fild qword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<u64>(ctx.cpu.regs.esp) as i64 as f64);
    // 0041e4fe fdivr dword ptr ds:[4200C8h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x4200c8u32) as f64 / ctx.cpu.fpu.get(0),
    );
    // 0041e504 fmul dword ptr ds:[4204B4h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204b4u32) as f64,
    );
    // 0041e50a add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041e50d ret
    ret(ctx, 0)
}

pub fn x41e510(ctx: &mut Context) -> Cont {
    // 0041e510 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e511 mov eax,ds:[425BD0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd0u32);
    // 0041e516 lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0041e519 lea eax,[eax+eax*4]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0041e51c cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0041e51d idiv dword ptr ds:[425BD4h]
    let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;
    let y = ctx.memory.read::<u32>(0x425bd4u32) as i64;
    ctx.cpu.regs.eax = (x / y) as i32 as u32;
    ctx.cpu.regs.edx = (x % y) as i32 as u32;
    // 0041e523 mov [esp],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, ctx.cpu.regs.eax);
    // 0041e527 fild dword ptr [esp]
    ctx.cpu
        .fpu
        .push(ctx.memory.read::<u32>(ctx.cpu.regs.esp) as i32 as f64);
    // 0041e52b fdivr dword ptr ds:[420428h]
    ctx.cpu.fpu.set(
        0,
        ctx.memory.read::<f32>(0x420428u32) as f64 / ctx.cpu.fpu.get(0),
    );
    // 0041e531 fmul dword ptr ds:[4204B8h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204b8u32) as f64,
    );
    // 0041e537 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x41e53c, dst)
}

pub fn x41e53c(ctx: &mut Context) -> Cont {
    // 0041e53c mov ds:[425BCCh],eax
    ctx.memory.write::<u32>(0x425bccu32, ctx.cpu.regs.eax);
    // 0041e541 pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 0041e542 ret
    ret(ctx, 0)
}

pub fn x41e550(ctx: &mut Context) -> Cont {
    // 0041e550 mov eax,[esp+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32));
    // 0041e554 sub esp,8
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041e557 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e559 jle near ptr 0041E8CDh
    jle(ctx, Cont(x41e55f), Cont(x41e8cd))
}

pub fn x41e55f(ctx: &mut Context) -> Cont {
    // 0041e55f inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e560 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e561 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041e562 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e563 shr eax,1
    ctx.cpu.regs.eax = shr(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041e565 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041e566 mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    Cont(x41e56a)
}

pub fn x41e56a(ctx: &mut Context) -> Cont {
    // 0041e56a mov eax,ds:[425BC8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bc8u32);
    // 0041e56f dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e570 mov ds:[425BC8h],eax
    ctx.memory.write::<u32>(0x425bc8u32, ctx.cpu.regs.eax);
    // 0041e575 jne near ptr 0041E702h
    jne(ctx, Cont(x41e57b), Cont(x41e702))
}

pub fn x41e57b(ctx: &mut Context) -> Cont {
    // 0041e57b mov eax,ds:[425BE8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425be8u32);
    // 0041e580 mov ecx,ds:[425BC0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bc0u32);
    // 0041e586 xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e588 mov edx,[eax+ecx*4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.ecx * 4)));
    // 0041e58b mov eax,ds:[425BF8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bf8u32);
    // 0041e590 lea ecx,[eax+edx*8]
    ctx.cpu.regs.ecx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.edx * 8));
    // 0041e593 mov [esp+10h],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.ecx);
    // 0041e597 mov ecx,ds:[425BD8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e59d test ecx,ecx
    and(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e59f jle near ptr 0041E6B8h
    jle(ctx, Cont(x41e5a5), Cont(x41e6b8))
}

pub fn x41e5a5(ctx: &mut Context) -> Cont {
    // 0041e5a5 xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    Cont(x41e5a7)
}

pub fn x41e5a7(ctx: &mut Context) -> Cont {
    // 0041e5a7 mov eax,ds:[425BC4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bc4u32);
    // 0041e5ac mov edx,ds:[425BDCh]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bdcu32);
    // 0041e5b2 imul eax,ecx
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.ecx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 0041e5b5 add eax,ebp
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e5b7 lea esi,[ebx+edx]
    ctx.cpu.regs.esi = ctx.cpu.regs.ebx.wrapping_add(ctx.cpu.regs.edx);
    // 0041e5ba lea edi,[eax+eax*4]
    ctx.cpu.regs.edi = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 4));
    // 0041e5bd mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041e5c1 mov edx,[eax+4]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0041e5c4 mov al,[edi+edx+1]
    ctx.cpu.regs.set_al(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .edi
                .wrapping_add(ctx.cpu.regs.edx)
                .wrapping_add(0x1u32),
        ),
    );
    // 0041e5c8 add edi,edx
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e5ca cmp al,0FFh
    sub(ctx.cpu.regs.get_al(), 0xffu8, &mut ctx.cpu.flags);
    // 0041e5cc je short 0041E63Ah
    je(ctx, Cont(x41e5ce), Cont(x41e63a))
}

pub fn x41e5ce(ctx: &mut Context) -> Cont {
    // 0041e5ce mov cl,[edi]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 0041e5d0 test cl,cl
    and(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_cl(),
        &mut ctx.cpu.flags,
    );
    // 0041e5d2 je short 0041E63Ah
    je(ctx, Cont(x41e5d4), Cont(x41e63a))
}

pub fn x41e5d4(ctx: &mut Context) -> Cont {
    // 0041e5d4 cmp byte ptr [edi+3],3
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x3u32)),
        0x3u8,
        &mut ctx.cpu.flags,
    );
    // 0041e5d8 jne short 0041E5FAh
    jne(ctx, Cont(x41e5da), Cont(x41e5fa))
}

pub fn x41e5da(ctx: &mut Context) -> Cont {
    // 0041e5da and ecx,0FFh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0xffu32, &mut ctx.cpu.flags);
    // 0041e5e0 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e5e1 call 0041E4E0h
    let dst = Cont(x41e4e0);
    call(ctx, 0x41e5e6, dst)
}

pub fn x41e5e6(ctx: &mut Context) -> Cont {
    // 0041e5e6 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e5e9 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x41e5ee, dst)
}

pub fn x41e5ee(ctx: &mut Context) -> Cont {
    // 0041e5ee mov [esi+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 0041e5f1 mov dword ptr [esi+10h],10h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x10u32), 0x10u32);
    // 0041e5f8 jmp short 0041E622h
    Cont(x41e622)
}

pub fn x41e5fa(ctx: &mut Context) -> Cont {
    // 0041e5fa xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e5fc mov [esi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi, ctx.cpu.regs.get_al());
    // 0041e5fe mov dword ptr [esi+4],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x4u32), 0x0u32);
    // 0041e605 mov cl,[edi]
    ctx.cpu.regs.set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 0041e607 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e608 call 0041E4E0h
    let dst = Cont(x41e4e0);
    call(ctx, 0x41e60d, dst)
}

pub fn x41e60d(ctx: &mut Context) -> Cont {
    // 0041e60d add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041e610 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x41e615, dst)
}

pub fn x41e615(ctx: &mut Context) -> Cont {
    // 0041e615 mov [esi+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0041e618 mov [esi+0Ch],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0xcu32), ctx.cpu.regs.eax);
    // 0041e61b mov dword ptr [esi+10h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x10u32), 0x0u32);
    Cont(x41e622)
}

pub fn x41e622(ctx: &mut Context) -> Cont {
    // 0041e622 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e624 mov al,[edi+1]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x1u32)));
    // 0041e627 lea edx,[eax+eax*2]
    ctx.cpu.regs.edx = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 2));
    // 0041e62a mov eax,ds:[425BF0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bf0u32);
    // 0041e62f mov cl,[eax+edx*8+5]
    ctx.cpu.regs.set_cl(
        ctx.memory.read::<u8>(
            ctx.cpu
                .regs
                .eax
                .wrapping_add((ctx.cpu.regs.edx * 8))
                .wrapping_add(0x5u32),
        ),
    );
    // 0041e633 mov byte ptr [esi+14h],80h
    ctx.memory
        .write::<u8>(ctx.cpu.regs.esi.wrapping_add(0x14u32), 0x80u8);
    // 0041e637 mov [esi+15h],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0x15u32),
        ctx.cpu.regs.get_cl(),
    );
    Cont(x41e63a)
}

pub fn x41e63a(ctx: &mut Context) -> Cont {
    // 0041e63a xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e63c mov al,[edi+3]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x3u32)));
    // 0041e63f add eax,0FFFFFFF7h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0xfffffff7u32, &mut ctx.cpu.flags);
    // 0041e642 cmp eax,6
    sub(ctx.cpu.regs.eax, 0x6u32, &mut ctx.cpu.flags);
    // 0041e645 ja short 0041E6A6h
    ja(ctx, Cont(x41e647), Cont(x41e6a6))
}

pub fn x41e647(ctx: &mut Context) -> Cont {
    // 0041e647 jmp dword ptr [eax*4+41E8D4h]
    indirect(
        ctx,
        ctx.memory
            .read((ctx.cpu.regs.eax * 4).wrapping_add(0x41e8d4u32)),
    )
}

pub fn x41e64e(ctx: &mut Context) -> Cont {
    // 0041e64e mov al,[edi+4]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 0041e651 test al,0Fh
    and(ctx.cpu.regs.get_al(), 0xfu8, &mut ctx.cpu.flags);
    // 0041e653 je short 0041E662h
    je(ctx, Cont(x41e655), Cont(x41e662))
}

pub fn x41e655(ctx: &mut Context) -> Cont {
    // 0041e655 mov cl,[esi+15h]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(0x15u32)),
    );
    // 0041e658 shr al,4
    ctx.cpu
        .regs
        .set_al(shr(ctx.cpu.regs.get_al(), 0x4u8, &mut ctx.cpu.flags));
    // 0041e65b add cl,al
    ctx.cpu.regs.set_cl(add(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    ));
    // 0041e65d mov [esi+15h],cl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0x15u32),
        ctx.cpu.regs.get_cl(),
    );
    // 0041e660 jmp short 0041E6A6h
    Cont(x41e6a6)
}

pub fn x41e662(ctx: &mut Context) -> Cont {
    // 0041e662 mov dl,[esi+15h]
    ctx.cpu.regs.set_dl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(0x15u32)),
    );
    // 0041e665 mov [esi+15h],dl
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0x15u32),
        ctx.cpu.regs.get_dl(),
    );
    // 0041e668 jmp short 0041E6A6h
    Cont(x41e6a6)
}

pub fn x41e66a(ctx: &mut Context) -> Cont {
    // 0041e66a mov al,[edi+4]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 0041e66d mov [esi+15h],al
    ctx.memory.write::<u8>(
        ctx.cpu.regs.esi.wrapping_add(0x15u32),
        ctx.cpu.regs.get_al(),
    );
    // 0041e670 jmp short 0041E6A6h
    Cont(x41e6a6)
}

pub fn x41e672(ctx: &mut Context) -> Cont {
    // 0041e672 mov al,[edi+4]
    ctx.cpu
        .regs
        .set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 0041e675 cmp al,20h
    sub(ctx.cpu.regs.get_al(), 0x20u8, &mut ctx.cpu.flags);
    // 0041e677 jae short 0041E68Ah
    jae(ctx, Cont(x41e679), Cont(x41e68a))
}

pub fn x41e679(ctx: &mut Context) -> Cont {
    // 0041e679 and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0041e67e mov ds:[425BD4h],eax
    ctx.memory.write::<u32>(0x425bd4u32, ctx.cpu.regs.eax);
    // 0041e683 call 0041E510h
    let dst = Cont(x41e510);
    call(ctx, 0x41e688, dst)
}

pub fn x41e688(ctx: &mut Context) -> Cont {
    // 0041e688 jmp short 0041E6A6h
    Cont(x41e6a6)
}

pub fn x41e68a(ctx: &mut Context) -> Cont {
    // 0041e68a and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0041e68f mov ds:[425BD0h],eax
    ctx.memory.write::<u32>(0x425bd0u32, ctx.cpu.regs.eax);
    // 0041e694 call 0041E510h
    let dst = Cont(x41e510);
    call(ctx, 0x41e699, dst)
}

pub fn x41e699(ctx: &mut Context) -> Cont {
    // 0041e699 jmp short 0041E6A6h
    Cont(x41e6a6)
}

pub fn x41e69b(ctx: &mut Context) -> Cont {
    // 0041e69b xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e69d mov cl,[edi+4]
    ctx.cpu
        .regs
        .set_cl(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 0041e6a0 shl ecx,12h
    ctx.cpu.regs.ecx = shl(ctx.cpu.regs.ecx, 0x12u8, &mut ctx.cpu.flags);
    // 0041e6a3 mov [esi+4],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x4u32), ctx.cpu.regs.ecx);
    Cont(x41e6a6)
}

pub fn x41e6a6(ctx: &mut Context) -> Cont {
    // 0041e6a6 mov ecx,ds:[425BD8h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e6ac inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e6ad add ebx,18h
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x18u32, &mut ctx.cpu.flags);
    // 0041e6b0 cmp ebp,ecx
    sub(ctx.cpu.regs.ebp, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e6b2 jl near ptr 0041E5A7h
    jl(ctx, Cont(x41e6b8), Cont(x41e5a7))
}

pub fn x41e6b8(ctx: &mut Context) -> Cont {
    // 0041e6b8 mov eax,ds:[425BC4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bc4u32);
    // 0041e6bd mov edx,[esp+10h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041e6c1 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e6c2 mov ds:[425BC4h],eax
    ctx.memory.write::<u32>(0x425bc4u32, ctx.cpu.regs.eax);
    // 0041e6c7 mov ecx,[edx]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(ctx.cpu.regs.edx);
    // 0041e6c9 cmp eax,ecx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e6cb jl short 0041E6F6h
    jl(ctx, Cont(x41e6cd), Cont(x41e6f6))
}

pub fn x41e6cd(ctx: &mut Context) -> Cont {
    // 0041e6cd mov eax,ds:[425BC0h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bc0u32);
    // 0041e6d2 mov ecx,ds:[425BE0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425be0u32);
    // 0041e6d8 inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e6d9 cmp eax,ecx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e6db mov ds:[425BC0h],eax
    ctx.memory.write::<u32>(0x425bc0u32, ctx.cpu.regs.eax);
    // 0041e6e0 jl short 0041E6ECh
    jl(ctx, Cont(x41e6e2), Cont(x41e6ec))
}

pub fn x41e6e2(ctx: &mut Context) -> Cont {
    // 0041e6e2 mov eax,ds:[425BE4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425be4u32);
    // 0041e6e7 mov ds:[425BC0h],eax
    ctx.memory.write::<u32>(0x425bc0u32, ctx.cpu.regs.eax);
    Cont(x41e6ec)
}

pub fn x41e6ec(ctx: &mut Context) -> Cont {
    // 0041e6ec mov dword ptr ds:[425BC4h],0
    ctx.memory.write::<u32>(0x425bc4u32, 0x0u32);
    Cont(x41e6f6)
}

pub fn x41e6f6(ctx: &mut Context) -> Cont {
    // 0041e6f6 mov ecx,ds:[425BCCh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bccu32);
    // 0041e6fc mov ds:[425BC8h],ecx
    ctx.memory.write::<u32>(0x425bc8u32, ctx.cpu.regs.ecx);
    Cont(x41e702)
}

pub fn x41e702(ctx: &mut Context) -> Cont {
    // 0041e702 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e707 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041e709 fld dword ptr ds:[420098h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x420098u32) as f64);
    // 0041e70f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e711 mov dword ptr [esp+20h],0
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0x0u32);
    // 0041e719 jle near ptr 0041E801h
    jle(ctx, Cont(x41e71f), Cont(x41e801))
}

pub fn x41e71f(ctx: &mut Context) -> Cont {
    // 0041e71f mov edx,ds:[425BDCh]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bdcu32);
    // 0041e725 mov ebp,ds:[425BF0h]
    ctx.cpu.regs.ebp = ctx.memory.read::<u32>(0x425bf0u32);
    // 0041e72b lea ecx,[edx+4]
    ctx.cpu.regs.ecx = ctx.cpu.regs.edx.wrapping_add(0x4u32);
    Cont(x41e72e)
}

pub fn x41e72e(ctx: &mut Context) -> Cont {
    // 0041e72e mov edx,edi
    ctx.cpu.regs.edx = ctx.cpu.regs.edi;
    // 0041e730 and edx,80000003h
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x80000003u32, &mut ctx.cpu.flags);
    // 0041e736 jns short 0041E73Dh
    jns(ctx, Cont(x41e738), Cont(x41e73d))
}

pub fn x41e738(ctx: &mut Context) -> Cont {
    // 0041e738 dec edx
    ctx.cpu.regs.edx = dec(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e739 or edx,0FFFFFFFCh
    ctx.cpu.regs.edx = or(ctx.cpu.regs.edx, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0041e73c inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    Cont(x41e73d)
}

pub fn x41e73d(ctx: &mut Context) -> Cont {
    // 0041e73d mov al,[ecx-4]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(0xfffffffcu32)),
    );
    // 0041e740 test al,al
    and(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    );
    // 0041e742 je short 0041E798h
    je(ctx, Cont(x41e744), Cont(x41e798))
}

pub fn x41e744(ctx: &mut Context) -> Cont {
    // 0041e744 and eax,0FFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffu32, &mut ctx.cpu.flags);
    // 0041e749 lea eax,[eax+eax*2]
    ctx.cpu.regs.eax = ctx.cpu.regs.eax.wrapping_add((ctx.cpu.regs.eax * 2));
    // 0041e74c lea esi,[ebp+eax*8]
    ctx.cpu.regs.esi = ctx.cpu.regs.ebp.wrapping_add((ctx.cpu.regs.eax * 8));
    // 0041e750 mov eax,[ecx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.ecx);
    // 0041e752 sar eax,0Ah
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0xau8, &mut ctx.cpu.flags);
    // 0041e755 js short 0041E798h
    js(ctx, Cont(x41e757), Cont(x41e798))
}

pub fn x41e757(ctx: &mut Context) -> Cont {
    // 0041e757 cmp eax,[esi+8]
    sub(
        ctx.cpu.regs.eax,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x8u32)),
        &mut ctx.cpu.flags,
    );
    // 0041e75a jge short 0041E798h
    jge(ctx, Cont(x41e75c), Cont(x41e798))
}

pub fn x41e75c(ctx: &mut Context) -> Cont {
    // 0041e75c mov esi,[esi+14h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esi.wrapping_add(0x14u32));
    // 0041e75f movsx eax,word ptr [esi+eax*2]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u16>(ctx.cpu.regs.esi.wrapping_add((ctx.cpu.regs.eax * 2)))
        as i16 as i32 as u32;
    // 0041e763 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 0041e767 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e769 mov al,[ecx+11h]
    ctx.cpu.regs.set_al(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.ecx.wrapping_add(0x11u32)),
    );
    // 0041e76c fild dword ptr [esp+10h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as i32 as f64,
    );
    // 0041e770 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 0041e774 fimul dword ptr [esp+10h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)) as f64,
    );
    // 0041e778 fmul dword ptr ds:[4204CCh]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0) * ctx.memory.read::<f32>(0x4204ccu32) as f64,
    );
    // 0041e77e fld st(0)
    ctx.cpu.fpu.push(ctx.cpu.fpu.get(0));
    // 0041e780 fmul dword ptr [edx*4+421A34h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x421a34u32)) as f64,
    );
    // 0041e787 fadd dword ptr [esp+20h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            + ctx
                .memory
                .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0041e78b fstp dword ptr [esp+20h]
    ctx.memory.write::<f32>(
        ctx.cpu.regs.esp.wrapping_add(0x20u32),
        ctx.cpu.fpu.get(0) as f32,
    );
    ctx.cpu.fpu.pop();
    // 0041e78f fmul dword ptr [edx*4+421A44h]
    ctx.cpu.fpu.set(
        0,
        ctx.cpu.fpu.get(0)
            * ctx
                .memory
                .read::<f32>((ctx.cpu.regs.edx * 4).wrapping_add(0x421a44u32)) as f64,
    );
    // 0041e796 faddp
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(1) + ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    Cont(x41e798)
}

pub fn x41e798(ctx: &mut Context) -> Cont {
    // 0041e798 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e79d inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041e79e add ecx,18h
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x18u32, &mut ctx.cpu.flags);
    // 0041e7a1 cmp edi,eax
    sub(ctx.cpu.regs.edi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e7a3 jl short 0041E72Eh
    jl(ctx, Cont(x41e7a5), Cont(x41e72e))
}

pub fn x41e7a5(ctx: &mut Context) -> Cont {
    // 0041e7a5 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0041e7a9 fcomp dword ptr ds:[4204C8h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4204c8u32) as f64));
    ctx.cpu.fpu.pop();
    // 0041e7af fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0041e7b1 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0041e7b4 je short 0041E7BEh
    je(ctx, Cont(x41e7b6), Cont(x41e7be))
}

pub fn x41e7b6(ctx: &mut Context) -> Cont {
    // 0041e7b6 mov dword ptr [esp+20h],0C6FFFC00h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0xc6fffc00u32);
    Cont(x41e7be)
}

pub fn x41e7be(ctx: &mut Context) -> Cont {
    // 0041e7be fcom dword ptr ds:[4204C8h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4204c8u32) as f64));
    // 0041e7c4 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0041e7c6 test ah,1
    and(ctx.cpu.regs.get_ah(), 0x1u8, &mut ctx.cpu.flags);
    // 0041e7c9 je short 0041E7D3h
    je(ctx, Cont(x41e7cb), Cont(x41e7d3))
}

pub fn x41e7cb(ctx: &mut Context) -> Cont {
    // 0041e7cb fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0041e7cd fld dword ptr ds:[4204C4h]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4204c4u32) as f64);
    Cont(x41e7d3)
}

pub fn x41e7d3(ctx: &mut Context) -> Cont {
    // 0041e7d3 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0041e7d7 fcomp dword ptr ds:[4204C0h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4204c0u32) as f64));
    ctx.cpu.fpu.pop();
    // 0041e7dd fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0041e7df test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 0041e7e2 jne short 0041E7ECh
    jne(ctx, Cont(x41e7e4), Cont(x41e7ec))
}

pub fn x41e7e4(ctx: &mut Context) -> Cont {
    // 0041e7e4 mov dword ptr [esp+20h],46FFFC00h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), 0x46fffc00u32);
    Cont(x41e7ec)
}

pub fn x41e7ec(ctx: &mut Context) -> Cont {
    // 0041e7ec fcom dword ptr ds:[4204C0h]
    ctx.cpu.fpu.cmp = ctx
        .cpu
        .fpu
        .get(0)
        .total_cmp(&(ctx.memory.read::<f32>(0x4204c0u32) as f64));
    // 0041e7f2 fnstsw ax
    ctx.cpu.regs.set_ax(ctx.cpu.fpu.status());
    // 0041e7f4 test ah,41h
    and(ctx.cpu.regs.get_ah(), 0x41u8, &mut ctx.cpu.flags);
    // 0041e7f7 jne short 0041E801h
    jne(ctx, Cont(x41e7f9), Cont(x41e801))
}

pub fn x41e7f9(ctx: &mut Context) -> Cont {
    // 0041e7f9 fstp st(0)
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0041e7fb fld dword ptr ds:[4204BCh]
    ctx.cpu.fpu.push(ctx.memory.read::<f32>(0x4204bcu32) as f64);
    Cont(x41e801)
}

pub fn x41e801(ctx: &mut Context) -> Cont {
    // 0041e801 fld dword ptr [esp+20h]
    ctx.cpu.fpu.push(
        ctx.memory
            .read::<f32>(ctx.cpu.regs.esp.wrapping_add(0x20u32)) as f64,
    );
    // 0041e805 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x41e80a, dst)
}

pub fn x41e80a(ctx: &mut Context) -> Cont {
    // 0041e80a mov esi,[esp+1Ch]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 0041e80e mov [esi],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esi, ctx.cpu.regs.get_ax());
    // 0041e811 add esi,2
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x2u32, &mut ctx.cpu.flags);
    // 0041e814 call 0041F090h
    let dst = Cont(x41f090);
    call(ctx, 0x41e819, dst)
}

pub fn x41e819(ctx: &mut Context) -> Cont {
    // 0041e819 mov [esi],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esi, ctx.cpu.regs.get_ax());
    // 0041e81c mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e821 add esi,2
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x2u32, &mut ctx.cpu.flags);
    // 0041e824 xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e826 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e828 mov [esp+1Ch],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), ctx.cpu.regs.esi);
    // 0041e82c jle near ptr 0041E8BAh
    jle(ctx, Cont(x41e832), Cont(x41e8ba))
}

pub fn x41e832(ctx: &mut Context) -> Cont {
    // 0041e832 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    Cont(x41e834)
}

pub fn x41e834(ctx: &mut Context) -> Cont {
    // 0041e834 mov ecx,ds:[425BDCh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bdcu32);
    // 0041e83a lea eax,[edi+ecx]
    ctx.cpu.regs.eax = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx);
    // 0041e83d mov cl,[edi+ecx]
    ctx.cpu.regs.set_cl(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ecx)),
    );
    // 0041e840 test cl,cl
    and(
        ctx.cpu.regs.get_cl(),
        ctx.cpu.regs.get_cl(),
        &mut ctx.cpu.flags,
    );
    // 0041e842 je short 0041E8A9h
    je(ctx, Cont(x41e844), Cont(x41e8a9))
}

pub fn x41e844(ctx: &mut Context) -> Cont {
    // 0041e844 and ecx,0FFh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0xffu32, &mut ctx.cpu.flags);
    // 0041e84a mov esi,[eax+4]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0041e84d lea edx,[ecx+ecx*2]
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.ecx * 2));
    // 0041e850 mov ecx,ds:[425BF0h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bf0u32);
    // 0041e856 lea edx,[ecx+edx*8]
    ctx.cpu.regs.edx = ctx.cpu.regs.ecx.wrapping_add((ctx.cpu.regs.edx * 8));
    // 0041e859 mov ecx,[eax+8]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32));
    // 0041e85c add esi,ecx
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e85e mov [eax+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    // 0041e861 mov bl,[edx]
    ctx.cpu.regs.set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edx));
    // 0041e863 test bl,1
    and(ctx.cpu.regs.get_bl(), 0x1u8, &mut ctx.cpu.flags);
    // 0041e866 je short 0041E87Bh
    je(ctx, Cont(x41e868), Cont(x41e87b))
}

pub fn x41e868(ctx: &mut Context) -> Cont {
    // 0041e868 mov ebx,[edx+10h]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add(0x10u32));
    // 0041e86b shl ebx,0Ah
    ctx.cpu.regs.ebx = shl(ctx.cpu.regs.ebx, 0xau8, &mut ctx.cpu.flags);
    // 0041e86e cmp esi,ebx
    sub(ctx.cpu.regs.esi, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0041e870 jl short 0041E87Bh
    jl(ctx, Cont(x41e872), Cont(x41e87b))
}

pub fn x41e872(ctx: &mut Context) -> Cont {
    // 0041e872 mov esi,[edx+0Ch]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add(0xcu32));
    // 0041e875 shl esi,0Ah
    ctx.cpu.regs.esi = shl(ctx.cpu.regs.esi, 0xau8, &mut ctx.cpu.flags);
    // 0041e878 mov [eax+4],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32), ctx.cpu.regs.esi);
    Cont(x41e87b)
}

pub fn x41e87b(ctx: &mut Context) -> Cont {
    // 0041e87b mov esi,[eax+0Ch]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0xcu32));
    // 0041e87e cmp ecx,esi
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e880 jge short 0041E88Ch
    jge(ctx, Cont(x41e882), Cont(x41e88c))
}

pub fn x41e882(ctx: &mut Context) -> Cont {
    // 0041e882 mov esi,[eax+10h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x10u32));
    // 0041e885 add esi,ecx
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0041e887 mov [eax+8],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.esi);
    // 0041e88a jmp short 0041E894h
    Cont(x41e894)
}

pub fn x41e88c(ctx: &mut Context) -> Cont {
    // 0041e88c jle short 0041E894h
    jle(ctx, Cont(x41e88e), Cont(x41e894))
}

pub fn x41e88e(ctx: &mut Context) -> Cont {
    // 0041e88e sub ecx,[eax+10h]
    ctx.cpu.regs.ecx = sub(
        ctx.cpu.regs.ecx,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x10u32)),
        &mut ctx.cpu.flags,
    );
    // 0041e891 mov [eax+8],ecx
    ctx.memory
        .write::<u32>(ctx.cpu.regs.eax.wrapping_add(0x8u32), ctx.cpu.regs.ecx);
    Cont(x41e894)
}

pub fn x41e894(ctx: &mut Context) -> Cont {
    // 0041e894 test byte ptr [edx],1
    and(
        ctx.memory.read::<u8>(ctx.cpu.regs.edx),
        0x1u8,
        &mut ctx.cpu.flags,
    );
    // 0041e897 jne short 0041E8A9h
    jne(ctx, Cont(x41e899), Cont(x41e8a9))
}

pub fn x41e899(ctx: &mut Context) -> Cont {
    // 0041e899 mov edx,[edx+8]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edx.wrapping_add(0x8u32));
    // 0041e89c mov ecx,[eax+4]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.eax.wrapping_add(0x4u32));
    // 0041e89f shl edx,0Ah
    ctx.cpu.regs.edx = shl(ctx.cpu.regs.edx, 0xau8, &mut ctx.cpu.flags);
    // 0041e8a2 cmp ecx,edx
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e8a4 jl short 0041E8A9h
    jl(ctx, Cont(x41e8a6), Cont(x41e8a9))
}

pub fn x41e8a6(ctx: &mut Context) -> Cont {
    // 0041e8a6 mov byte ptr [eax],0
    ctx.memory.write::<u8>(ctx.cpu.regs.eax, 0x0u8);
    Cont(x41e8a9)
}

pub fn x41e8a9(ctx: &mut Context) -> Cont {
    // 0041e8a9 mov eax,ds:[425BD8h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bd8u32);
    // 0041e8ae inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041e8af add edi,18h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x18u32, &mut ctx.cpu.flags);
    // 0041e8b2 cmp ebp,eax
    sub(ctx.cpu.regs.ebp, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e8b4 jl near ptr 0041E834h
    jl(ctx, Cont(x41e8ba), Cont(x41e834))
}

pub fn x41e8ba(ctx: &mut Context) -> Cont {
    // 0041e8ba mov eax,[esp+14h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32));
    // 0041e8be dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e8bf mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 0041e8c3 jne near ptr 0041E56Ah
    jne(ctx, Cont(x41e8c9), Cont(x41e56a))
}

pub fn x41e8c9(ctx: &mut Context) -> Cont {
    // 0041e8c9 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041e8ca pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e8cb pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041e8cc pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    Cont(x41e8cd)
}

pub fn x41e8cd(ctx: &mut Context) -> Cont {
    // 0041e8cd add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041e8d0 ret
    ret(ctx, 0)
}

pub fn x41e8f0(ctx: &mut Context) -> Cont {
    // 0041e8f0 cmp dword ptr [esp+8],3BDh
    sub(
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32)),
        0x3bdu32,
        &mut ctx.cpu.flags,
    );
    // 0041e8f8 jne short 0041E906h
    jne(ctx, Cont(x41e8fa), Cont(x41e906))
}

pub fn x41e8fa(ctx: &mut Context) -> Cont {
    // 0041e8fa mov eax,ds:[425BA4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425ba4u32);
    // 0041e8ff push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e900 call dword ptr ds:[420030h]
    let dst = Cont(kernel32::SetEvent_stdcall);
    call(ctx, 0x41e906, dst)
}

pub fn x41e906(ctx: &mut Context) -> Cont {
    // 0041e906 ret 14h
    ret(ctx, 20)
}

pub fn x41e910(ctx: &mut Context) -> Cont {
    // 0041e910 sub esp,14h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 0041e913 mov ecx,[esp+20h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 0041e917 mov eax,[esp+24h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0041e91b mov [esp+2],cx
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x2u32), ctx.cpu.regs.get_cx());
    // 0041e920 mov [esp+0Eh],ax
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0xeu32), ctx.cpu.regs.get_ax());
    // 0041e925 and eax,0FFFFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffffu32, &mut ctx.cpu.flags);
    // 0041e92a and ecx,0FFFFh
    ctx.cpu.regs.ecx = and(ctx.cpu.regs.ecx, 0xffffu32, &mut ctx.cpu.flags);
    // 0041e930 imul eax,ecx
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.ecx as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 0041e933 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0041e934 and edx,7
    ctx.cpu.regs.edx = and(ctx.cpu.regs.edx, 0x7u32, &mut ctx.cpu.flags);
    // 0041e937 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e938 mov esi,[esp+20h]
    ctx.cpu.regs.esi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32));
    // 0041e93c add eax,edx
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041e93e sar eax,3
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x3u8, &mut ctx.cpu.flags);
    // 0041e941 mov [esp+10h],ax
    ctx.memory.write::<u16>(
        ctx.cpu.regs.esp.wrapping_add(0x10u32),
        ctx.cpu.regs.get_ax(),
    );
    // 0041e946 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041e94a and eax,0FFFFh
    ctx.cpu.regs.eax = and(ctx.cpu.regs.eax, 0xffffu32, &mut ctx.cpu.flags);
    // 0041e94f mov edx,[esp+1Ch]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x1cu32));
    // 0041e953 imul eax,esi
    let x = ctx.cpu.regs.eax as i32;
    let y = ctx.cpu.regs.esi as i32;
    let (res, overflow) = x.overflowing_mul(y);
    ctx.cpu.flags.set(Flags::CF, overflow);
    ctx.cpu.flags.set(Flags::OF, overflow);
    ctx.cpu.regs.eax = res as u32;
    // 0041e956 push 0
    push(ctx, 0x0u32);
    // 0041e958 push 0
    push(ctx, 0x0u32);
    // 0041e95a lea ecx,[esp+0Ch]
    ctx.cpu.regs.ecx = ctx.cpu.regs.esp.wrapping_add(0xcu32);
    // 0041e95e mov [esp+14h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x14u32), ctx.cpu.regs.eax);
    // 0041e962 push 0
    push(ctx, 0x0u32);
    // 0041e964 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e965 lea eax,[esp+34h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x34u32);
    // 0041e969 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041e96a push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e96b mov word ptr [esp+1Ch],1
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x1cu32), 0x1u16);
    // 0041e972 mov [esp+20h],esi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x20u32), ctx.cpu.regs.esi);
    // 0041e976 mov word ptr [esp+2Ch],0
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), 0x0u16);
    // 0041e97d call dword ptr ds:[420084h]
    let dst = Cont(winmm::waveOutOpen_stdcall);
    call(ctx, 0x41e983, dst)
}

pub fn x41e983(ctx: &mut Context) -> Cont {
    // 0041e983 mov ecx,[esp+24h]
    ctx.cpu.regs.ecx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0041e987 mov esi,eax
    ctx.cpu.regs.esi = ctx.cpu.regs.eax;
    // 0041e989 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041e98a call dword ptr ds:[420088h]
    let dst = Cont(winmm::waveOutReset_stdcall);
    call(ctx, 0x41e990, dst)
}

pub fn x41e990(ctx: &mut Context) -> Cont {
    // 0041e990 mov edx,[esp+24h]
    ctx.cpu.regs.edx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x24u32));
    // 0041e994 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041e995 call dword ptr ds:[42008Ch]
    let dst = Cont(winmm::waveOutClose_stdcall);
    call(ctx, 0x41e99b, dst)
}

pub fn x41e99b(ctx: &mut Context) -> Cont {
    // 0041e99b test esi,esi
    and(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e99d pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041e99e je short 0041E9B3h
    je(ctx, Cont(x41e9a0), Cont(x41e9b3))
}

pub fn x41e9a0(ctx: &mut Context) -> Cont {
    // 0041e9a0 mov dword ptr ds:[425BB4h],0FFFFFFFFh
    ctx.memory.write::<u32>(0x425bb4u32, 0xffffffffu32);
    // 0041e9aa mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0041e9af add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 0041e9b2 ret
    ret(ctx, 0)
}

pub fn x41e9b3(ctx: &mut Context) -> Cont {
    // 0041e9b3 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e9b5 add esp,14h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x14u32, &mut ctx.cpu.flags);
    // 0041e9b8 ret
    ret(ctx, 0)
}

pub fn x41e9c0(ctx: &mut Context) -> Cont {
    // 0041e9c0 sub esp,38h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041e9c3 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041e9c4 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041e9c5 call dword ptr ds:[42007Ch]
    let dst = Cont(winmm::waveOutGetNumDevs_stdcall);
    call(ctx, 0x41e9cb, dst)
}

pub fn x41e9cb(ctx: &mut Context) -> Cont {
    // 0041e9cb xor ebx,ebx
    ctx.cpu.regs.ebx = xor(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0041e9cd mov [esp+8],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x8u32), ctx.cpu.regs.eax);
    // 0041e9d1 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e9d3 mov dword ptr ds:[425BB4h],0FFFFFFFFh
    ctx.memory.write::<u32>(0x425bb4u32, 0xffffffffu32);
    // 0041e9dd je near ptr 0041EC5Ch
    je(ctx, Cont(x41e9e3), Cont(x41ec5c))
}

pub fn x41e9e3(ctx: &mut Context) -> Cont {
    // 0041e9e3 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e9e4 xor esi,esi
    ctx.cpu.regs.esi = xor(ctx.cpu.regs.esi, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041e9e6 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041e9e8 jle short 0041EA1Eh
    jle(ctx, Cont(x41e9ea), Cont(x41ea1e))
}

pub fn x41e9ea(ctx: &mut Context) -> Cont {
    // 0041e9ea mov edi,[esp+0Ch]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    // 0041e9ee push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041e9ef mov ebp,ds:[420080h]
    ctx.cpu.regs.ebp = ctx.memory.read::<u32>(0x420080u32);
    Cont(x41e9f5)
}

pub fn x41e9f5(ctx: &mut Context) -> Cont {
    // 0041e9f5 lea eax,[esp+14h]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x14u32);
    // 0041e9f9 push 34h
    push(ctx, 0x34u32);
    // 0041e9fb push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041e9fc push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041e9fd call ebp
    let dst = indirect(ctx, ctx.cpu.regs.ebp);
    call(ctx, 0x41e9ff, dst)
}

pub fn x41e9ff(ctx: &mut Context) -> Cont {
    // 0041e9ff mov eax,[esp+3Ch]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x3cu32));
    // 0041ea03 cmp eax,ebx
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0041ea05 jbe short 0041EA12h
    jbe(ctx, Cont(x41ea07), Cont(x41ea12))
}

pub fn x41ea07(ctx: &mut Context) -> Cont {
    // 0041ea07 test eax,0FFFh
    and(ctx.cpu.regs.eax, 0xfffu32, &mut ctx.cpu.flags);
    // 0041ea0c je short 0041EA12h
    je(ctx, Cont(x41ea0e), Cont(x41ea12))
}

pub fn x41ea0e(ctx: &mut Context) -> Cont {
    // 0041ea0e mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 0041ea10 mov edi,esi
    ctx.cpu.regs.edi = ctx.cpu.regs.esi;
    Cont(x41ea12)
}

pub fn x41ea12(ctx: &mut Context) -> Cont {
    // 0041ea12 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041ea16 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0041ea17 cmp esi,eax
    sub(ctx.cpu.regs.esi, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ea19 jl short 0041E9F5h
    jl(ctx, Cont(x41ea1b), Cont(x41e9f5))
}

pub fn x41ea1b(ctx: &mut Context) -> Cont {
    // 0041ea1b pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041ea1c jmp short 0041EA22h
    Cont(x41ea22)
}

pub fn x41ea1e(ctx: &mut Context) -> Cont {
    // 0041ea1e mov edi,[esp+0Ch]
    ctx.cpu.regs.edi = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0xcu32));
    Cont(x41ea22)
}

pub fn x41ea22(ctx: &mut Context) -> Cont {
    // 0041ea22 test bh,8
    and(ctx.cpu.regs.get_bh(), 0x8u8, &mut ctx.cpu.flags);
    // 0041ea25 mov ds:[425BB4h],edi
    ctx.memory.write::<u32>(0x425bb4u32, ctx.cpu.regs.edi);
    // 0041ea2b mov word ptr ds:[425BB8h],0
    ctx.memory.write::<u16>(0x425bb8u32, 0x0u16);
    // 0041ea34 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041ea35 je short 0041EA61h
    je(ctx, Cont(x41ea37), Cont(x41ea61))
}

pub fn x41ea37(ctx: &mut Context) -> Cont {
    // 0041ea37 push 10h
    push(ctx, 0x10u32);
    // 0041ea39 push 2
    push(ctx, 0x2u32);
    // 0041ea3b push 0AC44h
    push(ctx, 0xac44u32);
    // 0041ea40 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ea41 mov word ptr ds:[425BB0h],0AC44h
    ctx.memory.write::<u16>(0x425bb0u32, 0xac44u16);
    // 0041ea4a mov word ptr ds:[425BA8h],11Ah
    ctx.memory.write::<u16>(0x425ba8u32, 0x11au16);
    // 0041ea53 call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41ea58, dst)
}

pub fn x41ea58(ctx: &mut Context) -> Cont {
    // 0041ea58 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041ea5b pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ea5c pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ea5d add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ea60 ret
    ret(ctx, 0)
}

pub fn x41ea61(ctx: &mut Context) -> Cont {
    // 0041ea61 test bh,2
    and(ctx.cpu.regs.get_bh(), 0x2u8, &mut ctx.cpu.flags);
    // 0041ea64 je short 0041EA90h
    je(ctx, Cont(x41ea66), Cont(x41ea90))
}

pub fn x41ea66(ctx: &mut Context) -> Cont {
    // 0041ea66 push 8
    push(ctx, 0x8u32);
    // 0041ea68 push 2
    push(ctx, 0x2u32);
    // 0041ea6a push 0AC44h
    push(ctx, 0xac44u32);
    // 0041ea6f push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ea70 mov word ptr ds:[425BB0h],0AC44h
    ctx.memory.write::<u16>(0x425bb0u32, 0xac44u16);
    // 0041ea79 mov word ptr ds:[425BA8h],109h
    ctx.memory.write::<u16>(0x425ba8u32, 0x109u16);
    // 0041ea82 call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41ea87, dst)
}

pub fn x41ea87(ctx: &mut Context) -> Cont {
    // 0041ea87 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041ea8a pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ea8b pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ea8c add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ea8f ret
    ret(ctx, 0)
}

pub fn x41ea90(ctx: &mut Context) -> Cont {
    // 0041ea90 test bh,4
    and(ctx.cpu.regs.get_bh(), 0x4u8, &mut ctx.cpu.flags);
    // 0041ea93 je short 0041EABFh
    je(ctx, Cont(x41ea95), Cont(x41eabf))
}

pub fn x41ea95(ctx: &mut Context) -> Cont {
    // 0041ea95 push 10h
    push(ctx, 0x10u32);
    // 0041ea97 push 1
    push(ctx, 0x1u32);
    // 0041ea99 push 0AC44h
    push(ctx, 0xac44u32);
    // 0041ea9e push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ea9f mov word ptr ds:[425BB0h],0AC44h
    ctx.memory.write::<u16>(0x425bb0u32, 0xac44u16);
    // 0041eaa8 mov word ptr ds:[425BA8h],116h
    ctx.memory.write::<u16>(0x425ba8u32, 0x116u16);
    // 0041eab1 call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eab6, dst)
}

pub fn x41eab6(ctx: &mut Context) -> Cont {
    // 0041eab6 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eab9 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eaba pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eabb add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eabe ret
    ret(ctx, 0)
}

pub fn x41eabf(ctx: &mut Context) -> Cont {
    // 0041eabf test bh,1
    and(ctx.cpu.regs.get_bh(), 0x1u8, &mut ctx.cpu.flags);
    // 0041eac2 je short 0041EAEEh
    je(ctx, Cont(x41eac4), Cont(x41eaee))
}

pub fn x41eac4(ctx: &mut Context) -> Cont {
    // 0041eac4 push 8
    push(ctx, 0x8u32);
    // 0041eac6 push 1
    push(ctx, 0x1u32);
    // 0041eac8 push 0AC44h
    push(ctx, 0xac44u32);
    // 0041eacd push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041eace mov word ptr ds:[425BB0h],0AC44h
    ctx.memory.write::<u16>(0x425bb0u32, 0xac44u16);
    // 0041ead7 mov word ptr ds:[425BA8h],105h
    ctx.memory.write::<u16>(0x425ba8u32, 0x105u16);
    // 0041eae0 call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eae5, dst)
}

pub fn x41eae5(ctx: &mut Context) -> Cont {
    // 0041eae5 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eae8 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eae9 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eaea add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eaed ret
    ret(ctx, 0)
}

pub fn x41eaee(ctx: &mut Context) -> Cont {
    // 0041eaee test bl,80h
    and(ctx.cpu.regs.get_bl(), 0x80u8, &mut ctx.cpu.flags);
    // 0041eaf1 je short 0041EB1Dh
    je(ctx, Cont(x41eaf3), Cont(x41eb1d))
}

pub fn x41eaf3(ctx: &mut Context) -> Cont {
    // 0041eaf3 push 10h
    push(ctx, 0x10u32);
    // 0041eaf5 push 2
    push(ctx, 0x2u32);
    // 0041eaf7 push 5622h
    push(ctx, 0x5622u32);
    // 0041eafc push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041eafd mov word ptr ds:[425BB0h],5622h
    ctx.memory.write::<u16>(0x425bb0u32, 0x5622u16);
    // 0041eb06 mov word ptr ds:[425BA8h],11Ah
    ctx.memory.write::<u16>(0x425ba8u32, 0x11au16);
    // 0041eb0f call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eb14, dst)
}

pub fn x41eb14(ctx: &mut Context) -> Cont {
    // 0041eb14 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eb17 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eb18 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eb19 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eb1c ret
    ret(ctx, 0)
}

pub fn x41eb1d(ctx: &mut Context) -> Cont {
    // 0041eb1d test bl,20h
    and(ctx.cpu.regs.get_bl(), 0x20u8, &mut ctx.cpu.flags);
    // 0041eb20 je short 0041EB4Ch
    je(ctx, Cont(x41eb22), Cont(x41eb4c))
}

pub fn x41eb22(ctx: &mut Context) -> Cont {
    // 0041eb22 push 8
    push(ctx, 0x8u32);
    // 0041eb24 push 2
    push(ctx, 0x2u32);
    // 0041eb26 push 5622h
    push(ctx, 0x5622u32);
    // 0041eb2b push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041eb2c mov word ptr ds:[425BB0h],5622h
    ctx.memory.write::<u16>(0x425bb0u32, 0x5622u16);
    // 0041eb35 mov word ptr ds:[425BA8h],109h
    ctx.memory.write::<u16>(0x425ba8u32, 0x109u16);
    // 0041eb3e call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eb43, dst)
}

pub fn x41eb43(ctx: &mut Context) -> Cont {
    // 0041eb43 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eb46 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eb47 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eb48 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eb4b ret
    ret(ctx, 0)
}

pub fn x41eb4c(ctx: &mut Context) -> Cont {
    // 0041eb4c test bl,40h
    and(ctx.cpu.regs.get_bl(), 0x40u8, &mut ctx.cpu.flags);
    // 0041eb4f je short 0041EB7Bh
    je(ctx, Cont(x41eb51), Cont(x41eb7b))
}

pub fn x41eb51(ctx: &mut Context) -> Cont {
    // 0041eb51 push 10h
    push(ctx, 0x10u32);
    // 0041eb53 push 1
    push(ctx, 0x1u32);
    // 0041eb55 push 5622h
    push(ctx, 0x5622u32);
    // 0041eb5a push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041eb5b mov word ptr ds:[425BB0h],5622h
    ctx.memory.write::<u16>(0x425bb0u32, 0x5622u16);
    // 0041eb64 mov word ptr ds:[425BA8h],116h
    ctx.memory.write::<u16>(0x425ba8u32, 0x116u16);
    // 0041eb6d call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eb72, dst)
}

pub fn x41eb72(ctx: &mut Context) -> Cont {
    // 0041eb72 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eb75 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eb76 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eb77 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eb7a ret
    ret(ctx, 0)
}

pub fn x41eb7b(ctx: &mut Context) -> Cont {
    // 0041eb7b test bl,10h
    and(ctx.cpu.regs.get_bl(), 0x10u8, &mut ctx.cpu.flags);
    // 0041eb7e je short 0041EBAAh
    je(ctx, Cont(x41eb80), Cont(x41ebaa))
}

pub fn x41eb80(ctx: &mut Context) -> Cont {
    // 0041eb80 push 8
    push(ctx, 0x8u32);
    // 0041eb82 push 1
    push(ctx, 0x1u32);
    // 0041eb84 push 5622h
    push(ctx, 0x5622u32);
    // 0041eb89 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041eb8a mov word ptr ds:[425BB0h],5622h
    ctx.memory.write::<u16>(0x425bb0u32, 0x5622u16);
    // 0041eb93 mov word ptr ds:[425BA8h],105h
    ctx.memory.write::<u16>(0x425ba8u32, 0x105u16);
    // 0041eb9c call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41eba1, dst)
}

pub fn x41eba1(ctx: &mut Context) -> Cont {
    // 0041eba1 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041eba4 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041eba5 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041eba6 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041eba9 ret
    ret(ctx, 0)
}

pub fn x41ebaa(ctx: &mut Context) -> Cont {
    // 0041ebaa test bl,8
    and(ctx.cpu.regs.get_bl(), 0x8u8, &mut ctx.cpu.flags);
    // 0041ebad je short 0041EBD9h
    je(ctx, Cont(x41ebaf), Cont(x41ebd9))
}

pub fn x41ebaf(ctx: &mut Context) -> Cont {
    // 0041ebaf push 10h
    push(ctx, 0x10u32);
    // 0041ebb1 push 2
    push(ctx, 0x2u32);
    // 0041ebb3 push 2B11h
    push(ctx, 0x2b11u32);
    // 0041ebb8 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ebb9 mov word ptr ds:[425BB0h],2B11h
    ctx.memory.write::<u16>(0x425bb0u32, 0x2b11u16);
    // 0041ebc2 mov word ptr ds:[425BA8h],11Ah
    ctx.memory.write::<u16>(0x425ba8u32, 0x11au16);
    // 0041ebcb call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41ebd0, dst)
}

pub fn x41ebd0(ctx: &mut Context) -> Cont {
    // 0041ebd0 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041ebd3 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ebd4 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ebd5 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ebd8 ret
    ret(ctx, 0)
}

pub fn x41ebd9(ctx: &mut Context) -> Cont {
    // 0041ebd9 test bl,2
    and(ctx.cpu.regs.get_bl(), 0x2u8, &mut ctx.cpu.flags);
    // 0041ebdc je short 0041EC08h
    je(ctx, Cont(x41ebde), Cont(x41ec08))
}

pub fn x41ebde(ctx: &mut Context) -> Cont {
    // 0041ebde push 8
    push(ctx, 0x8u32);
    // 0041ebe0 push 2
    push(ctx, 0x2u32);
    // 0041ebe2 push 2B11h
    push(ctx, 0x2b11u32);
    // 0041ebe7 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ebe8 mov word ptr ds:[425BB0h],2B11h
    ctx.memory.write::<u16>(0x425bb0u32, 0x2b11u16);
    // 0041ebf1 mov word ptr ds:[425BA8h],109h
    ctx.memory.write::<u16>(0x425ba8u32, 0x109u16);
    // 0041ebfa call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41ebff, dst)
}

pub fn x41ebff(ctx: &mut Context) -> Cont {
    // 0041ebff add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041ec02 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ec03 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ec04 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ec07 ret
    ret(ctx, 0)
}

pub fn x41ec08(ctx: &mut Context) -> Cont {
    // 0041ec08 test bl,4
    and(ctx.cpu.regs.get_bl(), 0x4u8, &mut ctx.cpu.flags);
    // 0041ec0b je short 0041EC23h
    je(ctx, Cont(x41ec0d), Cont(x41ec23))
}

pub fn x41ec0d(ctx: &mut Context) -> Cont {
    // 0041ec0d mov word ptr ds:[425BB0h],2B11h
    ctx.memory.write::<u16>(0x425bb0u32, 0x2b11u16);
    // 0041ec16 mov word ptr ds:[425BA8h],116h
    ctx.memory.write::<u16>(0x425ba8u32, 0x116u16);
    // 0041ec1f push 10h
    push(ctx, 0x10u32);
    // 0041ec21 jmp short 0041EC3Ch
    Cont(x41ec3c)
}

pub fn x41ec23(ctx: &mut Context) -> Cont {
    // 0041ec23 test bl,1
    and(ctx.cpu.regs.get_bl(), 0x1u8, &mut ctx.cpu.flags);
    // 0041ec26 je short 0041EC52h
    je(ctx, Cont(x41ec28), Cont(x41ec52))
}

pub fn x41ec28(ctx: &mut Context) -> Cont {
    // 0041ec28 mov word ptr ds:[425BB0h],2B11h
    ctx.memory.write::<u16>(0x425bb0u32, 0x2b11u16);
    // 0041ec31 mov word ptr ds:[425BA8h],105h
    ctx.memory.write::<u16>(0x425ba8u32, 0x105u16);
    // 0041ec3a push 8
    push(ctx, 0x8u32);
    Cont(x41ec3c)
}

pub fn x41ec3c(ctx: &mut Context) -> Cont {
    // 0041ec3c push 1
    push(ctx, 0x1u32);
    // 0041ec3e push 2B11h
    push(ctx, 0x2b11u32);
    // 0041ec43 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ec44 call 0041E910h
    let dst = Cont(x41e910);
    call(ctx, 0x41ec49, dst)
}

pub fn x41ec49(ctx: &mut Context) -> Cont {
    // 0041ec49 add esp,10h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x10u32, &mut ctx.cpu.flags);
    // 0041ec4c pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ec4d pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ec4e add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ec51 ret
    ret(ctx, 0)
}

pub fn x41ec52(ctx: &mut Context) -> Cont {
    // 0041ec52 mov dword ptr ds:[425BB4h],0FFFFFFFFh
    ctx.memory.write::<u32>(0x425bb4u32, 0xffffffffu32);
    Cont(x41ec5c)
}

pub fn x41ec5c(ctx: &mut Context) -> Cont {
    // 0041ec5c pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ec5d mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 0041ec62 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ec63 add esp,38h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x38u32, &mut ctx.cpu.flags);
    // 0041ec66 ret
    ret(ctx, 0)
}

pub fn x41ec70(ctx: &mut Context) -> Cont {
    // 0041ec70 mov eax,ds:[425BB4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bb4u32);
    // 0041ec75 sub esp,98h
    ctx.cpu.regs.esp = sub(ctx.cpu.regs.esp, 0x98u32, &mut ctx.cpu.flags);
    // 0041ec7b cmp eax,0FFFFFFFFh
    sub(ctx.cpu.regs.eax, 0xffffffffu32, &mut ctx.cpu.flags);
    // 0041ec7e jne short 0041EC89h
    jne(ctx, Cont(x41ec80), Cont(x41ec89))
}

pub fn x41ec80(ctx: &mut Context) -> Cont {
    // 0041ec80 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ec82 add esp,98h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x98u32, &mut ctx.cpu.flags);
    // 0041ec88 ret
    ret(ctx, 0)
}

pub fn x41ec89(ctx: &mut Context) -> Cont {
    // 0041ec89 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041ec8a push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041ec8b push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041ec8c push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041ec8d push 2
    push(ctx, 0x2u32);
    // 0041ec8f call dword ptr ds:[42001Ch]
    let dst = Cont(kernel32::GetCurrentThread_stdcall);
    call(ctx, 0x41ec95, dst)
}

pub fn x41ec95(ctx: &mut Context) -> Cont {
    // 0041ec95 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ec96 call dword ptr ds:[420018h]
    let dst = Cont(kernel32::SetThreadPriority_stdcall);
    call(ctx, 0x41ec9c, dst)
}

pub fn x41ec9c(ctx: &mut Context) -> Cont {
    // 0041ec9c xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041ec9e mov ds:[425B9Ch],ebp
    ctx.memory.write::<u32>(0x425b9cu32, ctx.cpu.regs.ebp);
    // 0041eca4 mov ecx,ds:[425BB4h]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bb4u32);
    // 0041ecaa push 30000h
    push(ctx, 0x30000u32);
    // 0041ecaf push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041ecb0 lea eax,[esp+1Ch]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp.wrapping_add(0x1cu32);
    // 0041ecb4 push 41E8F0h
    push(ctx, 0x41e8f0u32);
    // 0041ecb9 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ecba push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041ecbb push 425BACh
    push(ctx, 0x425bacu32);
    // 0041ecc0 mov word ptr [esp+2Ch],1
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x2cu32), 0x1u16);
    // 0041ecc7 mov word ptr [esp+2Eh],2
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x2eu32), 0x2u16);
    // 0041ecce mov word ptr [esp+3Ah],10h
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x3au32), 0x10u16);
    // 0041ecd5 mov dword ptr [esp+30h],0AC44h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x30u32), 0xac44u32);
    // 0041ecdd mov word ptr [esp+38h],4
    ctx.memory
        .write::<u16>(ctx.cpu.regs.esp.wrapping_add(0x38u32), 0x4u16);
    // 0041ece4 mov dword ptr [esp+34h],2B110h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x34u32), 0x2b110u32);
    // 0041ecec mov [esp+3Ch],bp
    ctx.memory.write::<u16>(
        ctx.cpu.regs.esp.wrapping_add(0x3cu32),
        ctx.cpu.regs.get_bp(),
    );
    // 0041ecf1 call dword ptr ds:[420084h]
    let dst = Cont(winmm::waveOutOpen_stdcall);
    call(ctx, 0x41ecf7, dst)
}

pub fn x41ecf7(ctx: &mut Context) -> Cont {
    // 0041ecf7 push 421A54h
    push(ctx, 0x421a54u32);
    // 0041ecfc push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041ecfd push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041ecfe push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041ecff call dword ptr ds:[420014h]
    let dst = Cont(kernel32::CreateEventA_stdcall);
    call(ctx, 0x41ed05, dst)
}

pub fn x41ed05(ctx: &mut Context) -> Cont {
    // 0041ed05 push 20000h
    push(ctx, 0x20000u32);
    // 0041ed0a push 1000h
    push(ctx, 0x1000u32);
    // 0041ed0f mov ds:[425BA4h],eax
    ctx.memory.write::<u32>(0x425ba4u32, ctx.cpu.regs.eax);
    // 0041ed14 call dword ptr ds:[420010h]
    let dst = Cont(kernel32::GlobalAlloc_stdcall);
    call(ctx, 0x41ed1a, dst)
}

pub fn x41ed1a(ctx: &mut Context) -> Cont {
    // 0041ed1a mov ebx,eax
    ctx.cpu.regs.ebx = ctx.cpu.regs.eax;
    // 0041ed1c xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041ed1e lea esi,[esp+28h]
    ctx.cpu.regs.esi = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    Cont(x41ed22)
}

pub fn x41ed22(ctx: &mut Context) -> Cont {
    // 0041ed22 mov eax,ds:[425BACh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ed27 lea edx,[edi+ebx]
    ctx.cpu.regs.edx = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ebx);
    // 0041ed2a push 20h
    push(ctx, 0x20u32);
    // 0041ed2c mov [esi],edx
    ctx.memory.write::<u32>(ctx.cpu.regs.esi, ctx.cpu.regs.edx);
    // 0041ed2e push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041ed2f mov [esi+10h],ebp
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x10u32), ctx.cpu.regs.ebp);
    // 0041ed32 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ed33 mov dword ptr [esi+4],8000h
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esi.wrapping_add(0x4u32), 0x8000u32);
    // 0041ed3a call dword ptr ds:[420078h]
    let dst = Cont(winmm::waveOutPrepareHeader_stdcall);
    call(ctx, 0x41ed40, dst)
}

pub fn x41ed40(ctx: &mut Context) -> Cont {
    // 0041ed40 mov eax,edi
    ctx.cpu.regs.eax = ctx.cpu.regs.edi;
    // 0041ed42 push 4000h
    push(ctx, 0x4000u32);
    // 0041ed47 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0041ed48 sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041ed4a sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041ed4c lea ecx,[ebx+eax*2]
    ctx.cpu.regs.ecx = ctx.cpu.regs.ebx.wrapping_add((ctx.cpu.regs.eax * 2));
    // 0041ed4f push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041ed50 call 0041E550h
    let dst = Cont(x41e550);
    call(ctx, 0x41ed55, dst)
}

pub fn x41ed55(ctx: &mut Context) -> Cont {
    // 0041ed55 mov edx,ds:[425BACh]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ed5b add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041ed5e push 20h
    push(ctx, 0x20u32);
    // 0041ed60 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041ed61 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041ed62 call dword ptr ds:[420074h]
    let dst = Cont(winmm::waveOutWrite_stdcall);
    call(ctx, 0x41ed68, dst)
}

pub fn x41ed68(ctx: &mut Context) -> Cont {
    // 0041ed68 add edi,8000h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x8000u32, &mut ctx.cpu.flags);
    // 0041ed6e add esi,20h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x20u32, &mut ctx.cpu.flags);
    // 0041ed71 cmp edi,20000h
    sub(ctx.cpu.regs.edi, 0x20000u32, &mut ctx.cpu.flags);
    // 0041ed77 jl short 0041ED22h
    jl(ctx, Cont(x41ed79), Cont(x41ed22))
}

pub fn x41ed79(ctx: &mut Context) -> Cont {
    // 0041ed79 xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041ed7b mov eax,ds:[425B94h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b94u32);
    // 0041ed80 cmp eax,edi
    sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041ed82 je near ptr 0041EE37h
    je(ctx, Cont(x41ed88), Cont(x41ee37))
}

pub fn x41ed88(ctx: &mut Context) -> Cont {
    // 0041ed88 jmp short 0041ED8Ch
    Cont(x41ed8c)
}

pub fn x41ed8a(ctx: &mut Context) -> Cont {
    // 0041ed8a xor edi,edi
    ctx.cpu.regs.edi = xor(ctx.cpu.regs.edi, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    Cont(x41ed8c)
}

pub fn x41ed8c(ctx: &mut Context) -> Cont {
    // 0041ed8c mov eax,ds:[425BA4h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425ba4u32);
    // 0041ed91 push 3E8h
    push(ctx, 0x3e8u32);
    // 0041ed96 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ed97 mov ds:[425BA0h],edi
    ctx.memory.write::<u32>(0x425ba0u32, ctx.cpu.regs.edi);
    // 0041ed9d call dword ptr ds:[42000Ch]
    let dst = Cont(kernel32::WaitForSingleObject_stdcall);
    call(ctx, 0x41eda3, dst)
}

pub fn x41eda3(ctx: &mut Context) -> Cont {
    // 0041eda3 mov dword ptr ds:[425BA0h],1
    ctx.memory.write::<u32>(0x425ba0u32, 0x1u32);
    // 0041edad mov [esp+10h],edi
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.edi);
    // 0041edb1 lea esi,[esp+28h]
    ctx.cpu.regs.esi = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    Cont(x41edb5)
}

pub fn x41edb5(ctx: &mut Context) -> Cont {
    // 0041edb5 test byte ptr [esi+10h],1
    and(
        ctx.memory
            .read::<u8>(ctx.cpu.regs.esi.wrapping_add(0x10u32)),
        0x1u8,
        &mut ctx.cpu.flags,
    );
    // 0041edb9 je short 0041EE10h
    je(ctx, Cont(x41edbb), Cont(x41ee10))
}

pub fn x41edbb(ctx: &mut Context) -> Cont {
    // 0041edbb cmp ebp,[esp+10h]
    sub(
        ctx.cpu.regs.ebp,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32)),
        &mut ctx.cpu.flags,
    );
    // 0041edbf jne short 0041EE10h
    jne(ctx, Cont(x41edc1), Cont(x41ee10))
}

pub fn x41edc1(ctx: &mut Context) -> Cont {
    // 0041edc1 mov ecx,ds:[425BACh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041edc7 push 20h
    push(ctx, 0x20u32);
    // 0041edc9 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041edca push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041edcb call dword ptr ds:[420070h]
    let dst = Cont(winmm::waveOutUnprepareHeader_stdcall);
    call(ctx, 0x41edd1, dst)
}

pub fn x41edd1(ctx: &mut Context) -> Cont {
    // 0041edd1 mov eax,edi
    ctx.cpu.regs.eax = ctx.cpu.regs.edi;
    // 0041edd3 push 4000h
    push(ctx, 0x4000u32);
    // 0041edd8 cdq
    let t = ctx.cpu.regs.eax as i32 as i64 as u64;
    ctx.cpu.regs.edx = (t >> 32) as u32;
    ctx.cpu.regs.eax = t as u32;
    // 0041edd9 sub eax,edx
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0041eddb sar eax,1
    ctx.cpu.regs.eax = sar(ctx.cpu.regs.eax, 0x1u8, &mut ctx.cpu.flags);
    // 0041eddd lea edx,[ebx+eax*2]
    ctx.cpu.regs.edx = ctx.cpu.regs.ebx.wrapping_add((ctx.cpu.regs.eax * 2));
    // 0041ede0 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041ede1 call 0041E550h
    let dst = Cont(x41e550);
    call(ctx, 0x41ede6, dst)
}

pub fn x41ede6(ctx: &mut Context) -> Cont {
    // 0041ede6 mov eax,ds:[425BACh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bacu32);
    // 0041edeb add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041edee push 20h
    push(ctx, 0x20u32);
    // 0041edf0 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041edf1 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041edf2 call dword ptr ds:[420078h]
    let dst = Cont(winmm::waveOutPrepareHeader_stdcall);
    call(ctx, 0x41edf8, dst)
}

pub fn x41edf8(ctx: &mut Context) -> Cont {
    // 0041edf8 mov ecx,ds:[425BACh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041edfe push 20h
    push(ctx, 0x20u32);
    // 0041ee00 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041ee01 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041ee02 call dword ptr ds:[420074h]
    let dst = Cont(winmm::waveOutWrite_stdcall);
    call(ctx, 0x41ee08, dst)
}

pub fn x41ee08(ctx: &mut Context) -> Cont {
    // 0041ee08 inc ebp
    ctx.cpu.regs.ebp = inc(ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    // 0041ee09 cmp ebp,4
    sub(ctx.cpu.regs.ebp, 0x4u32, &mut ctx.cpu.flags);
    // 0041ee0c jne short 0041EE10h
    jne(ctx, Cont(x41ee0e), Cont(x41ee10))
}

pub fn x41ee0e(ctx: &mut Context) -> Cont {
    // 0041ee0e xor ebp,ebp
    ctx.cpu.regs.ebp = xor(ctx.cpu.regs.ebp, ctx.cpu.regs.ebp, &mut ctx.cpu.flags);
    Cont(x41ee10)
}

pub fn x41ee10(ctx: &mut Context) -> Cont {
    // 0041ee10 mov eax,[esp+10h]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32));
    // 0041ee14 add edi,8000h
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x8000u32, &mut ctx.cpu.flags);
    // 0041ee1a inc eax
    ctx.cpu.regs.eax = inc(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ee1b add esi,20h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x20u32, &mut ctx.cpu.flags);
    // 0041ee1e cmp edi,20000h
    sub(ctx.cpu.regs.edi, 0x20000u32, &mut ctx.cpu.flags);
    // 0041ee24 mov [esp+10h],eax
    ctx.memory
        .write::<u32>(ctx.cpu.regs.esp.wrapping_add(0x10u32), ctx.cpu.regs.eax);
    // 0041ee28 jl short 0041EDB5h
    jl(ctx, Cont(x41ee2a), Cont(x41edb5))
}

pub fn x41ee2a(ctx: &mut Context) -> Cont {
    // 0041ee2a mov eax,ds:[425B94h]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425b94u32);
    // 0041ee2f test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ee31 jne near ptr 0041ED8Ah
    jne(ctx, Cont(x41ee37), Cont(x41ed8a))
}

pub fn x41ee37(ctx: &mut Context) -> Cont {
    // 0041ee37 mov edx,ds:[425BACh]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ee3d mov ebx,ds:[420088h]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(0x420088u32);
    // 0041ee43 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041ee44 call ebx
    let dst = indirect(ctx, ctx.cpu.regs.ebx);
    call(ctx, 0x41ee46, dst)
}

pub fn x41ee46(ctx: &mut Context) -> Cont {
    // 0041ee46 lea esi,[esp+28h]
    ctx.cpu.regs.esi = ctx.cpu.regs.esp.wrapping_add(0x28u32);
    // 0041ee4a mov edi,4
    ctx.cpu.regs.edi = 0x4u32;
    Cont(x41ee4f)
}

pub fn x41ee4f(ctx: &mut Context) -> Cont {
    // 0041ee4f mov eax,ds:[425BACh]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ee54 push 20h
    push(ctx, 0x20u32);
    // 0041ee56 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041ee57 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ee58 call dword ptr ds:[420070h]
    let dst = Cont(winmm::waveOutUnprepareHeader_stdcall);
    call(ctx, 0x41ee5e, dst)
}

pub fn x41ee5e(ctx: &mut Context) -> Cont {
    // 0041ee5e add esi,20h
    ctx.cpu.regs.esi = add(ctx.cpu.regs.esi, 0x20u32, &mut ctx.cpu.flags);
    // 0041ee61 dec edi
    ctx.cpu.regs.edi = dec(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0041ee62 jne short 0041EE4Fh
    jne(ctx, Cont(x41ee64), Cont(x41ee4f))
}

pub fn x41ee64(ctx: &mut Context) -> Cont {
    // 0041ee64 mov ecx,ds:[425BACh]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ee6a push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041ee6b call ebx
    let dst = indirect(ctx, ctx.cpu.regs.ebx);
    call(ctx, 0x41ee6d, dst)
}

pub fn x41ee6d(ctx: &mut Context) -> Cont {
    // 0041ee6d mov edx,ds:[425BACh]
    ctx.cpu.regs.edx = ctx.memory.read::<u32>(0x425bacu32);
    // 0041ee73 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0041ee74 call dword ptr ds:[42008Ch]
    let dst = Cont(winmm::waveOutClose_stdcall);
    call(ctx, 0x41ee7a, dst)
}

pub fn x41ee7a(ctx: &mut Context) -> Cont {
    // 0041ee7a mov dword ptr ds:[425B9Ch],1
    ctx.memory.write::<u32>(0x425b9cu32, 0x1u32);
    // 0041ee84 pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041ee85 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041ee86 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041ee87 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ee89 pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041ee8a add esp,98h
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x98u32, &mut ctx.cpu.flags);
    // 0041ee90 ret
    ret(ctx, 0)
}

pub fn x41eea0(ctx: &mut Context) -> Cont {
    // 0041eea0 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041eea1 cmp dword ptr ds:[425BB4h],0FFFFFFFFh
    sub(
        ctx.memory.read::<u32>(0x425bb4u32),
        0xffffffffu32,
        &mut ctx.cpu.flags,
    );
    // 0041eea8 je short 0041EECCh
    je(ctx, Cont(x41eeaa), Cont(x41eecc))
}

pub fn x41eeaa(ctx: &mut Context) -> Cont {
    // 0041eeaa mov dword ptr ds:[425B94h],1
    ctx.memory.write::<u32>(0x425b94u32, 0x1u32);
    // 0041eeb4 lea eax,[esp]
    ctx.cpu.regs.eax = ctx.cpu.regs.esp;
    // 0041eeb8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041eeb9 push 0
    push(ctx, 0x0u32);
    // 0041eebb push 0
    push(ctx, 0x0u32);
    // 0041eebd push 41EC70h
    push(ctx, 0x41ec70u32);
    // 0041eec2 push 0
    push(ctx, 0x0u32);
    // 0041eec4 push 0
    push(ctx, 0x0u32);
    // 0041eec6 call dword ptr ds:[420020h]
    let dst = Cont(kernel32::CreateThread_stdcall);
    call(ctx, 0x41eecc, dst)
}

pub fn x41eecc(ctx: &mut Context) -> Cont {
    // 0041eecc pop ecx
    let x = pop(ctx);
    ctx.cpu.regs.ecx = x;
    // 0041eecd ret
    ret(ctx, 0)
}

pub fn x41eed0(ctx: &mut Context) -> Cont {
    // 0041eed0 cmp dword ptr ds:[425BB4h],0FFFFFFFFh
    sub(
        ctx.memory.read::<u32>(0x425bb4u32),
        0xffffffffu32,
        &mut ctx.cpu.flags,
    );
    // 0041eed7 je short 0041EEF9h
    je(ctx, Cont(x41eed9), Cont(x41eef9))
}

pub fn x41eed9(ctx: &mut Context) -> Cont {
    // 0041eed9 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041eedb mov ds:[425B94h],eax
    ctx.memory.write::<u32>(0x425b94u32, ctx.cpu.regs.eax);
    // 0041eee0 mov ecx,ds:[425B9Ch]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425b9cu32);
    // 0041eee6 cmp ecx,eax
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041eee8 jne short 0041EEF4h
    jne(ctx, Cont(x41eeea), Cont(x41eef4))
}

pub fn x41eeea(ctx: &mut Context) -> Cont {
    // 0041eeea mov ecx,ds:[425B9Ch]
    ctx.cpu.regs.ecx = ctx.memory.read::<u32>(0x425b9cu32);
    // 0041eef0 cmp ecx,eax
    sub(ctx.cpu.regs.ecx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041eef2 je short 0041EEEAh
    je(ctx, Cont(x41eef4), Cont(x41eeea))
}

pub fn x41eef4(ctx: &mut Context) -> Cont {
    // 0041eef4 mov ds:[425B9Ch],eax
    ctx.memory.write::<u32>(0x425b9cu32, ctx.cpu.regs.eax);
    Cont(x41eef9)
}

pub fn x41eef9(ctx: &mut Context) -> Cont {
    // 0041eef9 ret
    ret(ctx, 0)
}

pub fn x41ef00(ctx: &mut Context) -> Cont {
    // 0041ef00 call 0041DF90h
    let dst = Cont(x41df90);
    call(ctx, 0x41ef05, dst)
}

pub fn x41ef05(ctx: &mut Context) -> Cont {
    // 0041ef05 push 100h
    push(ctx, 0x100u32);
    // 0041ef0a push 0
    push(ctx, 0x0u32);
    // 0041ef0c call dword ptr ds:[420028h]
    let dst = Cont(kernel32::GetModuleHandleA_stdcall);
    call(ctx, 0x41ef12, dst)
}

pub fn x41ef12(ctx: &mut Context) -> Cont {
    // 0041ef12 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041ef13 call dword ptr ds:[420024h]
    let dst = Cont(kernel32::SetPriorityClass_stdcall);
    call(ctx, 0x41ef19, dst)
}

pub fn x41ef19(ctx: &mut Context) -> Cont {
    // 0041ef19 push 0
    push(ctx, 0x0u32);
    // 0041ef1b call 00406960h
    let dst = Cont(x406960);
    call(ctx, 0x41ef20, dst)
}

pub fn x41ef20(ctx: &mut Context) -> Cont {
    // 0041ef20 add esp,4
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x4u32, &mut ctx.cpu.flags);
    // 0041ef23 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ef25 je near ptr 0041F060h
    je(ctx, Cont(x41ef2b), Cont(x41f060))
}

pub fn x41ef2b(ctx: &mut Context) -> Cont {
    // 0041ef2b push 408740h
    push(ctx, 0x408740u32);
    // 0041ef30 call 00407BF0h
    let dst = Cont(x407bf0);
    call(ctx, 0x41ef35, dst)
}

pub fn x41ef35(ctx: &mut Context) -> Cont {
    // 0041ef35 push 40CB40h
    push(ctx, 0x40cb40u32);
    // 0041ef3a mov ds:[428CE0h],eax
    ctx.memory.write::<u32>(0x428ce0u32, ctx.cpu.regs.eax);
    // 0041ef3f call 0041E090h
    let dst = Cont(x41e090);
    call(ctx, 0x41ef44, dst)
}

pub fn x41ef44(ctx: &mut Context) -> Cont {
    // 0041ef44 add esp,8
    ctx.cpu.regs.esp = add(ctx.cpu.regs.esp, 0x8u32, &mut ctx.cpu.flags);
    // 0041ef47 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41ef4c, dst)
}

pub fn x41ef4c(ctx: &mut Context) -> Cont {
    // 0041ef4c call 00403960h
    let dst = Cont(x403960);
    call(ctx, 0x41ef51, dst)
}

pub fn x41ef51(ctx: &mut Context) -> Cont {
    // 0041ef51 call 00405980h
    let dst = Cont(x405980);
    call(ctx, 0x41ef56, dst)
}

pub fn x41ef56(ctx: &mut Context) -> Cont {
    // 0041ef56 call 00406060h
    let dst = Cont(x406060);
    call(ctx, 0x41ef5b, dst)
}

pub fn x41ef5b(ctx: &mut Context) -> Cont {
    // 0041ef5b call 00404110h
    let dst = Cont(x404110);
    call(ctx, 0x41ef60, dst)
}

pub fn x41ef60(ctx: &mut Context) -> Cont {
    // 0041ef60 call 00404C50h
    let dst = Cont(x404c50);
    call(ctx, 0x41ef65, dst)
}

pub fn x41ef65(ctx: &mut Context) -> Cont {
    // 0041ef65 call 00405120h
    let dst = Cont(x405120);
    call(ctx, 0x41ef6a, dst)
}

pub fn x41ef6a(ctx: &mut Context) -> Cont {
    // 0041ef6a call 00403650h
    let dst = Cont(x403650);
    call(ctx, 0x41ef6f, dst)
}

pub fn x41ef6f(ctx: &mut Context) -> Cont {
    // 0041ef6f call 004057A0h
    let dst = Cont(x4057a0);
    call(ctx, 0x41ef74, dst)
}

pub fn x41ef74(ctx: &mut Context) -> Cont {
    // 0041ef74 call 0041DFB0h
    let dst = Cont(x41dfb0);
    call(ctx, 0x41ef79, dst)
}

pub fn x41ef79(ctx: &mut Context) -> Cont {
    // 0041ef79 call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41ef7e, dst)
}

pub fn x41ef7e(ctx: &mut Context) -> Cont {
    // 0041ef7e call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41ef83, dst)
}

pub fn x41ef83(ctx: &mut Context) -> Cont {
    // 0041ef83 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ef85 jne short 0041EF8Ch
    jne(ctx, Cont(x41ef87), Cont(x41ef8c))
}

pub fn x41ef87(ctx: &mut Context) -> Cont {
    // 0041ef87 call 00403E90h
    let dst = Cont(x403e90);
    call(ctx, 0x41ef8c, dst)
}

pub fn x41ef8c(ctx: &mut Context) -> Cont {
    // 0041ef8c call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41ef91, dst)
}

pub fn x41ef91(ctx: &mut Context) -> Cont {
    // 0041ef91 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41ef96, dst)
}

pub fn x41ef96(ctx: &mut Context) -> Cont {
    // 0041ef96 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041ef98 jne short 0041EF9Fh
    jne(ctx, Cont(x41ef9a), Cont(x41ef9f))
}

pub fn x41ef9a(ctx: &mut Context) -> Cont {
    // 0041ef9a call 004054D0h
    let dst = Cont(x4054d0);
    call(ctx, 0x41ef9f, dst)
}

pub fn x41ef9f(ctx: &mut Context) -> Cont {
    // 0041ef9f call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41efa4, dst)
}

pub fn x41efa4(ctx: &mut Context) -> Cont {
    // 0041efa4 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41efa9, dst)
}

pub fn x41efa9(ctx: &mut Context) -> Cont {
    // 0041efa9 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041efab jne short 0041EFB2h
    jne(ctx, Cont(x41efad), Cont(x41efb2))
}

pub fn x41efad(ctx: &mut Context) -> Cont {
    // 0041efad call 00403BD0h
    let dst = Cont(x403bd0);
    call(ctx, 0x41efb2, dst)
}

pub fn x41efb2(ctx: &mut Context) -> Cont {
    // 0041efb2 call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41efb7, dst)
}

pub fn x41efb7(ctx: &mut Context) -> Cont {
    // 0041efb7 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41efbc, dst)
}

pub fn x41efbc(ctx: &mut Context) -> Cont {
    // 0041efbc test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041efbe jne short 0041EFC5h
    jne(ctx, Cont(x41efc0), Cont(x41efc5))
}

pub fn x41efc0(ctx: &mut Context) -> Cont {
    // 0041efc0 call 00404630h
    let dst = Cont(x404630);
    call(ctx, 0x41efc5, dst)
}

pub fn x41efc5(ctx: &mut Context) -> Cont {
    // 0041efc5 call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41efca, dst)
}

pub fn x41efca(ctx: &mut Context) -> Cont {
    // 0041efca call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41efcf, dst)
}

pub fn x41efcf(ctx: &mut Context) -> Cont {
    // 0041efcf test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041efd1 jne short 0041EFD8h
    jne(ctx, Cont(x41efd3), Cont(x41efd8))
}

pub fn x41efd3(ctx: &mut Context) -> Cont {
    // 0041efd3 call 00405880h
    let dst = Cont(x405880);
    call(ctx, 0x41efd8, dst)
}

pub fn x41efd8(ctx: &mut Context) -> Cont {
    // 0041efd8 call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41efdd, dst)
}

pub fn x41efdd(ctx: &mut Context) -> Cont {
    // 0041efdd call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41efe2, dst)
}

pub fn x41efe2(ctx: &mut Context) -> Cont {
    // 0041efe2 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041efe4 jne short 0041EFEBh
    jne(ctx, Cont(x41efe6), Cont(x41efeb))
}

pub fn x41efe6(ctx: &mut Context) -> Cont {
    // 0041efe6 call 00404DB0h
    let dst = Cont(x404db0);
    call(ctx, 0x41efeb, dst)
}

pub fn x41efeb(ctx: &mut Context) -> Cont {
    // 0041efeb call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41eff0, dst)
}

pub fn x41eff0(ctx: &mut Context) -> Cont {
    // 0041eff0 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41eff5, dst)
}

pub fn x41eff5(ctx: &mut Context) -> Cont {
    // 0041eff5 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041eff7 jne short 0041EFFEh
    jne(ctx, Cont(x41eff9), Cont(x41effe))
}

pub fn x41eff9(ctx: &mut Context) -> Cont {
    // 0041eff9 call 004060B0h
    let dst = Cont(x4060b0);
    call(ctx, 0x41effe, dst)
}

pub fn x41effe(ctx: &mut Context) -> Cont {
    // 0041effe call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41f003, dst)
}

pub fn x41f003(ctx: &mut Context) -> Cont {
    // 0041f003 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41f008, dst)
}

pub fn x41f008(ctx: &mut Context) -> Cont {
    // 0041f008 test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041f00a jne short 0041F011h
    jne(ctx, Cont(x41f00c), Cont(x41f011))
}

pub fn x41f00c(ctx: &mut Context) -> Cont {
    // 0041f00c call 00405A10h
    let dst = Cont(x405a10);
    call(ctx, 0x41f011, dst)
}

pub fn x41f011(ctx: &mut Context) -> Cont {
    // 0041f011 call 00407EB0h
    let dst = Cont(x407eb0);
    call(ctx, 0x41f016, dst)
}

pub fn x41f016(ctx: &mut Context) -> Cont {
    // 0041f016 call 00406D60h
    let dst = Cont(x406d60);
    call(ctx, 0x41f01b, dst)
}

pub fn x41f01b(ctx: &mut Context) -> Cont {
    // 0041f01b test eax,eax
    and(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041f01d jne short 0041F024h
    jne(ctx, Cont(x41f01f), Cont(x41f024))
}

pub fn x41f01f(ctx: &mut Context) -> Cont {
    // 0041f01f call 00403770h
    let dst = Cont(x403770);
    call(ctx, 0x41f024, dst)
}

pub fn x41f024(ctx: &mut Context) -> Cont {
    // 0041f024 call 0041DFC0h
    let dst = Cont(x41dfc0);
    call(ctx, 0x41f029, dst)
}

pub fn x41f029(ctx: &mut Context) -> Cont {
    // 0041f029 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f02e, dst)
}

pub fn x41f02e(ctx: &mut Context) -> Cont {
    // 0041f02e call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f033, dst)
}

pub fn x41f033(ctx: &mut Context) -> Cont {
    // 0041f033 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f038, dst)
}

pub fn x41f038(ctx: &mut Context) -> Cont {
    // 0041f038 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f03d, dst)
}

pub fn x41f03d(ctx: &mut Context) -> Cont {
    // 0041f03d call 004044D0h
    let dst = Cont(x4044d0);
    call(ctx, 0x41f042, dst)
}

pub fn x41f042(ctx: &mut Context) -> Cont {
    // 0041f042 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f047, dst)
}

pub fn x41f047(ctx: &mut Context) -> Cont {
    // 0041f047 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f04c, dst)
}

pub fn x41f04c(ctx: &mut Context) -> Cont {
    // 0041f04c call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f051, dst)
}

pub fn x41f051(ctx: &mut Context) -> Cont {
    // 0041f051 call 00403980h
    let dst = Cont(x403980);
    call(ctx, 0x41f056, dst)
}

pub fn x41f056(ctx: &mut Context) -> Cont {
    // 0041f056 call 0041DFA0h
    let dst = Cont(x41dfa0);
    call(ctx, 0x41f05b, dst)
}

pub fn x41f05b(ctx: &mut Context) -> Cont {
    // 0041f05b call 00406C00h
    let dst = Cont(x406c00);
    call(ctx, 0x41f060, dst)
}

pub fn x41f060(ctx: &mut Context) -> Cont {
    // 0041f060 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0041f062 ret 10h
    ret(ctx, 16)
}

pub fn x41f070(ctx: &mut Context) -> Cont {
    // 0041f070 jmp dword ptr ds:[420000h]
    Cont(ddraw::DirectDrawCreate_stdcall)
}

pub fn x41f079(ctx: &mut Context) -> Cont {
    // 0041f079 push 0
    push(ctx, 0x0u32);
    // 0041f07b push 0
    push(ctx, 0x0u32);
    // 0041f07d push 0
    push(ctx, 0x0u32);
    // 0041f07f push 0
    push(ctx, 0x0u32);
    // 0041f081 call 0041EF00h
    let dst = Cont(x41ef00);
    call(ctx, 0x41f086, dst)
}

pub fn x41f086(ctx: &mut Context) -> Cont {
    // 0041f086 push 0
    push(ctx, 0x0u32);
    // 0041f088 call 0041F11Ch
    let dst = Cont(x41f11c);
    call(ctx, 0x41f08d, dst)
}

pub fn x41f08d(ctx: &mut Context) -> Cont {
    // 0041f08d int 3
    todo!();
}

pub fn x41f090(ctx: &mut Context) -> Cont {
    // 0041f090 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041f091 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 0041f093 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041f094 push ebx
    push(ctx, ctx.cpu.regs.ebx);
    // 0041f095 push esi
    push(ctx, ctx.cpu.regs.esi);
    // 0041f096 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0041f097 fistp dword ptr [ebp-4]
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32),
        ctx.cpu.fpu.get(0).round() as i32 as u32,
    );
    ctx.cpu.fpu.pop();
    // 0041f09a mov eax,[ebp-4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32));
    // 0041f09d pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    // 0041f09e pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0041f09f pop ebx
    let x = pop(ctx);
    ctx.cpu.regs.ebx = x;
    // 0041f0a0 mov esp,ebp
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    // 0041f0a2 pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041f0a3 ret
    ret(ctx, 0)
}

pub fn x41f0b0(ctx: &mut Context) -> Cont {
    // 0041f0b0 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041f0b1 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 0041f0b3 push ecx
    push(ctx, ctx.cpu.regs.ecx);
    // 0041f0b4 mov eax,[ebp+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32));
    // 0041f0b7 add eax,64h
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, 0x64u32, &mut ctx.cpu.flags);
    // 0041f0ba push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041f0bb push 1000h
    push(ctx, 0x1000u32);
    // 0041f0c0 call dword ptr ds:[420010h]
    let dst = Cont(kernel32::GlobalAlloc_stdcall);
    call(ctx, 0x41f0c6, dst)
}

pub fn x41f0c6(ctx: &mut Context) -> Cont {
    // 0041f0c6 mov [ebp-4],eax
    ctx.memory.write::<u32>(
        ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32),
        ctx.cpu.regs.eax,
    );
    // 0041f0c9 mov eax,[ebp-4]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0xfffffffcu32));
    // 0041f0cc mov esp,ebp
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    // 0041f0ce pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041f0cf ret
    ret(ctx, 0)
}

pub fn x41f0d0(ctx: &mut Context) -> Cont {
    // 0041f0d0 push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 0041f0d1 mov ebp,esp
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    // 0041f0d3 mov eax,[ebp+8]
    ctx.cpu.regs.eax = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.ebp.wrapping_add(0x8u32));
    // 0041f0d6 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0041f0d7 call dword ptr ds:[420034h]
    let dst = Cont(kernel32::GlobalFree_stdcall);
    call(ctx, 0x41f0dd, dst)
}

pub fn x41f0dd(ctx: &mut Context) -> Cont {
    // 0041f0dd pop ebp
    let x = pop(ctx);
    ctx.cpu.regs.ebp = x;
    // 0041f0de ret
    ret(ctx, 0)
}

pub fn x41f114(ctx: &mut Context) -> Cont {
    // 0041f114 fxch
    let t = ctx.cpu.fpu.get(0);
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(1));
    ctx.cpu.fpu.set(1, t);
    // 0041f116 fprem
    ctx.cpu.fpu.set(0, ctx.cpu.fpu.get(0) % ctx.cpu.fpu.get(1));
    // 0041f118 fstp st(1)
    ctx.cpu.fpu.set(1, ctx.cpu.fpu.get(0));
    ctx.cpu.fpu.pop();
    // 0041f11a ret
    ret(ctx, 0)
}

pub fn x41f11c(ctx: &mut Context) -> Cont {
    // 0041f11c jmp dword ptr ds:[42002Ch]
    Cont(kernel32::ExitProcess_stdcall)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 1428] = [
    (0x401000, x401000),
    (0x40118e, x40118e),
    (0x401195, x401195),
    (0x401197, x401197),
    (0x4011a8, x4011a8),
    (0x4011af, x4011af),
    (0x4011b1, x4011b1),
    (0x4011c6, x4011c6),
    (0x4011cd, x4011cd),
    (0x4011cf, x4011cf),
    (0x4011e9, x4011e9),
    (0x4011f0, x4011f0),
    (0x4011f2, x4011f2),
    (0x401208, x401208),
    (0x40120f, x40120f),
    (0x401211, x401211),
    (0x401227, x401227),
    (0x40122e, x40122e),
    (0x401230, x401230),
    (0x401249, x401249),
    (0x401250, x401250),
    (0x401252, x401252),
    (0x401264, x401264),
    (0x40126b, x40126b),
    (0x40126d, x40126d),
    (0x401278, x401278),
    (0x401284, x401284),
    (0x40132a, x40132a),
    (0x4013b7, x4013b7),
    (0x401448, x401448),
    (0x4014c3, x4014c3),
    (0x401544, x401544),
    (0x401547, x401547),
    (0x401560, x401560),
    (0x4016ee, x4016ee),
    (0x4016f5, x4016f5),
    (0x4016f7, x4016f7),
    (0x401708, x401708),
    (0x40170f, x40170f),
    (0x401711, x401711),
    (0x401726, x401726),
    (0x40172d, x40172d),
    (0x40172f, x40172f),
    (0x401749, x401749),
    (0x401750, x401750),
    (0x401752, x401752),
    (0x401768, x401768),
    (0x40176f, x40176f),
    (0x401771, x401771),
    (0x401787, x401787),
    (0x40178e, x40178e),
    (0x401790, x401790),
    (0x4017a9, x4017a9),
    (0x4017b0, x4017b0),
    (0x4017b2, x4017b2),
    (0x4017c4, x4017c4),
    (0x4017cb, x4017cb),
    (0x4017cd, x4017cd),
    (0x4017d8, x4017d8),
    (0x4017e4, x4017e4),
    (0x401878, x401878),
    (0x401911, x401911),
    (0x401995, x401995),
    (0x401a41, x401a41),
    (0x401ad1, x401ad1),
    (0x401ad4, x401ad4),
    (0x401ae0, x401ae0),
    (0x401aed, x401aed),
    (0x401b04, x401b04),
    (0x401b1f, x401b1f),
    (0x401b3f, x401b3f),
    (0x401b60, x401b60),
    (0x401b62, x401b62),
    (0x401b83, x401b83),
    (0x401b9b, x401b9b),
    (0x401ba5, x401ba5),
    (0x401baf, x401baf),
    (0x401f90, x401f90),
    (0x401fab, x401fab),
    (0x401fb2, x401fb2),
    (0x401fb4, x401fb4),
    (0x401fc8, x401fc8),
    (0x401fcf, x401fcf),
    (0x401fd1, x401fd1),
    (0x401fe5, x401fe5),
    (0x401fec, x401fec),
    (0x401fee, x401fee),
    (0x402002, x402002),
    (0x402009, x402009),
    (0x40200b, x40200b),
    (0x40201f, x40201f),
    (0x402028, x402028),
    (0x402032, x402032),
    (0x403310, x403310),
    (0x403360, x403360),
    (0x40341e, x40341e),
    (0x403426, x403426),
    (0x40342e, x40342e),
    (0x403443, x403443),
    (0x40352f, x40352f),
    (0x403535, x403535),
    (0x403546, x403546),
    (0x403551, x403551),
    (0x403562, x403562),
    (0x4035a0, x4035a0),
    (0x4035e1, x4035e1),
    (0x4035f1, x4035f1),
    (0x403605, x403605),
    (0x403618, x403618),
    (0x40362c, x40362c),
    (0x403643, x403643),
    (0x403650, x403650),
    (0x40365a, x40365a),
    (0x403670, x403670),
    (0x403678, x403678),
    (0x403689, x403689),
    (0x403693, x403693),
    (0x4036a0, x4036a0),
    (0x4036a8, x4036a8),
    (0x4036af, x4036af),
    (0x4036d3, x4036d3),
    (0x4036ec, x4036ec),
    (0x4036f2, x4036f2),
    (0x403707, x403707),
    (0x403711, x403711),
    (0x40371e, x40371e),
    (0x403726, x403726),
    (0x40372d, x40372d),
    (0x40374b, x40374b),
    (0x403764, x403764),
    (0x403769, x403769),
    (0x403770, x403770),
    (0x40379a, x40379a),
    (0x40379f, x40379f),
    (0x4037b0, x4037b0),
    (0x403826, x403826),
    (0x403859, x403859),
    (0x40386a, x40386a),
    (0x40387e, x40387e),
    (0x403883, x403883),
    (0x4038b4, x4038b4),
    (0x4038c8, x4038c8),
    (0x4038e4, x4038e4),
    (0x403900, x403900),
    (0x40391f, x40391f),
    (0x403922, x403922),
    (0x403927, x403927),
    (0x40392c, x40392c),
    (0x403931, x403931),
    (0x403935, x403935),
    (0x403950, x403950),
    (0x403960, x403960),
    (0x40396a, x40396a),
    (0x403980, x403980),
    (0x403990, x403990),
    (0x403a5b, x403a5b),
    (0x403aaf, x403aaf),
    (0x403ae2, x403ae2),
    (0x403aee, x403aee),
    (0x403afa, x403afa),
    (0x403b4e, x403b4e),
    (0x403b68, x403b68),
    (0x403b89, x403b89),
    (0x403ba0, x403ba0),
    (0x403bb2, x403bb2),
    (0x403bc2, x403bc2),
    (0x403bd0, x403bd0),
    (0x403bdf, x403bdf),
    (0x403be4, x403be4),
    (0x403bf5, x403bf5),
    (0x403bfa, x403bfa),
    (0x403c05, x403c05),
    (0x403c19, x403c19),
    (0x403c21, x403c21),
    (0x403c2b, x403c2b),
    (0x403c2e, x403c2e),
    (0x403c45, x403c45),
    (0x403c50, x403c50),
    (0x403c60, x403c60),
    (0x403c69, x403c69),
    (0x403c6e, x403c6e),
    (0x403c78, x403c78),
    (0x403c80, x403c80),
    (0x403c9d, x403c9d),
    (0x403ca4, x403ca4),
    (0x403ca9, x403ca9),
    (0x403cb8, x403cb8),
    (0x403cc7, x403cc7),
    (0x403ce7, x403ce7),
    (0x403d07, x403d07),
    (0x403d0a, x403d0a),
    (0x403d0f, x403d0f),
    (0x403d14, x403d14),
    (0x403d19, x403d19),
    (0x403d1d, x403d1d),
    (0x403d29, x403d29),
    (0x403d30, x403d30),
    (0x403d49, x403d49),
    (0x403d52, x403d52),
    (0x403d56, x403d56),
    (0x403d60, x403d60),
    (0x403d69, x403d69),
    (0x403d86, x403d86),
    (0x403d9b, x403d9b),
    (0x403df6, x403df6),
    (0x403e2e, x403e2e),
    (0x403e37, x403e37),
    (0x403e40, x403e40),
    (0x403e4c, x403e4c),
    (0x403e55, x403e55),
    (0x403e60, x403e60),
    (0x403e63, x403e63),
    (0x403e82, x403e82),
    (0x403e83, x403e83),
    (0x403e90, x403e90),
    (0x403e91, x403e91),
    (0x403e96, x403e96),
    (0x403ea7, x403ea7),
    (0x403ec0, x403ec0),
    (0x403edf, x403edf),
    (0x403efe, x403efe),
    (0x403f1a, x403f1a),
    (0x403f22, x403f22),
    (0x403f27, x403f27),
    (0x403f2c, x403f2c),
    (0x403f30, x403f30),
    (0x403f4b, x403f4b),
    (0x403f50, x403f50),
    (0x403f5c, x403f5c),
    (0x403fa1, x403fa1),
    (0x403fa6, x403fa6),
    (0x403feb, x403feb),
    (0x403ff0, x403ff0),
    (0x403ff5, x403ff5),
    (0x404035, x404035),
    (0x40403a, x40403a),
    (0x404078, x404078),
    (0x4040a0, x4040a0),
    (0x404110, x404110),
    (0x40411c, x40411c),
    (0x404121, x404121),
    (0x404145, x404145),
    (0x404159, x404159),
    (0x40416e, x40416e),
    (0x404185, x404185),
    (0x4041b8, x4041b8),
    (0x4041e0, x4041e0),
    (0x4041f5, x4041f5),
    (0x40420d, x40420d),
    (0x404250, x404250),
    (0x404260, x404260),
    (0x404275, x404275),
    (0x40428a, x40428a),
    (0x404296, x404296),
    (0x4042c1, x4042c1),
    (0x4042f2, x4042f2),
    (0x404312, x404312),
    (0x404335, x404335),
    (0x404364, x404364),
    (0x404379, x404379),
    (0x40438e, x40438e),
    (0x4043a3, x4043a3),
    (0x4043b1, x4043b1),
    (0x4043dc, x4043dc),
    (0x40440d, x40440d),
    (0x40442d, x40442d),
    (0x404450, x404450),
    (0x40447e, x40447e),
    (0x404488, x404488),
    (0x404497, x404497),
    (0x4044a6, x4044a6),
    (0x4044b5, x4044b5),
    (0x4044d0, x4044d0),
    (0x4044db, x4044db),
    (0x4044e7, x4044e7),
    (0x4044f3, x4044f3),
    (0x4044fe, x4044fe),
    (0x40450a, x40450a),
    (0x404630, x404630),
    (0x404637, x404637),
    (0x40463c, x40463c),
    (0x404651, x404651),
    (0x40465e, x40465e),
    (0x404665, x404665),
    (0x404667, x404667),
    (0x404670, x404670),
    (0x404688, x404688),
    (0x40468d, x40468d),
    (0x4046a4, x4046a4),
    (0x4046ac, x4046ac),
    (0x4046ae, x4046ae),
    (0x4046be, x4046be),
    (0x4046cc, x4046cc),
    (0x4046ce, x4046ce),
    (0x4046de, x4046de),
    (0x4046ec, x4046ec),
    (0x4046ee, x4046ee),
    (0x4046f0, x4046f0),
    (0x404703, x404703),
    (0x404711, x404711),
    (0x404716, x404716),
    (0x404726, x404726),
    (0x404734, x404734),
    (0x404743, x404743),
    (0x4047f2, x4047f2),
    (0x40486b, x40486b),
    (0x404877, x404877),
    (0x4049fa, x4049fa),
    (0x4049ff, x4049ff),
    (0x404a04, x404a04),
    (0x404a08, x404a08),
    (0x404a23, x404a23),
    (0x404a40, x404a40),
    (0x404b30, x404b30),
    (0x404b49, x404b49),
    (0x404b5b, x404b5b),
    (0x404b69, x404b69),
    (0x404b7a, x404b7a),
    (0x404ba4, x404ba4),
    (0x404bda, x404bda),
    (0x404c00, x404c00),
    (0x404c35, x404c35),
    (0x404c46, x404c46),
    (0x404c50, x404c50),
    (0x404c57, x404c57),
    (0x404c5c, x404c5c),
    (0x404c80, x404c80),
    (0x404c90, x404c90),
    (0x404c94, x404c94),
    (0x404cb6, x404cb6),
    (0x404cc0, x404cc0),
    (0x404ccd, x404ccd),
    (0x404cd5, x404cd5),
    (0x404cdc, x404cdc),
    (0x404d00, x404d00),
    (0x404d19, x404d19),
    (0x404d1b, x404d1b),
    (0x404d40, x404d40),
    (0x404d4a, x404d4a),
    (0x404d57, x404d57),
    (0x404d5f, x404d5f),
    (0x404d66, x404d66),
    (0x404d84, x404d84),
    (0x404d99, x404d99),
    (0x404d9e, x404d9e),
    (0x404db0, x404db0),
    (0x404dc1, x404dc1),
    (0x404dc6, x404dc6),
    (0x404dd5, x404dd5),
    (0x404dec, x404dec),
    (0x404e03, x404e03),
    (0x404e7f, x404e7f),
    (0x404ea7, x404ea7),
    (0x404f0e, x404f0e),
    (0x404f2c, x404f2c),
    (0x404f40, x404f40),
    (0x404f55, x404f55),
    (0x404f5b, x404f5b),
    (0x404f5e, x404f5e),
    (0x404f63, x404f63),
    (0x404f68, x404f68),
    (0x404f77, x404f77),
    (0x404f7f, x404f7f),
    (0x404f83, x404f83),
    (0x404f9e, x404f9e),
    (0x404fb0, x404fb0),
    (0x405120, x405120),
    (0x405131, x405131),
    (0x40513e, x40513e),
    (0x405143, x405143),
    (0x405167, x405167),
    (0x405172, x405172),
    (0x405176, x405176),
    (0x405187, x405187),
    (0x405193, x405193),
    (0x4051b2, x4051b2),
    (0x4051d1, x4051d1),
    (0x4051f0, x4051f0),
    (0x405213, x405213),
    (0x405243, x405243),
    (0x405279, x405279),
    (0x4052a5, x4052a5),
    (0x4052bb, x4052bb),
    (0x4052cf, x4052cf),
    (0x4052de, x4052de),
    (0x4052e3, x4052e3),
    (0x405311, x405311),
    (0x405322, x405322),
    (0x405338, x405338),
    (0x40537e, x40537e),
    (0x40539e, x40539e),
    (0x4053b0, x4053b0),
    (0x4053b4, x4053b4),
    (0x4053b9, x4053b9),
    (0x4053da, x4053da),
    (0x4053e4, x4053e4),
    (0x4053f1, x4053f1),
    (0x4053f9, x4053f9),
    (0x405400, x405400),
    (0x405424, x405424),
    (0x40543d, x40543d),
    (0x405443, x405443),
    (0x405460, x405460),
    (0x40546a, x40546a),
    (0x405477, x405477),
    (0x40547f, x40547f),
    (0x405486, x405486),
    (0x4054a4, x4054a4),
    (0x4054bd, x4054bd),
    (0x4054c2, x4054c2),
    (0x4054d0, x4054d0),
    (0x4054e0, x4054e0),
    (0x4054e5, x4054e5),
    (0x4054f0, x4054f0),
    (0x40550c, x40550c),
    (0x405522, x405522),
    (0x405536, x405536),
    (0x405542, x405542),
    (0x405555, x405555),
    (0x40555e, x40555e),
    (0x4055f4, x4055f4),
    (0x405654, x405654),
    (0x40567a, x40567a),
    (0x4056ab, x4056ab),
    (0x405708, x405708),
    (0x405724, x405724),
    (0x40572f, x40572f),
    (0x405732, x405732),
    (0x405739, x405739),
    (0x40573e, x40573e),
    (0x40574e, x40574e),
    (0x40575f, x40575f),
    (0x405773, x405773),
    (0x405776, x405776),
    (0x40577b, x40577b),
    (0x405780, x405780),
    (0x405785, x405785),
    (0x405789, x405789),
    (0x405796, x405796),
    (0x4057a0, x4057a0),
    (0x4057aa, x4057aa),
    (0x4057c0, x4057c0),
    (0x4057eb, x4057eb),
    (0x4057f1, x4057f1),
    (0x405801, x405801),
    (0x405803, x405803),
    (0x405809, x405809),
    (0x405812, x405812),
    (0x405820, x405820),
    (0x405848, x405848),
    (0x40584e, x40584e),
    (0x40586a, x40586a),
    (0x405873, x405873),
    (0x405880, x405880),
    (0x405883, x405883),
    (0x40588a, x40588a),
    (0x405892, x405892),
    (0x4058ad, x4058ad),
    (0x4058c2, x4058c2),
    (0x4058cd, x4058cd),
    (0x4058e5, x4058e5),
    (0x4058eb, x4058eb),
    (0x4058f0, x4058f0),
    (0x40590f, x40590f),
    (0x405927, x405927),
    (0x40592d, x40592d),
    (0x405936, x405936),
    (0x405939, x405939),
    (0x405945, x405945),
    (0x40594a, x40594a),
    (0x40594f, x40594f),
    (0x405952, x405952),
    (0x405957, x405957),
    (0x40595c, x40595c),
    (0x405961, x405961),
    (0x405965, x405965),
    (0x40597a, x40597a),
    (0x405980, x405980),
    (0x40598c, x40598c),
    (0x405991, x405991),
    (0x40599c, x40599c),
    (0x4059ba, x4059ba),
    (0x4059c9, x4059c9),
    (0x4059d3, x4059d3),
    (0x4059d8, x4059d8),
    (0x405a04, x405a04),
    (0x405a10, x405a10),
    (0x405a17, x405a17),
    (0x405a1c, x405a1c),
    (0x405a2d, x405a2d),
    (0x405a41, x405a41),
    (0x405a84, x405a84),
    (0x405aca, x405aca),
    (0x405af8, x405af8),
    (0x405afe, x405afe),
    (0x405b02, x405b02),
    (0x405b07, x405b07),
    (0x405b0c, x405b0c),
    (0x405b1b, x405b1b),
    (0x405b1f, x405b1f),
    (0x405b23, x405b23),
    (0x405b28, x405b28),
    (0x405b2d, x405b2d),
    (0x405b50, x405b50),
    (0x405b5a, x405b5a),
    (0x405b6b, x405b6b),
    (0x405b7c, x405b7c),
    (0x405b8d, x405b8d),
    (0x405b9e, x405b9e),
    (0x405baf, x405baf),
    (0x405bb8, x405bb8),
    (0x405bcf, x405bcf),
    (0x405bd9, x405bd9),
    (0x405bdf, x405bdf),
    (0x405be2, x405be2),
    (0x405bfb, x405bfb),
    (0x405c14, x405c14),
    (0x405c1d, x405c1d),
    (0x405c2b, x405c2b),
    (0x405c37, x405c37),
    (0x405c3e, x405c3e),
    (0x405c43, x405c43),
    (0x405c67, x405c67),
    (0x405c78, x405c78),
    (0x405c8d, x405c8d),
    (0x405cdf, x405cdf),
    (0x405d02, x405d02),
    (0x405d0f, x405d0f),
    (0x405d43, x405d43),
    (0x405d6b, x405d6b),
    (0x405d78, x405d78),
    (0x405dac, x405dac),
    (0x405dd4, x405dd4),
    (0x405de4, x405de4),
    (0x405de7, x405de7),
    (0x405dec, x405dec),
    (0x405df1, x405df1),
    (0x405df6, x405df6),
    (0x405dfa, x405dfa),
    (0x405e09, x405e09),
    (0x405e20, x405e20),
    (0x405e39, x405e39),
    (0x405e55, x405e55),
    (0x405e7c, x405e7c),
    (0x405eaf, x405eaf),
    (0x405ed1, x405ed1),
    (0x405f4f, x405f4f),
    (0x405f5f, x405f5f),
    (0x405fd2, x405fd2),
    (0x405fdc, x405fdc),
    (0x405fe9, x405fe9),
    (0x405ff1, x405ff1),
    (0x406000, x406000),
    (0x406012, x406012),
    (0x40601c, x40601c),
    (0x406032, x406032),
    (0x406049, x406049),
    (0x406060, x406060),
    (0x406067, x406067),
    (0x40606c, x40606c),
    (0x406090, x406090),
    (0x40609a, x40609a),
    (0x4060b0, x4060b0),
    (0x4060c1, x4060c1),
    (0x4060c6, x4060c6),
    (0x4060d1, x4060d1),
    (0x406228, x406228),
    (0x406283, x406283),
    (0x406292, x406292),
    (0x40629d, x40629d),
    (0x4062a5, x4062a5),
    (0x4062ad, x4062ad),
    (0x4062b4, x4062b4),
    (0x4062bc, x4062bc),
    (0x4062c4, x4062c4),
    (0x4062cb, x4062cb),
    (0x4062d3, x4062d3),
    (0x4062db, x4062db),
    (0x4062e2, x4062e2),
    (0x4062ea, x4062ea),
    (0x4062f2, x4062f2),
    (0x4062f7, x4062f7),
    (0x40630e, x40630e),
    (0x406311, x406311),
    (0x406316, x406316),
    (0x40631b, x40631b),
    (0x406320, x406320),
    (0x406324, x406324),
    (0x406331, x406331),
    (0x406340, x406340),
    (0x4063b0, x4063b0),
    (0x4063d0, x4063d0),
    (0x406410, x406410),
    (0x40644d, x40644d),
    (0x406484, x406484),
    (0x4064ba, x4064ba),
    (0x4064f6, x4064f6),
    (0x406550, x406550),
    (0x4065b0, x4065b0),
    (0x4065e0, x4065e0),
    (0x406610, x406610),
    (0x406639, x406639),
    (0x406640, x406640),
    (0x406670, x406670),
    (0x406688, x406688),
    (0x406692, x406692),
    (0x4066d1, x4066d1),
    (0x4066dc, x4066dc),
    (0x4066e7, x4066e7),
    (0x4066f2, x4066f2),
    (0x406706, x406706),
    (0x406711, x406711),
    (0x406725, x406725),
    (0x406730, x406730),
    (0x40673e, x40673e),
    (0x406748, x406748),
    (0x406752, x406752),
    (0x40675c, x40675c),
    (0x406762, x406762),
    (0x406770, x406770),
    (0x40677a, x40677a),
    (0x406785, x406785),
    (0x40678f, x40678f),
    (0x406796, x406796),
    (0x4067a4, x4067a4),
    (0x4067af, x4067af),
    (0x4067b9, x4067b9),
    (0x4067c4, x4067c4),
    (0x4067ca, x4067ca),
    (0x4067d8, x4067d8),
    (0x4067e3, x4067e3),
    (0x4067ee, x4067ee),
    (0x4067f9, x4067f9),
    (0x406800, x406800),
    (0x406803, x406803),
    (0x40681a, x40681a),
    (0x40681c, x40681c),
    (0x406830, x406830),
    (0x406851, x406851),
    (0x406859, x406859),
    (0x4068e7, x4068e7),
    (0x4068e9, x4068e9),
    (0x406960, x406960),
    (0x406972, x406972),
    (0x4069ac, x4069ac),
    (0x4069f0, x4069f0),
    (0x4069f5, x4069f5),
    (0x406a04, x406a04),
    (0x406a10, x406a10),
    (0x406a2f, x406a2f),
    (0x406a3d, x406a3d),
    (0x406a49, x406a49),
    (0x406a55, x406a55),
    (0x406a59, x406a59),
    (0x406a68, x406a68),
    (0x406a74, x406a74),
    (0x406a88, x406a88),
    (0x406a9f, x406a9f),
    (0x406add, x406add),
    (0x406ae1, x406ae1),
    (0x406af0, x406af0),
    (0x406afc, x406afc),
    (0x406b19, x406b19),
    (0x406b1b, x406b1b),
    (0x406b55, x406b55),
    (0x406b70, x406b70),
    (0x406b82, x406b82),
    (0x406b8c, x406b8c),
    (0x406b9b, x406b9b),
    (0x406baa, x406baa),
    (0x406bb9, x406bb9),
    (0x406bc8, x406bc8),
    (0x406bd7, x406bd7),
    (0x406be6, x406be6),
    (0x406c00, x406c00),
    (0x406c0b, x406c0b),
    (0x406c16, x406c16),
    (0x406c21, x406c21),
    (0x406c2e, x406c2e),
    (0x406c39, x406c39),
    (0x406c45, x406c45),
    (0x406c51, x406c51),
    (0x406c60, x406c60),
    (0x406c90, x406c90),
    (0x406cbe, x406cbe),
    (0x406cdd, x406cdd),
    (0x406cf7, x406cf7),
    (0x406d12, x406d12),
    (0x406d21, x406d21),
    (0x406d33, x406d33),
    (0x406d3a, x406d3a),
    (0x406d4a, x406d4a),
    (0x406d54, x406d54),
    (0x406d60, x406d60),
    (0x406d70, x406d70),
    (0x406dbe, x406dbe),
    (0x406dca, x406dca),
    (0x406dd4, x406dd4),
    (0x406dd8, x406dd8),
    (0x406ddc, x406ddc),
    (0x406de4, x406de4),
    (0x406de8, x406de8),
    (0x406df0, x406df0),
    (0x406e28, x406e28),
    (0x406e2d, x406e2d),
    (0x406e34, x406e34),
    (0x406e4e, x406e4e),
    (0x406e52, x406e52),
    (0x406e75, x406e75),
    (0x406e76, x406e76),
    (0x406e80, x406e80),
    (0x406e98, x406e98),
    (0x406ea8, x406ea8),
    (0x406fcb, x406fcb),
    (0x40705a, x40705a),
    (0x4070ec, x4070ec),
    (0x407161, x407161),
    (0x40717e, x40717e),
    (0x4071a6, x4071a6),
    (0x4071b0, x4071b0),
    (0x4071c2, x4071c2),
    (0x4071d5, x4071d5),
    (0x4071e1, x4071e1),
    (0x4071eb, x4071eb),
    (0x4071f5, x4071f5),
    (0x4071fa, x4071fa),
    (0x4071fc, x4071fc),
    (0x407201, x407201),
    (0x407209, x407209),
    (0x40725d, x40725d),
    (0x407268, x407268),
    (0x407278, x407278),
    (0x40727c, x40727c),
    (0x407280, x407280),
    (0x407288, x407288),
    (0x40728c, x40728c),
    (0x407294, x407294),
    (0x4072d5, x4072d5),
    (0x4072da, x4072da),
    (0x4072e1, x4072e1),
    (0x4072fb, x4072fb),
    (0x4072ff, x4072ff),
    (0x407322, x407322),
    (0x407330, x407330),
    (0x40733e, x40733e),
    (0x407359, x407359),
    (0x407377, x407377),
    (0x407385, x407385),
    (0x407390, x407390),
    (0x407397, x407397),
    (0x4073a4, x4073a4),
    (0x4073a9, x4073a9),
    (0x4073c3, x4073c3),
    (0x4073db, x4073db),
    (0x4073f3, x4073f3),
    (0x40740e, x40740e),
    (0x40742d, x40742d),
    (0x407436, x407436),
    (0x40743a, x40743a),
    (0x407441, x407441),
    (0x407446, x407446),
    (0x40745c, x40745c),
    (0x40746b, x40746b),
    (0x407470, x407470),
    (0x407480, x407480),
    (0x407488, x407488),
    (0x407494, x407494),
    (0x40749c, x40749c),
    (0x4074a4, x4074a4),
    (0x4074b0, x4074b0),
    (0x4074b8, x4074b8),
    (0x4074c4, x4074c4),
    (0x4074c8, x4074c8),
    (0x4074cc, x4074cc),
    (0x4074d2, x4074d2),
    (0x4074d6, x4074d6),
    (0x4074d8, x4074d8),
    (0x4074e0, x4074e0),
    (0x4074e5, x4074e5),
    (0x4074fd, x4074fd),
    (0x407522, x407522),
    (0x407526, x407526),
    (0x40752a, x40752a),
    (0x407530, x407530),
    (0x407534, x407534),
    (0x407536, x407536),
    (0x40753e, x40753e),
    (0x407543, x407543),
    (0x40755b, x40755b),
    (0x407561, x407561),
    (0x40756b, x40756b),
    (0x407570, x407570),
    (0x40757f, x40757f),
    (0x4075a1, x4075a1),
    (0x4075aa, x4075aa),
    (0x4075ac, x4075ac),
    (0x4075b0, x4075b0),
    (0x4075ca, x4075ca),
    (0x4075ce, x4075ce),
    (0x4075de, x4075de),
    (0x4075e2, x4075e2),
    (0x4075ea, x4075ea),
    (0x4075f2, x4075f2),
    (0x407601, x407601),
    (0x407610, x407610),
    (0x407620, x407620),
    (0x407631, x407631),
    (0x407633, x407633),
    (0x407717, x407717),
    (0x407726, x407726),
    (0x40773a, x40773a),
    (0x407749, x407749),
    (0x40774b, x40774b),
    (0x407751, x407751),
    (0x407770, x407770),
    (0x40777d, x40777d),
    (0x407783, x407783),
    (0x407788, x407788),
    (0x40778e, x40778e),
    (0x407793, x407793),
    (0x407798, x407798),
    (0x4077a0, x4077a0),
    (0x407800, x407800),
    (0x40781c, x40781c),
    (0x407821, x407821),
    (0x407849, x407849),
    (0x407852, x407852),
    (0x407854, x407854),
    (0x40785b, x40785b),
    (0x40786f, x40786f),
    (0x40787e, x40787e),
    (0x407880, x407880),
    (0x40788c, x40788c),
    (0x4078b1, x4078b1),
    (0x4078bd, x4078bd),
    (0x4078e4, x4078e4),
    (0x4078e9, x4078e9),
    (0x407910, x407910),
    (0x407912, x407912),
    (0x407919, x407919),
    (0x407929, x407929),
    (0x407938, x407938),
    (0x40793c, x40793c),
    (0x407980, x407980),
    (0x407986, x407986),
    (0x40799c, x40799c),
    (0x4079b0, x4079b0),
    (0x4079df, x4079df),
    (0x4079e9, x4079e9),
    (0x4079f3, x4079f3),
    (0x4079f9, x4079f9),
    (0x407a11, x407a11),
    (0x407a24, x407a24),
    (0x407a33, x407a33),
    (0x407a3c, x407a3c),
    (0x407a49, x407a49),
    (0x407a76, x407a76),
    (0x407a7f, x407a7f),
    (0x407a84, x407a84),
    (0x407a8d, x407a8d),
    (0x407a9a, x407a9a),
    (0x407aa3, x407aa3),
    (0x407aa5, x407aa5),
    (0x407ac5, x407ac5),
    (0x407ad2, x407ad2),
    (0x407adf, x407adf),
    (0x407aea, x407aea),
    (0x407af3, x407af3),
    (0x407b11, x407b11),
    (0x407b1d, x407b1d),
    (0x407b4c, x407b4c),
    (0x407b56, x407b56),
    (0x407b64, x407b64),
    (0x407b6c, x407b6c),
    (0x407b87, x407b87),
    (0x407b8f, x407b8f),
    (0x407b97, x407b97),
    (0x407b9c, x407b9c),
    (0x407bab, x407bab),
    (0x407bb5, x407bb5),
    (0x407bc3, x407bc3),
    (0x407bcd, x407bcd),
    (0x407bdb, x407bdb),
    (0x407bdf, x407bdf),
    (0x407bf0, x407bf0),
    (0x407c19, x407c19),
    (0x407c27, x407c27),
    (0x407c33, x407c33),
    (0x407c3a, x407c3a),
    (0x407c3d, x407c3d),
    (0x407c7f, x407c7f),
    (0x407c86, x407c86),
    (0x407c8a, x407c8a),
    (0x407c8f, x407c8f),
    (0x407ca0, x407ca0),
    (0x407ca5, x407ca5),
    (0x407cae, x407cae),
    (0x407cb0, x407cb0),
    (0x407cf4, x407cf4),
    (0x407d00, x407d00),
    (0x407d0c, x407d0c),
    (0x407d13, x407d13),
    (0x407d1a, x407d1a),
    (0x407d5a, x407d5a),
    (0x407d61, x407d61),
    (0x407d65, x407d65),
    (0x407d71, x407d71),
    (0x407d79, x407d79),
    (0x407d88, x407d88),
    (0x407d93, x407d93),
    (0x407d98, x407d98),
    (0x407db1, x407db1),
    (0x407dec, x407dec),
    (0x407df6, x407df6),
    (0x407e10, x407e10),
    (0x407e40, x407e40),
    (0x407e90, x407e90),
    (0x407e9b, x407e9b),
    (0x407ea4, x407ea4),
    (0x407eb0, x407eb0),
    (0x407eb6, x407eb6),
    (0x407ec0, x407ec0),
    (0x407ec9, x407ec9),
    (0x41df90, x41df90),
    (0x41df95, x41df95),
    (0x41dfa0, x41dfa0),
    (0x41dfb0, x41dfb0),
    (0x41dfc0, x41dfc0),
    (0x41dfd0, x41dfd0),
    (0x41dff3, x41dff3),
    (0x41e008, x41e008),
    (0x41e00e, x41e00e),
    (0x41e04c, x41e04c),
    (0x41e053, x41e053),
    (0x41e057, x41e057),
    (0x41e062, x41e062),
    (0x41e064, x41e064),
    (0x41e066, x41e066),
    (0x41e075, x41e075),
    (0x41e080, x41e080),
    (0x41e090, x41e090),
    (0x41e0e1, x41e0e1),
    (0x41e111, x41e111),
    (0x41e129, x41e129),
    (0x41e130, x41e130),
    (0x41e136, x41e136),
    (0x41e149, x41e149),
    (0x41e150, x41e150),
    (0x41e156, x41e156),
    (0x41e169, x41e169),
    (0x41e170, x41e170),
    (0x41e176, x41e176),
    (0x41e189, x41e189),
    (0x41e190, x41e190),
    (0x41e19a, x41e19a),
    (0x41e1ae, x41e1ae),
    (0x41e1b5, x41e1b5),
    (0x41e1c6, x41e1c6),
    (0x41e1da, x41e1da),
    (0x41e1e1, x41e1e1),
    (0x41e1ff, x41e1ff),
    (0x41e204, x41e204),
    (0x41e210, x41e210),
    (0x41e223, x41e223),
    (0x41e225, x41e225),
    (0x41e243, x41e243),
    (0x41e245, x41e245),
    (0x41e252, x41e252),
    (0x41e254, x41e254),
    (0x41e25d, x41e25d),
    (0x41e272, x41e272),
    (0x41e289, x41e289),
    (0x41e297, x41e297),
    (0x41e2f2, x41e2f2),
    (0x41e2f8, x41e2f8),
    (0x41e305, x41e305),
    (0x41e30c, x41e30c),
    (0x41e325, x41e325),
    (0x41e339, x41e339),
    (0x41e34c, x41e34c),
    (0x41e34e, x41e34e),
    (0x41e36c, x41e36c),
    (0x41e372, x41e372),
    (0x41e37f, x41e37f),
    (0x41e3be, x41e3be),
    (0x41e3c4, x41e3c4),
    (0x41e3ca, x41e3ca),
    (0x41e3df, x41e3df),
    (0x41e3ee, x41e3ee),
    (0x41e410, x41e410),
    (0x41e422, x41e422),
    (0x41e451, x41e451),
    (0x41e469, x41e469),
    (0x41e486, x41e486),
    (0x41e4a0, x41e4a0),
    (0x41e4ad, x41e4ad),
    (0x41e4b7, x41e4b7),
    (0x41e4c0, x41e4c0),
    (0x41e4c6, x41e4c6),
    (0x41e4cc, x41e4cc),
    (0x41e4e0, x41e4e0),
    (0x41e510, x41e510),
    (0x41e53c, x41e53c),
    (0x41e550, x41e550),
    (0x41e55f, x41e55f),
    (0x41e56a, x41e56a),
    (0x41e57b, x41e57b),
    (0x41e5a5, x41e5a5),
    (0x41e5a7, x41e5a7),
    (0x41e5ce, x41e5ce),
    (0x41e5d4, x41e5d4),
    (0x41e5da, x41e5da),
    (0x41e5e6, x41e5e6),
    (0x41e5ee, x41e5ee),
    (0x41e5fa, x41e5fa),
    (0x41e60d, x41e60d),
    (0x41e615, x41e615),
    (0x41e622, x41e622),
    (0x41e63a, x41e63a),
    (0x41e647, x41e647),
    (0x41e64e, x41e64e),
    (0x41e655, x41e655),
    (0x41e662, x41e662),
    (0x41e66a, x41e66a),
    (0x41e672, x41e672),
    (0x41e679, x41e679),
    (0x41e688, x41e688),
    (0x41e68a, x41e68a),
    (0x41e699, x41e699),
    (0x41e69b, x41e69b),
    (0x41e6a6, x41e6a6),
    (0x41e6b8, x41e6b8),
    (0x41e6cd, x41e6cd),
    (0x41e6e2, x41e6e2),
    (0x41e6ec, x41e6ec),
    (0x41e6f6, x41e6f6),
    (0x41e702, x41e702),
    (0x41e71f, x41e71f),
    (0x41e72e, x41e72e),
    (0x41e738, x41e738),
    (0x41e73d, x41e73d),
    (0x41e744, x41e744),
    (0x41e757, x41e757),
    (0x41e75c, x41e75c),
    (0x41e798, x41e798),
    (0x41e7a5, x41e7a5),
    (0x41e7b6, x41e7b6),
    (0x41e7be, x41e7be),
    (0x41e7cb, x41e7cb),
    (0x41e7d3, x41e7d3),
    (0x41e7e4, x41e7e4),
    (0x41e7ec, x41e7ec),
    (0x41e7f9, x41e7f9),
    (0x41e801, x41e801),
    (0x41e80a, x41e80a),
    (0x41e819, x41e819),
    (0x41e832, x41e832),
    (0x41e834, x41e834),
    (0x41e844, x41e844),
    (0x41e868, x41e868),
    (0x41e872, x41e872),
    (0x41e87b, x41e87b),
    (0x41e882, x41e882),
    (0x41e88c, x41e88c),
    (0x41e88e, x41e88e),
    (0x41e894, x41e894),
    (0x41e899, x41e899),
    (0x41e8a6, x41e8a6),
    (0x41e8a9, x41e8a9),
    (0x41e8ba, x41e8ba),
    (0x41e8c9, x41e8c9),
    (0x41e8cd, x41e8cd),
    (0x41e8f0, x41e8f0),
    (0x41e8fa, x41e8fa),
    (0x41e906, x41e906),
    (0x41e910, x41e910),
    (0x41e983, x41e983),
    (0x41e990, x41e990),
    (0x41e99b, x41e99b),
    (0x41e9a0, x41e9a0),
    (0x41e9b3, x41e9b3),
    (0x41e9c0, x41e9c0),
    (0x41e9cb, x41e9cb),
    (0x41e9e3, x41e9e3),
    (0x41e9ea, x41e9ea),
    (0x41e9f5, x41e9f5),
    (0x41e9ff, x41e9ff),
    (0x41ea07, x41ea07),
    (0x41ea0e, x41ea0e),
    (0x41ea12, x41ea12),
    (0x41ea1b, x41ea1b),
    (0x41ea1e, x41ea1e),
    (0x41ea22, x41ea22),
    (0x41ea37, x41ea37),
    (0x41ea58, x41ea58),
    (0x41ea61, x41ea61),
    (0x41ea66, x41ea66),
    (0x41ea87, x41ea87),
    (0x41ea90, x41ea90),
    (0x41ea95, x41ea95),
    (0x41eab6, x41eab6),
    (0x41eabf, x41eabf),
    (0x41eac4, x41eac4),
    (0x41eae5, x41eae5),
    (0x41eaee, x41eaee),
    (0x41eaf3, x41eaf3),
    (0x41eb14, x41eb14),
    (0x41eb1d, x41eb1d),
    (0x41eb22, x41eb22),
    (0x41eb43, x41eb43),
    (0x41eb4c, x41eb4c),
    (0x41eb51, x41eb51),
    (0x41eb72, x41eb72),
    (0x41eb7b, x41eb7b),
    (0x41eb80, x41eb80),
    (0x41eba1, x41eba1),
    (0x41ebaa, x41ebaa),
    (0x41ebaf, x41ebaf),
    (0x41ebd0, x41ebd0),
    (0x41ebd9, x41ebd9),
    (0x41ebde, x41ebde),
    (0x41ebff, x41ebff),
    (0x41ec08, x41ec08),
    (0x41ec0d, x41ec0d),
    (0x41ec23, x41ec23),
    (0x41ec28, x41ec28),
    (0x41ec3c, x41ec3c),
    (0x41ec49, x41ec49),
    (0x41ec52, x41ec52),
    (0x41ec5c, x41ec5c),
    (0x41ec70, x41ec70),
    (0x41ec80, x41ec80),
    (0x41ec89, x41ec89),
    (0x41ec95, x41ec95),
    (0x41ec9c, x41ec9c),
    (0x41ecf7, x41ecf7),
    (0x41ed05, x41ed05),
    (0x41ed1a, x41ed1a),
    (0x41ed22, x41ed22),
    (0x41ed40, x41ed40),
    (0x41ed55, x41ed55),
    (0x41ed68, x41ed68),
    (0x41ed79, x41ed79),
    (0x41ed88, x41ed88),
    (0x41ed8a, x41ed8a),
    (0x41ed8c, x41ed8c),
    (0x41eda3, x41eda3),
    (0x41edb5, x41edb5),
    (0x41edbb, x41edbb),
    (0x41edc1, x41edc1),
    (0x41edd1, x41edd1),
    (0x41ede6, x41ede6),
    (0x41edf8, x41edf8),
    (0x41ee08, x41ee08),
    (0x41ee0e, x41ee0e),
    (0x41ee10, x41ee10),
    (0x41ee2a, x41ee2a),
    (0x41ee37, x41ee37),
    (0x41ee46, x41ee46),
    (0x41ee4f, x41ee4f),
    (0x41ee5e, x41ee5e),
    (0x41ee64, x41ee64),
    (0x41ee6d, x41ee6d),
    (0x41ee7a, x41ee7a),
    (0x41eea0, x41eea0),
    (0x41eeaa, x41eeaa),
    (0x41eecc, x41eecc),
    (0x41eed0, x41eed0),
    (0x41eed9, x41eed9),
    (0x41eeea, x41eeea),
    (0x41eef4, x41eef4),
    (0x41eef9, x41eef9),
    (0x41ef00, x41ef00),
    (0x41ef05, x41ef05),
    (0x41ef12, x41ef12),
    (0x41ef19, x41ef19),
    (0x41ef20, x41ef20),
    (0x41ef2b, x41ef2b),
    (0x41ef35, x41ef35),
    (0x41ef44, x41ef44),
    (0x41ef4c, x41ef4c),
    (0x41ef51, x41ef51),
    (0x41ef56, x41ef56),
    (0x41ef5b, x41ef5b),
    (0x41ef60, x41ef60),
    (0x41ef65, x41ef65),
    (0x41ef6a, x41ef6a),
    (0x41ef6f, x41ef6f),
    (0x41ef74, x41ef74),
    (0x41ef79, x41ef79),
    (0x41ef7e, x41ef7e),
    (0x41ef83, x41ef83),
    (0x41ef87, x41ef87),
    (0x41ef8c, x41ef8c),
    (0x41ef91, x41ef91),
    (0x41ef96, x41ef96),
    (0x41ef9a, x41ef9a),
    (0x41ef9f, x41ef9f),
    (0x41efa4, x41efa4),
    (0x41efa9, x41efa9),
    (0x41efad, x41efad),
    (0x41efb2, x41efb2),
    (0x41efb7, x41efb7),
    (0x41efbc, x41efbc),
    (0x41efc0, x41efc0),
    (0x41efc5, x41efc5),
    (0x41efca, x41efca),
    (0x41efcf, x41efcf),
    (0x41efd3, x41efd3),
    (0x41efd8, x41efd8),
    (0x41efdd, x41efdd),
    (0x41efe2, x41efe2),
    (0x41efe6, x41efe6),
    (0x41efeb, x41efeb),
    (0x41eff0, x41eff0),
    (0x41eff5, x41eff5),
    (0x41eff9, x41eff9),
    (0x41effe, x41effe),
    (0x41f003, x41f003),
    (0x41f008, x41f008),
    (0x41f00c, x41f00c),
    (0x41f011, x41f011),
    (0x41f016, x41f016),
    (0x41f01b, x41f01b),
    (0x41f01f, x41f01f),
    (0x41f024, x41f024),
    (0x41f029, x41f029),
    (0x41f02e, x41f02e),
    (0x41f033, x41f033),
    (0x41f038, x41f038),
    (0x41f03d, x41f03d),
    (0x41f042, x41f042),
    (0x41f047, x41f047),
    (0x41f04c, x41f04c),
    (0x41f051, x41f051),
    (0x41f056, x41f056),
    (0x41f05b, x41f05b),
    (0x41f060, x41f060),
    (0x41f070, x41f070),
    (0x41f079, x41f079),
    (0x41f086, x41f086),
    (0x41f08d, x41f08d),
    (0x41f090, x41f090),
    (0x41f0b0, x41f0b0),
    (0x41f0c6, x41f0c6),
    (0x41f0d0, x41f0d0),
    (0x41f0dd, x41f0dd),
    (0x41f114, x41f114),
    (0x41f11c, x41f11c),
    (0xfafbfc00, kernel32::GetTickCount_stdcall),
    (0xfafbfc01, kernel32::WaitForSingleObject_stdcall),
    (0xfafbfc02, kernel32::GlobalAlloc_stdcall),
    (0xfafbfc03, kernel32::CreateEventA_stdcall),
    (0xfafbfc04, kernel32::SetThreadPriority_stdcall),
    (0xfafbfc05, kernel32::GetCurrentThread_stdcall),
    (0xfafbfc06, kernel32::CreateThread_stdcall),
    (0xfafbfc07, kernel32::SetPriorityClass_stdcall),
    (0xfafbfc08, kernel32::GetModuleHandleA_stdcall),
    (0xfafbfc09, kernel32::ExitProcess_stdcall),
    (0xfafbfc0a, kernel32::SetEvent_stdcall),
    (0xfafbfc0b, kernel32::GlobalFree_stdcall),
    (0xfafbfc0c, ddraw::DirectDrawCreate_stdcall),
    (0xfafbfc0d, user32::CreateWindowExA_stdcall),
    (0xfafbfc0e, user32::MessageBoxA_stdcall),
    (0xfafbfc0f, user32::ShowWindow_stdcall),
    (0xfafbfc10, user32::DestroyWindow_stdcall),
    (0xfafbfc11, user32::CreateCursor_stdcall),
    (0xfafbfc12, user32::DispatchMessageA_stdcall),
    (0xfafbfc13, user32::GetMessageA_stdcall),
    (0xfafbfc14, user32::PeekMessageA_stdcall),
    (0xfafbfc15, user32::DefWindowProcA_stdcall),
    (0xfafbfc16, user32::UpdateWindow_stdcall),
    (0xfafbfc17, user32::PostQuitMessage_stdcall),
    (0xfafbfc18, user32::RegisterClassA_stdcall),
    (0xfafbfc19, winmm::waveOutUnprepareHeader_stdcall),
    (0xfafbfc1a, winmm::waveOutWrite_stdcall),
    (0xfafbfc1b, winmm::waveOutPrepareHeader_stdcall),
    (0xfafbfc1c, winmm::waveOutGetNumDevs_stdcall),
    (0xfafbfc1d, winmm::waveOutGetDevCapsA_stdcall),
    (0xfafbfc1e, winmm::waveOutOpen_stdcall),
    (0xfafbfc1f, winmm::waveOutReset_stdcall),
    (0xfafbfc20, winmm::waveOutClose_stdcall),
    (0xfafbfc21, ddraw::IDirectDraw::QueryInterface_stdcall),
    (0xfafbfc22, ddraw::IDirectDraw::AddRef_stdcall),
    (0xfafbfc23, ddraw::IDirectDraw::Release_stdcall),
    (0xfafbfc24, ddraw::IDirectDraw::Compact_stdcall),
    (0xfafbfc25, ddraw::IDirectDraw::CreateClipper_stdcall),
    (0xfafbfc26, ddraw::IDirectDraw::CreatePalette_stdcall),
    (0xfafbfc27, ddraw::IDirectDraw::CreateSurface_stdcall),
    (0xfafbfc28, ddraw::IDirectDraw::DuplicateSurface_stdcall),
    (0xfafbfc29, ddraw::IDirectDraw::EnumDisplayModes_stdcall),
    (0xfafbfc2a, ddraw::IDirectDraw::EnumSurfaces_stdcall),
    (0xfafbfc2b, ddraw::IDirectDraw::FlipToGDISurface_stdcall),
    (0xfafbfc2c, ddraw::IDirectDraw::GetCaps_stdcall),
    (0xfafbfc2d, ddraw::IDirectDraw::GetDisplayMode_stdcall),
    (0xfafbfc2e, ddraw::IDirectDraw::GetFourCCCodes_stdcall),
    (0xfafbfc2f, ddraw::IDirectDraw::GetGDISurface_stdcall),
    (0xfafbfc30, ddraw::IDirectDraw::GetMonitorFrequency_stdcall),
    (0xfafbfc31, ddraw::IDirectDraw::GetScanLine_stdcall),
    (
        0xfafbfc32,
        ddraw::IDirectDraw::GetVerticalBlankStatus_stdcall,
    ),
    (0xfafbfc33, ddraw::IDirectDraw::Initialize_stdcall),
    (0xfafbfc34, ddraw::IDirectDraw::RestoreDisplayMode_stdcall),
    (0xfafbfc35, ddraw::IDirectDraw::SetCooperativeLevel_stdcall),
    (0xfafbfc36, ddraw::IDirectDraw::SetDisplayMode_stdcall),
    (0xfafbfc37, ddraw::IDirectDraw::WaitForVerticalBlank_stdcall),
    (
        0xfafbfc38,
        ddraw::IDirectDrawSurface::QueryInterface_stdcall,
    ),
    (0xfafbfc39, ddraw::IDirectDrawSurface::AddRef_stdcall),
    (0xfafbfc3a, ddraw::IDirectDrawSurface::Release_stdcall),
    (
        0xfafbfc3b,
        ddraw::IDirectDrawSurface::AddAttachedSurface_stdcall,
    ),
    (
        0xfafbfc3c,
        ddraw::IDirectDrawSurface::AddOverlayDirtyRect_stdcall,
    ),
    (0xfafbfc3d, ddraw::IDirectDrawSurface::Blt_stdcall),
    (0xfafbfc3e, ddraw::IDirectDrawSurface::BltBatch_stdcall),
    (0xfafbfc3f, ddraw::IDirectDrawSurface::BltFast_stdcall),
    (
        0xfafbfc40,
        ddraw::IDirectDrawSurface::DeleteAttachedSurface_stdcall,
    ),
    (
        0xfafbfc41,
        ddraw::IDirectDrawSurface::EnumAttachedSurfaces_stdcall,
    ),
    (
        0xfafbfc42,
        ddraw::IDirectDrawSurface::EnumOverlayZOrders_stdcall,
    ),
    (0xfafbfc43, ddraw::IDirectDrawSurface::Flip_stdcall),
    (
        0xfafbfc44,
        ddraw::IDirectDrawSurface::GetAttachedSurface_stdcall,
    ),
    (0xfafbfc45, ddraw::IDirectDrawSurface::GetBltStatus_stdcall),
    (0xfafbfc46, ddraw::IDirectDrawSurface::GetCaps_stdcall),
    (0xfafbfc47, ddraw::IDirectDrawSurface::GetClipper_stdcall),
    (0xfafbfc48, ddraw::IDirectDrawSurface::GetColorKey_stdcall),
    (0xfafbfc49, ddraw::IDirectDrawSurface::GetDC_stdcall),
    (0xfafbfc4a, ddraw::IDirectDrawSurface::GetFlipStatus_stdcall),
    (
        0xfafbfc4b,
        ddraw::IDirectDrawSurface::GetOverlayPosition_stdcall,
    ),
    (0xfafbfc4c, ddraw::IDirectDrawSurface::GetPalette_stdcall),
    (
        0xfafbfc4d,
        ddraw::IDirectDrawSurface::GetPixelFormat_stdcall,
    ),
    (
        0xfafbfc4e,
        ddraw::IDirectDrawSurface::GetSurfaceDesc_stdcall,
    ),
    (0xfafbfc4f, ddraw::IDirectDrawSurface::Initialize_stdcall),
    (0xfafbfc50, ddraw::IDirectDrawSurface::IsLost_stdcall),
    (0xfafbfc51, ddraw::IDirectDrawSurface::Lock_stdcall),
    (0xfafbfc52, ddraw::IDirectDrawSurface::ReleaseDC_stdcall),
    (0xfafbfc53, ddraw::IDirectDrawSurface::Restore_stdcall),
    (0xfafbfc54, ddraw::IDirectDrawSurface::SetClipper_stdcall),
    (0xfafbfc55, ddraw::IDirectDrawSurface::SetColorKey_stdcall),
    (
        0xfafbfc56,
        ddraw::IDirectDrawSurface::SetOverlayPosition_stdcall,
    ),
    (0xfafbfc57, ddraw::IDirectDrawSurface::SetPalette_stdcall),
    (0xfafbfc58, ddraw::IDirectDrawSurface::Unlock_stdcall),
    (0xfafbfc59, ddraw::IDirectDrawSurface::UpdateOverlay_stdcall),
    (
        0xfafbfc5a,
        ddraw::IDirectDrawSurface::UpdateOverlayDisplay_stdcall,
    ),
    (
        0xfafbfc5b,
        ddraw::IDirectDrawSurface::UpdateOverlayZOrder_stdcall,
    ),
    (0xfafbfc5c, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0xfafbfc5d, ddraw::IDirectDraw7::AddRef_stdcall),
    (0xfafbfc5e, ddraw::IDirectDraw7::Release_stdcall),
    (0xfafbfc5f, ddraw::IDirectDraw7::Compact_stdcall),
    (0xfafbfc60, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0xfafbfc61, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0xfafbfc62, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0xfafbfc63, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0xfafbfc64, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0xfafbfc65, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0xfafbfc66, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0xfafbfc67, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0xfafbfc68, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0xfafbfc69, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0xfafbfc6a, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0xfafbfc6b, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0xfafbfc6c, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (
        0xfafbfc6d,
        ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall,
    ),
    (0xfafbfc6e, ddraw::IDirectDraw7::Initialize_stdcall),
    (0xfafbfc6f, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0xfafbfc70, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0xfafbfc71, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (
        0xfafbfc72,
        ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall,
    ),
    (0xfafbfc73, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0xfafbfc74, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0xfafbfc75, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (
        0xfafbfc76,
        ddraw::IDirectDraw7::TestCooperativeLevel_stdcall,
    ),
    (0xfafbfc77, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0xfafbfc78, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0xfafbfc79, ddraw::IDirectDraw7::EvaluateMode_stdcall),
    (
        0xfafbfc7a,
        ddraw::IDirectDrawSurface7::QueryInterface_stdcall,
    ),
    (0xfafbfc7b, ddraw::IDirectDrawSurface7::AddRef_stdcall),
    (0xfafbfc7c, ddraw::IDirectDrawSurface7::Release_stdcall),
    (
        0xfafbfc7d,
        ddraw::IDirectDrawSurface7::AddAttachedSurface_stdcall,
    ),
    (
        0xfafbfc7e,
        ddraw::IDirectDrawSurface7::AddOverlayDirtyRect_stdcall,
    ),
    (0xfafbfc7f, ddraw::IDirectDrawSurface7::Blt_stdcall),
    (0xfafbfc80, ddraw::IDirectDrawSurface7::BltBatch_stdcall),
    (0xfafbfc81, ddraw::IDirectDrawSurface7::BltFast_stdcall),
    (
        0xfafbfc82,
        ddraw::IDirectDrawSurface7::DeleteAttachedSurface_stdcall,
    ),
    (
        0xfafbfc83,
        ddraw::IDirectDrawSurface7::EnumAttachedSurfaces_stdcall,
    ),
    (
        0xfafbfc84,
        ddraw::IDirectDrawSurface7::EnumOverlayZOrders_stdcall,
    ),
    (0xfafbfc85, ddraw::IDirectDrawSurface7::Flip_stdcall),
    (
        0xfafbfc86,
        ddraw::IDirectDrawSurface7::GetAttachedSurface_stdcall,
    ),
    (0xfafbfc87, ddraw::IDirectDrawSurface7::GetBltStatus_stdcall),
    (0xfafbfc88, ddraw::IDirectDrawSurface7::GetCaps_stdcall),
    (0xfafbfc89, ddraw::IDirectDrawSurface7::GetClipper_stdcall),
    (0xfafbfc8a, ddraw::IDirectDrawSurface7::GetColorKey_stdcall),
    (0xfafbfc8b, ddraw::IDirectDrawSurface7::GetDC_stdcall),
    (
        0xfafbfc8c,
        ddraw::IDirectDrawSurface7::GetFlipStatus_stdcall,
    ),
    (
        0xfafbfc8d,
        ddraw::IDirectDrawSurface7::GetOverlayPosition_stdcall,
    ),
    (0xfafbfc8e, ddraw::IDirectDrawSurface7::GetPalette_stdcall),
    (
        0xfafbfc8f,
        ddraw::IDirectDrawSurface7::GetPixelFormat_stdcall,
    ),
    (
        0xfafbfc90,
        ddraw::IDirectDrawSurface7::GetSurfaceDesc_stdcall,
    ),
    (0xfafbfc91, ddraw::IDirectDrawSurface7::Initialize_stdcall),
    (0xfafbfc92, ddraw::IDirectDrawSurface7::IsLost_stdcall),
    (0xfafbfc93, ddraw::IDirectDrawSurface7::Lock_stdcall),
    (0xfafbfc94, ddraw::IDirectDrawSurface7::ReleaseDC_stdcall),
    (0xfafbfc95, ddraw::IDirectDrawSurface7::Restore_stdcall),
    (0xfafbfc96, ddraw::IDirectDrawSurface7::SetClipper_stdcall),
    (0xfafbfc97, ddraw::IDirectDrawSurface7::SetColorKey_stdcall),
    (
        0xfafbfc98,
        ddraw::IDirectDrawSurface7::SetOverlayPosition_stdcall,
    ),
    (0xfafbfc99, ddraw::IDirectDrawSurface7::SetPalette_stdcall),
    (0xfafbfc9a, ddraw::IDirectDrawSurface7::Unlock_stdcall),
    (
        0xfafbfc9b,
        ddraw::IDirectDrawSurface7::UpdateOverlay_stdcall,
    ),
    (
        0xfafbfc9c,
        ddraw::IDirectDrawSurface7::UpdateOverlayDisplay_stdcall,
    ),
    (
        0xfafbfc9d,
        ddraw::IDirectDrawSurface7::UpdateOverlayZOrder_stdcall,
    ),
    (
        0xfafbfc9e,
        ddraw::IDirectDrawSurface7::GetDDInterface_stdcall,
    ),
    (0xfafbfc9f, ddraw::IDirectDrawSurface7::PageLock_stdcall),
    (0xfafbfca0, ddraw::IDirectDrawSurface7::PageUnlock_stdcall),
    (
        0xfafbfca1,
        ddraw::IDirectDrawSurface7::SetSurfaceDesc_stdcall,
    ),
    (
        0xfafbfca2,
        ddraw::IDirectDrawSurface7::SetPrivateData_stdcall,
    ),
    (
        0xfafbfca3,
        ddraw::IDirectDrawSurface7::GetPrivateData_stdcall,
    ),
    (
        0xfafbfca4,
        ddraw::IDirectDrawSurface7::FreePrivateData_stdcall,
    ),
    (
        0xfafbfca5,
        ddraw::IDirectDrawSurface7::GetUniquenessValue_stdcall,
    ),
    (
        0xfafbfca6,
        ddraw::IDirectDrawSurface7::ChangeUniquenessValue_stdcall,
    ),
    (0xfafbfca7, ddraw::IDirectDrawSurface7::SetPriority_stdcall),
    (0xfafbfca8, ddraw::IDirectDrawSurface7::GetPriority_stdcall),
    (0xfafbfca9, ddraw::IDirectDrawSurface7::SetLOD_stdcall),
    (0xfafbfcaa, ddraw::IDirectDrawSurface7::GetLOD_stdcall),
    (
        0xfafbfcab,
        ddraw::IDirectDrawPalette::QueryInterface_stdcall,
    ),
    (0xfafbfcac, ddraw::IDirectDrawPalette::AddRef_stdcall),
    (0xfafbfcad, ddraw::IDirectDrawPalette::Release_stdcall),
    (0xfafbfcae, ddraw::IDirectDrawPalette::GetCaps_stdcall),
    (0xfafbfcaf, ddraw::IDirectDrawPalette::GetEntries_stdcall),
    (0xfafbfcb0, ddraw::IDirectDrawPalette::Initialize_stdcall),
    (0xfafbfcb1, ddraw::IDirectDrawPalette::SetEntries_stdcall),
    (runtime::RETURN_FROM_X86_ADDR, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x41f079),
};
