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
    mappings.alloc(".data".to_string(), Some(0x403000), 0x1000);
    let bytes = include_bytes!("../data/00403000.raw").as_slice();
    let out = &mut m.memory.bytes[0x403000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
    mappings.alloc(".reloc".to_string(), Some(0x404000), 0x1000);
    let bytes = include_bytes!("../data/00404000.raw").as_slice();
    let out = &mut m.memory.bytes[0x404000 as usize..][..bytes.len()];
    out.copy_from_slice(bytes);
}
#[allow(unused_variables)]
pub fn x00401000(m: &mut Machine) -> Cont {
    // 00401000 push ebp
    push(m, m.regs.ebp);
    // 00401001 push ebx
    push(m, m.regs.ebx);
    // 00401002 push edi
    push(m, m.regs.edi);
    // 00401003 push esi
    push(m, m.regs.esi);
    // 00401004 sub esp,408h
    m.regs.esp = sub(m.regs.esp, 0x408u32, &mut m.flags);
    // 0040100a mov esi,ecx
    m.regs.esi = m.regs.ecx;
    // 0040100c cmp dword ptr [ecx+4],0
    sub(
        m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x4u32)),
        0x0u32,
        &mut m.flags,
    );
    // 00401010 jle near ptr 00401170h
    jle(m, Cont(x00401016), Cont(x00401170))
}

#[allow(unused_variables)]
pub fn x00401016(m: &mut Machine) -> Cont {
    // 00401016 xor ebx,ebx
    m.regs.ebx = xor(m.regs.ebx, m.regs.ebx, &mut m.flags);
    // 00401018 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(m, 0x40101e, dst)
}

#[allow(unused_variables)]
pub fn x00401018(m: &mut Machine) -> Cont {
    // 00401018 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(m, 0x40101e, dst)
}

#[allow(unused_variables)]
pub fn x0040101e(m: &mut Machine) -> Cont {
    // 0040101e mov edi,eax
    m.regs.edi = m.regs.eax;
    // 00401020 mov ebp,esi
    m.regs.ebp = m.regs.esi;
    // 00401022 push dword ptr [esi+8]
    push(m, m.memory.read::<u32>(m.regs.esi.wrapping_add(0x8u32)));
    // 00401025 call dword ptr ds:[402198h]
    let dst = Cont(kernel32::TlsGetValue_stdcall);
    call(m, 0x40102b, dst)
}

#[allow(unused_variables)]
pub fn x0040102b(m: &mut Machine) -> Cont {
    // 0040102b mov [esp],eax
    m.memory.write::<u32>(m.regs.esp, m.regs.eax);
    // 0040102e mov dword ptr [esp+404h],0
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), 0x0u32);
    // 00401039 mov eax,0FFFFFFF6h
    m.regs.eax = 0xfffffff6u32;
    // 0040103e mov cl,[eax+402112h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402112u32)));
    // 00401044 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 0040104b lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 0040104e mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401055 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401059 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 0040105a jne short 0040103Eh
    jne(m, Cont(x0040105c), Cont(x0040103e))
}

#[allow(unused_variables)]
pub fn x0040103e(m: &mut Machine) -> Cont {
    // 0040103e mov cl,[eax+402112h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402112u32)));
    // 00401044 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 0040104b lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 0040104e mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401055 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401059 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 0040105a jne short 0040103Eh
    jne(m, Cont(x0040105c), Cont(x0040103e))
}

#[allow(unused_variables)]
pub fn x0040105c(m: &mut Machine) -> Cont {
    // 0040105c lea ecx,[esp+4]
    m.regs.ecx = m.regs.esp.wrapping_add(0x4u32);
    // 00401060 mov edx,edi
    m.regs.edx = m.regs.edi;
    // 00401062 call 004012A4h
    let dst = Cont(x004012a4);
    call(m, 0x401067, dst)
}

#[allow(unused_variables)]
pub fn x00401067(m: &mut Machine) -> Cont {
    // 00401067 mov eax,0FFFFFFFAh
    m.regs.eax = 0xfffffffau32;
    // 0040106c mov cl,[eax+402107h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402107u32)));
    // 00401072 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401079 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 0040107c mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401083 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401087 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401088 jne short 0040106Ch
    jne(m, Cont(x0040108a), Cont(x0040106c))
}

#[allow(unused_variables)]
pub fn x0040106c(m: &mut Machine) -> Cont {
    // 0040106c mov cl,[eax+402107h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402107u32)));
    // 00401072 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401079 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 0040107c mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401083 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401087 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401088 jne short 0040106Ch
    jne(m, Cont(x0040108a), Cont(x0040106c))
}

#[allow(unused_variables)]
pub fn x0040108a(m: &mut Machine) -> Cont {
    // 0040108a mov eax,[ebp]
    m.regs.eax = m.memory.read::<u32>(m.regs.ebp);
    // 0040108d cmp byte ptr [eax],0
    sub(m.memory.read::<u8>(m.regs.eax), 0x0u8, &mut m.flags);
    // 00401090 je short 004010C2h
    je(m, Cont(x00401092), Cont(x004010c2))
}

