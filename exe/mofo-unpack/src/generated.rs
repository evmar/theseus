#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use runtime::*;
use winapi::*;

use crate::externs::*;
fn init_mappings(ctx: &mut Context, mappings: &mut kernel32::Mappings) {
    mappings.alloc("null page".to_string(), Some(0x0), 0x1000);
    mappings.alloc("imported functions".to_string(), Some(0x1000), 0x1000);
    mappings.alloc("exe header".to_string(), Some(0x400000), 0x1000);
    let bytes = include_bytes!("../data/00400000.raw").as_slice();
    let out = &mut ctx.memory[0x400000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc("UPX0".to_string(), Some(0x401000), 0x3f000);
    mappings.alloc("UPX1".to_string(), Some(0x440000), 0xe000);
    let bytes = include_bytes!("../data/00440000.raw").as_slice();
    let out = &mut ctx.memory[0x440000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rsrc".to_string(), Some(0x44e000), 0x2000);
    let bytes = include_bytes!("../data/0044e000.raw").as_slice();
    let out = &mut ctx.memory[0x44e000..][..bytes.len()];
    out.copy_from_slice(bytes);
}
pub fn x44d840(ctx: &mut Context) -> Cont {
    // 0044d840 pusha
    pushad(ctx);
    // 0044d841 mov esi,440000h
    ctx.cpu.regs.esi = 0x440000u32;
    // 0044d846 lea edi,[esi-3F000h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0xfffc1000u32);
    // 0044d84c push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0044d84d or ebp,0FFFFFFFFh
    ctx.cpu.regs.ebp = or(ctx.cpu.regs.ebp, 0xffffffffu32, &mut ctx.cpu.flags);
    // 0044d850 jmp short 0044D862h
    Cont(x44d862)
}

pub fn x44d858(ctx: &mut Context) -> Cont {
    // 0044d858 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 0044d85a inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0044d85b mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 0044d85d inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    Cont(x44d85e)
}

pub fn x44d85e(ctx: &mut Context) -> Cont {
    // 0044d85e add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d860 jne short 0044D869h
    jne(ctx, Cont(x44d862), Cont(x44d869))
}

pub fn x44d862(ctx: &mut Context) -> Cont {
    // 0044d862 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d864 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d867 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    Cont(x44d869)
}

pub fn x44d869(ctx: &mut Context) -> Cont {
    // 0044d869 jb short 0044D858h
    jb(ctx, Cont(x44d86b), Cont(x44d858))
}

pub fn x44d86b(ctx: &mut Context) -> Cont {
    // 0044d86b mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    Cont(x44d870)
}

pub fn x44d870(ctx: &mut Context) -> Cont {
    // 0044d870 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d872 jne short 0044D87Bh
    jne(ctx, Cont(x44d874), Cont(x44d87b))
}

pub fn x44d874(ctx: &mut Context) -> Cont {
    // 0044d874 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d876 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d879 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    Cont(x44d87b)
}

pub fn x44d87b(ctx: &mut Context) -> Cont {
    // 0044d87b adc eax,eax
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.eax = addc(
        ctx.cpu.regs.eax,
        ctx.cpu.regs.eax,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d87d add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d87f ja short 0044D870h
    ja(ctx, Cont(x44d881), Cont(x44d870))
}

pub fn x44d881(ctx: &mut Context) -> Cont {
    // 0044d881 jne short 0044D88Ch
    jne(ctx, Cont(x44d883), Cont(x44d88c))
}

pub fn x44d883(ctx: &mut Context) -> Cont {
    // 0044d883 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d885 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d888 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d88a jae short 0044D870h
    jae(ctx, Cont(x44d88c), Cont(x44d870))
}

pub fn x44d88c(ctx: &mut Context) -> Cont {
    // 0044d88c xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0044d88e sub eax,3
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, 0x3u32, &mut ctx.cpu.flags);
    // 0044d891 jb short 0044D8A0h
    jb(ctx, Cont(x44d893), Cont(x44d8a0))
}

pub fn x44d893(ctx: &mut Context) -> Cont {
    // 0044d893 shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 0044d896 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 0044d898 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0044d899 xor eax,0FFFFFFFFh
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, 0xffffffffu32, &mut ctx.cpu.flags);
    // 0044d89c je short 0044D912h
    je(ctx, Cont(x44d89e), Cont(x44d912))
}

pub fn x44d89e(ctx: &mut Context) -> Cont {
    // 0044d89e mov ebp,eax
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    Cont(x44d8a0)
}

pub fn x44d8a0(ctx: &mut Context) -> Cont {
    // 0044d8a0 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d8a2 jne short 0044D8ABh
    jne(ctx, Cont(x44d8a4), Cont(x44d8ab))
}

pub fn x44d8a4(ctx: &mut Context) -> Cont {
    // 0044d8a4 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d8a6 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d8a9 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    Cont(x44d8ab)
}

pub fn x44d8ab(ctx: &mut Context) -> Cont {
    // 0044d8ab adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d8ad add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d8af jne short 0044D8B8h
    jne(ctx, Cont(x44d8b1), Cont(x44d8b8))
}

pub fn x44d8b1(ctx: &mut Context) -> Cont {
    // 0044d8b1 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d8b3 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d8b6 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    Cont(x44d8b8)
}

pub fn x44d8b8(ctx: &mut Context) -> Cont {
    // 0044d8b8 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d8ba jne short 0044D8DCh
    jne(ctx, Cont(x44d8bc), Cont(x44d8dc))
}

pub fn x44d8bc(ctx: &mut Context) -> Cont {
    // 0044d8bc inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    Cont(x44d8bd)
}

pub fn x44d8bd(ctx: &mut Context) -> Cont {
    // 0044d8bd add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d8bf jne short 0044D8C8h
    jne(ctx, Cont(x44d8c1), Cont(x44d8c8))
}

pub fn x44d8c1(ctx: &mut Context) -> Cont {
    // 0044d8c1 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d8c3 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d8c6 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    Cont(x44d8c8)
}

pub fn x44d8c8(ctx: &mut Context) -> Cont {
    // 0044d8c8 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d8ca add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 0044d8cc ja short 0044D8BDh
    ja(ctx, Cont(x44d8ce), Cont(x44d8bd))
}

pub fn x44d8ce(ctx: &mut Context) -> Cont {
    // 0044d8ce jne short 0044D8D9h
    jne(ctx, Cont(x44d8d0), Cont(x44d8d9))
}

pub fn x44d8d0(ctx: &mut Context) -> Cont {
    // 0044d8d0 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 0044d8d2 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d8d5 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 0044d8d7 jae short 0044D8BDh
    jae(ctx, Cont(x44d8d9), Cont(x44d8bd))
}

pub fn x44d8d9(ctx: &mut Context) -> Cont {
    // 0044d8d9 add ecx,2
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x2u32, &mut ctx.cpu.flags);
    Cont(x44d8dc)
}

