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
    mappings.alloc("UPX0".to_string(), Some(0x401000), 0xbb000);
    mappings.alloc("UPX1".to_string(), Some(0x4bc000), 0x10000);
    let bytes = include_bytes!("../data/004bc000.raw").as_slice();
    let out = &mut ctx.memory[0x4bc000..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".rsrc".to_string(), Some(0x4cc000), 0x1000);
    let bytes = include_bytes!("../data/004cc000.raw").as_slice();
    let out = &mut ctx.memory[0x4cc000..][..bytes.len()];
    out.copy_from_slice(bytes);
}
pub fn x004cbca0(ctx: &mut Context) -> Cont {
    // 004cbca0 pusha
    pushad(ctx);
    // 004cbca1 mov esi,4BC118h
    ctx.cpu.regs.esi = 0x4bc118u32;
    // 004cbca6 lea edi,[esi-0BB118h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0xfff44ee8u32);
    // 004cbcac push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004cbcad or ebp,0FFFFFFFFh
    ctx.cpu.regs.ebp = or(ctx.cpu.regs.ebp, 0xffffffffu32, &mut ctx.cpu.flags);
    // 004cbcb0 jmp short 004CBCC2h
    Cont(x004cbcc2)
}

pub fn x004cbcb8(ctx: &mut Context) -> Cont {
    // 004cbcb8 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 004cbcba inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004cbcbb mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 004cbcbd inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbcbe add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcc0 jne short 004CBCC9h
    jne(ctx, Cont(x004cbcc2), Cont(x004cbcc9))
}

pub fn x004cbcbe(ctx: &mut Context) -> Cont {
    // 004cbcbe add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcc0 jne short 004CBCC9h
    jne(ctx, Cont(x004cbcc2), Cont(x004cbcc9))
}

pub fn x004cbcc2(ctx: &mut Context) -> Cont {
    // 004cbcc2 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbcc4 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbcc7 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbcc9 jb short 004CBCB8h
    jb(ctx, Cont(x004cbccb), Cont(x004cbcb8))
}

pub fn x004cbcc9(ctx: &mut Context) -> Cont {
    // 004cbcc9 jb short 004CBCB8h
    jb(ctx, Cont(x004cbccb), Cont(x004cbcb8))
}

pub fn x004cbccb(ctx: &mut Context) -> Cont {
    // 004cbccb mov eax,1
    ctx.cpu.regs.eax = 0x1u32;
    // 004cbcd0 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcd2 jne short 004CBCDBh
    jne(ctx, Cont(x004cbcd4), Cont(x004cbcdb))
}

pub fn x004cbcd0(ctx: &mut Context) -> Cont {
    // 004cbcd0 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcd2 jne short 004CBCDBh
    jne(ctx, Cont(x004cbcd4), Cont(x004cbcdb))
}