#[allow(unused_variables)]
pub fn x00401092(m: &mut Machine) -> Cont {
    // 00401092 xor edx,edx
    m.regs.edx = xor(m.regs.edx, m.regs.edx, &mut m.flags);
    // 00401094 inc edx
    m.regs.edx = inc(m.regs.edx, &mut m.flags);
    // 00401095 xor edi,edi
    m.regs.edi = xor(m.regs.edi, m.regs.edi, &mut m.flags);
    // 00401097 dec edi
    m.regs.edi = dec(m.regs.edi, &mut m.flags);
    // 00401098 cmp byte ptr [eax+edi+2],0
    sub(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.edi).wrapping_add(0x2u32)),
        0x0u8,
        &mut m.flags,
    );
    // 0040109d mov ecx,edx
    m.regs.ecx = m.regs.edx;
    // 0040109f lea edi,[edi+1]
    m.regs.edi = m.regs.edi.wrapping_add(0x1u32);
    // 004010a2 lea edx,[edx+1]
    m.regs.edx = m.regs.edx.wrapping_add(0x1u32);
    // 004010a5 jne short 00401098h
    jne(m, Cont(x004010a7), Cont(x00401098))
}

#[allow(unused_variables)]
pub fn x00401098(m: &mut Machine) -> Cont {
    // 00401098 cmp byte ptr [eax+edi+2],0
    sub(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.edi).wrapping_add(0x2u32)),
        0x0u8,
        &mut m.flags,
    );
    // 0040109d mov ecx,edx
    m.regs.ecx = m.regs.edx;
    // 0040109f lea edi,[edi+1]
    m.regs.edi = m.regs.edi.wrapping_add(0x1u32);
    // 004010a2 lea edx,[edx+1]
    m.regs.edx = m.regs.edx.wrapping_add(0x1u32);
    // 004010a5 jne short 00401098h
    jne(m, Cont(x004010a7), Cont(x00401098))
}

#[allow(unused_variables)]
pub fn x004010a7(m: &mut Machine) -> Cont {
    // 004010a7 mov dl,[eax]
    m.regs.set_dl(m.memory.read::<u8>(m.regs.eax));
    // 004010a9 mov esi,[esp+404h]
    m.regs.esi = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004010b0 lea edi,[esi+1]
    m.regs.edi = m.regs.esi.wrapping_add(0x1u32);
    // 004010b3 mov [esp+404h],edi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.edi);
    // 004010ba mov [esp+esi+4],dl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.esi).wrapping_add(0x4u32),
        m.regs.get_dl(),
    );
    // 004010be inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004010bf dec ecx
    m.regs.ecx = dec(m.regs.ecx, &mut m.flags);
    // 004010c0 jne short 004010A7h
    jne(m, Cont(x004010c2), Cont(x004010a7))
}

#[allow(unused_variables)]
pub fn x004010c2(m: &mut Machine) -> Cont {
    // 004010c2 mov eax,0FFFFFFFBh
    m.regs.eax = 0xfffffffbu32;
    // 004010c7 mov cl,[eax+4020FCh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020fcu32)));
    // 004010cd mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004010d4 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 004010d7 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 004010de mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004010e2 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004010e3 jne short 004010C7h
    jne(m, Cont(x004010e5), Cont(x004010c7))
}

#[allow(unused_variables)]
pub fn x004010c7(m: &mut Machine) -> Cont {
    // 004010c7 mov cl,[eax+4020FCh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020fcu32)));
    // 004010cd mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004010d4 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 004010d7 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 004010de mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004010e2 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004010e3 jne short 004010C7h
    jne(m, Cont(x004010e5), Cont(x004010c7))
}

#[allow(unused_variables)]
pub fn x004010e5(m: &mut Machine) -> Cont {
    // 004010e5 lea ecx,[esp+4]
    m.regs.ecx = m.regs.esp.wrapping_add(0x4u32);
    // 004010e9 mov edx,[esp]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp);
    // 004010ec call 004012A4h
    let dst = Cont(x004012a4);
    call(m, 0x4010f1, dst)
}

#[allow(unused_variables)]
pub fn x004010f1(m: &mut Machine) -> Cont {
    // 004010f1 mov eax,0FFFFFFFDh
    m.regs.eax = 0xfffffffdu32;
    // 004010f6 mov cl,[eax+402100h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402100u32)));
    // 004010fc mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401103 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 00401106 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 0040110d mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401111 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401112 jne short 004010F6h
    jne(m, Cont(x00401114), Cont(x004010f6))
}

#[allow(unused_variables)]
pub fn x004010f6(m: &mut Machine) -> Cont {
    // 004010f6 mov cl,[eax+402100h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402100u32)));
    // 004010fc mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401103 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 00401106 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 0040110d mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401111 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401112 jne short 004010F6h
    jne(m, Cont(x00401114), Cont(x004010f6))
}

#[allow(unused_variables)]
pub fn x00401114(m: &mut Machine) -> Cont {
    // 00401114 lea edi,[esp+4]
    m.regs.edi = m.regs.esp.wrapping_add(0x4u32);
    // 00401118 mov ecx,edi
    m.regs.ecx = m.regs.edi;
    // 0040111a mov edx,ebx
    m.regs.edx = m.regs.ebx;
    // 0040111c call 004012A4h
    let dst = Cont(x004012a4);
    call(m, 0x401121, dst)
}