pub fn x44d8dc(ctx: &mut Context) -> Cont {
    // 0044d8dc cmp ebp,0FFFFF300h
    sub(ctx.cpu.regs.ebp, 0xfffff300u32, &mut ctx.cpu.flags);
    // 0044d8e2 adc ecx,1
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(ctx.cpu.regs.ecx, 0x1u32, carry as _, &mut ctx.cpu.flags);
    // 0044d8e5 lea edx,[edi+ebp]
    ctx.cpu.regs.edx = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ebp);
    // 0044d8e8 cmp ebp,0FFFFFFFCh
    sub(ctx.cpu.regs.ebp, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 0044d8eb jle short 0044D8FCh
    jle(ctx, Cont(x44d8ed), Cont(x44d8fc))
}

pub fn x44d8ed(ctx: &mut Context) -> Cont {
    // 0044d8ed mov al,[edx]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edx));
    // 0044d8ef inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 0044d8f0 mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 0044d8f2 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0044d8f3 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0044d8f4 jne short 0044D8EDh
    jne(ctx, Cont(x44d8f6), Cont(x44d8ed))
}

pub fn x44d8f6(ctx: &mut Context) -> Cont {
    // 0044d8f6 jmp near ptr 0044D85Eh
    Cont(x44d85e)
}