pub fn x004cbcd4(ctx: &mut Context) -> Cont {
    // 004cbcd4 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbcd6 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbcd9 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbcdb adc eax,eax
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.eax = addc(
        ctx.cpu.regs.eax,
        ctx.cpu.regs.eax,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbcdd add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcdf jae short 004CBCD0h
    jae(ctx, Cont(x004cbce1), Cont(x004cbcd0))
}

pub fn x004cbcdb(ctx: &mut Context) -> Cont {
    // 004cbcdb adc eax,eax
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.eax = addc(
        ctx.cpu.regs.eax,
        ctx.cpu.regs.eax,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbcdd add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbcdf jae short 004CBCD0h
    jae(ctx, Cont(x004cbce1), Cont(x004cbcd0))
}

pub fn x004cbce1(ctx: &mut Context) -> Cont {
    // 004cbce1 jne short 004CBCECh
    jne(ctx, Cont(x004cbce3), Cont(x004cbcec))
}

pub fn x004cbce3(ctx: &mut Context) -> Cont {
    // 004cbce3 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbce5 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbce8 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbcea jae short 004CBCD0h
    jae(ctx, Cont(x004cbcec), Cont(x004cbcd0))
}

pub fn x004cbcec(ctx: &mut Context) -> Cont {
    // 004cbcec xor ecx,ecx
    ctx.cpu.regs.ecx = xor(ctx.cpu.regs.ecx, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004cbcee sub eax,3
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, 0x3u32, &mut ctx.cpu.flags);
    // 004cbcf1 jb short 004CBD00h
    jb(ctx, Cont(x004cbcf3), Cont(x004cbd00))
}

pub fn x004cbcf3(ctx: &mut Context) -> Cont {
    // 004cbcf3 shl eax,8
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x8u8, &mut ctx.cpu.flags);
    // 004cbcf6 mov al,[esi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.esi));
    // 004cbcf8 inc esi
    ctx.cpu.regs.esi = inc(ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004cbcf9 xor eax,0FFFFFFFFh
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, 0xffffffffu32, &mut ctx.cpu.flags);
    // 004cbcfc je short 004CBD72h
    je(ctx, Cont(x004cbcfe), Cont(x004cbd72))
}

pub fn x004cbcfe(ctx: &mut Context) -> Cont {
    // 004cbcfe mov ebp,eax
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    // 004cbd00 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd02 jne short 004CBD0Bh
    jne(ctx, Cont(x004cbd04), Cont(x004cbd0b))
}

pub fn x004cbd00(ctx: &mut Context) -> Cont {
    // 004cbd00 add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd02 jne short 004CBD0Bh
    jne(ctx, Cont(x004cbd04), Cont(x004cbd0b))
}

pub fn x004cbd04(ctx: &mut Context) -> Cont {
    // 004cbd04 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbd06 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd09 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd0b adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd0d add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd0f jne short 004CBD18h
    jne(ctx, Cont(x004cbd11), Cont(x004cbd18))
}

pub fn x004cbd0b(ctx: &mut Context) -> Cont {
    // 004cbd0b adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd0d add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd0f jne short 004CBD18h
    jne(ctx, Cont(x004cbd11), Cont(x004cbd18))
}

pub fn x004cbd11(ctx: &mut Context) -> Cont {
    // 004cbd11 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbd13 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd16 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd18 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd1a jne short 004CBD3Ch
    jne(ctx, Cont(x004cbd1c), Cont(x004cbd3c))
}

pub fn x004cbd18(ctx: &mut Context) -> Cont {
    // 004cbd18 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd1a jne short 004CBD3Ch
    jne(ctx, Cont(x004cbd1c), Cont(x004cbd3c))
}

pub fn x004cbd1c(ctx: &mut Context) -> Cont {
    // 004cbd1c inc ecx
    ctx.cpu.regs.ecx = inc(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004cbd1d add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd1f jne short 004CBD28h
    jne(ctx, Cont(x004cbd21), Cont(x004cbd28))
}

pub fn x004cbd1d(ctx: &mut Context) -> Cont {
    // 004cbd1d add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd1f jne short 004CBD28h
    jne(ctx, Cont(x004cbd21), Cont(x004cbd28))
}

pub fn x004cbd21(ctx: &mut Context) -> Cont {
    // 004cbd21 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbd23 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd26 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd28 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd2a add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd2c jae short 004CBD1Dh
    jae(ctx, Cont(x004cbd2e), Cont(x004cbd1d))
}

pub fn x004cbd28(ctx: &mut Context) -> Cont {
    // 004cbd28 adc ecx,ecx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(
        ctx.cpu.regs.ecx,
        ctx.cpu.regs.ecx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd2a add ebx,ebx
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.ebx, &mut ctx.cpu.flags);
    // 004cbd2c jae short 004CBD1Dh
    jae(ctx, Cont(x004cbd2e), Cont(x004cbd1d))
}

pub fn x004cbd2e(ctx: &mut Context) -> Cont {
    // 004cbd2e jne short 004CBD39h
    jne(ctx, Cont(x004cbd30), Cont(x004cbd39))
}

pub fn x004cbd30(ctx: &mut Context) -> Cont {
    // 004cbd30 mov ebx,[esi]
    ctx.cpu.regs.ebx = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    // 004cbd32 sub esi,0FFFFFFFCh
    ctx.cpu.regs.esi = sub(ctx.cpu.regs.esi, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd35 adc ebx,ebx
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ebx = addc(
        ctx.cpu.regs.ebx,
        ctx.cpu.regs.ebx,
        carry as _,
        &mut ctx.cpu.flags,
    );
    // 004cbd37 jae short 004CBD1Dh
    jae(ctx, Cont(x004cbd39), Cont(x004cbd1d))
}

pub fn x004cbd39(ctx: &mut Context) -> Cont {
    // 004cbd39 add ecx,2
    ctx.cpu.regs.ecx = add(ctx.cpu.regs.ecx, 0x2u32, &mut ctx.cpu.flags);
    // 004cbd3c cmp ebp,0FFFFF300h
    sub(ctx.cpu.regs.ebp, 0xfffff300u32, &mut ctx.cpu.flags);
    // 004cbd42 adc ecx,1
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(ctx.cpu.regs.ecx, 0x1u32, carry as _, &mut ctx.cpu.flags);
    // 004cbd45 lea edx,[edi+ebp]
    ctx.cpu.regs.edx = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ebp);
    // 004cbd48 cmp ebp,0FFFFFFFCh
    sub(ctx.cpu.regs.ebp, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd4b jbe short 004CBD5Ch
    jbe(ctx, Cont(x004cbd4d), Cont(x004cbd5c))
}

pub fn x004cbd3c(ctx: &mut Context) -> Cont {
    // 004cbd3c cmp ebp,0FFFFF300h
    sub(ctx.cpu.regs.ebp, 0xfffff300u32, &mut ctx.cpu.flags);
    // 004cbd42 adc ecx,1
    let carry = ctx.cpu.flags.contains(Flags::CF) as u32;
    ctx.cpu.regs.ecx = addc(ctx.cpu.regs.ecx, 0x1u32, carry as _, &mut ctx.cpu.flags);
    // 004cbd45 lea edx,[edi+ebp]
    ctx.cpu.regs.edx = ctx.cpu.regs.edi.wrapping_add(ctx.cpu.regs.ebp);
    // 004cbd48 cmp ebp,0FFFFFFFCh
    sub(ctx.cpu.regs.ebp, 0xfffffffcu32, &mut ctx.cpu.flags);
    // 004cbd4b jbe short 004CBD5Ch
    jbe(ctx, Cont(x004cbd4d), Cont(x004cbd5c))
}

pub fn x004cbd4d(ctx: &mut Context) -> Cont {
    // 004cbd4d mov al,[edx]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edx));
    // 004cbd4f inc edx
    ctx.cpu.regs.edx = inc(ctx.cpu.regs.edx, &mut ctx.cpu.flags);
    // 004cbd50 mov [edi],al
    ctx.memory
        .write::<u8>(ctx.cpu.regs.edi, ctx.cpu.regs.get_al());
    // 004cbd52 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbd53 dec ecx
    ctx.cpu.regs.ecx = dec(ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004cbd54 jne short 004CBD4Dh
    jne(ctx, Cont(x004cbd56), Cont(x004cbd4d))
}

pub fn x004cbd56(ctx: &mut Context) -> Cont {
    // 004cbd56 jmp near ptr 004CBCBEh
    Cont(x004cbcbe)
}

pub fn x004cbd5c(ctx: &mut Context) -> Cont {
    // 004cbd5c mov eax,[edx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edx);
    // 004cbd5e add edx,4
    ctx.cpu.regs.edx = add(ctx.cpu.regs.edx, 0x4u32, &mut ctx.cpu.flags);
    // 004cbd61 mov [edi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, ctx.cpu.regs.eax);
    // 004cbd63 add edi,4
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 004cbd66 sub ecx,4
    ctx.cpu.regs.ecx = sub(ctx.cpu.regs.ecx, 0x4u32, &mut ctx.cpu.flags);
    // 004cbd69 ja short 004CBD5Ch
    ja(ctx, Cont(x004cbd6b), Cont(x004cbd5c))
}

pub fn x004cbd6b(ctx: &mut Context) -> Cont {
    // 004cbd6b add edi,ecx
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, ctx.cpu.regs.ecx, &mut ctx.cpu.flags);
    // 004cbd6d jmp near ptr 004CBCBEh
    Cont(x004cbcbe)
}

pub fn x004cbd72(ctx: &mut Context) -> Cont {
    // 004cbd72 pop esi
    let x = pop(ctx);
    ctx.cpu.regs.esi = x;
    // 004cbd73 mov edi,esi
    ctx.cpu.regs.edi = ctx.cpu.regs.esi;
    // 004cbd75 mov ecx,193h
    ctx.cpu.regs.ecx = 0x193u32;
    // 004cbd7a mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbd7c inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbd7d sub al,0E8h
    ctx.cpu
        .regs
        .set_al(sub(ctx.cpu.regs.get_al(), 0xe8u8, &mut ctx.cpu.flags));
    // 004cbd7f cmp al,1
    sub(ctx.cpu.regs.get_al(), 0x1u8, &mut ctx.cpu.flags);
    // 004cbd81 ja short 004CBD7Ah
    ja(ctx, Cont(x004cbd83), Cont(x004cbd7a))
}

pub fn x004cbd7a(ctx: &mut Context) -> Cont {
    // 004cbd7a mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbd7c inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbd7d sub al,0E8h
    ctx.cpu
        .regs
        .set_al(sub(ctx.cpu.regs.get_al(), 0xe8u8, &mut ctx.cpu.flags));
    // 004cbd7f cmp al,1
    sub(ctx.cpu.regs.get_al(), 0x1u8, &mut ctx.cpu.flags);
    // 004cbd81 ja short 004CBD7Ah
    ja(ctx, Cont(x004cbd83), Cont(x004cbd7a))
}

pub fn x004cbd7f(ctx: &mut Context) -> Cont {
    // 004cbd7f cmp al,1
    sub(ctx.cpu.regs.get_al(), 0x1u8, &mut ctx.cpu.flags);
    // 004cbd81 ja short 004CBD7Ah
    ja(ctx, Cont(x004cbd83), Cont(x004cbd7a))
}

pub fn x004cbd83(ctx: &mut Context) -> Cont {
    // 004cbd83 cmp byte ptr [edi],9
    sub(
        ctx.memory.read::<u8>(ctx.cpu.regs.edi),
        0x9u8,
        &mut ctx.cpu.flags,
    );
    // 004cbd86 jne short 004CBD7Ah
    jne(ctx, Cont(x004cbd88), Cont(x004cbd7a))
}

pub fn x004cbd88(ctx: &mut Context) -> Cont {
    // 004cbd88 mov eax,[edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edi);
    // 004cbd8a mov bl,[edi+4]
    ctx.cpu
        .regs
        .set_bl(ctx.memory.read::<u8>(ctx.cpu.regs.edi.wrapping_add(0x4u32)));
    // 004cbd8d shr ax,8
    ctx.cpu
        .regs
        .set_ax(shr(ctx.cpu.regs.get_ax(), 0x8u8, &mut ctx.cpu.flags));
    // 004cbd91 rol eax,10h
    ctx.cpu.regs.eax = rol(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 004cbd94 xchg al,ah
    let t = ctx.cpu.regs.get_ah();
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    ctx.cpu.regs.set_al(t);
    // 004cbd96 sub eax,edi
    ctx.cpu.regs.eax = sub(ctx.cpu.regs.eax, ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbd98 sub bl,0E8h
    ctx.cpu
        .regs
        .set_bl(sub(ctx.cpu.regs.get_bl(), 0xe8u8, &mut ctx.cpu.flags));
    // 004cbd9b add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004cbd9d mov [edi],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, ctx.cpu.regs.eax);
    // 004cbd9f add edi,5
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x5u32, &mut ctx.cpu.flags);
    // 004cbda2 mov eax,ebx
    ctx.cpu.regs.eax = ctx.cpu.regs.ebx;
    // 004cbda4 loop 004CBD7Fh
    ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
    if ctx.cpu.regs.ecx == 0 {
        Cont(x004cbda6)
    } else {
        Cont(x004cbd7f)
    }
}

pub fn x004cbda6(ctx: &mut Context) -> Cont {
    // 004cbda6 lea edi,[esi+0C9000h]
    ctx.cpu.regs.edi = ctx.cpu.regs.esi.wrapping_add(0xc9000u32);
    // 004cbdac mov eax,[edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edi);
    // 004cbdae or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbdb0 je short 004CBDF3h
    je(ctx, Cont(x004cbdb2), Cont(x004cbdf3))
}

pub fn x004cbdac(ctx: &mut Context) -> Cont {
    // 004cbdac mov eax,[edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.edi);
    // 004cbdae or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbdb0 je short 004CBDF3h
    je(ctx, Cont(x004cbdb2), Cont(x004cbdf3))
}

pub fn x004cbdb2(ctx: &mut Context) -> Cont {
    // 004cbdb2 mov ebx,[edi+4]
    ctx.cpu.regs.ebx = ctx
        .memory
        .read::<u32>(ctx.cpu.regs.edi.wrapping_add(0x4u32));
    // 004cbdb5 lea eax,[eax+esi+0CB06Ch]
    ctx.cpu.regs.eax = ctx
        .cpu
        .regs
        .eax
        .wrapping_add(ctx.cpu.regs.esi)
        .wrapping_add(0xcb06cu32);
    // 004cbdbc add ebx,esi
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004cbdbe push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004cbdbf add edi,8
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x8u32, &mut ctx.cpu.flags);
    // 004cbdc2 call dword ptr [esi+0CB0E4h]
    let dst = indirect(
        ctx,
        ctx.memory.read(ctx.cpu.regs.esi.wrapping_add(0xcb0e4u32)),
    );
    call(ctx, 0x4cbdc8, dst)
}

pub fn x004cbdc8(ctx: &mut Context) -> Cont {
    // 004cbdc8 xchg ebp,eax
    let t = ctx.cpu.regs.ebp;
    ctx.cpu.regs.ebp = ctx.cpu.regs.eax;
    ctx.cpu.regs.eax = t;
    // 004cbdc9 mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbdcb inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdcc or al,al
    ctx.cpu.regs.set_al(or(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    ));
    // 004cbdce je short 004CBDACh
    je(ctx, Cont(x004cbdd0), Cont(x004cbdac))
}

pub fn x004cbdc9(ctx: &mut Context) -> Cont {
    // 004cbdc9 mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbdcb inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdcc or al,al
    ctx.cpu.regs.set_al(or(
        ctx.cpu.regs.get_al(),
        ctx.cpu.regs.get_al(),
        &mut ctx.cpu.flags,
    ));
    // 004cbdce je short 004CBDACh
    je(ctx, Cont(x004cbdd0), Cont(x004cbdac))
}

pub fn x004cbdd0(ctx: &mut Context) -> Cont {
    // 004cbdd0 mov ecx,edi
    ctx.cpu.regs.ecx = ctx.cpu.regs.edi;
    // 004cbdd2 jns short 004CBDDBh
    jns(ctx, Cont(x004cbdd4), Cont(x004cbddb))
}

pub fn x004cbdd4(ctx: &mut Context) -> Cont {
    // 004cbdd4 movzx eax,word ptr [edi]
    ctx.cpu.regs.eax = ctx.memory.read::<u16>(ctx.cpu.regs.edi) as _;
    // 004cbdd7 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdd8 push eax
    push(ctx, ctx.cpu.regs.eax);
    // 004cbdd9 inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdda mov ecx,0AEF24857h
    ctx.cpu.regs.ecx = 0xaef24857u32;
    // 004cbddf push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004cbde0 call dword ptr [esi+0CB0E8h]
    let dst = indirect(
        ctx,
        ctx.memory.read(ctx.cpu.regs.esi.wrapping_add(0xcb0e8u32)),
    );
    call(ctx, 0x4cbde6, dst)
}

pub fn x004cbddb(ctx: &mut Context) -> Cont {
    // 004cbddb push edi
    push(ctx, ctx.cpu.regs.edi);
    // 004cbddc dec eax
    ctx.cpu.regs.eax = dec(ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbddd repne scasb
    rep(ctx, Rep::REPNE, scasb);
    // 004cbddf push ebp
    push(ctx, ctx.cpu.regs.ebp);
    // 004cbde0 call dword ptr [esi+0CB0E8h]
    let dst = indirect(
        ctx,
        ctx.memory.read(ctx.cpu.regs.esi.wrapping_add(0xcb0e8u32)),
    );
    call(ctx, 0x4cbde6, dst)
}

pub fn x004cbde6(ctx: &mut Context) -> Cont {
    // 004cbde6 or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbde8 je short 004CBDF1h
    je(ctx, Cont(x004cbdea), Cont(x004cbdf1))
}

pub fn x004cbdea(ctx: &mut Context) -> Cont {
    // 004cbdea mov [ebx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.ebx, ctx.cpu.regs.eax);
    // 004cbdec add ebx,4
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, 0x4u32, &mut ctx.cpu.flags);
    // 004cbdef jmp short 004CBDC9h
    Cont(x004cbdc9)
}

pub fn x004cbdf1(ctx: &mut Context) -> Cont {
    // 004cbdf1 popa
    popad(ctx);
    // 004cbdf2 ret
    ret(ctx, 0)
}

pub fn x004cbdf3(ctx: &mut Context) -> Cont {
    // 004cbdf3 add edi,4
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x4u32, &mut ctx.cpu.flags);
    // 004cbdf6 lea ebx,[esi-4]
    ctx.cpu.regs.ebx = ctx.cpu.regs.esi.wrapping_add(0xfffffffcu32);
    // 004cbdf9 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbdfb mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbdfd inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdfe or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbe00 je short 004CBE24h
    je(ctx, Cont(x004cbe02), Cont(x004cbe24))
}

pub fn x004cbdf9(ctx: &mut Context) -> Cont {
    // 004cbdf9 xor eax,eax
    ctx.cpu.regs.eax = xor(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbdfb mov al,[edi]
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(ctx.cpu.regs.edi));
    // 004cbdfd inc edi
    ctx.cpu.regs.edi = inc(ctx.cpu.regs.edi, &mut ctx.cpu.flags);
    // 004cbdfe or eax,eax
    ctx.cpu.regs.eax = or(ctx.cpu.regs.eax, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbe00 je short 004CBE24h
    je(ctx, Cont(x004cbe02), Cont(x004cbe24))
}

pub fn x004cbe02(ctx: &mut Context) -> Cont {
    // 004cbe02 cmp al,0EFh
    sub(ctx.cpu.regs.get_al(), 0xefu8, &mut ctx.cpu.flags);
    // 004cbe04 ja short 004CBE17h
    ja(ctx, Cont(x004cbe06), Cont(x004cbe17))
}

pub fn x004cbe06(ctx: &mut Context) -> Cont {
    // 004cbe06 add ebx,eax
    ctx.cpu.regs.ebx = add(ctx.cpu.regs.ebx, ctx.cpu.regs.eax, &mut ctx.cpu.flags);
    // 004cbe08 mov eax,[ebx]
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(ctx.cpu.regs.ebx);
    // 004cbe0a xchg al,ah
    let t = ctx.cpu.regs.get_ah();
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    ctx.cpu.regs.set_al(t);
    // 004cbe0c rol eax,10h
    ctx.cpu.regs.eax = rol(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 004cbe0f xchg al,ah
    let t = ctx.cpu.regs.get_ah();
    ctx.cpu.regs.set_ah(ctx.cpu.regs.get_al());
    ctx.cpu.regs.set_al(t);
    // 004cbe11 add eax,esi
    ctx.cpu.regs.eax = add(ctx.cpu.regs.eax, ctx.cpu.regs.esi, &mut ctx.cpu.flags);
    // 004cbe13 mov [ebx],eax
    ctx.memory.write::<u32>(ctx.cpu.regs.ebx, ctx.cpu.regs.eax);
    // 004cbe15 jmp short 004CBDF9h
    Cont(x004cbdf9)
}

pub fn x004cbe17(ctx: &mut Context) -> Cont {
    // 004cbe17 and al,0Fh
    ctx.cpu
        .regs
        .set_al(and(ctx.cpu.regs.get_al(), 0xfu8, &mut ctx.cpu.flags));
    // 004cbe19 shl eax,10h
    ctx.cpu.regs.eax = shl(ctx.cpu.regs.eax, 0x10u8, &mut ctx.cpu.flags);
    // 004cbe1c mov ax,[edi]
    ctx.cpu
        .regs
        .set_ax(ctx.memory.read::<u16>(ctx.cpu.regs.edi));
    // 004cbe1f add edi,2
    ctx.cpu.regs.edi = add(ctx.cpu.regs.edi, 0x2u32, &mut ctx.cpu.flags);
    // 004cbe22 jmp short 004CBE06h
    Cont(x004cbe06)
}

pub fn x004cbe24(ctx: &mut Context) -> Cont {
    // 004cbe24 popa
    popad(ctx);
    // 004cbe25 jmp near ptr 004085DDh
    Cont(x004085dd)
}

const BLOCKS: [(u32, fn(&mut Context) -> Cont); 231] = [
    (0x001000, kernel32::LoadLibraryA_stdcall),
    (0x001001, kernel32::GetProcAddress_stdcall),
    (0x001002, ddraw::DirectDrawCreate_stdcall),
    (0x001003, dsound::ordinal1_stdcall),
    (0x001004, user32::EndDialog_stdcall),
    (0x001005, winmm::timeSetEvent_stdcall),
    (0x001006, ddraw::IDirectDraw::QueryInterface_stdcall),
    (0x001007, ddraw::IDirectDraw::AddRef_stdcall),
    (0x001008, ddraw::IDirectDraw::Release_stdcall),
    (0x001009, ddraw::IDirectDraw::Compact_stdcall),
    (0x00100a, ddraw::IDirectDraw::CreateClipper_stdcall),
    (0x00100b, ddraw::IDirectDraw::CreatePalette_stdcall),
    (0x00100c, ddraw::IDirectDraw::CreateSurface_stdcall),
    (0x00100d, ddraw::IDirectDraw::DuplicateSurface_stdcall),
    (0x00100e, ddraw::IDirectDraw::EnumDisplayModes_stdcall),
    (0x00100f, ddraw::IDirectDraw::EnumSurfaces_stdcall),
    (0x001010, ddraw::IDirectDraw::FlipToGDISurface_stdcall),
    (0x001011, ddraw::IDirectDraw::GetCaps_stdcall),
    (0x001012, ddraw::IDirectDraw::GetDisplayMode_stdcall),
    (0x001013, ddraw::IDirectDraw::GetFourCCCodes_stdcall),
    (0x001014, ddraw::IDirectDraw::GetGDISurface_stdcall),
    (0x001015, ddraw::IDirectDraw::GetMonitorFrequency_stdcall),
    (0x001016, ddraw::IDirectDraw::GetScanLine_stdcall),
    (0x001017, ddraw::IDirectDraw::GetVerticalBlankStatus_stdcall),
    (0x001018, ddraw::IDirectDraw::Initialize_stdcall),
    (0x001019, ddraw::IDirectDraw::RestoreDisplayMode_stdcall),
    (0x00101a, ddraw::IDirectDraw::SetCooperativeLevel_stdcall),
    (0x00101b, ddraw::IDirectDraw::SetDisplayMode_stdcall),
    (0x00101c, ddraw::IDirectDraw::WaitForVerticalBlank_stdcall),
    (0x00101d, ddraw::IDirectDrawSurface::QueryInterface_stdcall),
    (0x00101e, ddraw::IDirectDrawSurface::AddRef_stdcall),
    (0x00101f, ddraw::IDirectDrawSurface::Release_stdcall),
    (
        0x001020,
        ddraw::IDirectDrawSurface::AddAttachedSurface_stdcall,
    ),
    (
        0x001021,
        ddraw::IDirectDrawSurface::AddOverlayDirtyRect_stdcall,
    ),
    (0x001022, ddraw::IDirectDrawSurface::Blt_stdcall),
    (0x001023, ddraw::IDirectDrawSurface::BltBatch_stdcall),
    (0x001024, ddraw::IDirectDrawSurface::BltFast_stdcall),
    (
        0x001025,
        ddraw::IDirectDrawSurface::DeleteAttachedSurface_stdcall,
    ),
    (
        0x001026,
        ddraw::IDirectDrawSurface::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x001027,
        ddraw::IDirectDrawSurface::EnumOverlayZOrders_stdcall,
    ),
    (0x001028, ddraw::IDirectDrawSurface::Flip_stdcall),
    (
        0x001029,
        ddraw::IDirectDrawSurface::GetAttachedSurface_stdcall,
    ),
    (0x00102a, ddraw::IDirectDrawSurface::GetBltStatus_stdcall),
    (0x00102b, ddraw::IDirectDrawSurface::GetCaps_stdcall),
    (0x00102c, ddraw::IDirectDrawSurface::GetClipper_stdcall),
    (0x00102d, ddraw::IDirectDrawSurface::GetColorKey_stdcall),
    (0x00102e, ddraw::IDirectDrawSurface::GetDC_stdcall),
    (0x00102f, ddraw::IDirectDrawSurface::GetFlipStatus_stdcall),
    (
        0x001030,
        ddraw::IDirectDrawSurface::GetOverlayPosition_stdcall,
    ),
    (0x001031, ddraw::IDirectDrawSurface::GetPalette_stdcall),
    (0x001032, ddraw::IDirectDrawSurface::GetPixelFormat_stdcall),
    (0x001033, ddraw::IDirectDrawSurface::GetSurfaceDesc_stdcall),
    (0x001034, ddraw::IDirectDrawSurface::Initialize_stdcall),
    (0x001035, ddraw::IDirectDrawSurface::IsLost_stdcall),
    (0x001036, ddraw::IDirectDrawSurface::Lock_stdcall),
    (0x001037, ddraw::IDirectDrawSurface::ReleaseDC_stdcall),
    (0x001038, ddraw::IDirectDrawSurface::Restore_stdcall),
    (0x001039, ddraw::IDirectDrawSurface::SetClipper_stdcall),
    (0x00103a, ddraw::IDirectDrawSurface::SetColorKey_stdcall),
    (
        0x00103b,
        ddraw::IDirectDrawSurface::SetOverlayPosition_stdcall,
    ),
    (0x00103c, ddraw::IDirectDrawSurface::SetPalette_stdcall),
    (0x00103d, ddraw::IDirectDrawSurface::Unlock_stdcall),
    (0x00103e, ddraw::IDirectDrawSurface::UpdateOverlay_stdcall),
    (
        0x00103f,
        ddraw::IDirectDrawSurface::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x001040,
        ddraw::IDirectDrawSurface::UpdateOverlayZOrder_stdcall,
    ),
    (0x001041, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0x001042, ddraw::IDirectDraw7::AddRef_stdcall),
    (0x001043, ddraw::IDirectDraw7::Release_stdcall),
    (0x001044, ddraw::IDirectDraw7::Compact_stdcall),
    (0x001045, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0x001046, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0x001047, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0x001048, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0x001049, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0x00104a, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0x00104b, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0x00104c, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0x00104d, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0x00104e, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0x00104f, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0x001050, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0x001051, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (
        0x001052,
        ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall,
    ),
    (0x001053, ddraw::IDirectDraw7::Initialize_stdcall),
    (0x001054, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0x001055, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0x001056, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (0x001057, ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall),
    (0x001058, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0x001059, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0x00105a, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (0x00105b, ddraw::IDirectDraw7::TestCooperativeLevel_stdcall),
    (0x00105c, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0x00105d, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0x00105e, ddraw::IDirectDraw7::EvaluateMode_stdcall),
    (0x00105f, ddraw::IDirectDrawSurface7::QueryInterface_stdcall),
    (0x001060, ddraw::IDirectDrawSurface7::AddRef_stdcall),
    (0x001061, ddraw::IDirectDrawSurface7::Release_stdcall),
    (
        0x001062,
        ddraw::IDirectDrawSurface7::AddAttachedSurface_stdcall,
    ),
    (
        0x001063,
        ddraw::IDirectDrawSurface7::AddOverlayDirtyRect_stdcall,
    ),
    (0x001064, ddraw::IDirectDrawSurface7::Blt_stdcall),
    (0x001065, ddraw::IDirectDrawSurface7::BltBatch_stdcall),
    (0x001066, ddraw::IDirectDrawSurface7::BltFast_stdcall),
    (
        0x001067,
        ddraw::IDirectDrawSurface7::DeleteAttachedSurface_stdcall,
    ),
    (
        0x001068,
        ddraw::IDirectDrawSurface7::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x001069,
        ddraw::IDirectDrawSurface7::EnumOverlayZOrders_stdcall,
    ),
    (0x00106a, ddraw::IDirectDrawSurface7::Flip_stdcall),
    (
        0x00106b,
        ddraw::IDirectDrawSurface7::GetAttachedSurface_stdcall,
    ),
    (0x00106c, ddraw::IDirectDrawSurface7::GetBltStatus_stdcall),
    (0x00106d, ddraw::IDirectDrawSurface7::GetCaps_stdcall),
    (0x00106e, ddraw::IDirectDrawSurface7::GetClipper_stdcall),
    (0x00106f, ddraw::IDirectDrawSurface7::GetColorKey_stdcall),
    (0x001070, ddraw::IDirectDrawSurface7::GetDC_stdcall),
    (0x001071, ddraw::IDirectDrawSurface7::GetFlipStatus_stdcall),
    (
        0x001072,
        ddraw::IDirectDrawSurface7::GetOverlayPosition_stdcall,
    ),
    (0x001073, ddraw::IDirectDrawSurface7::GetPalette_stdcall),
    (0x001074, ddraw::IDirectDrawSurface7::GetPixelFormat_stdcall),
    (0x001075, ddraw::IDirectDrawSurface7::GetSurfaceDesc_stdcall),
    (0x001076, ddraw::IDirectDrawSurface7::Initialize_stdcall),
    (0x001077, ddraw::IDirectDrawSurface7::IsLost_stdcall),
    (0x001078, ddraw::IDirectDrawSurface7::Lock_stdcall),
    (0x001079, ddraw::IDirectDrawSurface7::ReleaseDC_stdcall),
    (0x00107a, ddraw::IDirectDrawSurface7::Restore_stdcall),
    (0x00107b, ddraw::IDirectDrawSurface7::SetClipper_stdcall),
    (0x00107c, ddraw::IDirectDrawSurface7::SetColorKey_stdcall),
    (
        0x00107d,
        ddraw::IDirectDrawSurface7::SetOverlayPosition_stdcall,
    ),
    (0x00107e, ddraw::IDirectDrawSurface7::SetPalette_stdcall),
    (0x00107f, ddraw::IDirectDrawSurface7::Unlock_stdcall),
    (0x001080, ddraw::IDirectDrawSurface7::UpdateOverlay_stdcall),
    (
        0x001081,
        ddraw::IDirectDrawSurface7::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x001082,
        ddraw::IDirectDrawSurface7::UpdateOverlayZOrder_stdcall,
    ),
    (0x001083, ddraw::IDirectDrawSurface7::GetDDInterface_stdcall),
    (0x001084, ddraw::IDirectDrawSurface7::PageLock_stdcall),
    (0x001085, ddraw::IDirectDrawSurface7::PageUnlock_stdcall),
    (0x001086, ddraw::IDirectDrawSurface7::SetSurfaceDesc_stdcall),
    (0x001087, ddraw::IDirectDrawSurface7::SetPrivateData_stdcall),
    (0x001088, ddraw::IDirectDrawSurface7::GetPrivateData_stdcall),
    (
        0x001089,
        ddraw::IDirectDrawSurface7::FreePrivateData_stdcall,
    ),
    (
        0x00108a,
        ddraw::IDirectDrawSurface7::GetUniquenessValue_stdcall,
    ),
    (
        0x00108b,
        ddraw::IDirectDrawSurface7::ChangeUniquenessValue_stdcall,
    ),
    (0x00108c, ddraw::IDirectDrawSurface7::SetPriority_stdcall),
    (0x00108d, ddraw::IDirectDrawSurface7::GetPriority_stdcall),
    (0x00108e, ddraw::IDirectDrawSurface7::SetLOD_stdcall),
    (0x00108f, ddraw::IDirectDrawSurface7::GetLOD_stdcall),
    (0x001090, dsound::IDirectSound::QueryInterface_stdcall),
    (0x001091, dsound::IDirectSound::AddRef_stdcall),
    (0x001092, dsound::IDirectSound::Release_stdcall),
    (0x001093, dsound::IDirectSound::CreateSoundBuffer_stdcall),
    (0x001094, dsound::IDirectSound::GetCaps_stdcall),
    (0x001095, dsound::IDirectSound::DuplicateSoundBuffer_stdcall),
    (0x001096, dsound::IDirectSound::SetCooperativeLevel_stdcall),
    (0x001097, dsound::IDirectSound::Compact_stdcall),
    (0x001098, dsound::IDirectSound::GetSpeakerConfig_stdcall),
    (0x001099, dsound::IDirectSound::SetSpeakerConfig_stdcall),
    (0x00109a, dsound::IDirectSound::Initialize_stdcall),
    (0x00109b, dsound::IDirectSoundBuffer::QueryInterface_stdcall),
    (0x00109c, dsound::IDirectSoundBuffer::AddRef_stdcall),
    (0x00109d, dsound::IDirectSoundBuffer::Release_stdcall),
    (0x00109e, dsound::IDirectSoundBuffer::GetCaps_stdcall),
    (
        0x00109f,
        dsound::IDirectSoundBuffer::GetCurrentPosition_stdcall,
    ),
    (0x0010a0, dsound::IDirectSoundBuffer::GetFormat_stdcall),
    (0x0010a1, dsound::IDirectSoundBuffer::GetVolume_stdcall),
    (0x0010a2, dsound::IDirectSoundBuffer::GetPan_stdcall),
    (0x0010a3, dsound::IDirectSoundBuffer::GetFrequency_stdcall),
    (0x0010a4, dsound::IDirectSoundBuffer::GetStatus_stdcall),
    (0x0010a5, dsound::IDirectSoundBuffer::Initialize_stdcall),
    (0x0010a6, dsound::IDirectSoundBuffer::Lock_stdcall),
    (0x0010a7, dsound::IDirectSoundBuffer::Play_stdcall),
    (
        0x0010a8,
        dsound::IDirectSoundBuffer::SetCurrentPosition_stdcall,
    ),
    (0x0010a9, dsound::IDirectSoundBuffer::SetFormat_stdcall),
    (0x0010aa, dsound::IDirectSoundBuffer::SetVolume_stdcall),
    (0x0010ab, dsound::IDirectSoundBuffer::SetPan_stdcall),
    (0x0010ac, dsound::IDirectSoundBuffer::SetFrequency_stdcall),
    (0x0010ad, dsound::IDirectSoundBuffer::Stop_stdcall),
    (0x0010ae, dsound::IDirectSoundBuffer::Unlock_stdcall),
    (0x0010af, dsound::IDirectSoundBuffer::Restore_stdcall),
    (0x4085dd, x004085dd),
    (0x4cbca0, x004cbca0),
    (0x4cbcb8, x004cbcb8),
    (0x4cbcbe, x004cbcbe),
    (0x4cbcc2, x004cbcc2),
    (0x4cbcc9, x004cbcc9),
    (0x4cbccb, x004cbccb),
    (0x4cbcd0, x004cbcd0),
    (0x4cbcd4, x004cbcd4),
    (0x4cbcdb, x004cbcdb),
    (0x4cbce1, x004cbce1),
    (0x4cbce3, x004cbce3),
    (0x4cbcec, x004cbcec),
    (0x4cbcf3, x004cbcf3),
    (0x4cbcfe, x004cbcfe),
    (0x4cbd00, x004cbd00),
    (0x4cbd04, x004cbd04),
    (0x4cbd0b, x004cbd0b),
    (0x4cbd11, x004cbd11),
    (0x4cbd18, x004cbd18),
    (0x4cbd1c, x004cbd1c),
    (0x4cbd1d, x004cbd1d),
    (0x4cbd21, x004cbd21),
    (0x4cbd28, x004cbd28),
    (0x4cbd2e, x004cbd2e),
    (0x4cbd30, x004cbd30),
    (0x4cbd39, x004cbd39),
    (0x4cbd3c, x004cbd3c),
    (0x4cbd4d, x004cbd4d),
    (0x4cbd56, x004cbd56),
    (0x4cbd5c, x004cbd5c),
    (0x4cbd6b, x004cbd6b),
    (0x4cbd72, x004cbd72),
    (0x4cbd7a, x004cbd7a),
    (0x4cbd7f, x004cbd7f),
    (0x4cbd83, x004cbd83),
    (0x4cbd88, x004cbd88),
    (0x4cbda6, x004cbda6),
    (0x4cbdac, x004cbdac),
    (0x4cbdb2, x004cbdb2),
    (0x4cbdc8, x004cbdc8),
    (0x4cbdc9, x004cbdc9),
    (0x4cbdd0, x004cbdd0),
    (0x4cbdd4, x004cbdd4),
    (0x4cbddb, x004cbddb),
    (0x4cbde6, x004cbde6),
    (0x4cbdea, x004cbdea),
    (0x4cbdf1, x004cbdf1),
    (0x4cbdf3, x004cbdf3),
    (0x4cbdf9, x004cbdf9),
    (0x4cbe02, x004cbe02),
    (0x4cbe06, x004cbe06),
    (0x4cbe17, x004cbe17),
    (0x4cbe24, x004cbe24),
    (runtime::RETURN_FROM_X86_ADDR, runtime::return_from_x86),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0x4cc000..0x4cc06c,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x004cbca0),
};