#[allow(unused_variables)]
pub fn x00401121(m: &mut Machine) -> Cont {
    // 00401121 mov eax,[esp+404h]
    m.regs.eax = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401128 lea ecx,[eax+1]
    m.regs.ecx = m.regs.eax.wrapping_add(0x1u32);
    // 0040112b mov [esp+404h],ecx
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ecx);
    // 00401132 mov byte ptr [esp+eax+4],0Ah
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.eax).wrapping_add(0x4u32),
        0xau8,
    );
    // 00401137 mov esi,[esp+404h]
    m.regs.esi = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 0040113e push 0FFFFFFF5h
    push(m, 0xfffffff5u32);
    // 00401140 call dword ptr ds:[40218Ch]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(m, 0x401146, dst)
}

#[allow(unused_variables)]
pub fn x00401146(m: &mut Machine) -> Cont {
    // 00401146 xor ecx,ecx
    m.regs.ecx = xor(m.regs.ecx, m.regs.ecx, &mut m.flags);
    // 00401148 push ecx
    push(m, m.regs.ecx);
    // 00401149 push ecx
    push(m, m.regs.ecx);
    // 0040114a push esi
    push(m, m.regs.esi);
    // 0040114b push edi
    push(m, m.regs.edi);
    // 0040114c push eax
    push(m, m.regs.eax);
    // 0040114d call dword ptr ds:[4021A0h]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(m, 0x401153, dst)
}

#[allow(unused_variables)]
pub fn x00401153(m: &mut Machine) -> Cont {
    // 00401153 mov eax,3E8h
    m.regs.eax = 0x3e8u32;
    // 00401158 xor edx,edx
    m.regs.edx = xor(m.regs.edx, m.regs.edx, &mut m.flags);
    // 0040115a mov esi,ebp
    m.regs.esi = m.regs.ebp;
    // 0040115c idiv dword ptr [ebp+4]
    let x = (((m.regs.edx as u64) << 32) | (m.regs.eax as u64)) as i64;
    let y = m.memory.read::<u32>(m.regs.ebp.wrapping_add(0x4u32)) as i64;
    m.regs.eax = (x / y) as i32 as u32;
    m.regs.edx = (x % y) as i32 as u32;
    // 0040115f push eax
    push(m, m.regs.eax);
    // 00401160 call dword ptr ds:[402190h]
    let dst = Cont(kernel32::Sleep_stdcall);
    call(m, 0x401166, dst)
}

#[allow(unused_variables)]
pub fn x00401166(m: &mut Machine) -> Cont {
    // 00401166 inc ebx
    m.regs.ebx = inc(m.regs.ebx, &mut m.flags);
    // 00401167 cmp ebx,[ebp+4]
    sub(
        m.regs.ebx,
        m.memory.read::<u32>(m.regs.ebp.wrapping_add(0x4u32)),
        &mut m.flags,
    );
    // 0040116a jl near ptr 00401018h
    jl(m, Cont(x00401170), Cont(x00401018))
}

#[allow(unused_variables)]
pub fn x00401170(m: &mut Machine) -> Cont {
    // 00401170 call dword ptr ds:[402188h]
    let dst = Cont(kernel32::GetCurrentThreadId_stdcall);
    call(m, 0x401176, dst)
}

#[allow(unused_variables)]
pub fn x00401176(m: &mut Machine) -> Cont {
    // 00401176 mov ebx,eax
    m.regs.ebx = m.regs.eax;
    // 00401178 push dword ptr [esi+8]
    push(m, m.memory.read::<u32>(m.regs.esi.wrapping_add(0x8u32)));
    // 0040117b call dword ptr ds:[402198h]
    let dst = Cont(kernel32::TlsGetValue_stdcall);
    call(m, 0x401181, dst)
}

#[allow(unused_variables)]
pub fn x00401181(m: &mut Machine) -> Cont {
    // 00401181 mov edi,eax
    m.regs.edi = m.regs.eax;
    // 00401183 mov dword ptr [esp+404h],0
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), 0x0u32);
    // 0040118e mov eax,0FFFFFFF6h
    m.regs.eax = 0xfffffff6u32;
    // 00401193 mov cl,[eax+402112h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402112u32)));
    // 00401199 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004011a0 lea ebp,[edx+1]
    m.regs.ebp = m.regs.edx.wrapping_add(0x1u32);
    // 004011a3 mov [esp+404h],ebp
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebp);
    // 004011aa mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004011ae inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004011af jne short 00401193h
    jne(m, Cont(x004011b1), Cont(x00401193))
}

#[allow(unused_variables)]
pub fn x00401193(m: &mut Machine) -> Cont {
    // 00401193 mov cl,[eax+402112h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402112u32)));
    // 00401199 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004011a0 lea ebp,[edx+1]
    m.regs.ebp = m.regs.edx.wrapping_add(0x1u32);
    // 004011a3 mov [esp+404h],ebp
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebp);
    // 004011aa mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004011ae inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004011af jne short 00401193h
    jne(m, Cont(x004011b1), Cont(x00401193))
}