pub fn x44d8fc(ctx: &mut Context) -> Cont {
    // 0044d8fc mov eax,[edx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edx);
    // 0044d8fe add edx,4
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x4u32, &mut ctx.cpu.flags);
    // 0044d901 mov [edi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, ctx.cpu.regs.eax);
    // 0044d903 add edi,4
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 0044d906 sub ecx,4
    ctx.cpu.regs.ecx = sub(ctx.cpu.regs.ecx, 0x4u32, &mut ctx.cpu.flags);
    // 0044d909 ja short 0044D8FCh
    ja(ctx, Cont(x44d90b), Cont(x44d8fc))
}

pub fn x44d90b(ctx: &mut Context) -> Cont {
    // 0044d90b add edi,ecx
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 0044d90d jmp near ptr 0044D85Eh
    Cont(x44d85e)
}

pub fn x44d912(ctx: &mut Context) -> Cont {
    // 0044d912 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 0044d913 sub edi,[edi-4]
    ctx.cpu.regs.edi = sub(
        ctx.cpu.regs.edi,
        ctx.memory
            .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0xfffffffcu32)),
        &mut ctx.cpu.flags,
    );
    // 0044d916 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0044d917 mov edi,esi
    ctx.cpu.regs.edi = ctx.cpu.regs.esi;
    // 0044d919 mov ecx,1EDh
    ctx.cpu.regs.ecx = 0x1edu32;
    Cont(x44d91e)
}

pub fn x44d91e(ctx: &mut Context) -> Cont {
    // 0044d91e mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 0044d920 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0044d921 sub al,0E8h
    ctx.cpu
        .regs
        .set_al(sub(ctx.cpu.regs.get_al(), 0xe8u8, &mut ctx.cpu.flags));
    Cont(x44d923)
}

pub fn x44d923(ctx: &mut Context) -> Cont {
    // 0044d923 cmp al,1
    sub(ctx.cpu.regs.get_al(), 0x1u8, &mut ctx.cpu.flags);
    // 0044d925 ja short 0044D91Eh
    ja(ctx, Cont(x44d927), Cont(x44d91e))
}

pub fn x44d927(ctx: &mut Context) -> Cont {
    // 0044d927 cmp byte ptr [edi],5
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.edi),
        0x5u8,
        &mut ctx.cpu.flags,
    );
    // 0044d92a jne short 0044D91Eh
    jne(ctx, Cont(x44d92c), Cont(x44d91e))
}

pub fn x44d92c(ctx: &mut Context) -> Cont {
    // 0044d92c mov eax,[edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edi);
    // 0044d92e mov bl,[edi+4]
    ctx.cpu
        .regs
        .set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 0044d931 shr ax,8
    ctx.cpu
        .regs
        .set_ax(shr(ctx.cpu.regs.get_ax(), 0x8u8, &mut ctx.cpu.flags));
    // 0044d935 rol eax,10h
    ctx.cpu.regs.eax = rol(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 0044d938 xchg al,ah
    let t = ctx.cpu.regs.get_ah();
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    ctx.cpu.regs.set_al(t);
    // 0044d93a sub eax,edi
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0044d93c sub bl,0E8h
    ctx.cpu
        .regs
        .set_bl(sub(ctx.cpu.regs.get_bl(), 0xe8u8, &mut ctx.cpu.flags));
    // 0044d93f add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0044d941 mov [edi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, ctx.cpu.regs.eax);
    // 0044d943 add edi,5
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x5u32, &mut ctx.cpu.flags);
    // 0044d946 mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 0044d948 loop 0044D923h
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x44d94a)
    } else {
        Cont(x44d923)
    }
}

pub fn x44d94a(ctx: &mut Context) -> Cont {
    // 0044d94a pop edi
    let x = pop(ctx);
    ctx.cpu.regs.edi = x;
    Cont(x44d94b)
}

pub fn x44d94b(ctx: &mut Context) -> Cont {
    // 0044d94b mov eax,[edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edi);
    // 0044d94d or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0044d94f je short 0044D98Bh
    je(ctx, Cont(x44d951), Cont(x44d98b))
}

pub fn x44d951(ctx: &mut Context) -> Cont {
    // 0044d951 mov ebx,[edi+4]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4u32));
    // 0044d954 lea eax,[eax+esi+4E390h]
    ctx.cpu.regs.eax = ctx
        .cpu
        .regs
        .eax
        .wrapping_add(ctx.cpu.regs.esi)
        .wrapping_add(0x4e390u32);
    // 0044d95b add ebx,esi
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 0044d95d push eax
    push(ctx, ctx.cpu.regs.eax);
    // 0044d95e add edi,8
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x8u32, &mut ctx.cpu.flags);
    // 0044d961 call dword ptr [esi+4E3F4h]
    let dst = indirect(
        ctx,
        ctx.memory.read(ctx.cpu.regs.esi.wrapping_add(0x4e3f4u32)),
    );
    call(ctx, 0x44d967, dst)
}

pub fn x44d967(ctx: &mut Context) -> Cont {
    // 0044d967 xchg edx,eax
    let t = ctx.cpu.regs.edx;
    ctx.cpu.regs.edx = ctx.cpu.regs.eax;
    ctx.cpu.regs.eax = t;
    Cont(x44d968)
}

pub fn x44d968(ctx: &mut Context) -> Cont {
    // 0044d968 mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 0044d96a inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 0044d96b or al,al
    ctx.cpu.regs.set_al(or(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    ));
    // 0044d96d je short 0044D94Bh
    je(ctx, Cont(x44d96f), Cont(x44d94b))
}

pub fn x44d96f(ctx: &mut Context) -> Cont {
    // 0044d96f push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0044d970 mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 0044d972 push edi
    push(ctx, ctx.cpu.regs.edi);
    // 0044d973 dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0044d974 repne scasb
    rep(ctx, Rep::REPNE, scasb);
    // 0044d976 push edx
    push(ctx, ctx.cpu.regs.edx);
    // 0044d977 call dword ptr [esi+4E3F8h]
    let dst = indirect(
        ctx,
        ctx.memory.read(ctx.cpu.regs.esi.wrapping_add(0x4e3f8u32)),
    );
    call(ctx, 0x44d97d, dst)
}

pub fn x44d97d(ctx: &mut Context) -> Cont {
    // 0044d97d pop edx
    let x = pop(ctx);
    ctx.cpu.regs.edx = x;
    // 0044d97e or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 0044d980 je short 0044D989h
    je(ctx, Cont(x44d982), Cont(x44d989))
}

pub fn x44d982(ctx: &mut Context) -> Cont {
    // 0044d982 mov [ebx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.ebx, ctx.cpu.regs.eax);
    // 0044d984 add ebx,4
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x4u32, &mut ctx.cpu.flags);
    // 0044d987 jmp short 0044D968h
    Cont(x44d968)
}

pub fn x44d989(ctx: &mut Context) -> Cont {
    // 0044d989 popa
    popad(ctx);
    // 0044d98a ret
    ret(ctx, 0)
}