#[allow(unused_variables)]
pub fn x004011b1(m: &mut Machine) -> Cont {
    // 004011b1 lea ecx,[esp+4]
    m.regs.ecx = m.regs.esp.wrapping_add(0x4u32);
    // 004011b5 mov edx,ebx
    m.regs.edx = m.regs.ebx;
    // 004011b7 call 004012A4h
    let dst = Cont(x004012a4);
    call(m, 0x4011bc, dst)
}

#[allow(unused_variables)]
pub fn x004011bc(m: &mut Machine) -> Cont {
    // 004011bc mov eax,0FFFFFFFAh
    m.regs.eax = 0xfffffffau32;
    // 004011c1 mov cl,[eax+402107h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402107u32)));
    // 004011c7 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004011ce lea ebx,[edx+1]
    m.regs.ebx = m.regs.edx.wrapping_add(0x1u32);
    // 004011d1 mov [esp+404h],ebx
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebx);
    // 004011d8 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004011dc inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004011dd jne short 004011C1h
    jne(m, Cont(x004011df), Cont(x004011c1))
}

#[allow(unused_variables)]
pub fn x004011c1(m: &mut Machine) -> Cont {
    // 004011c1 mov cl,[eax+402107h]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x402107u32)));
    // 004011c7 mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 004011ce lea ebx,[edx+1]
    m.regs.ebx = m.regs.edx.wrapping_add(0x1u32);
    // 004011d1 mov [esp+404h],ebx
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebx);
    // 004011d8 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 004011dc inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 004011dd jne short 004011C1h
    jne(m, Cont(x004011df), Cont(x004011c1))
}

#[allow(unused_variables)]
pub fn x004011df(m: &mut Machine) -> Cont {
    // 004011df mov eax,[esi]
    m.regs.eax = m.memory.read::<u32>(m.regs.esi);
    // 004011e1 cmp byte ptr [eax],0
    sub(m.memory.read::<u8>(m.regs.eax), 0x0u8, &mut m.flags);
    // 004011e4 je short 00401214h
    je(m, Cont(x004011e6), Cont(x00401214))
}

#[allow(unused_variables)]
pub fn x004011e6(m: &mut Machine) -> Cont {
    // 004011e6 xor ecx,ecx
    m.regs.ecx = xor(m.regs.ecx, m.regs.ecx, &mut m.flags);
    // 004011e8 dec ecx
    m.regs.ecx = dec(m.regs.ecx, &mut m.flags);
    // 004011e9 cmp byte ptr [eax+ecx+2],0
    sub(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.ecx).wrapping_add(0x2u32)),
        0x0u8,
        &mut m.flags,
    );
    // 004011ee lea ecx,[ecx+1]
    m.regs.ecx = m.regs.ecx.wrapping_add(0x1u32);
    // 004011f1 jne short 004011E9h
    jne(m, Cont(x004011f3), Cont(x004011e9))
}

#[allow(unused_variables)]
pub fn x004011e9(m: &mut Machine) -> Cont {
    // 004011e9 cmp byte ptr [eax+ecx+2],0
    sub(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.ecx).wrapping_add(0x2u32)),
        0x0u8,
        &mut m.flags,
    );
    // 004011ee lea ecx,[ecx+1]
    m.regs.ecx = m.regs.ecx.wrapping_add(0x1u32);
    // 004011f1 jne short 004011E9h
    jne(m, Cont(x004011f3), Cont(x004011e9))
}

#[allow(unused_variables)]
pub fn x004011f3(m: &mut Machine) -> Cont {
    // 004011f3 xor edx,edx
    m.regs.edx = xor(m.regs.edx, m.regs.edx, &mut m.flags);
    // 004011f5 dec edx
    m.regs.edx = dec(m.regs.edx, &mut m.flags);
    // 004011f6 mov bl,[eax+edx+1]
    m.regs.set_bl(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.edx).wrapping_add(0x1u32)),
    );
    // 004011fa mov esi,[esp+404h]
    m.regs.esi = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401201 lea ebp,[esi+1]
    m.regs.ebp = m.regs.esi.wrapping_add(0x1u32);
    // 00401204 mov [esp+404h],ebp
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebp);
    // 0040120b mov [esp+esi+4],bl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.esi).wrapping_add(0x4u32),
        m.regs.get_bl(),
    );
    // 0040120f inc edx
    m.regs.edx = inc(m.regs.edx, &mut m.flags);
    // 00401210 cmp ecx,edx
    sub(m.regs.ecx, m.regs.edx, &mut m.flags);
    // 00401212 jne short 004011F6h
    jne(m, Cont(x00401214), Cont(x004011f6))
}

#[allow(unused_variables)]
pub fn x004011f6(m: &mut Machine) -> Cont {
    // 004011f6 mov bl,[eax+edx+1]
    m.regs.set_bl(
        m.memory
            .read::<u8>(m.regs.eax.wrapping_add(m.regs.edx).wrapping_add(0x1u32)),
    );
    // 004011fa mov esi,[esp+404h]
    m.regs.esi = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401201 lea ebp,[esi+1]
    m.regs.ebp = m.regs.esi.wrapping_add(0x1u32);
    // 00401204 mov [esp+404h],ebp
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ebp);
    // 0040120b mov [esp+esi+4],bl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.esi).wrapping_add(0x4u32),
        m.regs.get_bl(),
    );
    // 0040120f inc edx
    m.regs.edx = inc(m.regs.edx, &mut m.flags);
    // 00401210 cmp ecx,edx
    sub(m.regs.ecx, m.regs.edx, &mut m.flags);
    // 00401212 jne short 004011F6h
    jne(m, Cont(x00401214), Cont(x004011f6))
}

#[allow(unused_variables)]
pub fn x00401214(m: &mut Machine) -> Cont {
    // 00401214 mov eax,0FFFFFFFBh
    m.regs.eax = 0xfffffffbu32;
    // 00401219 mov cl,[eax+4020FCh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020fcu32)));
    // 0040121f mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401226 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 00401229 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401230 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401234 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401235 jne short 00401219h
    jne(m, Cont(x00401237), Cont(x00401219))
}

#[allow(unused_variables)]
pub fn x00401219(m: &mut Machine) -> Cont {
    // 00401219 mov cl,[eax+4020FCh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020fcu32)));
    // 0040121f mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401226 lea esi,[edx+1]
    m.regs.esi = m.regs.edx.wrapping_add(0x1u32);
    // 00401229 mov [esp+404h],esi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.esi);
    // 00401230 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401234 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401235 jne short 00401219h
    jne(m, Cont(x00401237), Cont(x00401219))
}

#[allow(unused_variables)]
pub fn x00401237(m: &mut Machine) -> Cont {
    // 00401237 lea esi,[esp+4]
    m.regs.esi = m.regs.esp.wrapping_add(0x4u32);
    // 0040123b mov ecx,esi
    m.regs.ecx = m.regs.esi;
    // 0040123d mov edx,edi
    m.regs.edx = m.regs.edi;
    // 0040123f call 004012A4h
    let dst = Cont(x004012a4);
    call(m, 0x401244, dst)
}

#[allow(unused_variables)]
pub fn x00401244(m: &mut Machine) -> Cont {
    // 00401244 mov eax,0FFFFFFF6h
    m.regs.eax = 0xfffffff6u32;
    // 00401249 mov cl,[eax+4020EAh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020eau32)));
    // 0040124f mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401256 lea edi,[edx+1]
    m.regs.edi = m.regs.edx.wrapping_add(0x1u32);
    // 00401259 mov [esp+404h],edi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.edi);
    // 00401260 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401264 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401265 jne short 00401249h
    jne(m, Cont(x00401267), Cont(x00401249))
}

#[allow(unused_variables)]
pub fn x00401249(m: &mut Machine) -> Cont {
    // 00401249 mov cl,[eax+4020EAh]
    m.regs
        .set_cl(m.memory.read::<u8>(m.regs.eax.wrapping_add(0x4020eau32)));
    // 0040124f mov edx,[esp+404h]
    m.regs.edx = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401256 lea edi,[edx+1]
    m.regs.edi = m.regs.edx.wrapping_add(0x1u32);
    // 00401259 mov [esp+404h],edi
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.edi);
    // 00401260 mov [esp+edx+4],cl
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.edx).wrapping_add(0x4u32),
        m.regs.get_cl(),
    );
    // 00401264 inc eax
    m.regs.eax = inc(m.regs.eax, &mut m.flags);
    // 00401265 jne short 00401249h
    jne(m, Cont(x00401267), Cont(x00401249))
}

#[allow(unused_variables)]
pub fn x00401267(m: &mut Machine) -> Cont {
    // 00401267 mov eax,[esp+404h]
    m.regs.eax = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 0040126e lea ecx,[eax+1]
    m.regs.ecx = m.regs.eax.wrapping_add(0x1u32);
    // 00401271 mov [esp+404h],ecx
    m.memory
        .write::<u32>(m.regs.esp.wrapping_add(0x404u32), m.regs.ecx);
    // 00401278 mov byte ptr [esp+eax+4],0Ah
    m.memory.write::<u8>(
        m.regs.esp.wrapping_add(m.regs.eax).wrapping_add(0x4u32),
        0xau8,
    );
    // 0040127d mov edi,[esp+404h]
    m.regs.edi = m.memory.read::<u32>(m.regs.esp.wrapping_add(0x404u32));
    // 00401284 push 0FFFFFFF5h
    push(m, 0xfffffff5u32);
    // 00401286 call dword ptr ds:[40218Ch]
    let dst = Cont(kernel32::GetStdHandle_stdcall);
    call(m, 0x40128c, dst)
}

#[allow(unused_variables)]
pub fn x0040128c(m: &mut Machine) -> Cont {
    // 0040128c xor ecx,ecx
    m.regs.ecx = xor(m.regs.ecx, m.regs.ecx, &mut m.flags);
    // 0040128e push ecx
    push(m, m.regs.ecx);
    // 0040128f push ecx
    push(m, m.regs.ecx);
    // 00401290 push edi
    push(m, m.regs.edi);
    // 00401291 push esi
    push(m, m.regs.esi);
    // 00401292 push eax
    push(m, m.regs.eax);
    // 00401293 call dword ptr ds:[4021A0h]
    let dst = Cont(kernel32::WriteFile_stdcall);
    call(m, 0x401299, dst)
}