pub fn x44d98b(ctx: &mut Context) -> Cont {
    // 0044d98b popa
    popad(ctx);
    // 0044d98c jmp near ptr 0041F079h
    Cont(x41f079)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 191] = [
    (0x1000, kernel32::LoadLibraryA_stdcall),
    (0x1001, kernel32::GetProcAddress_stdcall),
    (0x1002, ddraw::DirectDrawCreate_stdcall),
    (0x1003, user32::ShowWindow_stdcall),
    (0x1004, winmm::waveOutOpen_stdcall),
    (0x1005, ddraw::IDirectDraw::QueryInterface_stdcall),
    (0x1006, ddraw::IDirectDraw::AddRef_stdcall),
    (0x1007, ddraw::IDirectDraw::Release_stdcall),
    (0x1008, ddraw::IDirectDraw::Compact_stdcall),
    (0x1009, ddraw::IDirectDraw::CreateClipper_stdcall),
    (0x100a, ddraw::IDirectDraw::CreatePalette_stdcall),
    (0x100b, ddraw::IDirectDraw::CreateSurface_stdcall),
    (0x100c, ddraw::IDirectDraw::DuplicateSurface_stdcall),
    (0x100d, ddraw::IDirectDraw::EnumDisplayModes_stdcall),
    (0x100e, ddraw::IDirectDraw::EnumSurfaces_stdcall),
    (0x100f, ddraw::IDirectDraw::FlipToGDISurface_stdcall),
    (0x1010, ddraw::IDirectDraw::GetCaps_stdcall),
    (0x1011, ddraw::IDirectDraw::GetDisplayMode_stdcall),
    (0x1012, ddraw::IDirectDraw::GetFourCCCodes_stdcall),
    (0x1013, ddraw::IDirectDraw::GetGDISurface_stdcall),
    (0x1014, ddraw::IDirectDraw::GetMonitorFrequency_stdcall),
    (0x1015, ddraw::IDirectDraw::GetScanLine_stdcall),
    (0x1016, ddraw::IDirectDraw::GetVerticalBlankStatus_stdcall),
    (0x1017, ddraw::IDirectDraw::Initialize_stdcall),
    (0x1018, ddraw::IDirectDraw::RestoreDisplayMode_stdcall),
    (0x1019, ddraw::IDirectDraw::SetCooperativeLevel_stdcall),
    (0x101a, ddraw::IDirectDraw::SetDisplayMode_stdcall),
    (0x101b, ddraw::IDirectDraw::WaitForVerticalBlank_stdcall),
    (0x101c, ddraw::IDirectDrawSurface::QueryInterface_stdcall),
    (0x101d, ddraw::IDirectDrawSurface::AddRef_stdcall),
    (0x101e, ddraw::IDirectDrawSurface::Release_stdcall),
    (
        0x101f,
        ddraw::IDirectDrawSurface::AddAttachedSurface_stdcall,
    ),
    (
        0x1020,
        ddraw::IDirectDrawSurface::AddOverlayDirtyRect_stdcall,
    ),
    (0x1021, ddraw::IDirectDrawSurface::Blt_stdcall),
    (0x1022, ddraw::IDirectDrawSurface::BltBatch_stdcall),
    (0x1023, ddraw::IDirectDrawSurface::BltFast_stdcall),
    (
        0x1024,
        ddraw::IDirectDrawSurface::DeleteAttachedSurface_stdcall,
    ),
    (
        0x1025,
        ddraw::IDirectDrawSurface::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x1026,
        ddraw::IDirectDrawSurface::EnumOverlayZOrders_stdcall,
    ),
    (0x1027, ddraw::IDirectDrawSurface::Flip_stdcall),
    (
        0x1028,
        ddraw::IDirectDrawSurface::GetAttachedSurface_stdcall,
    ),
    (0x1029, ddraw::IDirectDrawSurface::GetBltStatus_stdcall),
    (0x102a, ddraw::IDirectDrawSurface::GetCaps_stdcall),
    (0x102b, ddraw::IDirectDrawSurface::GetClipper_stdcall),
    (0x102c, ddraw::IDirectDrawSurface::GetColorKey_stdcall),
    (0x102d, ddraw::IDirectDrawSurface::GetDC_stdcall),
    (0x102e, ddraw::IDirectDrawSurface::GetFlipStatus_stdcall),
    (
        0x102f,
        ddraw::IDirectDrawSurface::GetOverlayPosition_stdcall,
    ),
    (0x1030, ddraw::IDirectDrawSurface::GetPalette_stdcall),
    (0x1031, ddraw::IDirectDrawSurface::GetPixelFormat_stdcall),
    (0x1032, ddraw::IDirectDrawSurface::GetSurfaceDesc_stdcall),
    (0x1033, ddraw::IDirectDrawSurface::Initialize_stdcall),
    (0x1034, ddraw::IDirectDrawSurface::IsLost_stdcall),
    (0x1035, ddraw::IDirectDrawSurface::Lock_stdcall),
    (0x1036, ddraw::IDirectDrawSurface::ReleaseDC_stdcall),
    (0x1037, ddraw::IDirectDrawSurface::Restore_stdcall),
    (0x1038, ddraw::IDirectDrawSurface::SetClipper_stdcall),
    (0x1039, ddraw::IDirectDrawSurface::SetColorKey_stdcall),
    (
        0x103a,
        ddraw::IDirectDrawSurface::SetOverlayPosition_stdcall,
    ),
    (0x103b, ddraw::IDirectDrawSurface::SetPalette_stdcall),
    (0x103c, ddraw::IDirectDrawSurface::Unlock_stdcall),
    (0x103d, ddraw::IDirectDrawSurface::UpdateOverlay_stdcall),
    (
        0x103e,
        ddraw::IDirectDrawSurface::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x103f,
        ddraw::IDirectDrawSurface::UpdateOverlayZOrder_stdcall,
    ),
    (0x1040, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0x1041, ddraw::IDirectDraw7::AddRef_stdcall),
    (0x1042, ddraw::IDirectDraw7::Release_stdcall),
    (0x1043, ddraw::IDirectDraw7::Compact_stdcall),
    (0x1044, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0x1045, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0x1046, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0x1047, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0x1048, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0x1049, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0x104a, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0x104b, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0x104c, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0x104d, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0x104e, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0x104f, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0x1050, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (0x1051, ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall),
    (0x1052, ddraw::IDirectDraw7::Initialize_stdcall),
    (0x1053, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0x1054, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0x1055, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (0x1056, ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall),
    (0x1057, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0x1058, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0x1059, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (0x105a, ddraw::IDirectDraw7::TestCooperativeLevel_stdcall),
    (0x105b, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0x105c, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0x105d, ddraw::IDirectDraw7::EvaluateMode_stdcall),
    (0x105e, ddraw::IDirectDrawSurface7::QueryInterface_stdcall),
    (0x105f, ddraw::IDirectDrawSurface7::AddRef_stdcall),
    (0x1060, ddraw::IDirectDrawSurface7::Release_stdcall),
    (
        0x1061,
        ddraw::IDirectDrawSurface7::AddAttachedSurface_stdcall,
    ),
    (
        0x1062,
        ddraw::IDirectDrawSurface7::AddOverlayDirtyRect_stdcall,
    ),
    (0x1063, ddraw::IDirectDrawSurface7::Blt_stdcall),
    (0x1064, ddraw::IDirectDrawSurface7::BltBatch_stdcall),
    (0x1065, ddraw::IDirectDrawSurface7::BltFast_stdcall),
    (
        0x1066,
        ddraw::IDirectDrawSurface7::DeleteAttachedSurface_stdcall,
    ),
    (
        0x1067,
        ddraw::IDirectDrawSurface7::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x1068,
        ddraw::IDirectDrawSurface7::EnumOverlayZOrders_stdcall,
    ),
    (0x1069, ddraw::IDirectDrawSurface7::Flip_stdcall),
    (
        0x106a,
        ddraw::IDirectDrawSurface7::GetAttachedSurface_stdcall,
    ),
    (0x106b, ddraw::IDirectDrawSurface7::GetBltStatus_stdcall),
    (0x106c, ddraw::IDirectDrawSurface7::GetCaps_stdcall),
    (0x106d, ddraw::IDirectDrawSurface7::GetClipper_stdcall),
    (0x106e, ddraw::IDirectDrawSurface7::GetColorKey_stdcall),
    (0x106f, ddraw::IDirectDrawSurface7::GetDC_stdcall),
    (0x1070, ddraw::IDirectDrawSurface7::GetFlipStatus_stdcall),
    (
        0x1071,
        ddraw::IDirectDrawSurface7::GetOverlayPosition_stdcall,
    ),
    (0x1072, ddraw::IDirectDrawSurface7::GetPalette_stdcall),
    (0x1073, ddraw::IDirectDrawSurface7::GetPixelFormat_stdcall),
    (0x1074, ddraw::IDirectDrawSurface7::GetSurfaceDesc_stdcall),
    (0x1075, ddraw::IDirectDrawSurface7::Initialize_stdcall),
    (0x1076, ddraw::IDirectDrawSurface7::IsLost_stdcall),
    (0x1077, ddraw::IDirectDrawSurface7::Lock_stdcall),
    (0x1078, ddraw::IDirectDrawSurface7::ReleaseDC_stdcall),
    (0x1079, ddraw::IDirectDrawSurface7::Restore_stdcall),
    (0x107a, ddraw::IDirectDrawSurface7::SetClipper_stdcall),
    (0x107b, ddraw::IDirectDrawSurface7::SetColorKey_stdcall),
    (
        0x107c,
        ddraw::IDirectDrawSurface7::SetOverlayPosition_stdcall,
    ),
    (0x107d, ddraw::IDirectDrawSurface7::SetPalette_stdcall),
    (0x107e, ddraw::IDirectDrawSurface7::Unlock_stdcall),
    (0x107f, ddraw::IDirectDrawSurface7::UpdateOverlay_stdcall),
    (
        0x1080,
        ddraw::IDirectDrawSurface7::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x1081,
        ddraw::IDirectDrawSurface7::UpdateOverlayZOrder_stdcall,
    ),
    (0x1082, ddraw::IDirectDrawSurface7::GetDDInterface_stdcall),
    (0x1083, ddraw::IDirectDrawSurface7::PageLock_stdcall),
    (0x1084, ddraw::IDirectDrawSurface7::PageUnlock_stdcall),
    (0x1085, ddraw::IDirectDrawSurface7::SetSurfaceDesc_stdcall),
    (0x1086, ddraw::IDirectDrawSurface7::SetPrivateData_stdcall),
    (0x1087, ddraw::IDirectDrawSurface7::GetPrivateData_stdcall),
    (0x1088, ddraw::IDirectDrawSurface7::FreePrivateData_stdcall),
    (
        0x1089,
        ddraw::IDirectDrawSurface7::GetUniquenessValue_stdcall,
    ),
    (
        0x108a,
        ddraw::IDirectDrawSurface7::ChangeUniquenessValue_stdcall,
    ),
    (0x108b, ddraw::IDirectDrawSurface7::SetPriority_stdcall),
    (0x108c, ddraw::IDirectDrawSurface7::GetPriority_stdcall),
    (0x108d, ddraw::IDirectDrawSurface7::SetLOD_stdcall),
    (0x108e, ddraw::IDirectDrawSurface7::GetLOD_stdcall),
    (0x41f079, x41f079),
    (0x44d840, x44d840),
    (0x44d858, x44d858),
    (0x44d85e, x44d85e),
    (0x44d862, x44d862),
    (0x44d869, x44d869),
    (0x44d86b, x44d86b),
    (0x44d870, x44d870),
    (0x44d874, x44d874),
    (0x44d87b, x44d87b),
    (0x44d881, x44d881),
    (0x44d883, x44d883),
    (0x44d88c, x44d88c),
    (0x44d893, x44d893),
    (0x44d89e, x44d89e),
    (0x44d8a0, x44d8a0),
    (0x44d8a4, x44d8a4),
    (0x44d8ab, x44d8ab),
    (0x44d8b1, x44d8b1),
    (0x44d8b8, x44d8b8),
    (0x44d8bc, x44d8bc),
    (0x44d8bd, x44d8bd),
    (0x44d8c1, x44d8c1),
    (0x44d8c8, x44d8c8),
    (0x44d8ce, x44d8ce),
    (0x44d8d0, x44d8d0),
    (0x44d8d9, x44d8d9),
    (0x44d8dc, x44d8dc),
    (0x44d8ed, x44d8ed),
    (0x44d8f6, x44d8f6),
    (0x44d8fc, x44d8fc),
    (0x44d90b, x44d90b),
    (0x44d912, x44d912),
    (0x44d91e, x44d91e),
    (0x44d923, x44d923),
    (0x44d927, x44d927),
    (0x44d92c, x44d92c),
    (0x44d94a, x44d94a),
    (0x44d94b, x44d94b),
    (0x44d951, x44d951),
    (0x44d967, x44d967),
    (0x44d968, x44d968),
    (0x44d96f, x44d96f),
    (0x44d97d, x44d97d),
    (0x44d982, x44d982),
    (0x44d989, x44d989),
    (0x44d98b, x44d98b),
    (runtime::RETURN_FROM_X86_ADDR, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0x44e000..0x44f390,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x44d840),
};