#[allow(unused_variables)]
pub fn x00401299(m: &mut Machine) -> Cont {
    // 00401299 add esp,408h
    m.regs.esp = add(m.regs.esp, 0x408u32, &mut m.flags);
    // 0040129f pop esi
    m.regs.esi = pop(m);
    // 004012a0 pop edi
    m.regs.edi = pop(m);
    // 004012a1 pop ebx
    m.regs.ebx = pop(m);
    // 004012a2 pop ebp
    m.regs.ebp = pop(m);
    // 004012a3 ret
    ret(m, 0)
}

#[allow(unused_variables)]
pub fn x004012a4(m: &mut Machine) -> Cont {
    // 004012a4 push ebp
    push(m, m.regs.ebp);
    // 004012a5 push ebx
    push(m, m.regs.ebx);
    // 004012a6 push edi
    push(m, m.regs.edi);
    // 004012a7 push esi
    push(m, m.regs.esi);
    // 004012a8 mov esi,edx
    m.regs.esi = m.regs.edx;
    // 004012aa test edx,edx
    and(m.regs.edx, m.regs.edx, &mut m.flags);
    // 004012ac jns short 004012C3h
    jns(m, Cont(x004012ae), Cont(x004012c3))
}

#[allow(unused_variables)]
pub fn x004012ae(m: &mut Machine) -> Cont {
    // 004012ae mov eax,[ecx+400h]
    m.regs.eax = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012b4 lea edx,[eax+1]
    m.regs.edx = m.regs.eax.wrapping_add(0x1u32);
    // 004012b7 mov [ecx+400h],edx
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x400u32), m.regs.edx);
    // 004012bd mov byte ptr [ecx+eax],2Dh
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.eax), 0x2du8);
    // 004012c1 neg esi
    m.regs.esi = neg(m.regs.esi, &mut m.flags);
    // 004012c3 mov edi,[ecx+400h]
    m.regs.edi = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012c9 mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    m.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    todo!();
    // 004012d2 shr edx,3
    m.regs.edx = shr(m.regs.edx, 0x3u8, &mut m.flags);
    // 004012d5 lea eax,[edx+edx]
    m.regs.eax = m.regs.edx.wrapping_add(m.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    m.regs.ebp = m.regs.eax.wrapping_add((m.regs.eax * 4));
    // 004012db mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012dd sub eax,ebp
    m.regs.eax = sub(m.regs.eax, m.regs.ebp, &mut m.flags);
    // 004012df or al,30h
    m.regs.set_al(or(m.regs.get_al(), 0x30u8, &mut m.flags));
    // 004012e1 mov ebp,[ecx+400h]
    m.regs.ebp = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    m.regs.ebx = m.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x400u32), m.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.ebp), m.regs.get_al());
    // 004012f3 cmp esi,9
    sub(m.regs.esi, 0x9u32, &mut m.flags);
    // 004012f6 mov esi,edx
    m.regs.esi = m.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(m, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012c3(m: &mut Machine) -> Cont {
    // 004012c3 mov edi,[ecx+400h]
    m.regs.edi = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012c9 mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    m.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    todo!();
    // 004012d2 shr edx,3
    m.regs.edx = shr(m.regs.edx, 0x3u8, &mut m.flags);
    // 004012d5 lea eax,[edx+edx]
    m.regs.eax = m.regs.edx.wrapping_add(m.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    m.regs.ebp = m.regs.eax.wrapping_add((m.regs.eax * 4));
    // 004012db mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012dd sub eax,ebp
    m.regs.eax = sub(m.regs.eax, m.regs.ebp, &mut m.flags);
    // 004012df or al,30h
    m.regs.set_al(or(m.regs.get_al(), 0x30u8, &mut m.flags));
    // 004012e1 mov ebp,[ecx+400h]
    m.regs.ebp = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    m.regs.ebx = m.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x400u32), m.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.ebp), m.regs.get_al());
    // 004012f3 cmp esi,9
    sub(m.regs.esi, 0x9u32, &mut m.flags);
    // 004012f6 mov esi,edx
    m.regs.esi = m.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(m, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012c9(m: &mut Machine) -> Cont {
    // 004012c9 mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012cb mov edx,0CCCCCCCDh
    m.regs.edx = 0xcccccccdu32;
    // 004012d0 mul edx
    todo!();
    // 004012d2 shr edx,3
    m.regs.edx = shr(m.regs.edx, 0x3u8, &mut m.flags);
    // 004012d5 lea eax,[edx+edx]
    m.regs.eax = m.regs.edx.wrapping_add(m.regs.edx);
    // 004012d8 lea ebp,[eax+eax*4]
    m.regs.ebp = m.regs.eax.wrapping_add((m.regs.eax * 4));
    // 004012db mov eax,esi
    m.regs.eax = m.regs.esi;
    // 004012dd sub eax,ebp
    m.regs.eax = sub(m.regs.eax, m.regs.ebp, &mut m.flags);
    // 004012df or al,30h
    m.regs.set_al(or(m.regs.get_al(), 0x30u8, &mut m.flags));
    // 004012e1 mov ebp,[ecx+400h]
    m.regs.ebp = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 004012e7 lea ebx,[ebp+1]
    m.regs.ebx = m.regs.ebp.wrapping_add(0x1u32);
    // 004012ea mov [ecx+400h],ebx
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x400u32), m.regs.ebx);
    // 004012f0 mov [ecx+ebp],al
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.ebp), m.regs.get_al());
    // 004012f3 cmp esi,9
    sub(m.regs.esi, 0x9u32, &mut m.flags);
    // 004012f6 mov esi,edx
    m.regs.esi = m.regs.edx;
    // 004012f8 ja short 004012C9h
    ja(m, Cont(x004012fa), Cont(x004012c9))
}

#[allow(unused_variables)]
pub fn x004012fa(m: &mut Machine) -> Cont {
    // 004012fa mov eax,[ecx+400h]
    m.regs.eax = m.memory.read::<u32>(m.regs.ecx.wrapping_add(0x400u32));
    // 00401300 inc edi
    m.regs.edi = inc(m.regs.edi, &mut m.flags);
    // 00401301 cmp eax,edi
    sub(m.regs.eax, m.regs.edi, &mut m.flags);
    // 00401303 jle short 0040131Eh
    jle(m, Cont(x00401305), Cont(x0040131e))
}

#[allow(unused_variables)]
pub fn x00401305(m: &mut Machine) -> Cont {
    // 00401305 dec eax
    m.regs.eax = dec(m.regs.eax, &mut m.flags);
    // 00401306 mov dl,[ecx+edi-1]
    m.regs.set_dl(
        m.memory.read::<u8>(
            m.regs
                .ecx
                .wrapping_add(m.regs.edi)
                .wrapping_add(0xffffffffu32),
        ),
    );
    // 0040130a mov dh,[ecx+eax]
    m.regs
        .set_dh(m.memory.read::<u8>(m.regs.ecx.wrapping_add(m.regs.eax)));
    // 0040130d mov [ecx+edi-1],dh
    m.memory.write::<u8>(
        m.regs
            .ecx
            .wrapping_add(m.regs.edi)
            .wrapping_add(0xffffffffu32),
        m.regs.get_dh(),
    );
    // 00401311 mov [ecx+eax],dl
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.eax), m.regs.get_dl());
    // 00401314 inc edi
    m.regs.edi = inc(m.regs.edi, &mut m.flags);
    // 00401315 lea edx,[eax-1]
    m.regs.edx = m.regs.eax.wrapping_add(0xffffffffu32);
    // 00401318 cmp eax,edi
    sub(m.regs.eax, m.regs.edi, &mut m.flags);
    // 0040131a mov eax,edx
    m.regs.eax = m.regs.edx;
    // 0040131c jg short 00401306h
    jg(m, Cont(x0040131e), Cont(x00401306))
}

#[allow(unused_variables)]
pub fn x00401306(m: &mut Machine) -> Cont {
    // 00401306 mov dl,[ecx+edi-1]
    m.regs.set_dl(
        m.memory.read::<u8>(
            m.regs
                .ecx
                .wrapping_add(m.regs.edi)
                .wrapping_add(0xffffffffu32),
        ),
    );
    // 0040130a mov dh,[ecx+eax]
    m.regs
        .set_dh(m.memory.read::<u8>(m.regs.ecx.wrapping_add(m.regs.eax)));
    // 0040130d mov [ecx+edi-1],dh
    m.memory.write::<u8>(
        m.regs
            .ecx
            .wrapping_add(m.regs.edi)
            .wrapping_add(0xffffffffu32),
        m.regs.get_dh(),
    );
    // 00401311 mov [ecx+eax],dl
    m.memory
        .write::<u8>(m.regs.ecx.wrapping_add(m.regs.eax), m.regs.get_dl());
    // 00401314 inc edi
    m.regs.edi = inc(m.regs.edi, &mut m.flags);
    // 00401315 lea edx,[eax-1]
    m.regs.edx = m.regs.eax.wrapping_add(0xffffffffu32);
    // 00401318 cmp eax,edi
    sub(m.regs.eax, m.regs.edi, &mut m.flags);
    // 0040131a mov eax,edx
    m.regs.eax = m.regs.edx;
    // 0040131c jg short 00401306h
    jg(m, Cont(x0040131e), Cont(x00401306))
}

#[allow(unused_variables)]
pub fn x0040131e(m: &mut Machine) -> Cont {
    // 0040131e mov eax,ecx
    m.regs.eax = m.regs.ecx;
    // 00401320 pop esi
    m.regs.esi = pop(m);
    // 00401321 pop edi
    m.regs.edi = pop(m);
    // 00401322 pop ebx
    m.regs.ebx = pop(m);
    // 00401323 pop ebp
    m.regs.ebp = pop(m);
    // 00401324 ret
    ret(m, 0)
}

#[allow(unused_variables)]
pub fn x004013e8(m: &mut Machine) -> Cont {
    // 004013e8 push ebx
    push(m, m.regs.ebx);
    // 004013e9 push edi
    push(m, m.regs.edi);
    // 004013ea push esi
    push(m, m.regs.esi);
    // 004013eb sub esp,18h
    m.regs.esp = sub(m.regs.esp, 0x18u32, &mut m.flags);
    // 004013ee xor ebx,ebx
    m.regs.ebx = xor(m.regs.ebx, m.regs.ebx, &mut m.flags);
    // 004013f0 mov edi,ds:[402184h]
    m.regs.edi = m.memory.read::<u32>(0x402184u32);
    // 004013f6 push ebx
    push(m, m.regs.ebx);
    // 004013f7 push ebx
    push(m, m.regs.ebx);
    // 004013f8 push ebx
    push(m, m.regs.ebx);
    // 004013f9 push 401342h
    push(m, 0x401342u32);
    // 004013fe push 1000h
    push(m, 0x1000u32);
    // 00401403 push ebx
    push(m, m.regs.ebx);
    // 00401404 call edi
    let dst = indirect(m, m.regs.edi);
    call(m, 0x401406, dst)
}

#[allow(unused_variables)]
pub fn x00401406(m: &mut Machine) -> Cont {
    // 00401406 call dword ptr ds:[402194h]
    let dst = Cont(kernel32::TlsAlloc_stdcall);
    call(m, 0x40140c, dst)
}

#[allow(unused_variables)]
pub fn x0040140c(m: &mut Machine) -> Cont {
    // 0040140c mov esi,eax
    m.regs.esi = m.regs.eax;
    // 0040140e push 1
    push(m, 0x1u32);
    // 00401410 push eax
    push(m, m.regs.eax);
    // 00401411 call dword ptr ds:[40219Ch]
    let dst = Cont(kernel32::TlsSetValue_stdcall);
    call(m, 0x401417, dst)
}

#[allow(unused_variables)]
pub fn x00401417(m: &mut Machine) -> Cont {
    // 00401417 lea eax,[esp+0Ch]
    m.regs.eax = m.regs.esp.wrapping_add(0xcu32);
    // 0040141b mov dword ptr [eax],4020EBh
    m.memory.write::<u32>(m.regs.eax, 0x4020ebu32);
    // 00401421 mov dword ptr [eax+4],5
    m.memory
        .write::<u32>(m.regs.eax.wrapping_add(0x4u32), 0x5u32);
    // 00401428 mov [eax+8],esi
    m.memory
        .write::<u32>(m.regs.eax.wrapping_add(0x8u32), m.regs.esi);
    // 0040142b push ebx
    push(m, m.regs.ebx);
    // 0040142c push ebx
    push(m, m.regs.ebx);
    // 0040142d push eax
    push(m, m.regs.eax);
    // 0040142e push 401325h
    push(m, 0x401325u32);
    // 00401433 push 8000h
    push(m, 0x8000u32);
    // 00401438 push ebx
    push(m, m.regs.ebx);
    // 00401439 call edi
    let dst = indirect(m, m.regs.edi);
    call(m, 0x40143b, dst)
}

#[allow(unused_variables)]
pub fn x0040143b(m: &mut Machine) -> Cont {
    // 0040143b mov ecx,esp
    m.regs.ecx = m.regs.esp;
    // 0040143d mov dword ptr [ecx],4020C0h
    m.memory.write::<u32>(m.regs.ecx, 0x4020c0u32);
    // 00401443 mov dword ptr [ecx+4],0Ah
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x4u32), 0xau32);
    // 0040144a mov [ecx+8],esi
    m.memory
        .write::<u32>(m.regs.ecx.wrapping_add(0x8u32), m.regs.esi);
    // 0040144d call 00401000h
    let dst = Cont(x00401000);
    call(m, 0x401452, dst)
}

#[allow(unused_variables)]
pub fn x00401452(m: &mut Machine) -> Cont {
    // 00401452 add esp,18h
    m.regs.esp = add(m.regs.esp, 0x18u32, &mut m.flags);
    // 00401455 pop esi
    m.regs.esi = pop(m);
    // 00401456 pop edi
    m.regs.edi = pop(m);
    // 00401457 pop ebx
    m.regs.ebx = pop(m);
    // 00401458 ret
    ret(m, 0)
}

const BLOCKS: [(u32, fn(&mut Machine) -> Cont); 66] = [
    (0x001001, kernel32::CreateThread_stdcall),
    (0x001002, kernel32::GetCurrentThreadId_stdcall),
    (0x001003, kernel32::GetStdHandle_stdcall),
    (0x001004, kernel32::Sleep_stdcall),
    (0x001005, kernel32::TlsAlloc_stdcall),
    (0x001006, kernel32::TlsGetValue_stdcall),
    (0x001007, kernel32::TlsSetValue_stdcall),
    (0x001008, kernel32::WriteFile_stdcall),
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
    (0x4013e8, x004013e8),
    (0x401406, x00401406),
    (0x40140c, x0040140c),
    (0x401417, x00401417),
    (0x40143b, x0040143b),
    (0x401452, x00401452),
    (0xf000_0000, runtime::return_from_main),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x004013e8),
};
