#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(static_mut_refs)]
#![allow(unused_parens)]

use runtime::*;
use winapi::*;

pub fn init_mappings() {
    unsafe {
        let mut mappings = kernel32::state().mappings.borrow_mut();
        mappings.alloc("null page".to_string(), 0x0, 0x1000);
        mappings.alloc("imported functions".to_string(), 0x1000, 0x1000);
        mappings.alloc("exe header".to_string(), 0x400000, 0x1000);
        let bytes = include_bytes!("../data/00400000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x400000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".text".to_string(), 0x401000, 0x5000);
        let bytes = include_bytes!("../data/00401000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x401000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".rdata".to_string(), 0x406000, 0x1000);
        let bytes = include_bytes!("../data/00406000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x406000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".data".to_string(), 0x407000, 0x3000);
        let bytes = include_bytes!("../data/00407000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x407000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".rsrc".to_string(), 0x40a000, 0x67000);
        let bytes = include_bytes!("../data/0040a000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x40a000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
    }
}
pub fn x00401000() -> Cont {
    unsafe {
        // 00401000 call 00401010h
        call(0x401005, Cont(x00401010))
    }
}

pub fn x00401005() -> Cont {
    unsafe {
        // 00401005 jmp near ptr 00401020h
        Cont(x00401020)
    }
}

pub fn x00401010() -> Cont {
    unsafe {
        // 00401010 mov ecx,409550h
        MACHINE.regs.ecx = 0x409550u32;
        // 00401015 jmp near ptr 00401460h
        Cont(x00401460)
    }
}

pub fn x00401020() -> Cont {
    unsafe {
        // 00401020 push 401030h
        push(0x401030u32);
        // 00401025 call 00401873h
        call(0x40102a, Cont(x00401873))
    }
}

pub fn x0040102a() -> Cont {
    unsafe {
        // 0040102a pop ecx
        MACHINE.regs.ecx = pop();
        // 0040102b ret
        ret(0)
    }
}

pub fn x00401040() -> Cont {
    unsafe {
        // 00401040 mov ecx,[esp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 00401044 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00401048 sub esp,1Ch
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x1cu32);
        // 0040104b mov ds:[40957Ch],eax
        MACHINE.memory.write::<u32>(0x40957cu32, MACHINE.regs.eax);
        // 00401050 push ecx
        push(MACHINE.regs.ecx);
        // 00401051 call 00401200h
        call(0x401056, Cont(x00401200))
    }
}

pub fn x00401056() -> Cont {
    unsafe {
        // 00401056 add esp,4
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x4u32);
        // 00401059 mov ds:[409580h],eax
        MACHINE.memory.write::<u32>(0x409580u32, MACHINE.regs.eax);
        // 0040105e test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401060 jne short 0040106Bh
        jne(Cont(x00401062), Cont(x0040106b))
    }
}

pub fn x00401062() -> Cont {
    unsafe {
        // 00401062 or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 00401065 add esp,1Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x1cu32);
        // 00401068 ret 10h
        ret(16)
    }
}

pub fn x0040106b() -> Cont {
    unsafe {
        // 0040106b call 00401310h
        call(0x401070, Cont(x00401310))
    }
}

pub fn x00401070() -> Cont {
    unsafe {
        // 00401070 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401072 jge short 0040109Ah
        jge(Cont(x00401074), Cont(x0040109a))
    }
}

pub fn x00401074() -> Cont {
    unsafe {
        // 00401074 call 00401420h
        call(0x401079, Cont(x00401420))
    }
}

pub fn x00401079() -> Cont {
    unsafe {
        // 00401079 mov edx,ds:[409580h]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x409580u32);
        // 0040107f push 30h
        push(0x30u32);
        // 00401081 push 40709Ch
        push(0x40709cu32);
        // 00401086 push 407030h
        push(0x407030u32);
        // 0040108b push edx
        push(MACHINE.regs.edx);
        // 0040108c call dword ptr ds:[4060D8h]
        call(0x401092, Cont(user32::MessageBoxA_stdcall))
    }
}

pub fn x00401092() -> Cont {
    unsafe {
        // 00401092 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401094 add esp,1Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x1cu32);
        // 00401097 ret 10h
        ret(16)
    }
}

pub fn x0040109a() -> Cont {
    unsafe {
        // 0040109a mov eax,ds:[409584h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409584u32);
        // 0040109f push ebx
        push(MACHINE.regs.ebx);
        // 004010a0 push esi
        push(MACHINE.regs.esi);
        // 004010a1 push edi
        push(MACHINE.regs.edi);
        // 004010a2 push 0FFFFFFFFh
        push(0xffffffffu32);
        // 004010a4 push 118h
        push(0x118u32);
        // 004010a9 push 5DCh
        push(0x5dcu32);
        // 004010ae push eax
        push(MACHINE.regs.eax);
        // 004010af mov ecx,409550h
        MACHINE.regs.ecx = 0x409550u32;
        // 004010b4 call 00401640h
        call(0x4010b9, Cont(x00401640))
    }
}

pub fn x004010b9() -> Cont {
    unsafe {
        // 004010b9 mov ecx,ds:[40957Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40957cu32);
        // 004010bf push 118h
        push(0x118u32);
        // 004010c4 push 5DCh
        push(0x5dcu32);
        // 004010c9 push 0
        push(0x0u32);
        // 004010cb push 0
        push(0x0u32);
        // 004010cd push 65h
        push(0x65u32);
        // 004010cf push ecx
        push(MACHINE.regs.ecx);
        // 004010d0 mov ecx,409550h
        MACHINE.regs.ecx = 0x409550u32;
        // 004010d5 call 004014D0h
        call(0x4010da, Cont(x004014d0))
    }
}

pub fn x004010da() -> Cont {
    unsafe {
        // 004010da mov esi,ds:[4060D4h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x4060d4u32);
        // 004010e0 mov edi,ds:[4060D0h]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(0x4060d0u32);
        // 004010e6 mov ebx,ds:[4060CCh]
        MACHINE.regs.ebx = MACHINE.memory.read::<u32>(0x4060ccu32);
        // 004010ec push 1
        push(0x1u32);
        // 004010ee push 0
        push(0x0u32);
        // 004010f0 push 0
        push(0x0u32);
        // 004010f2 lea edx,[esp+18h]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x18u32);
        // 004010f6 push 0
        push(0x0u32);
        // 004010f8 push edx
        push(MACHINE.regs.edx);
        // 004010f9 call esi
        call(0x4010fb, indirect(MACHINE.regs.esi))
    }
}

pub fn x004010ec() -> Cont {
    unsafe {
        // 004010ec push 1
        push(0x1u32);
        // 004010ee push 0
        push(0x0u32);
        // 004010f0 push 0
        push(0x0u32);
        // 004010f2 lea edx,[esp+18h]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x18u32);
        // 004010f6 push 0
        push(0x0u32);
        // 004010f8 push edx
        push(MACHINE.regs.edx);
        // 004010f9 call esi
        call(0x4010fb, indirect(MACHINE.regs.esi))
    }
}

pub fn x004010fb() -> Cont {
    unsafe {
        // 004010fb test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004010fd je short 00401116h
        je(Cont(x004010ff), Cont(x00401116))
    }
}

pub fn x004010ff() -> Cont {
    unsafe {
        // 004010ff cmp dword ptr [esp+10h],12h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32)),
            0x12u32,
        );
        // 00401104 je short 0040111Dh
        je(Cont(x00401106), Cont(x0040111d))
    }
}

pub fn x00401106() -> Cont {
    unsafe {
        // 00401106 lea eax,[esp+0Ch]
        MACHINE.regs.eax = MACHINE.regs.esp.wrapping_add(0xcu32);
        // 0040110a push eax
        push(MACHINE.regs.eax);
        // 0040110b call edi
        call(0x40110d, indirect(MACHINE.regs.edi))
    }
}

pub fn x0040110d() -> Cont {
    unsafe {
        // 0040110d lea ecx,[esp+0Ch]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0xcu32);
        // 00401111 push ecx
        push(MACHINE.regs.ecx);
        // 00401112 call ebx
        call(0x401114, indirect(MACHINE.regs.ebx))
    }
}

pub fn x00401114() -> Cont {
    unsafe {
        // 00401114 jmp short 004010ECh
        Cont(x004010ec)
    }
}

pub fn x00401116() -> Cont {
    unsafe {
        // 00401116 call 00401130h
        call(0x40111b, Cont(x00401130))
    }
}

pub fn x0040111b() -> Cont {
    unsafe {
        // 0040111b jmp short 004010ECh
        Cont(x004010ec)
    }
}

pub fn x0040111d() -> Cont {
    unsafe {
        // 0040111d call 00401420h
        call(0x401122, Cont(x00401420))
    }
}

pub fn x00401122() -> Cont {
    unsafe {
        // 00401122 pop edi
        MACHINE.regs.edi = pop();
        // 00401123 pop esi
        MACHINE.regs.esi = pop();
        // 00401124 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401126 pop ebx
        MACHINE.regs.ebx = pop();
        // 00401127 add esp,1Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x1cu32);
        // 0040112a ret 10h
        ret(16)
    }
}

pub fn x00401130() -> Cont {
    unsafe {
        // 00401130 push edi
        push(MACHINE.regs.edi);
        // 00401131 mov edi,ds:[406080h]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(0x406080u32);
        // 00401137 call edi
        call(0x401139, indirect(MACHINE.regs.edi))
    }
}

pub fn x00401139() -> Cont {
    unsafe {
        // 00401139 sub eax,ds:[409548h]
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.memory.read::<u32>(0x409548u32));
        // 0040113f cmp eax,32h
        sub(MACHINE.regs.eax, 0x32u32);
        // 00401142 jb near ptr 004011EFh
        jb(Cont(x00401148), Cont(x004011ef))
    }
}

pub fn x00401148() -> Cont {
    unsafe {
        // 00401148 mov eax,ds:[409594h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409594u32);
        // 0040114d mov ecx,ds:[409590h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409590u32);
        // 00401153 mov edx,ds:[40958Ch]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x40958cu32);
        // 00401159 push 8Ch
        push(0x8cu32);
        // 0040115e push 96h
        push(0x96u32);
        // 00401163 push eax
        push(MACHINE.regs.eax);
        // 00401164 push ecx
        push(MACHINE.regs.ecx);
        // 00401165 push 0AAh
        push(0xaau32);
        // 0040116a push 0F5h
        push(0xf5u32);
        // 0040116f push edx
        push(MACHINE.regs.edx);
        // 00401170 mov ecx,409550h
        MACHINE.regs.ecx = 0x409550u32;
        // 00401175 call 00401730h
        call(0x40117a, Cont(x00401730))
    }
}

pub fn x0040117a() -> Cont {
    unsafe {
        // 0040117a mov eax,ds:[409588h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409588u32);
        // 0040117f push 0
        push(0x0u32);
        // 00401181 push 0
        push(0x0u32);
        // 00401183 push eax
        push(MACHINE.regs.eax);
        // 00401184 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401186 call dword ptr [ecx+2Ch]
        call(
            0x401189,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x2cu32))),
        )
    }
}

pub fn x00401189() -> Cont {
    unsafe {
        // 00401189 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040118b je short 004011A8h
        je(Cont(x0040118d), Cont(x004011a8))
    }
}

pub fn x0040118d() -> Cont {
    unsafe {
        // 0040118d cmp eax,887601C2h
        sub(MACHINE.regs.eax, 0x887601c2u32);
        // 00401192 je short 0040119Dh
        je(Cont(x00401194), Cont(x0040119d))
    }
}

pub fn x00401194() -> Cont {
    unsafe {
        // 00401194 cmp eax,8876021Ch
        sub(MACHINE.regs.eax, 0x8876021cu32);
        // 00401199 jne short 004011A8h
        jne(Cont(x0040119b), Cont(x004011a8))
    }
}

pub fn x0040119b() -> Cont {
    unsafe {
        // 0040119b jmp short 0040117Ah
        Cont(x0040117a)
    }
}

pub fn x0040119d() -> Cont {
    unsafe {
        // 0040119d mov eax,ds:[409588h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409588u32);
        // 004011a2 push eax
        push(MACHINE.regs.eax);
        // 004011a3 mov edx,[eax]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004011a5 call dword ptr [edx+6Ch]
        call(
            0x4011a8,
            indirect(MACHINE.memory.read(MACHINE.regs.edx.wrapping_add(0x6cu32))),
        )
    }
}

pub fn x004011a8() -> Cont {
    unsafe {
        // 004011a8 mov eax,ds:[409590h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409590u32);
        // 004011ad add eax,96h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x96u32);
        // 004011b2 cmp eax,5DCh
        sub(MACHINE.regs.eax, 0x5dcu32);
        // 004011b7 mov ds:[409590h],eax
        MACHINE.memory.write::<u32>(0x409590u32, MACHINE.regs.eax);
        // 004011bc jl short 004011E8h
        jl(Cont(x004011be), Cont(x004011e8))
    }
}

pub fn x004011be() -> Cont {
    unsafe {
        // 004011be mov eax,ds:[409594h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409594u32);
        // 004011c3 mov dword ptr ds:[409590h],0
        MACHINE.memory.write::<u32>(0x409590u32, 0x0u32);
        // 004011cd add eax,8Ch
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8cu32);
        // 004011d2 cmp eax,118h
        sub(MACHINE.regs.eax, 0x118u32);
        // 004011d7 mov ds:[409594h],eax
        MACHINE.memory.write::<u32>(0x409594u32, MACHINE.regs.eax);
        // 004011dc jl short 004011E8h
        jl(Cont(x004011de), Cont(x004011e8))
    }
}

pub fn x004011de() -> Cont {
    unsafe {
        // 004011de mov dword ptr ds:[409594h],0
        MACHINE.memory.write::<u32>(0x409594u32, 0x0u32);
        // 004011e8 call edi
        call(0x4011ea, indirect(MACHINE.regs.edi))
    }
}

pub fn x004011e8() -> Cont {
    unsafe {
        // 004011e8 call edi
        call(0x4011ea, indirect(MACHINE.regs.edi))
    }
}

pub fn x004011ea() -> Cont {
    unsafe {
        // 004011ea mov ds:[409548h],eax
        MACHINE.memory.write::<u32>(0x409548u32, MACHINE.regs.eax);
        // 004011ef pop edi
        MACHINE.regs.edi = pop();
        // 004011f0 ret
        ret(0)
    }
}

pub fn x004011ef() -> Cont {
    unsafe {
        // 004011ef pop edi
        MACHINE.regs.edi = pop();
        // 004011f0 ret
        ret(0)
    }
}

pub fn x00401200() -> Cont {
    unsafe {
        // 00401200 sub esp,28h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x28u32);
        // 00401203 mov eax,ds:[40957Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40957cu32);
        // 00401208 push esi
        push(MACHINE.regs.esi);
        // 00401209 push 7F00h
        push(0x7f00u32);
        // 0040120e push eax
        push(MACHINE.regs.eax);
        // 0040120f mov dword ptr [esp+0Ch],3
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32), 0x3u32);
        // 00401217 mov dword ptr [esp+10h],4012D0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), 0x4012d0u32);
        // 0040121f mov dword ptr [esp+14h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32), 0x0u32);
        // 00401227 mov dword ptr [esp+18h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32), 0x0u32);
        // 0040122f mov [esp+1Ch],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x1cu32), MACHINE.regs.eax);
        // 00401233 call dword ptr ds:[4060FCh]
        call(0x401239, Cont(user32::LoadIconA_stdcall))
    }
}

pub fn x00401239() -> Cont {
    unsafe {
        // 00401239 push 7F00h
        push(0x7f00u32);
        // 0040123e push 0
        push(0x0u32);
        // 00401240 mov [esp+20h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x20u32), MACHINE.regs.eax);
        // 00401244 call dword ptr ds:[406100h]
        call(0x40124a, Cont(user32::LoadCursorA_stdcall))
    }
}

pub fn x0040124a() -> Cont {
    unsafe {
        // 0040124a push 4
        push(0x4u32);
        // 0040124c mov [esp+20h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x20u32), MACHINE.regs.eax);
        // 00401250 call dword ptr ds:[40601Ch]
        call(0x401256, Cont(gdi32::GetStockObject_stdcall))
    }
}

pub fn x00401256() -> Cont {
    unsafe {
        // 00401256 mov [esp+20h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x20u32), MACHINE.regs.eax);
        // 0040125a lea eax,[esp+4]
        MACHINE.regs.eax = MACHINE.regs.esp.wrapping_add(0x4u32);
        // 0040125e push eax
        push(MACHINE.regs.eax);
        // 0040125f mov dword ptr [esp+28h],409598h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x28u32), 0x409598u32);
        // 00401267 mov dword ptr [esp+2Ch],4070A4h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32), 0x4070a4u32);
        // 0040126f call dword ptr ds:[406104h]
        call(0x401275, Cont(user32::RegisterClassA_stdcall))
    }
}

pub fn x00401275() -> Cont {
    unsafe {
        // 00401275 mov ecx,ds:[40957Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40957cu32);
        // 0040127b mov esi,ds:[4060ECh]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x4060ecu32);
        // 00401281 push 0
        push(0x0u32);
        // 00401283 push ecx
        push(MACHINE.regs.ecx);
        // 00401284 push 0
        push(0x0u32);
        // 00401286 push 0
        push(0x0u32);
        // 00401288 push 1
        push(0x1u32);
        // 0040128a call esi
        call(0x40128c, indirect(MACHINE.regs.esi))
    }
}

pub fn x0040128c() -> Cont {
    unsafe {
        // 0040128c push eax
        push(MACHINE.regs.eax);
        // 0040128d push 0
        push(0x0u32);
        // 0040128f call esi
        call(0x401291, indirect(MACHINE.regs.esi))
    }
}

pub fn x00401291() -> Cont {
    unsafe {
        // 00401291 push eax
        push(MACHINE.regs.eax);
        // 00401292 push 0
        push(0x0u32);
        // 00401294 push 0
        push(0x0u32);
        // 00401296 push 80000000h
        push(0x80000000u32);
        // 0040129b push 4070A4h
        push(0x4070a4u32);
        // 004012a0 push 4070A4h
        push(0x4070a4u32);
        // 004012a5 push 8
        push(0x8u32);
        // 004012a7 call dword ptr ds:[4060E8h]
        call(0x4012ad, Cont(user32::CreateWindowExA_stdcall))
    }
}

pub fn x004012ad() -> Cont {
    unsafe {
        // 004012ad mov edx,[esp+30h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32));
        // 004012b1 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 004012b3 push edx
        push(MACHINE.regs.edx);
        // 004012b4 push esi
        push(MACHINE.regs.esi);
        // 004012b5 call dword ptr ds:[4060E4h]
        call(0x4012bb, Cont(user32::ShowWindow_stdcall))
    }
}

pub fn x004012bb() -> Cont {
    unsafe {
        // 004012bb push esi
        push(MACHINE.regs.esi);
        // 004012bc call dword ptr ds:[4060E0h]
        call(0x4012c2, Cont(user32::UpdateWindow_stdcall))
    }
}

pub fn x004012c2() -> Cont {
    unsafe {
        // 004012c2 push esi
        push(MACHINE.regs.esi);
        // 004012c3 call dword ptr ds:[4060DCh]
        call(0x4012c9, Cont(user32::SetFocus_stdcall))
    }
}

pub fn x004012c9() -> Cont {
    unsafe {
        // 004012c9 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 004012cb pop esi
        MACHINE.regs.esi = pop();
        // 004012cc add esp,28h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x28u32);
        // 004012cf ret
        ret(0)
    }
}

pub fn x00401310() -> Cont {
    unsafe {
        // 00401310 sub esp,8Ch
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x8cu32);
        // 00401316 push 0
        push(0x0u32);
        // 00401318 push 406114h
        push(0x406114u32);
        // 0040131d push 409584h
        push(0x409584u32);
        // 00401322 push 0
        push(0x0u32);
        // 00401324 call 00401800h
        call(0x401329, Cont(x00401800))
    }
}

pub fn x00401329() -> Cont {
    unsafe {
        // 00401329 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040132b je short 00401337h
        je(Cont(x0040132d), Cont(x00401337))
    }
}

pub fn x0040132d() -> Cont {
    unsafe {
        // 0040132d or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 00401330 add esp,8Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x8cu32);
        // 00401336 ret
        ret(0)
    }
}

pub fn x00401337() -> Cont {
    unsafe {
        // 00401337 mov eax,ds:[409584h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409584u32);
        // 0040133c mov edx,ds:[409580h]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x409580u32);
        // 00401342 push 11h
        push(0x11u32);
        // 00401344 push edx
        push(MACHINE.regs.edx);
        // 00401345 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401347 push eax
        push(MACHINE.regs.eax);
        // 00401348 call dword ptr [ecx+50h]
        call(
            0x40134b,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x50u32))),
        )
    }
}

pub fn x0040134b() -> Cont {
    unsafe {
        // 0040134b test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040134d je short 0040135Bh
        je(Cont(x0040134f), Cont(x0040135b))
    }
}

pub fn x0040134f() -> Cont {
    unsafe {
        // 0040134f mov eax,0FFFFFFFEh
        MACHINE.regs.eax = 0xfffffffeu32;
        // 00401354 add esp,8Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x8cu32);
        // 0040135a ret
        ret(0)
    }
}

pub fn x0040135b() -> Cont {
    unsafe {
        // 0040135b mov eax,ds:[409584h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409584u32);
        // 00401360 push 0
        push(0x0u32);
        // 00401362 push 0
        push(0x0u32);
        // 00401364 push 10h
        push(0x10u32);
        // 00401366 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401368 push 1E0h
        push(0x1e0u32);
        // 0040136d push 280h
        push(0x280u32);
        // 00401372 push eax
        push(MACHINE.regs.eax);
        // 00401373 call dword ptr [ecx+54h]
        call(
            0x401376,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x54u32))),
        )
    }
}

pub fn x00401376() -> Cont {
    unsafe {
        // 00401376 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401378 je short 00401386h
        je(Cont(x0040137a), Cont(x00401386))
    }
}

pub fn x0040137a() -> Cont {
    unsafe {
        // 0040137a mov eax,0FFFFFFFDh
        MACHINE.regs.eax = 0xfffffffdu32;
        // 0040137f add esp,8Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x8cu32);
        // 00401385 ret
        ret(0)
    }
}

pub fn x00401386() -> Cont {
    unsafe {
        // 00401386 push edi
        push(MACHINE.regs.edi);
        // 00401387 mov ecx,1Fh
        MACHINE.regs.ecx = 0x1fu32;
        // 0040138c xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 0040138e lea edi,[esp+14h]
        MACHINE.regs.edi = MACHINE.regs.esp.wrapping_add(0x14u32);
        // 00401392 rep stosd
        stosd();
        // 00401394 mov eax,ds:[409584h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409584u32);
        // 00401399 push 0
        push(0x0u32);
        // 0040139b lea ecx,[esp+18h]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0x18u32);
        // 0040139f mov dword ptr [esp+18h],7Ch
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32), 0x7cu32);
        // 004013a7 mov dword ptr [esp+1Ch],21h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x1cu32), 0x21u32);
        // 004013af mov dword ptr [esp+80h],218h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x80u32), 0x218u32);
        // 004013ba mov dword ptr [esp+2Ch],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32), 0x1u32);
        // 004013c2 mov edx,[eax]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004013c4 push 409588h
        push(0x409588u32);
        // 004013c9 push ecx
        push(MACHINE.regs.ecx);
        // 004013ca push eax
        push(MACHINE.regs.eax);
        // 004013cb call dword ptr [edx+18h]
        call(
            0x4013ce,
            indirect(MACHINE.memory.read(MACHINE.regs.edx.wrapping_add(0x18u32))),
        )
    }
}

pub fn x004013ce() -> Cont {
    unsafe {
        // 004013ce test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004013d0 pop edi
        MACHINE.regs.edi = pop();
        // 004013d1 je short 004013DDh
        je(Cont(x004013d3), Cont(x004013dd))
    }
}

pub fn x004013d3() -> Cont {
    unsafe {
        // 004013d3 or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 004013d6 add esp,8Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x8cu32);
        // 004013dc ret
        ret(0)
    }
}

pub fn x004013dd() -> Cont {
    unsafe {
        // 004013dd xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 004013df mov eax,ds:[409588h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409588u32);
        // 004013e4 mov [esp],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 004013e8 mov dword ptr [esp],4
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x0u32), 0x4u32);
        // 004013f0 mov [esp+4],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32), MACHINE.regs.edx);
        // 004013f4 push 40958Ch
        push(0x40958cu32);
        // 004013f9 mov [esp+0Ch],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32), MACHINE.regs.edx);
        // 004013fd mov [esp+10h],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.edx);
        // 00401401 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401403 lea edx,[esp+4]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x4u32);
        // 00401407 push edx
        push(MACHINE.regs.edx);
        // 00401408 push eax
        push(MACHINE.regs.eax);
        // 00401409 call dword ptr [ecx+30h]
        call(
            0x40140c,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x30u32))),
        )
    }
}

pub fn x0040140c() -> Cont {
    unsafe {
        // 0040140c neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 0040140e sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401410 add esp,8Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x8cu32);
        // 00401416 ret
        ret(0)
    }
}

pub fn x00401420() -> Cont {
    unsafe {
        // 00401420 mov ecx,409550h
        MACHINE.regs.ecx = 0x409550u32;
        // 00401425 call 004017D0h
        call(0x40142a, Cont(x004017d0))
    }
}

pub fn x0040142a() -> Cont {
    unsafe {
        // 0040142a mov eax,ds:[40958Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40958cu32);
        // 0040142f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401431 je short 00401439h
        je(Cont(x00401433), Cont(x00401439))
    }
}

pub fn x00401433() -> Cont {
    unsafe {
        // 00401433 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401435 push eax
        push(MACHINE.regs.eax);
        // 00401436 call dword ptr [ecx+8]
        call(
            0x401439,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x8u32))),
        )
    }
}

pub fn x00401439() -> Cont {
    unsafe {
        // 00401439 mov eax,ds:[409588h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409588u32);
        // 0040143e test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401440 je short 00401448h
        je(Cont(x00401442), Cont(x00401448))
    }
}

pub fn x00401442() -> Cont {
    unsafe {
        // 00401442 mov edx,[eax]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401444 push eax
        push(MACHINE.regs.eax);
        // 00401445 call dword ptr [edx+8]
        call(
            0x401448,
            indirect(MACHINE.memory.read(MACHINE.regs.edx.wrapping_add(0x8u32))),
        )
    }
}

pub fn x00401448() -> Cont {
    unsafe {
        // 00401448 mov eax,ds:[409584h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409584u32);
        // 0040144d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040144f je short 00401457h
        je(Cont(x00401451), Cont(x00401457))
    }
}

pub fn x00401451() -> Cont {
    unsafe {
        // 00401451 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401453 push eax
        push(MACHINE.regs.eax);
        // 00401454 call dword ptr [ecx+8]
        call(
            0x401457,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x8u32))),
        )
    }
}

pub fn x00401457() -> Cont {
    unsafe {
        // 00401457 ret
        ret(0)
    }
}

pub fn x00401460() -> Cont {
    unsafe {
        // 00401460 mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00401462 mov dword ptr [eax],406110h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), 0x406110u32);
        // 00401468 mov dword ptr [eax+28h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x28u32), 0x0u32);
        // 0040146f mov dword ptr [eax+1Ch],0FFFFFFFFh
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x1cu32), 0xffffffffu32);
        // 00401476 ret
        ret(0)
    }
}

pub fn x00401480() -> Cont {
    unsafe {
        // 00401480 push esi
        push(MACHINE.regs.esi);
        // 00401481 mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 00401483 call 004014A0h
        call(0x401488, Cont(x004014a0))
    }
}

pub fn x00401488() -> Cont {
    unsafe {
        // 00401488 test byte ptr [esp+8],1
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esp.wrapping_add(0x8u32)),
            0x1u8,
        );
        // 0040148d je short 00401498h
        je(Cont(x0040148f), Cont(x00401498))
    }
}

pub fn x0040148f() -> Cont {
    unsafe {
        // 0040148f push esi
        push(MACHINE.regs.esi);
        // 00401490 call 004018B4h
        call(0x401495, Cont(x004018b4))
    }
}

pub fn x00401495() -> Cont {
    unsafe {
        // 00401495 add esp,4
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x4u32);
        // 00401498 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 0040149a pop esi
        MACHINE.regs.esi = pop();
        // 0040149b ret 4
        ret(4)
    }
}

pub fn x00401498() -> Cont {
    unsafe {
        // 00401498 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 0040149a pop esi
        MACHINE.regs.esi = pop();
        // 0040149b ret 4
        ret(4)
    }
}

pub fn x004014a0() -> Cont {
    unsafe {
        // 004014a0 push esi
        push(MACHINE.regs.esi);
        // 004014a1 mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 004014a3 mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 004014a6 mov dword ptr [esi],406110h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), 0x406110u32);
        // 004014ac test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004014ae je short 004014CBh
        je(Cont(x004014b0), Cont(x004014cb))
    }
}

pub fn x004014b0() -> Cont {
    unsafe {
        // 004014b0 push 4070B0h
        push(0x4070b0u32);
        // 004014b5 call dword ptr ds:[406024h]
        call(0x4014bb, Cont(kernel32::OutputDebugStringA_stdcall))
    }
}

pub fn x004014bb() -> Cont {
    unsafe {
        // 004014bb mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 004014be push eax
        push(MACHINE.regs.eax);
        // 004014bf mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004014c1 call dword ptr [ecx+8]
        call(
            0x4014c4,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x8u32))),
        )
    }
}

pub fn x004014c4() -> Cont {
    unsafe {
        // 004014c4 mov dword ptr [esi+28h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32), 0x0u32);
        // 004014cb pop esi
        MACHINE.regs.esi = pop();
        // 004014cc ret
        ret(0)
    }
}

pub fn x004014cb() -> Cont {
    unsafe {
        // 004014cb pop esi
        MACHINE.regs.esi = pop();
        // 004014cc ret
        ret(0)
    }
}

pub fn x004014d0() -> Cont {
    unsafe {
        // 004014d0 mov eax,[esp+18h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32));
        // 004014d4 sub esp,98h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x98u32);
        // 004014da mov edx,[esp+9Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x9cu32));
        // 004014e1 push ebx
        push(MACHINE.regs.ebx);
        // 004014e2 mov ebx,[esp+0B0h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb0u32));
        // 004014e9 push ebp
        push(MACHINE.regs.ebp);
        // 004014ea push esi
        push(MACHINE.regs.esi);
        // 004014eb mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 004014ed push edi
        push(MACHINE.regs.edi);
        // 004014ee mov ecx,[esp+0B0h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb0u32));
        // 004014f5 push 0
        push(0x0u32);
        // 004014f7 push eax
        push(MACHINE.regs.eax);
        // 004014f8 push ebx
        push(MACHINE.regs.ebx);
        // 004014f9 and ecx,0FFFFh
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0xffffu32);
        // 004014ff push 0
        push(0x0u32);
        // 00401501 push ecx
        push(MACHINE.regs.ecx);
        // 00401502 push edx
        push(MACHINE.regs.edx);
        // 00401503 call dword ptr ds:[4060F0h]
        call(0x401509, Cont(user32::LoadImageA_stdcall))
    }
}

pub fn x00401509() -> Cont {
    unsafe {
        // 00401509 mov ebp,eax
        MACHINE.regs.ebp = MACHINE.regs.eax;
        // 0040150b test ebp,ebp
        and(MACHINE.regs.ebp, MACHINE.regs.ebp);
        // 0040150d je near ptr 00401627h
        je(Cont(x00401513), Cont(x00401627))
    }
}

pub fn x00401513() -> Cont {
    unsafe {
        // 00401513 mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 00401516 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401518 je near ptr 00401627h
        je(Cont(x0040151e), Cont(x00401627))
    }
}

pub fn x0040151e() -> Cont {
    unsafe {
        // 0040151e mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401520 push eax
        push(MACHINE.regs.eax);
        // 00401521 call dword ptr [ecx+6Ch]
        call(
            0x401524,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x6cu32))),
        )
    }
}

pub fn x00401524() -> Cont {
    unsafe {
        // 00401524 push 0
        push(0x0u32);
        // 00401526 call dword ptr ds:[40600Ch]
        call(0x40152c, Cont(gdi32::CreateCompatibleDC_stdcall))
    }
}

pub fn x0040152c() -> Cont {
    unsafe {
        // 0040152c mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 0040152e test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401530 je near ptr 00401627h
        je(Cont(x00401536), Cont(x00401627))
    }
}

pub fn x00401536() -> Cont {
    unsafe {
        // 00401536 push ebp
        push(MACHINE.regs.ebp);
        // 00401537 push edi
        push(MACHINE.regs.edi);
        // 00401538 call dword ptr ds:[406008h]
        call(0x40153e, Cont(gdi32::SelectObject_stdcall))
    }
}

pub fn x0040153e() -> Cont {
    unsafe {
        // 0040153e lea edx,[esp+14h]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x14u32);
        // 00401542 push edx
        push(MACHINE.regs.edx);
        // 00401543 push 18h
        push(0x18u32);
        // 00401545 push ebp
        push(MACHINE.regs.ebp);
        // 00401546 call dword ptr ds:[406010h]
        call(0x40154c, Cont(gdi32::GetObjectA_stdcall))
    }
}

pub fn x0040154c() -> Cont {
    unsafe {
        // 0040154c test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 0040154e jne short 00401554h
        jne(Cont(x00401550), Cont(x00401554))
    }
}

pub fn x00401550() -> Cont {
    unsafe {
        // 00401550 mov ebx,[esp+18h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32));
        // 00401554 mov eax,[esp+0C0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xc0u32));
        // 0040155b test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040155d jne short 0040156Ah
        jne(Cont(x0040155f), Cont(x0040156a))
    }
}

pub fn x00401554() -> Cont {
    unsafe {
        // 00401554 mov eax,[esp+0C0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xc0u32));
        // 0040155b test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040155d jne short 0040156Ah
        jne(Cont(x0040155f), Cont(x0040156a))
    }
}

pub fn x0040155f() -> Cont {
    unsafe {
        // 0040155f mov eax,[esp+1Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x1cu32));
        // 00401563 mov [esp+0C0h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0xc0u32), MACHINE.regs.eax);
        // 0040156a mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 0040156d lea edx,[esp+2Ch]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x2cu32);
        // 00401571 mov dword ptr [esp+2Ch],7Ch
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32), 0x7cu32);
        // 00401579 mov dword ptr [esp+30h],6
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32), 0x6u32);
        // 00401581 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401583 push edx
        push(MACHINE.regs.edx);
        // 00401584 push eax
        push(MACHINE.regs.eax);
        // 00401585 call dword ptr [ecx+58h]
        call(
            0x401588,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x58u32))),
        )
    }
}

pub fn x0040156a() -> Cont {
    unsafe {
        // 0040156a mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 0040156d lea edx,[esp+2Ch]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x2cu32);
        // 00401571 mov dword ptr [esp+2Ch],7Ch
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32), 0x7cu32);
        // 00401579 mov dword ptr [esp+30h],6
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32), 0x6u32);
        // 00401581 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401583 push edx
        push(MACHINE.regs.edx);
        // 00401584 push eax
        push(MACHINE.regs.eax);
        // 00401585 call dword ptr [ecx+58h]
        call(
            0x401588,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x58u32))),
        )
    }
}

pub fn x00401588() -> Cont {
    unsafe {
        // 00401588 mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 0040158b lea edx,[esp+10h]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x10u32);
        // 0040158f push edx
        push(MACHINE.regs.edx);
        // 00401590 push eax
        push(MACHINE.regs.eax);
        // 00401591 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401593 call dword ptr [ecx+44h]
        call(
            0x401596,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x44u32))),
        )
    }
}

pub fn x00401596() -> Cont {
    unsafe {
        // 00401596 mov ebp,[esp+0B8h]
        MACHINE.regs.ebp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb8u32));
        // 0040159d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040159f jne short 004015E0h
        jne(Cont(x004015a1), Cont(x004015e0))
    }
}

pub fn x004015a1() -> Cont {
    unsafe {
        // 004015a1 mov eax,[esp+0C0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xc0u32));
        // 004015a8 mov ecx,[esp+0B4h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb4u32));
        // 004015af mov edx,[esp+34h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32));
        // 004015b3 push 0CC0020h
        push(0xcc0020u32);
        // 004015b8 push eax
        push(MACHINE.regs.eax);
        // 004015b9 mov eax,[esp+40h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x40u32));
        // 004015bd push ebx
        push(MACHINE.regs.ebx);
        // 004015be push ebp
        push(MACHINE.regs.ebp);
        // 004015bf push ecx
        push(MACHINE.regs.ecx);
        // 004015c0 mov ecx,[esp+24h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x24u32));
        // 004015c4 push edi
        push(MACHINE.regs.edi);
        // 004015c5 push edx
        push(MACHINE.regs.edx);
        // 004015c6 push eax
        push(MACHINE.regs.eax);
        // 004015c7 push 0
        push(0x0u32);
        // 004015c9 push 0
        push(0x0u32);
        // 004015cb push ecx
        push(MACHINE.regs.ecx);
        // 004015cc call dword ptr ds:[406014h]
        call(0x4015d2, Cont(gdi32::StretchBlt_stdcall))
    }
}

pub fn x004015d2() -> Cont {
    unsafe {
        // 004015d2 mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 004015d5 mov ecx,[esp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 004015d9 push ecx
        push(MACHINE.regs.ecx);
        // 004015da push eax
        push(MACHINE.regs.eax);
        // 004015db mov edx,[eax]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004015dd call dword ptr [edx+68h]
        call(
            0x4015e0,
            indirect(MACHINE.memory.read(MACHINE.regs.edx.wrapping_add(0x68u32))),
        )
    }
}

pub fn x004015e0() -> Cont {
    unsafe {
        // 004015e0 push edi
        push(MACHINE.regs.edi);
        // 004015e1 call dword ptr ds:[406018h]
        call(0x4015e7, Cont(gdi32::DeleteDC_stdcall))
    }
}

pub fn x004015e7() -> Cont {
    unsafe {
        // 004015e7 mov edx,[esp+0ACh]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xacu32));
        // 004015ee mov eax,[esp+0B0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb0u32));
        // 004015f5 mov ecx,[esp+0B4h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xb4u32));
        // 004015fc mov [esi+4],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.edx);
        // 004015ff mov edx,[esp+0C0h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xc0u32));
        // 00401606 mov [esi+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 00401609 mov [esi+0Ch],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32), MACHINE.regs.ecx);
        // 0040160c mov [esi+10h],ebp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32), MACHINE.regs.ebp);
        // 0040160f mov [esi+14h],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x14u32), MACHINE.regs.ebx);
        // 00401612 mov [esi+18h],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x18u32), MACHINE.regs.edx);
        // 00401615 pop edi
        MACHINE.regs.edi = pop();
        // 00401616 pop esi
        MACHINE.regs.esi = pop();
        // 00401617 pop ebp
        MACHINE.regs.ebp = pop();
        // 00401618 mov eax,1
        MACHINE.regs.eax = 0x1u32;
        // 0040161d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040161e add esp,98h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x98u32);
        // 00401624 ret 18h
        ret(24)
    }
}

pub fn x00401627() -> Cont {
    unsafe {
        // 00401627 pop edi
        MACHINE.regs.edi = pop();
        // 00401628 pop esi
        MACHINE.regs.esi = pop();
        // 00401629 pop ebp
        MACHINE.regs.ebp = pop();
        // 0040162a xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 0040162c pop ebx
        MACHINE.regs.ebx = pop();
        // 0040162d add esp,98h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x98u32);
        // 00401633 ret 18h
        ret(24)
    }
}

pub fn x00401640() -> Cont {
    unsafe {
        // 00401640 sub esp,84h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x84u32);
        // 00401646 push ebx
        push(MACHINE.regs.ebx);
        // 00401647 push ebp
        push(MACHINE.regs.ebp);
        // 00401648 push esi
        push(MACHINE.regs.esi);
        // 00401649 mov esi,[esp+94h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x94u32));
        // 00401650 mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 00401652 push edi
        push(MACHINE.regs.edi);
        // 00401653 mov ebp,[esp+9Ch]
        MACHINE.regs.ebp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x9cu32));
        // 0040165a mov ecx,1Fh
        MACHINE.regs.ecx = 0x1fu32;
        // 0040165f xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401661 lea edi,[esp+18h]
        MACHINE.regs.edi = MACHINE.regs.esp.wrapping_add(0x18u32);
        // 00401665 rep stosd
        stosd();
        // 00401667 mov eax,[esp+0A0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xa0u32));
        // 0040166e mov ecx,[esi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00401670 lea edi,[ebx+28h]
        MACHINE.regs.edi = MACHINE.regs.ebx.wrapping_add(0x28u32);
        // 00401673 push 0
        push(0x0u32);
        // 00401675 lea edx,[esp+1Ch]
        MACHINE.regs.edx = MACHINE.regs.esp.wrapping_add(0x1cu32);
        // 00401679 push edi
        push(MACHINE.regs.edi);
        // 0040167a push edx
        push(MACHINE.regs.edx);
        // 0040167b push esi
        push(MACHINE.regs.esi);
        // 0040167c mov dword ptr [esp+28h],7Ch
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x28u32), 0x7cu32);
        // 00401684 mov dword ptr [esp+2Ch],7
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32), 0x7u32);
        // 0040168c mov dword ptr [esp+90h],4040h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x90u32), 0x4040u32);
        // 00401697 mov [esp+34h],ebp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32), MACHINE.regs.ebp);
        // 0040169b mov [esp+30h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32), MACHINE.regs.eax);
        // 0040169f call dword ptr [ecx+18h]
        call(
            0x4016a2,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x18u32))),
        )
    }
}

pub fn x004016a2() -> Cont {
    unsafe {
        // 004016a2 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004016a4 je short 004016D9h
        je(Cont(x004016a6), Cont(x004016d9))
    }
}

pub fn x004016a6() -> Cont {
    unsafe {
        // 004016a6 cmp eax,8876017Ch
        sub(MACHINE.regs.eax, 0x8876017cu32);
        // 004016ab jne short 004016C6h
        jne(Cont(x004016ad), Cont(x004016c6))
    }
}

pub fn x004016ad() -> Cont {
    unsafe {
        // 004016ad mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 004016af push 0
        push(0x0u32);
        // 004016b1 lea ecx,[esp+1Ch]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0x1cu32);
        // 004016b5 push edi
        push(MACHINE.regs.edi);
        // 004016b6 push ecx
        push(MACHINE.regs.ecx);
        // 004016b7 push esi
        push(MACHINE.regs.esi);
        // 004016b8 mov dword ptr [esp+90h],840h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x90u32), 0x840u32);
        // 004016c3 call dword ptr [eax+18h]
        call(
            0x4016c6,
            indirect(MACHINE.memory.read(MACHINE.regs.eax.wrapping_add(0x18u32))),
        )
    }
}

pub fn x004016c6() -> Cont {
    unsafe {
        // 004016c6 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004016c8 je short 004016D9h
        je(Cont(x004016ca), Cont(x004016d9))
    }
}

pub fn x004016ca() -> Cont {
    unsafe {
        // 004016ca pop edi
        MACHINE.regs.edi = pop();
        // 004016cb pop esi
        MACHINE.regs.esi = pop();
        // 004016cc pop ebp
        MACHINE.regs.ebp = pop();
        // 004016cd xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004016cf pop ebx
        MACHINE.regs.ebx = pop();
        // 004016d0 add esp,84h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x84u32);
        // 004016d6 ret 10h
        ret(16)
    }
}

pub fn x004016d9() -> Cont {
    unsafe {
        // 004016d9 mov esi,[esp+0A4h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xa4u32));
        // 004016e0 cmp esi,0FFFFFFFFh
        sub(MACHINE.regs.esi, 0xffffffffu32);
        // 004016e3 je short 00401700h
        je(Cont(x004016e5), Cont(x00401700))
    }
}

pub fn x004016e5() -> Cont {
    unsafe {
        // 004016e5 mov edi,[edi]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 004016e7 lea eax,[esp+10h]
        MACHINE.regs.eax = MACHINE.regs.esp.wrapping_add(0x10u32);
        // 004016eb mov [esp+10h],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.esi);
        // 004016ef mov dword ptr [esp+14h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32), 0x0u32);
        // 004016f7 mov edx,[edi]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 004016f9 push eax
        push(MACHINE.regs.eax);
        // 004016fa push 8
        push(0x8u32);
        // 004016fc push edi
        push(MACHINE.regs.edi);
        // 004016fd call dword ptr [edx+74h]
        call(
            0x401700,
            indirect(MACHINE.memory.read(MACHINE.regs.edx.wrapping_add(0x74u32))),
        )
    }
}

pub fn x00401700() -> Cont {
    unsafe {
        // 00401700 mov ecx,[esp+0A0h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xa0u32));
        // 00401707 mov [ebx+1Ch],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x1cu32), MACHINE.regs.esi);
        // 0040170a pop edi
        MACHINE.regs.edi = pop();
        // 0040170b mov [ebx+24h],ebp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x24u32), MACHINE.regs.ebp);
        // 0040170e pop esi
        MACHINE.regs.esi = pop();
        // 0040170f mov [ebx+20h],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x20u32), MACHINE.regs.ecx);
        // 00401712 pop ebp
        MACHINE.regs.ebp = pop();
        // 00401713 mov eax,1
        MACHINE.regs.eax = 0x1u32;
        // 00401718 pop ebx
        MACHINE.regs.ebx = pop();
        // 00401719 add esp,84h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x84u32);
        // 0040171f ret 10h
        ret(16)
    }
}

pub fn x00401730() -> Cont {
    unsafe {
        // 00401730 sub esp,10h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x10u32);
        // 00401733 push ebx
        push(MACHINE.regs.ebx);
        // 00401734 push ebp
        push(MACHINE.regs.ebp);
        // 00401735 push esi
        push(MACHINE.regs.esi);
        // 00401736 mov esi,[esp+34h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32));
        // 0040173a push edi
        push(MACHINE.regs.edi);
        // 0040173b mov edi,ecx
        MACHINE.regs.edi = MACHINE.regs.ecx;
        // 0040173d test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 0040173f jne short 00401744h
        jne(Cont(x00401741), Cont(x00401744))
    }
}

pub fn x00401741() -> Cont {
    unsafe {
        // 00401741 mov esi,[edi+24h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x24u32));
        // 00401744 mov edx,[esp+3Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x3cu32));
        // 00401748 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040174a jne short 0040174Fh
        jne(Cont(x0040174c), Cont(x0040174f))
    }
}

pub fn x00401744() -> Cont {
    unsafe {
        // 00401744 mov edx,[esp+3Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x3cu32));
        // 00401748 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040174a jne short 0040174Fh
        jne(Cont(x0040174c), Cont(x0040174f))
    }
}

pub fn x0040174c() -> Cont {
    unsafe {
        // 0040174c mov edx,[edi+20h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x20u32));
        // 0040174f mov eax,[esp+30h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32));
        // 00401753 mov ecx,[esp+34h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32));
        // 00401757 mov ebx,[esp+2Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32));
        // 0040175b mov ebp,[esp+28h]
        MACHINE.regs.ebp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x28u32));
        // 0040175f mov [esp+10h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.eax);
        // 00401763 mov [esp+14h],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32), MACHINE.regs.ecx);
        // 00401767 add eax,esi
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401769 mov esi,[esp+24h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x24u32));
        // 0040176d add ecx,edx
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 0040176f mov [esp+18h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32), MACHINE.regs.eax);
        // 00401773 mov [esp+1Ch],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x1cu32), MACHINE.regs.ecx);
        // 00401777 mov eax,[edi+1Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x1cu32));
        // 0040177a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040177c mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 0040177e jge short 00401784h
        jge(Cont(x00401780), Cont(x00401784))
    }
}

pub fn x0040174f() -> Cont {
    unsafe {
        // 0040174f mov eax,[esp+30h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x30u32));
        // 00401753 mov ecx,[esp+34h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32));
        // 00401757 mov ebx,[esp+2Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x2cu32));
        // 0040175b mov ebp,[esp+28h]
        MACHINE.regs.ebp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x28u32));
        // 0040175f mov [esp+10h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.eax);
        // 00401763 mov [esp+14h],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32), MACHINE.regs.ecx);
        // 00401767 add eax,esi
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401769 mov esi,[esp+24h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x24u32));
        // 0040176d add ecx,edx
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 0040176f mov [esp+18h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32), MACHINE.regs.eax);
        // 00401773 mov [esp+1Ch],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x1cu32), MACHINE.regs.ecx);
        // 00401777 mov eax,[edi+1Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x1cu32));
        // 0040177a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040177c mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 0040177e jge short 00401784h
        jge(Cont(x00401780), Cont(x00401784))
    }
}

pub fn x00401777() -> Cont {
    unsafe {
        // 00401777 mov eax,[edi+1Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x1cu32));
        // 0040177a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040177c mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 0040177e jge short 00401784h
        jge(Cont(x00401780), Cont(x00401784))
    }
}

pub fn x00401780() -> Cont {
    unsafe {
        // 00401780 push 0
        push(0x0u32);
        // 00401782 jmp short 00401786h
        Cont(x00401786)
    }
}

pub fn x00401784() -> Cont {
    unsafe {
        // 00401784 push 1
        push(0x1u32);
        // 00401786 mov edx,[edi+28h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x28u32));
        // 00401789 lea ecx,[esp+14h]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0x14u32);
        // 0040178d push ecx
        push(MACHINE.regs.ecx);
        // 0040178e push edx
        push(MACHINE.regs.edx);
        // 0040178f push ebx
        push(MACHINE.regs.ebx);
        // 00401790 push ebp
        push(MACHINE.regs.ebp);
        // 00401791 push esi
        push(MACHINE.regs.esi);
        // 00401792 call dword ptr [eax+1Ch]
        call(
            0x401795,
            indirect(MACHINE.memory.read(MACHINE.regs.eax.wrapping_add(0x1cu32))),
        )
    }
}

pub fn x00401786() -> Cont {
    unsafe {
        // 00401786 mov edx,[edi+28h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x28u32));
        // 00401789 lea ecx,[esp+14h]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0x14u32);
        // 0040178d push ecx
        push(MACHINE.regs.ecx);
        // 0040178e push edx
        push(MACHINE.regs.edx);
        // 0040178f push ebx
        push(MACHINE.regs.ebx);
        // 00401790 push ebp
        push(MACHINE.regs.ebp);
        // 00401791 push esi
        push(MACHINE.regs.esi);
        // 00401792 call dword ptr [eax+1Ch]
        call(
            0x401795,
            indirect(MACHINE.memory.read(MACHINE.regs.eax.wrapping_add(0x1cu32))),
        )
    }
}

pub fn x00401795() -> Cont {
    unsafe {
        // 00401795 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401797 je short 004017BCh
        je(Cont(x00401799), Cont(x004017bc))
    }
}

pub fn x00401799() -> Cont {
    unsafe {
        // 00401799 cmp eax,887601C2h
        sub(MACHINE.regs.eax, 0x887601c2u32);
        // 0040179e jne short 004017A9h
        jne(Cont(x004017a0), Cont(x004017a9))
    }
}

pub fn x004017a0() -> Cont {
    unsafe {
        // 004017a0 mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 004017a2 call 004017F0h
        call(0x4017a7, Cont(x004017f0))
    }
}

pub fn x004017a7() -> Cont {
    unsafe {
        // 004017a7 jmp short 00401777h
        Cont(x00401777)
    }
}

pub fn x004017a9() -> Cont {
    unsafe {
        // 004017a9 cmp eax,8876021Ch
        sub(MACHINE.regs.eax, 0x8876021cu32);
        // 004017ae je short 00401777h
        je(Cont(x004017b0), Cont(x00401777))
    }
}

pub fn x004017b0() -> Cont {
    unsafe {
        // 004017b0 pop edi
        MACHINE.regs.edi = pop();
        // 004017b1 pop esi
        MACHINE.regs.esi = pop();
        // 004017b2 pop ebp
        MACHINE.regs.ebp = pop();
        // 004017b3 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004017b5 pop ebx
        MACHINE.regs.ebx = pop();
        // 004017b6 add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 004017b9 ret 1Ch
        ret(28)
    }
}

pub fn x004017bc() -> Cont {
    unsafe {
        // 004017bc pop edi
        MACHINE.regs.edi = pop();
        // 004017bd pop esi
        MACHINE.regs.esi = pop();
        // 004017be pop ebp
        MACHINE.regs.ebp = pop();
        // 004017bf mov eax,1
        MACHINE.regs.eax = 0x1u32;
        // 004017c4 pop ebx
        MACHINE.regs.ebx = pop();
        // 004017c5 add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 004017c8 ret 1Ch
        ret(28)
    }
}

pub fn x004017d0() -> Cont {
    unsafe {
        // 004017d0 push esi
        push(MACHINE.regs.esi);
        // 004017d1 mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 004017d3 mov eax,[esi+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32));
        // 004017d6 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004017d8 je short 004017E7h
        je(Cont(x004017da), Cont(x004017e7))
    }
}

pub fn x004017da() -> Cont {
    unsafe {
        // 004017da mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004017dc push eax
        push(MACHINE.regs.eax);
        // 004017dd call dword ptr [ecx+8]
        call(
            0x4017e0,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x8u32))),
        )
    }
}

pub fn x004017e0() -> Cont {
    unsafe {
        // 004017e0 mov dword ptr [esi+28h],0
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x28u32), 0x0u32);
        // 004017e7 pop esi
        MACHINE.regs.esi = pop();
        // 004017e8 ret
        ret(0)
    }
}

pub fn x004017e7() -> Cont {
    unsafe {
        // 004017e7 pop esi
        MACHINE.regs.esi = pop();
        // 004017e8 ret
        ret(0)
    }
}

pub fn x004017f0() -> Cont {
    unsafe {
        // 004017f0 mov eax,[ecx+28h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x28u32));
        // 004017f3 push eax
        push(MACHINE.regs.eax);
        // 004017f4 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004017f6 call dword ptr [ecx+6Ch]
        call(
            0x4017f9,
            indirect(MACHINE.memory.read(MACHINE.regs.ecx.wrapping_add(0x6cu32))),
        )
    }
}

pub fn x004017f9() -> Cont {
    unsafe {
        // 004017f9 ret
        ret(0)
    }
}

pub fn x00401800() -> Cont {
    unsafe {
        // 00401800 jmp dword ptr ds:[406000h]
        Cont(ddraw::DirectDrawCreateEx_stdcall)
    }
}

pub fn x00401806() -> Cont {
    unsafe {
        // 00401806 push esi
        push(MACHINE.regs.esi);
        // 00401807 push dword ptr ds:[409AB0h]
        push(MACHINE.memory.read::<u32>(0x409ab0u32));
        // 0040180d call 00401DA0h
        call(0x401812, Cont(x00401da0))
    }
}

pub fn x00401812() -> Cont {
    unsafe {
        // 00401812 mov edx,ds:[409AB0h]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x409ab0u32);
        // 00401818 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401819 mov ecx,ds:[409AACh]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409aacu32);
        // 0040181f mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 00401821 sub esi,edx
        MACHINE.regs.esi = sub(MACHINE.regs.esi, MACHINE.regs.edx);
        // 00401823 add esi,4
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x4u32);
        // 00401826 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401828 pop esi
        MACHINE.regs.esi = pop();
        // 00401829 jae short 00401865h
        jae(Cont(x0040182b), Cont(x00401865))
    }
}

pub fn x0040182b() -> Cont {
    unsafe {
        // 0040182b push edx
        push(MACHINE.regs.edx);
        // 0040182c call 00401DA0h
        call(0x401831, Cont(x00401da0))
    }
}

pub fn x00401831() -> Cont {
    unsafe {
        // 00401831 add eax,10h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x10u32);
        // 00401834 push eax
        push(MACHINE.regs.eax);
        // 00401835 push dword ptr ds:[409AB0h]
        push(MACHINE.memory.read::<u32>(0x409ab0u32));
        // 0040183b call 004019FEh
        call(0x401840, Cont(x004019fe))
    }
}

pub fn x00401840() -> Cont {
    unsafe {
        // 00401840 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401843 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401845 jne short 00401848h
        jne(Cont(x00401847), Cont(x00401848))
    }
}

pub fn x00401847() -> Cont {
    unsafe {
        // 00401847 ret
        ret(0)
    }
}

pub fn x00401848() -> Cont {
    unsafe {
        // 00401848 mov ecx,ds:[409AACh]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409aacu32);
        // 0040184e sub ecx,ds:[409AB0h]
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.memory.read::<u32>(0x409ab0u32));
        // 00401854 mov ds:[409AB0h],eax
        MACHINE.memory.write::<u32>(0x409ab0u32, MACHINE.regs.eax);
        // 00401859 sar ecx,2
        sar();
        // 0040185c lea ecx,[eax+ecx*4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0x0u32);
        // 0040185f mov ds:[409AACh],ecx
        MACHINE.memory.write::<u32>(0x409aacu32, MACHINE.regs.ecx);
        // 00401865 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00401869 mov [ecx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 0040186b add dword ptr ds:[409AACh],4
        MACHINE.memory.write::<u32>(
            0x409aacu32,
            add(MACHINE.memory.read::<u32>(0x409aacu32), 0x4u32),
        );
        // 00401872 ret
        ret(0)
    }
}

pub fn x00401865() -> Cont {
    unsafe {
        // 00401865 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00401869 mov [ecx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 0040186b add dword ptr ds:[409AACh],4
        MACHINE.memory.write::<u32>(
            0x409aacu32,
            add(MACHINE.memory.read::<u32>(0x409aacu32), 0x4u32),
        );
        // 00401872 ret
        ret(0)
    }
}

pub fn x00401873() -> Cont {
    unsafe {
        // 00401873 push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 00401877 call 00401806h
        call(0x40187c, Cont(x00401806))
    }
}

pub fn x0040187c() -> Cont {
    unsafe {
        // 0040187c neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 0040187e sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401880 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401881 neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 00401883 dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 00401884 ret
        ret(0)
    }
}

pub fn x00401885() -> Cont {
    unsafe {
        // 00401885 push 80h
        push(0x80u32);
        // 0040188a call 00401E01h
        call(0x40188f, Cont(x00401e01))
    }
}

pub fn x0040188f() -> Cont {
    unsafe {
        // 0040188f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401891 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401892 mov ds:[409AB0h],eax
        MACHINE.memory.write::<u32>(0x409ab0u32, MACHINE.regs.eax);
        // 00401897 jne short 004018A6h
        jne(Cont(x00401899), Cont(x004018a6))
    }
}

pub fn x00401899() -> Cont {
    unsafe {
        // 00401899 push 18h
        push(0x18u32);
        // 0040189b call 004019B5h
        call(0x4018a0, Cont(x004019b5))
    }
}

pub fn x004018a0() -> Cont {
    unsafe {
        // 004018a0 mov eax,ds:[409AB0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab0u32);
        // 004018a5 pop ecx
        MACHINE.regs.ecx = pop();
        // 004018a6 and dword ptr [eax],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 004018a9 mov eax,ds:[409AB0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab0u32);
        // 004018ae mov ds:[409AACh],eax
        MACHINE.memory.write::<u32>(0x409aacu32, MACHINE.regs.eax);
        // 004018b3 ret
        ret(0)
    }
}

pub fn x004018a6() -> Cont {
    unsafe {
        // 004018a6 and dword ptr [eax],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 004018a9 mov eax,ds:[409AB0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab0u32);
        // 004018ae mov ds:[409AACh],eax
        MACHINE.memory.write::<u32>(0x409aacu32, MACHINE.regs.eax);
        // 004018b3 ret
        ret(0)
    }
}

pub fn x004018b4() -> Cont {
    unsafe {
        // 004018b4 push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 004018b8 call 00401EB3h
        call(0x4018bd, Cont(x00401eb3))
    }
}

pub fn x004018bd() -> Cont {
    unsafe {
        // 004018bd pop ecx
        MACHINE.regs.ecx = pop();
        // 004018be ret
        ret(0)
    }
}

pub fn x004018bf() -> Cont {
    unsafe {
        // 004018bf push ebp
        push(MACHINE.regs.ebp);
        // 004018c0 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004018c2 push 0FFFFFFFFh
        push(0xffffffffu32);
        // 004018c4 push 406128h
        push(0x406128u32);
        // 004018c9 push 4029A8h
        push(0x4029a8u32);
        // 004018ce mov eax,fs:[0]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32));
        // 004018d4 push eax
        push(MACHINE.regs.eax);
        // 004018d5 mov fs:[0],esp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.esp);
        // 004018dc sub esp,58h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x58u32);
        // 004018df push ebx
        push(MACHINE.regs.ebx);
        // 004018e0 push esi
        push(MACHINE.regs.esi);
        // 004018e1 push edi
        push(MACHINE.regs.edi);
        // 004018e2 mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 004018e5 call dword ptr ds:[406074h]
        call(0x4018eb, Cont(kernel32::GetVersion_stdcall))
    }
}

pub fn x004018eb() -> Cont {
    unsafe {
        // 004018eb xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 004018ed mov dl,ah
        MACHINE.regs.set_dl(MACHINE.regs.get_ah());
        // 004018ef mov ds:[4095C0h],edx
        MACHINE.memory.write::<u32>(0x4095c0u32, MACHINE.regs.edx);
        // 004018f5 mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 004018f7 and ecx,0FFh
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0xffu32);
        // 004018fd mov ds:[4095BCh],ecx
        MACHINE.memory.write::<u32>(0x4095bcu32, MACHINE.regs.ecx);
        // 00401903 shl ecx,8
        MACHINE.regs.ecx = shl(MACHINE.regs.ecx, 0x8u8);
        // 00401906 add ecx,edx
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 00401908 mov ds:[4095B8h],ecx
        MACHINE.memory.write::<u32>(0x4095b8u32, MACHINE.regs.ecx);
        // 0040190e shr eax,10h
        MACHINE.regs.eax = shr(MACHINE.regs.eax, 0x10u8);
        // 00401911 mov ds:[4095B4h],eax
        MACHINE.memory.write::<u32>(0x4095b4u32, MACHINE.regs.eax);
        // 00401916 xor esi,esi
        MACHINE.regs.esi ^= MACHINE.regs.esi;
        // 00401918 push esi
        push(MACHINE.regs.esi);
        // 00401919 call 00402850h
        call(0x40191e, Cont(x00402850))
    }
}

pub fn x0040191e() -> Cont {
    unsafe {
        // 0040191e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040191f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401921 jne short 0040192Bh
        jne(Cont(x00401923), Cont(x0040192b))
    }
}

pub fn x00401923() -> Cont {
    unsafe {
        // 00401923 push 1Ch
        push(0x1cu32);
        // 00401925 call 004019DAh
        call(0x40192a, Cont(x004019da))
    }
}

pub fn x0040192a() -> Cont {
    unsafe {
        // 0040192a pop ecx
        MACHINE.regs.ecx = pop();
        // 0040192b mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 0040192e call 00402530h
        call(0x401933, Cont(x00402530))
    }
}

pub fn x0040192b() -> Cont {
    unsafe {
        // 0040192b mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 0040192e call 00402530h
        call(0x401933, Cont(x00402530))
    }
}

pub fn x00401933() -> Cont {
    unsafe {
        // 00401933 call dword ptr ds:[406070h]
        call(0x401939, Cont(kernel32::GetCommandLineA_stdcall))
    }
}

pub fn x00401939() -> Cont {
    unsafe {
        // 00401939 mov ds:[409AB8h],eax
        MACHINE.memory.write::<u32>(0x409ab8u32, MACHINE.regs.eax);
        // 0040193e call 004023FEh
        call(0x401943, Cont(x004023fe))
    }
}

pub fn x00401943() -> Cont {
    unsafe {
        // 00401943 mov ds:[40959Ch],eax
        MACHINE.memory.write::<u32>(0x40959cu32, MACHINE.regs.eax);
        // 00401948 call 004021B1h
        call(0x40194d, Cont(x004021b1))
    }
}

pub fn x0040194d() -> Cont {
    unsafe {
        // 0040194d call 004020F8h
        call(0x401952, Cont(x004020f8))
    }
}

pub fn x00401952() -> Cont {
    unsafe {
        // 00401952 call 00401C9Eh
        call(0x401957, Cont(x00401c9e))
    }
}

pub fn x00401957() -> Cont {
    unsafe {
        // 00401957 mov [ebp-30h],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffd0u32),
            MACHINE.regs.esi,
        );
        // 0040195a lea eax,[ebp-5Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffffa4u32);
        // 0040195d push eax
        push(MACHINE.regs.eax);
        // 0040195e call dword ptr ds:[40606Ch]
        call(0x401964, Cont(kernel32::GetStartupInfoA_stdcall))
    }
}

pub fn x00401964() -> Cont {
    unsafe {
        // 00401964 call 004020A0h
        call(0x401969, Cont(x004020a0))
    }
}

pub fn x00401969() -> Cont {
    unsafe {
        // 00401969 mov [ebp-64h],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffff9cu32),
            MACHINE.regs.eax,
        );
        // 0040196c test byte ptr [ebp-30h],1
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xffffffd0u32)),
            0x1u8,
        );
        // 00401970 je short 00401978h
        je(Cont(x00401972), Cont(x00401978))
    }
}

pub fn x00401972() -> Cont {
    unsafe {
        // 00401972 movzx eax,word ptr [ebp-2Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u16>(MACHINE.regs.ebp.wrapping_add(0xffffffd4u32))
            as _;
        // 00401976 jmp short 0040197Bh
        Cont(x0040197b)
    }
}

pub fn x00401978() -> Cont {
    unsafe {
        // 00401978 push 0Ah
        push(0xau32);
        // 0040197a pop eax
        MACHINE.regs.eax = pop();
        // 0040197b push eax
        push(MACHINE.regs.eax);
        // 0040197c push dword ptr [ebp-64h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff9cu32)),
        );
        // 0040197f push esi
        push(MACHINE.regs.esi);
        // 00401980 push esi
        push(MACHINE.regs.esi);
        // 00401981 call dword ptr ds:[406068h]
        call(0x401987, Cont(kernel32::GetModuleHandleA_stdcall))
    }
}

pub fn x0040197b() -> Cont {
    unsafe {
        // 0040197b push eax
        push(MACHINE.regs.eax);
        // 0040197c push dword ptr [ebp-64h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff9cu32)),
        );
        // 0040197f push esi
        push(MACHINE.regs.esi);
        // 00401980 push esi
        push(MACHINE.regs.esi);
        // 00401981 call dword ptr ds:[406068h]
        call(0x401987, Cont(kernel32::GetModuleHandleA_stdcall))
    }
}

pub fn x00401987() -> Cont {
    unsafe {
        // 00401987 push eax
        push(MACHINE.regs.eax);
        // 00401988 call 00401040h
        call(0x40198d, Cont(x00401040))
    }
}

pub fn x0040198d() -> Cont {
    unsafe {
        // 0040198d mov [ebp-60h],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffa0u32),
            MACHINE.regs.eax,
        );
        // 00401990 push eax
        push(MACHINE.regs.eax);
        // 00401991 call 00401CCBh
        call(0x401996, Cont(x00401ccb))
    }
}

pub fn x00401996() -> Cont {
    unsafe {
        // 00401996 mov eax,[ebp-14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffecu32));
        // 00401999 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 0040199b mov ecx,[ecx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 0040199d mov [ebp-68h],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffff98u32),
            MACHINE.regs.ecx,
        );
        // 004019a0 push eax
        push(MACHINE.regs.eax);
        // 004019a1 push ecx
        push(MACHINE.regs.ecx);
        // 004019a2 call 00401F1Ch
        call(0x4019a7, Cont(x00401f1c))
    }
}

pub fn x004019a7() -> Cont {
    unsafe {
        // 004019a7 pop ecx
        MACHINE.regs.ecx = pop();
        // 004019a8 pop ecx
        MACHINE.regs.ecx = pop();
        // 004019a9 ret
        ret(0)
    }
}

pub fn x004019aa() -> Cont {
    unsafe {
        // 004019aa mov esp,[ebp-18h]
        MACHINE.regs.esp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe8u32));
        // 004019ad push dword ptr [ebp-68h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff98u32)),
        );
        // 004019b0 call 00401CDCh
        call(0x4019b5, Cont(x00401cdc))
    }
}

pub fn x004019b5() -> Cont {
    unsafe {
        // 004019b5 cmp dword ptr ds:[4095A4h],1
        sub(MACHINE.memory.read::<u32>(0x4095a4u32), 0x1u32);
        // 004019bc jne short 004019C3h
        jne(Cont(x004019be), Cont(x004019c3))
    }
}

pub fn x004019be() -> Cont {
    unsafe {
        // 004019be call 00402A80h
        call(0x4019c3, Cont(x00402a80))
    }
}

pub fn x004019c3() -> Cont {
    unsafe {
        // 004019c3 push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 004019c7 call 00402AB9h
        call(0x4019cc, Cont(x00402ab9))
    }
}

pub fn x004019cc() -> Cont {
    unsafe {
        // 004019cc push 0FFh
        push(0xffu32);
        // 004019d1 call dword ptr ds:[4070C4h]
        call(0x4019d7, indirect(MACHINE.memory.read(0x4070c4u32)))
    }
}

pub fn x004019d7() -> Cont {
    unsafe {
        // 004019d7 pop ecx
        MACHINE.regs.ecx = pop();
        // 004019d8 pop ecx
        MACHINE.regs.ecx = pop();
        // 004019d9 ret
        ret(0)
    }
}

pub fn x004019da() -> Cont {
    unsafe {
        // 004019da cmp dword ptr ds:[4095A4h],1
        sub(MACHINE.memory.read::<u32>(0x4095a4u32), 0x1u32);
        // 004019e1 jne short 004019E8h
        jne(Cont(x004019e3), Cont(x004019e8))
    }
}

pub fn x004019e3() -> Cont {
    unsafe {
        // 004019e3 call 00402A80h
        call(0x4019e8, Cont(x00402a80))
    }
}

pub fn x004019e8() -> Cont {
    unsafe {
        // 004019e8 push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 004019ec call 00402AB9h
        call(0x4019f1, Cont(x00402ab9))
    }
}

pub fn x004019f1() -> Cont {
    unsafe {
        // 004019f1 pop ecx
        MACHINE.regs.ecx = pop();
        // 004019f2 push 0FFh
        push(0xffu32);
        // 004019f7 call dword ptr ds:[406078h]
        call(0x4019fd, Cont(kernel32::ExitProcess_stdcall))
    }
}

pub fn x004019fd() -> Cont {
    unsafe {
        // 004019fd ret
        ret(0)
    }
}

pub fn x004019fe() -> Cont {
    unsafe {
        // 004019fe push ebp
        push(MACHINE.regs.ebp);
        // 004019ff mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401a01 push ecx
        push(MACHINE.regs.ecx);
        // 00401a02 cmp dword ptr [ebp+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 00401a06 push ebx
        push(MACHINE.regs.ebx);
        // 00401a07 push esi
        push(MACHINE.regs.esi);
        // 00401a08 push edi
        push(MACHINE.regs.edi);
        // 00401a09 jne short 00401A19h
        jne(Cont(x00401a0b), Cont(x00401a19))
    }
}

pub fn x00401a0b() -> Cont {
    unsafe {
        // 00401a0b push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00401a0e call 00401E01h
        call(0x401a13, Cont(x00401e01))
    }
}

pub fn x00401a13() -> Cont {
    unsafe {
        // 00401a13 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401a14 jmp near ptr 00401C99h
        Cont(x00401c99)
    }
}

pub fn x00401a19() -> Cont {
    unsafe {
        // 00401a19 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00401a1c test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401a1e jne short 00401A2Eh
        jne(Cont(x00401a20), Cont(x00401a2e))
    }
}

pub fn x00401a20() -> Cont {
    unsafe {
        // 00401a20 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401a23 call 00401EB3h
        call(0x401a28, Cont(x00401eb3))
    }
}

pub fn x00401a28() -> Cont {
    unsafe {
        // 00401a28 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401a29 jmp near ptr 00401C97h
        Cont(x00401c97)
    }
}

pub fn x00401a2e() -> Cont {
    unsafe {
        // 00401a2e mov eax,ds:[409988h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409988u32);
        // 00401a33 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00401a36 jne near ptr 00401B3Eh
        jne(Cont(x00401a3c), Cont(x00401b3e))
    }
}

pub fn x00401a3c() -> Cont {
    unsafe {
        // 00401a3c xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401a3e cmp esi,0FFFFFFE0h
        sub(MACHINE.regs.esi, 0xffffffe0u32);
        // 00401a41 ja near ptr 00401B1Ah
        ja(Cont(x00401a47), Cont(x00401b1a))
    }
}

pub fn x00401a47() -> Cont {
    unsafe {
        // 00401a47 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401a4a call 00402C54h
        call(0x401a4f, Cont(x00402c54))
    }
}

pub fn x00401a4f() -> Cont {
    unsafe {
        // 00401a4f mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00401a51 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401a52 test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401a54 je near ptr 00401AF5h
        je(Cont(x00401a5a), Cont(x00401af5))
    }
}

pub fn x00401a5a() -> Cont {
    unsafe {
        // 00401a5a cmp esi,ds:[409980h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409980u32));
        // 00401a60 ja short 00401AAEh
        ja(Cont(x00401a62), Cont(x00401aae))
    }
}

pub fn x00401a62() -> Cont {
    unsafe {
        // 00401a62 mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401a65 push esi
        push(MACHINE.regs.esi);
        // 00401a66 push edi
        push(MACHINE.regs.edi);
        // 00401a67 push ebx
        push(MACHINE.regs.ebx);
        // 00401a68 call 0040345Dh
        call(0x401a6d, Cont(x0040345d))
    }
}

pub fn x00401a6d() -> Cont {
    unsafe {
        // 00401a6d add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401a70 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401a72 jne short 00401AAAh
        jne(Cont(x00401a74), Cont(x00401aaa))
    }
}

pub fn x00401a74() -> Cont {
    unsafe {
        // 00401a74 push esi
        push(MACHINE.regs.esi);
        // 00401a75 call 00402FA8h
        call(0x401a7a, Cont(x00402fa8))
    }
}

pub fn x00401a7a() -> Cont {
    unsafe {
        // 00401a7a mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401a7c pop ecx
        MACHINE.regs.ecx = pop();
        // 00401a7d test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401a7f je short 00401AAEh
        je(Cont(x00401a81), Cont(x00401aae))
    }
}

pub fn x00401a81() -> Cont {
    unsafe {
        // 00401a81 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401a84 mov eax,[ebx-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0xfffffffcu32));
        // 00401a87 dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 00401a88 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401a8a jb short 00401A8Eh
        jb(Cont(x00401a8c), Cont(x00401a8e))
    }
}

pub fn x00401a8c() -> Cont {
    unsafe {
        // 00401a8c mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00401a8e push eax
        push(MACHINE.regs.eax);
        // 00401a8f push ebx
        push(MACHINE.regs.ebx);
        // 00401a90 push edi
        push(MACHINE.regs.edi);
        // 00401a91 call 00403E40h
        call(0x401a96, Cont(x00403e40))
    }
}

pub fn x00401a8e() -> Cont {
    unsafe {
        // 00401a8e push eax
        push(MACHINE.regs.eax);
        // 00401a8f push ebx
        push(MACHINE.regs.ebx);
        // 00401a90 push edi
        push(MACHINE.regs.edi);
        // 00401a91 call 00403E40h
        call(0x401a96, Cont(x00403e40))
    }
}

pub fn x00401a96() -> Cont {
    unsafe {
        // 00401a96 push ebx
        push(MACHINE.regs.ebx);
        // 00401a97 call 00402C54h
        call(0x401a9c, Cont(x00402c54))
    }
}

pub fn x00401a9c() -> Cont {
    unsafe {
        // 00401a9c push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401a9f mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00401aa1 push ebx
        push(MACHINE.regs.ebx);
        // 00401aa2 call 00402C7Fh
        call(0x401aa7, Cont(x00402c7f))
    }
}

pub fn x00401aa7() -> Cont {
    unsafe {
        // 00401aa7 add esp,18h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x18u32);
        // 00401aaa test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401aac jne short 00401AF1h
        jne(Cont(x00401aae), Cont(x00401af1))
    }
}

pub fn x00401aaa() -> Cont {
    unsafe {
        // 00401aaa test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401aac jne short 00401AF1h
        jne(Cont(x00401aae), Cont(x00401af1))
    }
}

pub fn x00401aae() -> Cont {
    unsafe {
        // 00401aae test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401ab0 jne short 00401AB5h
        jne(Cont(x00401ab2), Cont(x00401ab5))
    }
}

pub fn x00401ab2() -> Cont {
    unsafe {
        // 00401ab2 push 1
        push(0x1u32);
        // 00401ab4 pop esi
        MACHINE.regs.esi = pop();
        // 00401ab5 add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401ab8 and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401abb push esi
        push(MACHINE.regs.esi);
        // 00401abc push 0
        push(0x0u32);
        // 00401abe push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401ac4 call dword ptr ds:[406028h]
        call(0x401aca, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401ab5() -> Cont {
    unsafe {
        // 00401ab5 add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401ab8 and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401abb push esi
        push(MACHINE.regs.esi);
        // 00401abc push 0
        push(0x0u32);
        // 00401abe push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401ac4 call dword ptr ds:[406028h]
        call(0x401aca, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401aca() -> Cont {
    unsafe {
        // 00401aca mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401acc test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401ace je short 00401AF1h
        je(Cont(x00401ad0), Cont(x00401af1))
    }
}

pub fn x00401ad0() -> Cont {
    unsafe {
        // 00401ad0 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401ad3 mov eax,[ecx-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xfffffffcu32));
        // 00401ad6 dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 00401ad7 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401ad9 jb short 00401ADDh
        jb(Cont(x00401adb), Cont(x00401add))
    }
}

pub fn x00401adb() -> Cont {
    unsafe {
        // 00401adb mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00401add push eax
        push(MACHINE.regs.eax);
        // 00401ade push ecx
        push(MACHINE.regs.ecx);
        // 00401adf push edi
        push(MACHINE.regs.edi);
        // 00401ae0 call 00403E40h
        call(0x401ae5, Cont(x00403e40))
    }
}

pub fn x00401add() -> Cont {
    unsafe {
        // 00401add push eax
        push(MACHINE.regs.eax);
        // 00401ade push ecx
        push(MACHINE.regs.ecx);
        // 00401adf push edi
        push(MACHINE.regs.edi);
        // 00401ae0 call 00403E40h
        call(0x401ae5, Cont(x00403e40))
    }
}

pub fn x00401ae5() -> Cont {
    unsafe {
        // 00401ae5 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401ae8 push ebx
        push(MACHINE.regs.ebx);
        // 00401ae9 call 00402C7Fh
        call(0x401aee, Cont(x00402c7f))
    }
}

pub fn x00401aee() -> Cont {
    unsafe {
        // 00401aee add esp,14h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x14u32);
        // 00401af1 test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401af3 jne short 00401B16h
        jne(Cont(x00401af5), Cont(x00401b16))
    }
}

pub fn x00401af1() -> Cont {
    unsafe {
        // 00401af1 test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401af3 jne short 00401B16h
        jne(Cont(x00401af5), Cont(x00401b16))
    }
}

pub fn x00401af5() -> Cont {
    unsafe {
        // 00401af5 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401af7 jne short 00401AFCh
        jne(Cont(x00401af9), Cont(x00401afc))
    }
}

pub fn x00401af9() -> Cont {
    unsafe {
        // 00401af9 push 1
        push(0x1u32);
        // 00401afb pop esi
        MACHINE.regs.esi = pop();
        // 00401afc add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401aff and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401b02 push esi
        push(MACHINE.regs.esi);
        // 00401b03 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401b06 push 0
        push(0x0u32);
        // 00401b08 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401b0e call dword ptr ds:[40607Ch]
        call(0x401b14, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x00401afc() -> Cont {
    unsafe {
        // 00401afc add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401aff and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401b02 push esi
        push(MACHINE.regs.esi);
        // 00401b03 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401b06 push 0
        push(0x0u32);
        // 00401b08 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401b0e call dword ptr ds:[40607Ch]
        call(0x401b14, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x00401b14() -> Cont {
    unsafe {
        // 00401b14 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401b16 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401b18 jne short 00401B37h
        jne(Cont(x00401b1a), Cont(x00401b37))
    }
}

pub fn x00401b16() -> Cont {
    unsafe {
        // 00401b16 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401b18 jne short 00401B37h
        jne(Cont(x00401b1a), Cont(x00401b37))
    }
}

pub fn x00401b1a() -> Cont {
    unsafe {
        // 00401b1a cmp dword ptr ds:[40970Ch],0
        sub(MACHINE.memory.read::<u32>(0x40970cu32), 0x0u32);
        // 00401b21 je short 00401B37h
        je(Cont(x00401b23), Cont(x00401b37))
    }
}

pub fn x00401b23() -> Cont {
    unsafe {
        // 00401b23 push esi
        push(MACHINE.regs.esi);
        // 00401b24 call 00403E20h
        call(0x401b29, Cont(x00403e20))
    }
}

pub fn x00401b29() -> Cont {
    unsafe {
        // 00401b29 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401b2b pop ecx
        MACHINE.regs.ecx = pop();
        // 00401b2c jne near ptr 00401A3Ch
        jne(Cont(x00401b32), Cont(x00401a3c))
    }
}

pub fn x00401b32() -> Cont {
    unsafe {
        // 00401b32 jmp near ptr 00401C97h
        Cont(x00401c97)
    }
}

pub fn x00401b37() -> Cont {
    unsafe {
        // 00401b37 mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00401b39 jmp near ptr 00401C99h
        Cont(x00401c99)
    }
}

pub fn x00401b3e() -> Cont {
    unsafe {
        // 00401b3e cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 00401b41 jne near ptr 00401C59h
        jne(Cont(x00401b47), Cont(x00401c59))
    }
}

pub fn x00401b47() -> Cont {
    unsafe {
        // 00401b47 cmp esi,0FFFFFFE0h
        sub(MACHINE.regs.esi, 0xffffffe0u32);
        // 00401b4a ja short 00401B5Bh
        ja(Cont(x00401b4c), Cont(x00401b5b))
    }
}

pub fn x00401b4c() -> Cont {
    unsafe {
        // 00401b4c test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401b4e jbe short 00401B58h
        jbe(Cont(x00401b50), Cont(x00401b58))
    }
}

pub fn x00401b50() -> Cont {
    unsafe {
        // 00401b50 add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401b53 and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401b56 jmp short 00401B5Bh
        Cont(x00401b5b)
    }
}

pub fn x00401b58() -> Cont {
    unsafe {
        // 00401b58 push 10h
        push(0x10u32);
        // 00401b5a pop esi
        MACHINE.regs.esi = pop();
        // 00401b5b xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401b5d cmp esi,0FFFFFFE0h
        sub(MACHINE.regs.esi, 0xffffffe0u32);
        // 00401b60 ja near ptr 00401C3Bh
        ja(Cont(x00401b66), Cont(x00401c3b))
    }
}

pub fn x00401b5b() -> Cont {
    unsafe {
        // 00401b5b xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401b5d cmp esi,0FFFFFFE0h
        sub(MACHINE.regs.esi, 0xffffffe0u32);
        // 00401b60 ja near ptr 00401C3Bh
        ja(Cont(x00401b66), Cont(x00401c3b))
    }
}

pub fn x00401b66() -> Cont {
    unsafe {
        // 00401b66 lea eax,[ebp+0Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xcu32);
        // 00401b69 push eax
        push(MACHINE.regs.eax);
        // 00401b6a lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 00401b6d push eax
        push(MACHINE.regs.eax);
        // 00401b6e push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401b71 call 004039AFh
        call(0x401b76, Cont(x004039af))
    }
}

pub fn x00401b76() -> Cont {
    unsafe {
        // 00401b76 mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00401b78 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401b7b test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401b7d je near ptr 00401C1Fh
        je(Cont(x00401b83), Cont(x00401c1f))
    }
}

pub fn x00401b83() -> Cont {
    unsafe {
        // 00401b83 cmp esi,ds:[40922Ch]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x40922cu32));
        // 00401b89 jae short 00401BE3h
        jae(Cont(x00401b8b), Cont(x00401be3))
    }
}

pub fn x00401b8b() -> Cont {
    unsafe {
        // 00401b8b mov edi,esi
        MACHINE.regs.edi = MACHINE.regs.esi;
        // 00401b8d shr edi,4
        MACHINE.regs.edi = shr(MACHINE.regs.edi, 0x4u8);
        // 00401b90 push edi
        push(MACHINE.regs.edi);
        // 00401b91 push ebx
        push(MACHINE.regs.ebx);
        // 00401b92 push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00401b95 push dword ptr [ebp-4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00401b98 call 00403D77h
        call(0x401b9d, Cont(x00403d77))
    }
}

pub fn x00401b9d() -> Cont {
    unsafe {
        // 00401b9d add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 00401ba0 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401ba2 je short 00401BA9h
        je(Cont(x00401ba4), Cont(x00401ba9))
    }
}

pub fn x00401ba4() -> Cont {
    unsafe {
        // 00401ba4 mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401ba7 jmp short 00401BDBh
        Cont(x00401bdb)
    }
}

pub fn x00401ba9() -> Cont {
    unsafe {
        // 00401ba9 push edi
        push(MACHINE.regs.edi);
        // 00401baa call 00403A4Bh
        call(0x401baf, Cont(x00403a4b))
    }
}

pub fn x00401baf() -> Cont {
    unsafe {
        // 00401baf mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401bb1 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401bb2 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401bb4 je short 00401BE3h
        je(Cont(x00401bb6), Cont(x00401be3))
    }
}

pub fn x00401bb6() -> Cont {
    unsafe {
        // 00401bb6 movzx eax,byte ptr [ebx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)) as _;
        // 00401bb9 shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00401bbc cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401bbe jb short 00401BC2h
        jb(Cont(x00401bc0), Cont(x00401bc2))
    }
}

pub fn x00401bc0() -> Cont {
    unsafe {
        // 00401bc0 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00401bc2 push eax
        push(MACHINE.regs.eax);
        // 00401bc3 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401bc6 push edi
        push(MACHINE.regs.edi);
        // 00401bc7 call 00403E40h
        call(0x401bcc, Cont(x00403e40))
    }
}

pub fn x00401bc2() -> Cont {
    unsafe {
        // 00401bc2 push eax
        push(MACHINE.regs.eax);
        // 00401bc3 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401bc6 push edi
        push(MACHINE.regs.edi);
        // 00401bc7 call 00403E40h
        call(0x401bcc, Cont(x00403e40))
    }
}

pub fn x00401bcc() -> Cont {
    unsafe {
        // 00401bcc push ebx
        push(MACHINE.regs.ebx);
        // 00401bcd push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00401bd0 push dword ptr [ebp-4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00401bd3 call 00403A06h
        call(0x401bd8, Cont(x00403a06))
    }
}

pub fn x00401bd8() -> Cont {
    unsafe {
        // 00401bd8 add esp,18h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x18u32);
        // 00401bdb test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401bdd jne near ptr 00401B37h
        jne(Cont(x00401be3), Cont(x00401b37))
    }
}

pub fn x00401bdb() -> Cont {
    unsafe {
        // 00401bdb test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401bdd jne near ptr 00401B37h
        jne(Cont(x00401be3), Cont(x00401b37))
    }
}

pub fn x00401be3() -> Cont {
    unsafe {
        // 00401be3 push esi
        push(MACHINE.regs.esi);
        // 00401be4 push 0
        push(0x0u32);
        // 00401be6 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401bec call dword ptr ds:[406028h]
        call(0x401bf2, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401bf2() -> Cont {
    unsafe {
        // 00401bf2 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401bf4 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401bf6 je short 00401C3Bh
        je(Cont(x00401bf8), Cont(x00401c3b))
    }
}

pub fn x00401bf8() -> Cont {
    unsafe {
        // 00401bf8 movzx eax,byte ptr [ebx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)) as _;
        // 00401bfb shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00401bfe cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00401c00 jb short 00401C04h
        jb(Cont(x00401c02), Cont(x00401c04))
    }
}

pub fn x00401c02() -> Cont {
    unsafe {
        // 00401c02 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00401c04 push eax
        push(MACHINE.regs.eax);
        // 00401c05 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401c08 push edi
        push(MACHINE.regs.edi);
        // 00401c09 call 00403E40h
        call(0x401c0e, Cont(x00403e40))
    }
}

pub fn x00401c04() -> Cont {
    unsafe {
        // 00401c04 push eax
        push(MACHINE.regs.eax);
        // 00401c05 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401c08 push edi
        push(MACHINE.regs.edi);
        // 00401c09 call 00403E40h
        call(0x401c0e, Cont(x00403e40))
    }
}

pub fn x00401c0e() -> Cont {
    unsafe {
        // 00401c0e push ebx
        push(MACHINE.regs.ebx);
        // 00401c0f push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00401c12 push dword ptr [ebp-4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00401c15 call 00403A06h
        call(0x401c1a, Cont(x00403a06))
    }
}

pub fn x00401c1a() -> Cont {
    unsafe {
        // 00401c1a add esp,18h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x18u32);
        // 00401c1d jmp short 00401C33h
        Cont(x00401c33)
    }
}

pub fn x00401c1f() -> Cont {
    unsafe {
        // 00401c1f push esi
        push(MACHINE.regs.esi);
        // 00401c20 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401c23 push 0
        push(0x0u32);
        // 00401c25 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401c2b call dword ptr ds:[40607Ch]
        call(0x401c31, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x00401c31() -> Cont {
    unsafe {
        // 00401c31 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00401c33 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401c35 jne near ptr 00401B37h
        jne(Cont(x00401c3b), Cont(x00401b37))
    }
}

pub fn x00401c33() -> Cont {
    unsafe {
        // 00401c33 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00401c35 jne near ptr 00401B37h
        jne(Cont(x00401c3b), Cont(x00401b37))
    }
}

pub fn x00401c3b() -> Cont {
    unsafe {
        // 00401c3b cmp dword ptr ds:[40970Ch],0
        sub(MACHINE.memory.read::<u32>(0x40970cu32), 0x0u32);
        // 00401c42 je near ptr 00401B37h
        je(Cont(x00401c48), Cont(x00401b37))
    }
}

pub fn x00401c48() -> Cont {
    unsafe {
        // 00401c48 push esi
        push(MACHINE.regs.esi);
        // 00401c49 call 00403E20h
        call(0x401c4e, Cont(x00403e20))
    }
}

pub fn x00401c4e() -> Cont {
    unsafe {
        // 00401c4e test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401c50 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401c51 jne near ptr 00401B5Bh
        jne(Cont(x00401c57), Cont(x00401b5b))
    }
}

pub fn x00401c57() -> Cont {
    unsafe {
        // 00401c57 jmp short 00401C97h
        Cont(x00401c97)
    }
}

pub fn x00401c59() -> Cont {
    unsafe {
        // 00401c59 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401c5b cmp esi,0FFFFFFE0h
        sub(MACHINE.regs.esi, 0xffffffe0u32);
        // 00401c5e ja short 00401C83h
        ja(Cont(x00401c60), Cont(x00401c83))
    }
}

pub fn x00401c60() -> Cont {
    unsafe {
        // 00401c60 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401c62 jne short 00401C67h
        jne(Cont(x00401c64), Cont(x00401c67))
    }
}

pub fn x00401c64() -> Cont {
    unsafe {
        // 00401c64 push 1
        push(0x1u32);
        // 00401c66 pop esi
        MACHINE.regs.esi = pop();
        // 00401c67 add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401c6a and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401c6d push esi
        push(MACHINE.regs.esi);
        // 00401c6e push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401c71 push 0
        push(0x0u32);
        // 00401c73 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401c79 call dword ptr ds:[40607Ch]
        call(0x401c7f, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x00401c67() -> Cont {
    unsafe {
        // 00401c67 add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401c6a and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401c6d push esi
        push(MACHINE.regs.esi);
        // 00401c6e push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401c71 push 0
        push(0x0u32);
        // 00401c73 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401c79 call dword ptr ds:[40607Ch]
        call(0x401c7f, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x00401c7f() -> Cont {
    unsafe {
        // 00401c7f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401c81 jne short 00401C99h
        jne(Cont(x00401c83), Cont(x00401c99))
    }
}

pub fn x00401c83() -> Cont {
    unsafe {
        // 00401c83 cmp dword ptr ds:[40970Ch],0
        sub(MACHINE.memory.read::<u32>(0x40970cu32), 0x0u32);
        // 00401c8a je short 00401C99h
        je(Cont(x00401c8c), Cont(x00401c99))
    }
}

pub fn x00401c8c() -> Cont {
    unsafe {
        // 00401c8c push esi
        push(MACHINE.regs.esi);
        // 00401c8d call 00403E20h
        call(0x401c92, Cont(x00403e20))
    }
}

pub fn x00401c92() -> Cont {
    unsafe {
        // 00401c92 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401c94 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401c95 jne short 00401C59h
        jne(Cont(x00401c97), Cont(x00401c59))
    }
}

pub fn x00401c97() -> Cont {
    unsafe {
        // 00401c97 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401c99 pop edi
        MACHINE.regs.edi = pop();
        // 00401c9a pop esi
        MACHINE.regs.esi = pop();
        // 00401c9b pop ebx
        MACHINE.regs.ebx = pop();
        // 00401c9c leave
        leave();
        // 00401c9d ret
        ret(0)
    }
}

pub fn x00401c99() -> Cont {
    unsafe {
        // 00401c99 pop edi
        MACHINE.regs.edi = pop();
        // 00401c9a pop esi
        MACHINE.regs.esi = pop();
        // 00401c9b pop ebx
        MACHINE.regs.ebx = pop();
        // 00401c9c leave
        leave();
        // 00401c9d ret
        ret(0)
    }
}

pub fn x00401c9e() -> Cont {
    unsafe {
        // 00401c9e mov eax,ds:[409AB4h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab4u32);
        // 00401ca3 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401ca5 je short 00401CA9h
        je(Cont(x00401ca7), Cont(x00401ca9))
    }
}

pub fn x00401ca7() -> Cont {
    unsafe {
        // 00401ca7 call eax
        call(0x401ca9, indirect(MACHINE.regs.eax))
    }
}

pub fn x00401ca9() -> Cont {
    unsafe {
        // 00401ca9 push 407018h
        push(0x407018u32);
        // 00401cae push 40700Ch
        push(0x40700cu32);
        // 00401cb3 call 00401D86h
        call(0x401cb8, Cont(x00401d86))
    }
}

pub fn x00401cb8() -> Cont {
    unsafe {
        // 00401cb8 push 407008h
        push(0x407008u32);
        // 00401cbd push 407000h
        push(0x407000u32);
        // 00401cc2 call 00401D86h
        call(0x401cc7, Cont(x00401d86))
    }
}

pub fn x00401cc7() -> Cont {
    unsafe {
        // 00401cc7 add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 00401cca ret
        ret(0)
    }
}

pub fn x00401ccb() -> Cont {
    unsafe {
        // 00401ccb push 0
        push(0x0u32);
        // 00401ccd push 0
        push(0x0u32);
        // 00401ccf push dword ptr [esp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 00401cd3 call 00401CEDh
        call(0x401cd8, Cont(x00401ced))
    }
}

pub fn x00401cd8() -> Cont {
    unsafe {
        // 00401cd8 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401cdb ret
        ret(0)
    }
}

pub fn x00401cdc() -> Cont {
    unsafe {
        // 00401cdc push 0
        push(0x0u32);
        // 00401cde push 1
        push(0x1u32);
        // 00401ce0 push dword ptr [esp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 00401ce4 call 00401CEDh
        call(0x401ce9, Cont(x00401ced))
    }
}

pub fn x00401ce9() -> Cont {
    unsafe {
        // 00401ce9 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401cec ret
        ret(0)
    }
}

pub fn x00401ced() -> Cont {
    unsafe {
        // 00401ced push edi
        push(MACHINE.regs.edi);
        // 00401cee push 1
        push(0x1u32);
        // 00401cf0 pop edi
        MACHINE.regs.edi = pop();
        // 00401cf1 cmp ds:[4095F0h],edi
        sub(MACHINE.memory.read::<u32>(0x4095f0u32), MACHINE.regs.edi);
        // 00401cf7 jne short 00401D0Ah
        jne(Cont(x00401cf9), Cont(x00401d0a))
    }
}

pub fn x00401cf9() -> Cont {
    unsafe {
        // 00401cf9 push dword ptr [esp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 00401cfd call dword ptr ds:[406088h]
        call(0x401d03, Cont(kernel32::GetCurrentProcess_stdcall))
    }
}

pub fn x00401d03() -> Cont {
    unsafe {
        // 00401d03 push eax
        push(MACHINE.regs.eax);
        // 00401d04 call dword ptr ds:[406084h]
        call(0x401d0a, Cont(kernel32::TerminateProcess_stdcall))
    }
}

pub fn x00401d0a() -> Cont {
    unsafe {
        // 00401d0a cmp dword ptr [esp+0Ch],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
            0x0u32,
        );
        // 00401d0f push ebx
        push(MACHINE.regs.ebx);
        // 00401d10 mov ebx,[esp+14h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32));
        // 00401d14 mov ds:[4095ECh],edi
        MACHINE.memory.write::<u32>(0x4095ecu32, MACHINE.regs.edi);
        // 00401d1a mov ds:[4095E8h],bl
        MACHINE
            .memory
            .write::<u8>(0x4095e8u32, MACHINE.regs.get_bl());
        // 00401d20 jne short 00401D5Eh
        jne(Cont(x00401d22), Cont(x00401d5e))
    }
}

pub fn x00401d22() -> Cont {
    unsafe {
        // 00401d22 mov eax,ds:[409AB0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab0u32);
        // 00401d27 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401d29 je short 00401D4Dh
        je(Cont(x00401d2b), Cont(x00401d4d))
    }
}

pub fn x00401d2b() -> Cont {
    unsafe {
        // 00401d2b mov ecx,ds:[409AACh]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409aacu32);
        // 00401d31 push esi
        push(MACHINE.regs.esi);
        // 00401d32 lea esi,[ecx-4]
        MACHINE.regs.esi = MACHINE.regs.ecx.wrapping_add(0xfffffffcu32);
        // 00401d35 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00401d37 jb short 00401D4Ch
        jb(Cont(x00401d39), Cont(x00401d4c))
    }
}

pub fn x00401d39() -> Cont {
    unsafe {
        // 00401d39 mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00401d3b test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401d3d je short 00401D41h
        je(Cont(x00401d3f), Cont(x00401d41))
    }
}

pub fn x00401d3f() -> Cont {
    unsafe {
        // 00401d3f call eax
        call(0x401d41, indirect(MACHINE.regs.eax))
    }
}

pub fn x00401d41() -> Cont {
    unsafe {
        // 00401d41 sub esi,4
        MACHINE.regs.esi = sub(MACHINE.regs.esi, 0x4u32);
        // 00401d44 cmp esi,ds:[409AB0h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409ab0u32));
        // 00401d4a jae short 00401D39h
        jae(Cont(x00401d4c), Cont(x00401d39))
    }
}

pub fn x00401d4c() -> Cont {
    unsafe {
        // 00401d4c pop esi
        MACHINE.regs.esi = pop();
        // 00401d4d push 407020h
        push(0x407020u32);
        // 00401d52 push 40701Ch
        push(0x40701cu32);
        // 00401d57 call 00401D86h
        call(0x401d5c, Cont(x00401d86))
    }
}

pub fn x00401d4d() -> Cont {
    unsafe {
        // 00401d4d push 407020h
        push(0x407020u32);
        // 00401d52 push 40701Ch
        push(0x40701cu32);
        // 00401d57 call 00401D86h
        call(0x401d5c, Cont(x00401d86))
    }
}

pub fn x00401d5c() -> Cont {
    unsafe {
        // 00401d5c pop ecx
        MACHINE.regs.ecx = pop();
        // 00401d5d pop ecx
        MACHINE.regs.ecx = pop();
        // 00401d5e push 407028h
        push(0x407028u32);
        // 00401d63 push 407024h
        push(0x407024u32);
        // 00401d68 call 00401D86h
        call(0x401d6d, Cont(x00401d86))
    }
}

pub fn x00401d5e() -> Cont {
    unsafe {
        // 00401d5e push 407028h
        push(0x407028u32);
        // 00401d63 push 407024h
        push(0x407024u32);
        // 00401d68 call 00401D86h
        call(0x401d6d, Cont(x00401d86))
    }
}

pub fn x00401d6d() -> Cont {
    unsafe {
        // 00401d6d pop ecx
        MACHINE.regs.ecx = pop();
        // 00401d6e pop ecx
        MACHINE.regs.ecx = pop();
        // 00401d6f test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401d71 pop ebx
        MACHINE.regs.ebx = pop();
        // 00401d72 jne short 00401D84h
        jne(Cont(x00401d74), Cont(x00401d84))
    }
}

pub fn x00401d74() -> Cont {
    unsafe {
        // 00401d74 push dword ptr [esp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 00401d78 mov ds:[4095F0h],edi
        MACHINE.memory.write::<u32>(0x4095f0u32, MACHINE.regs.edi);
        // 00401d7e call dword ptr ds:[406078h]
        call(0x401d84, Cont(kernel32::ExitProcess_stdcall))
    }
}

pub fn x00401d84() -> Cont {
    unsafe {
        // 00401d84 pop edi
        MACHINE.regs.edi = pop();
        // 00401d85 ret
        ret(0)
    }
}

pub fn x00401d86() -> Cont {
    unsafe {
        // 00401d86 push esi
        push(MACHINE.regs.esi);
        // 00401d87 mov esi,[esp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00401d8b cmp esi,[esp+0Ch]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 00401d8f jae short 00401D9Eh
        jae(Cont(x00401d91), Cont(x00401d9e))
    }
}

pub fn x00401d8b() -> Cont {
    unsafe {
        // 00401d8b cmp esi,[esp+0Ch]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 00401d8f jae short 00401D9Eh
        jae(Cont(x00401d91), Cont(x00401d9e))
    }
}

pub fn x00401d91() -> Cont {
    unsafe {
        // 00401d91 mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00401d93 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401d95 je short 00401D99h
        je(Cont(x00401d97), Cont(x00401d99))
    }
}

pub fn x00401d97() -> Cont {
    unsafe {
        // 00401d97 call eax
        call(0x401d99, indirect(MACHINE.regs.eax))
    }
}

pub fn x00401d99() -> Cont {
    unsafe {
        // 00401d99 add esi,4
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x4u32);
        // 00401d9c jmp short 00401D8Bh
        Cont(x00401d8b)
    }
}

pub fn x00401d9e() -> Cont {
    unsafe {
        // 00401d9e pop esi
        MACHINE.regs.esi = pop();
        // 00401d9f ret
        ret(0)
    }
}

pub fn x00401da0() -> Cont {
    unsafe {
        // 00401da0 push ebp
        push(MACHINE.regs.ebp);
        // 00401da1 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401da3 push ecx
        push(MACHINE.regs.ecx);
        // 00401da4 push ecx
        push(MACHINE.regs.ecx);
        // 00401da5 mov eax,ds:[409988h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409988u32);
        // 00401daa push esi
        push(MACHINE.regs.esi);
        // 00401dab cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00401dae jne short 00401DC9h
        jne(Cont(x00401db0), Cont(x00401dc9))
    }
}

pub fn x00401db0() -> Cont {
    unsafe {
        // 00401db0 mov esi,[ebp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401db3 push esi
        push(MACHINE.regs.esi);
        // 00401db4 call 00402C54h
        call(0x401db9, Cont(x00402c54))
    }
}

pub fn x00401db9() -> Cont {
    unsafe {
        // 00401db9 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401dbb pop ecx
        MACHINE.regs.ecx = pop();
        // 00401dbc je short 00401DC6h
        je(Cont(x00401dbe), Cont(x00401dc6))
    }
}

pub fn x00401dbe() -> Cont {
    unsafe {
        // 00401dbe mov eax,[esi-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0xfffffffcu32));
        // 00401dc1 sub eax,9
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x9u32);
        // 00401dc4 jmp short 00401DFEh
        Cont(x00401dfe)
    }
}

pub fn x00401dc6() -> Cont {
    unsafe {
        // 00401dc6 push esi
        push(MACHINE.regs.esi);
        // 00401dc7 jmp short 00401DF0h
        Cont(x00401df0)
    }
}

pub fn x00401dc9() -> Cont {
    unsafe {
        // 00401dc9 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 00401dcc jne short 00401DEDh
        jne(Cont(x00401dce), Cont(x00401ded))
    }
}

pub fn x00401dce() -> Cont {
    unsafe {
        // 00401dce lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 00401dd1 push eax
        push(MACHINE.regs.eax);
        // 00401dd2 lea eax,[ebp-8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffff8u32);
        // 00401dd5 push eax
        push(MACHINE.regs.eax);
        // 00401dd6 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401dd9 call 004039AFh
        call(0x401dde, Cont(x004039af))
    }
}

pub fn x00401dde() -> Cont {
    unsafe {
        // 00401dde add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401de1 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401de3 je short 00401DEDh
        je(Cont(x00401de5), Cont(x00401ded))
    }
}

pub fn x00401de5() -> Cont {
    unsafe {
        // 00401de5 movzx eax,byte ptr [eax]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)) as _;
        // 00401de8 shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00401deb jmp short 00401DFEh
        Cont(x00401dfe)
    }
}

pub fn x00401ded() -> Cont {
    unsafe {
        // 00401ded push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401df0 push 0
        push(0x0u32);
        // 00401df2 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401df8 call dword ptr ds:[40608Ch]
        call(0x401dfe, Cont(kernel32::HeapSize_stdcall))
    }
}

pub fn x00401df0() -> Cont {
    unsafe {
        // 00401df0 push 0
        push(0x0u32);
        // 00401df2 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401df8 call dword ptr ds:[40608Ch]
        call(0x401dfe, Cont(kernel32::HeapSize_stdcall))
    }
}

pub fn x00401dfe() -> Cont {
    unsafe {
        // 00401dfe pop esi
        MACHINE.regs.esi = pop();
        // 00401dff leave
        leave();
        // 00401e00 ret
        ret(0)
    }
}

pub fn x00401e01() -> Cont {
    unsafe {
        // 00401e01 push dword ptr ds:[40970Ch]
        push(MACHINE.memory.read::<u32>(0x40970cu32));
        // 00401e07 push dword ptr [esp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 00401e0b call 00401E13h
        call(0x401e10, Cont(x00401e13))
    }
}

pub fn x00401e10() -> Cont {
    unsafe {
        // 00401e10 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e11 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e12 ret
        ret(0)
    }
}

pub fn x00401e13() -> Cont {
    unsafe {
        // 00401e13 cmp dword ptr [esp+4],0FFFFFFE0h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
            0xffffffe0u32,
        );
        // 00401e18 ja short 00401E3Ch
        ja(Cont(x00401e1a), Cont(x00401e3c))
    }
}

pub fn x00401e1a() -> Cont {
    unsafe {
        // 00401e1a push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 00401e1e call 00401E3Fh
        call(0x401e23, Cont(x00401e3f))
    }
}

pub fn x00401e23() -> Cont {
    unsafe {
        // 00401e23 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401e25 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e26 jne short 00401E3Eh
        jne(Cont(x00401e28), Cont(x00401e3e))
    }
}

pub fn x00401e28() -> Cont {
    unsafe {
        // 00401e28 cmp [esp+8],eax
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
            MACHINE.regs.eax,
        );
        // 00401e2c je short 00401E3Eh
        je(Cont(x00401e2e), Cont(x00401e3e))
    }
}

pub fn x00401e2e() -> Cont {
    unsafe {
        // 00401e2e push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 00401e32 call 00403E20h
        call(0x401e37, Cont(x00403e20))
    }
}

pub fn x00401e37() -> Cont {
    unsafe {
        // 00401e37 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401e39 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e3a jne short 00401E1Ah
        jne(Cont(x00401e3c), Cont(x00401e1a))
    }
}

pub fn x00401e3c() -> Cont {
    unsafe {
        // 00401e3c xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00401e3e ret
        ret(0)
    }
}

pub fn x00401e3e() -> Cont {
    unsafe {
        // 00401e3e ret
        ret(0)
    }
}

pub fn x00401e3f() -> Cont {
    unsafe {
        // 00401e3f mov eax,ds:[409988h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409988u32);
        // 00401e44 push esi
        push(MACHINE.regs.esi);
        // 00401e45 mov esi,[esp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00401e49 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00401e4c jne short 00401E63h
        jne(Cont(x00401e4e), Cont(x00401e63))
    }
}

pub fn x00401e4e() -> Cont {
    unsafe {
        // 00401e4e cmp esi,ds:[409980h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409980u32));
        // 00401e54 ja short 00401E95h
        ja(Cont(x00401e56), Cont(x00401e95))
    }
}

pub fn x00401e56() -> Cont {
    unsafe {
        // 00401e56 push esi
        push(MACHINE.regs.esi);
        // 00401e57 call 00402FA8h
        call(0x401e5c, Cont(x00402fa8))
    }
}

pub fn x00401e5c() -> Cont {
    unsafe {
        // 00401e5c test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401e5e pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e5f je short 00401E95h
        je(Cont(x00401e61), Cont(x00401e95))
    }
}

pub fn x00401e61() -> Cont {
    unsafe {
        // 00401e61 pop esi
        MACHINE.regs.esi = pop();
        // 00401e62 ret
        ret(0)
    }
}

pub fn x00401e63() -> Cont {
    unsafe {
        // 00401e63 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 00401e66 jne short 00401E95h
        jne(Cont(x00401e68), Cont(x00401e95))
    }
}

pub fn x00401e68() -> Cont {
    unsafe {
        // 00401e68 mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00401e6c test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401e6e je short 00401E78h
        je(Cont(x00401e70), Cont(x00401e78))
    }
}

pub fn x00401e70() -> Cont {
    unsafe {
        // 00401e70 lea esi,[eax+0Fh]
        MACHINE.regs.esi = MACHINE.regs.eax.wrapping_add(0xfu32);
        // 00401e73 and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401e76 jmp short 00401E7Bh
        Cont(x00401e7b)
    }
}

pub fn x00401e78() -> Cont {
    unsafe {
        // 00401e78 push 10h
        push(0x10u32);
        // 00401e7a pop esi
        MACHINE.regs.esi = pop();
        // 00401e7b cmp esi,ds:[40922Ch]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x40922cu32));
        // 00401e81 ja short 00401EA2h
        ja(Cont(x00401e83), Cont(x00401ea2))
    }
}

pub fn x00401e7b() -> Cont {
    unsafe {
        // 00401e7b cmp esi,ds:[40922Ch]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x40922cu32));
        // 00401e81 ja short 00401EA2h
        ja(Cont(x00401e83), Cont(x00401ea2))
    }
}

pub fn x00401e83() -> Cont {
    unsafe {
        // 00401e83 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00401e85 shr eax,4
        MACHINE.regs.eax = shr(MACHINE.regs.eax, 0x4u8);
        // 00401e88 push eax
        push(MACHINE.regs.eax);
        // 00401e89 call 00403A4Bh
        call(0x401e8e, Cont(x00403a4b))
    }
}

pub fn x00401e8e() -> Cont {
    unsafe {
        // 00401e8e test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401e90 pop ecx
        MACHINE.regs.ecx = pop();
        // 00401e91 jne short 00401EB1h
        jne(Cont(x00401e93), Cont(x00401eb1))
    }
}

pub fn x00401e93() -> Cont {
    unsafe {
        // 00401e93 jmp short 00401EA2h
        Cont(x00401ea2)
    }
}

pub fn x00401e95() -> Cont {
    unsafe {
        // 00401e95 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401e97 jne short 00401E9Ch
        jne(Cont(x00401e99), Cont(x00401e9c))
    }
}

pub fn x00401e99() -> Cont {
    unsafe {
        // 00401e99 push 1
        push(0x1u32);
        // 00401e9b pop esi
        MACHINE.regs.esi = pop();
        // 00401e9c add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401e9f and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401ea2 push esi
        push(MACHINE.regs.esi);
        // 00401ea3 push 0
        push(0x0u32);
        // 00401ea5 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401eab call dword ptr ds:[406028h]
        call(0x401eb1, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401e9c() -> Cont {
    unsafe {
        // 00401e9c add esi,0Fh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfu32);
        // 00401e9f and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 00401ea2 push esi
        push(MACHINE.regs.esi);
        // 00401ea3 push 0
        push(0x0u32);
        // 00401ea5 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401eab call dword ptr ds:[406028h]
        call(0x401eb1, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401ea2() -> Cont {
    unsafe {
        // 00401ea2 push esi
        push(MACHINE.regs.esi);
        // 00401ea3 push 0
        push(0x0u32);
        // 00401ea5 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401eab call dword ptr ds:[406028h]
        call(0x401eb1, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00401eb1() -> Cont {
    unsafe {
        // 00401eb1 pop esi
        MACHINE.regs.esi = pop();
        // 00401eb2 ret
        ret(0)
    }
}

pub fn x00401eb3() -> Cont {
    unsafe {
        // 00401eb3 push ebp
        push(MACHINE.regs.ebp);
        // 00401eb4 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401eb6 push ecx
        push(MACHINE.regs.ecx);
        // 00401eb7 push esi
        push(MACHINE.regs.esi);
        // 00401eb8 mov esi,[ebp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00401ebb test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00401ebd je short 00401F19h
        je(Cont(x00401ebf), Cont(x00401f19))
    }
}

pub fn x00401ebf() -> Cont {
    unsafe {
        // 00401ebf mov eax,ds:[409988h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409988u32);
        // 00401ec4 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00401ec7 jne short 00401EDFh
        jne(Cont(x00401ec9), Cont(x00401edf))
    }
}

pub fn x00401ec9() -> Cont {
    unsafe {
        // 00401ec9 push esi
        push(MACHINE.regs.esi);
        // 00401eca call 00402C54h
        call(0x401ecf, Cont(x00402c54))
    }
}

pub fn x00401ecf() -> Cont {
    unsafe {
        // 00401ecf pop ecx
        MACHINE.regs.ecx = pop();
        // 00401ed0 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401ed2 push esi
        push(MACHINE.regs.esi);
        // 00401ed3 je short 00401F0Bh
        je(Cont(x00401ed5), Cont(x00401f0b))
    }
}

pub fn x00401ed5() -> Cont {
    unsafe {
        // 00401ed5 push eax
        push(MACHINE.regs.eax);
        // 00401ed6 call 00402C7Fh
        call(0x401edb, Cont(x00402c7f))
    }
}

pub fn x00401edb() -> Cont {
    unsafe {
        // 00401edb pop ecx
        MACHINE.regs.ecx = pop();
        // 00401edc pop ecx
        MACHINE.regs.ecx = pop();
        // 00401edd jmp short 00401F19h
        Cont(x00401f19)
    }
}

pub fn x00401edf() -> Cont {
    unsafe {
        // 00401edf cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 00401ee2 jne short 00401F0Ah
        jne(Cont(x00401ee4), Cont(x00401f0a))
    }
}

pub fn x00401ee4() -> Cont {
    unsafe {
        // 00401ee4 lea eax,[ebp+8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0x8u32);
        // 00401ee7 push eax
        push(MACHINE.regs.eax);
        // 00401ee8 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 00401eeb push eax
        push(MACHINE.regs.eax);
        // 00401eec push esi
        push(MACHINE.regs.esi);
        // 00401eed call 004039AFh
        call(0x401ef2, Cont(x004039af))
    }
}

pub fn x00401ef2() -> Cont {
    unsafe {
        // 00401ef2 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401ef5 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401ef7 je short 00401F0Ah
        je(Cont(x00401ef9), Cont(x00401f0a))
    }
}

pub fn x00401ef9() -> Cont {
    unsafe {
        // 00401ef9 push eax
        push(MACHINE.regs.eax);
        // 00401efa push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401efd push dword ptr [ebp-4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00401f00 call 00403A06h
        call(0x401f05, Cont(x00403a06))
    }
}

pub fn x00401f05() -> Cont {
    unsafe {
        // 00401f05 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00401f08 jmp short 00401F19h
        Cont(x00401f19)
    }
}

pub fn x00401f0a() -> Cont {
    unsafe {
        // 00401f0a push esi
        push(MACHINE.regs.esi);
        // 00401f0b push 0
        push(0x0u32);
        // 00401f0d push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401f13 call dword ptr ds:[406090h]
        call(0x401f19, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x00401f0b() -> Cont {
    unsafe {
        // 00401f0b push 0
        push(0x0u32);
        // 00401f0d push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00401f13 call dword ptr ds:[406090h]
        call(0x401f19, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x00401f19() -> Cont {
    unsafe {
        // 00401f19 pop esi
        MACHINE.regs.esi = pop();
        // 00401f1a leave
        leave();
        // 00401f1b ret
        ret(0)
    }
}

pub fn x00401f1c() -> Cont {
    unsafe {
        // 00401f1c push ebp
        push(MACHINE.regs.ebp);
        // 00401f1d mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401f1f push ebx
        push(MACHINE.regs.ebx);
        // 00401f20 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00401f23 call 0040205Dh
        call(0x401f28, Cont(x0040205d))
    }
}

pub fn x00401f28() -> Cont {
    unsafe {
        // 00401f28 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00401f2a pop ecx
        MACHINE.regs.ecx = pop();
        // 00401f2b je near ptr 00402051h
        je(Cont(x00401f31), Cont(x00402051))
    }
}

pub fn x00401f31() -> Cont {
    unsafe {
        // 00401f31 mov ebx,[eax+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32));
        // 00401f34 test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00401f36 je near ptr 00402051h
        je(Cont(x00401f3c), Cont(x00402051))
    }
}

pub fn x00401f3c() -> Cont {
    unsafe {
        // 00401f3c cmp ebx,5
        sub(MACHINE.regs.ebx, 0x5u32);
        // 00401f3f jne short 00401F4Dh
        jne(Cont(x00401f41), Cont(x00401f4d))
    }
}

pub fn x00401f41() -> Cont {
    unsafe {
        // 00401f41 and dword ptr [eax+8],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
                0x0u32,
            ),
        );
        // 00401f45 push 1
        push(0x1u32);
        // 00401f47 pop eax
        MACHINE.regs.eax = pop();
        // 00401f48 jmp near ptr 0040205Ah
        Cont(x0040205a)
    }
}

pub fn x00401f4d() -> Cont {
    unsafe {
        // 00401f4d cmp ebx,1
        sub(MACHINE.regs.ebx, 0x1u32);
        // 00401f50 je near ptr 0040204Ch
        je(Cont(x00401f56), Cont(x0040204c))
    }
}

pub fn x00401f56() -> Cont {
    unsafe {
        // 00401f56 mov ecx,ds:[4095F4h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x4095f4u32);
        // 00401f5c mov [ebp+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 00401f5f mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00401f62 mov ds:[4095F4h],ecx
        MACHINE.memory.write::<u32>(0x4095f4u32, MACHINE.regs.ecx);
        // 00401f68 mov ecx,[eax+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32));
        // 00401f6b cmp ecx,8
        sub(MACHINE.regs.ecx, 0x8u32);
        // 00401f6e jne near ptr 0040203Ch
        jne(Cont(x00401f74), Cont(x0040203c))
    }
}

pub fn x00401f74() -> Cont {
    unsafe {
        // 00401f74 mov ecx,ds:[407148h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x407148u32);
        // 00401f7a mov edx,ds:[40714Ch]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x40714cu32);
        // 00401f80 add edx,ecx
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00401f82 push esi
        push(MACHINE.regs.esi);
        // 00401f83 cmp ecx,edx
        sub(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 00401f85 jge short 00401F9Ch
        jge(Cont(x00401f87), Cont(x00401f9c))
    }
}

pub fn x00401f87() -> Cont {
    unsafe {
        // 00401f87 lea esi,[ecx+ecx*2]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.ecx * 2))
            .wrapping_add(0x0u32);
        // 00401f8a sub edx,ecx
        MACHINE.regs.edx = sub(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00401f8c lea esi,[esi*4+4070D8h]
        MACHINE.regs.esi = (MACHINE.regs.esi * 4).wrapping_add(0x4070d8u32);
        // 00401f93 and dword ptr [esi],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 00401f96 add esi,0Ch
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xcu32);
        // 00401f99 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00401f9a jne short 00401F93h
        jne(Cont(x00401f9c), Cont(x00401f93))
    }
}

pub fn x00401f93() -> Cont {
    unsafe {
        // 00401f93 and dword ptr [esi],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 00401f96 add esi,0Ch
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xcu32);
        // 00401f99 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00401f9a jne short 00401F93h
        jne(Cont(x00401f9c), Cont(x00401f93))
    }
}

pub fn x00401f9c() -> Cont {
    unsafe {
        // 00401f9c mov eax,[eax]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00401f9e mov esi,ds:[407154h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x407154u32);
        // 00401fa4 cmp eax,0C000008Eh
        sub(MACHINE.regs.eax, 0xc000008eu32);
        // 00401fa9 jne short 00401FB7h
        jne(Cont(x00401fab), Cont(x00401fb7))
    }
}

pub fn x00401fab() -> Cont {
    unsafe {
        // 00401fab mov dword ptr ds:[407154h],83h
        MACHINE.memory.write::<u32>(0x407154u32, 0x83u32);
        // 00401fb5 jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00401fb7() -> Cont {
    unsafe {
        // 00401fb7 cmp eax,0C0000090h
        sub(MACHINE.regs.eax, 0xc0000090u32);
        // 00401fbc jne short 00401FCAh
        jne(Cont(x00401fbe), Cont(x00401fca))
    }
}

pub fn x00401fbe() -> Cont {
    unsafe {
        // 00401fbe mov dword ptr ds:[407154h],81h
        MACHINE.memory.write::<u32>(0x407154u32, 0x81u32);
        // 00401fc8 jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00401fca() -> Cont {
    unsafe {
        // 00401fca cmp eax,0C0000091h
        sub(MACHINE.regs.eax, 0xc0000091u32);
        // 00401fcf jne short 00401FDDh
        jne(Cont(x00401fd1), Cont(x00401fdd))
    }
}

pub fn x00401fd1() -> Cont {
    unsafe {
        // 00401fd1 mov dword ptr ds:[407154h],84h
        MACHINE.memory.write::<u32>(0x407154u32, 0x84u32);
        // 00401fdb jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00401fdd() -> Cont {
    unsafe {
        // 00401fdd cmp eax,0C0000093h
        sub(MACHINE.regs.eax, 0xc0000093u32);
        // 00401fe2 jne short 00401FF0h
        jne(Cont(x00401fe4), Cont(x00401ff0))
    }
}

pub fn x00401fe4() -> Cont {
    unsafe {
        // 00401fe4 mov dword ptr ds:[407154h],85h
        MACHINE.memory.write::<u32>(0x407154u32, 0x85u32);
        // 00401fee jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00401ff0() -> Cont {
    unsafe {
        // 00401ff0 cmp eax,0C000008Dh
        sub(MACHINE.regs.eax, 0xc000008du32);
        // 00401ff5 jne short 00402003h
        jne(Cont(x00401ff7), Cont(x00402003))
    }
}

pub fn x00401ff7() -> Cont {
    unsafe {
        // 00401ff7 mov dword ptr ds:[407154h],82h
        MACHINE.memory.write::<u32>(0x407154u32, 0x82u32);
        // 00402001 jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00402003() -> Cont {
    unsafe {
        // 00402003 cmp eax,0C000008Fh
        sub(MACHINE.regs.eax, 0xc000008fu32);
        // 00402008 jne short 00402016h
        jne(Cont(x0040200a), Cont(x00402016))
    }
}

pub fn x0040200a() -> Cont {
    unsafe {
        // 0040200a mov dword ptr ds:[407154h],86h
        MACHINE.memory.write::<u32>(0x407154u32, 0x86u32);
        // 00402014 jmp short 00402027h
        Cont(x00402027)
    }
}

pub fn x00402016() -> Cont {
    unsafe {
        // 00402016 cmp eax,0C0000092h
        sub(MACHINE.regs.eax, 0xc0000092u32);
        // 0040201b jne short 00402027h
        jne(Cont(x0040201d), Cont(x00402027))
    }
}

pub fn x0040201d() -> Cont {
    unsafe {
        // 0040201d mov dword ptr ds:[407154h],8Ah
        MACHINE.memory.write::<u32>(0x407154u32, 0x8au32);
        // 00402027 push dword ptr ds:[407154h]
        push(MACHINE.memory.read::<u32>(0x407154u32));
        // 0040202d push 8
        push(0x8u32);
        // 0040202f call ebx
        call(0x402031, indirect(MACHINE.regs.ebx))
    }
}

pub fn x00402027() -> Cont {
    unsafe {
        // 00402027 push dword ptr ds:[407154h]
        push(MACHINE.memory.read::<u32>(0x407154u32));
        // 0040202d push 8
        push(0x8u32);
        // 0040202f call ebx
        call(0x402031, indirect(MACHINE.regs.ebx))
    }
}

pub fn x00402031() -> Cont {
    unsafe {
        // 00402031 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402032 mov ds:[407154h],esi
        MACHINE.memory.write::<u32>(0x407154u32, MACHINE.regs.esi);
        // 00402038 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402039 pop esi
        MACHINE.regs.esi = pop();
        // 0040203a jmp short 00402044h
        Cont(x00402044)
    }
}

pub fn x0040203c() -> Cont {
    unsafe {
        // 0040203c and dword ptr [eax+8],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
                0x0u32,
            ),
        );
        // 00402040 push ecx
        push(MACHINE.regs.ecx);
        // 00402041 call ebx
        call(0x402043, indirect(MACHINE.regs.ebx))
    }
}

pub fn x00402043() -> Cont {
    unsafe {
        // 00402043 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402044 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402047 mov ds:[4095F4h],eax
        MACHINE.memory.write::<u32>(0x4095f4u32, MACHINE.regs.eax);
        // 0040204c or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 0040204f jmp short 0040205Ah
        Cont(x0040205a)
    }
}

pub fn x00402044() -> Cont {
    unsafe {
        // 00402044 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402047 mov ds:[4095F4h],eax
        MACHINE.memory.write::<u32>(0x4095f4u32, MACHINE.regs.eax);
        // 0040204c or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 0040204f jmp short 0040205Ah
        Cont(x0040205a)
    }
}

pub fn x0040204c() -> Cont {
    unsafe {
        // 0040204c or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 0040204f jmp short 0040205Ah
        Cont(x0040205a)
    }
}

pub fn x00402051() -> Cont {
    unsafe {
        // 00402051 push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00402054 call dword ptr ds:[406030h]
        call(0x40205a, Cont(kernel32::UnhandledExceptionFilter_stdcall))
    }
}

pub fn x0040205a() -> Cont {
    unsafe {
        // 0040205a pop ebx
        MACHINE.regs.ebx = pop();
        // 0040205b pop ebp
        MACHINE.regs.ebp = pop();
        // 0040205c ret
        ret(0)
    }
}

pub fn x0040205d() -> Cont {
    unsafe {
        // 0040205d mov edx,[esp+4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00402061 mov ecx,ds:[407150h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x407150u32);
        // 00402067 cmp ds:[4070D0h],edx
        sub(MACHINE.memory.read::<u32>(0x4070d0u32), MACHINE.regs.edx);
        // 0040206d push esi
        push(MACHINE.regs.esi);
        // 0040206e mov eax,4070D0h
        MACHINE.regs.eax = 0x4070d0u32;
        // 00402073 je short 0040208Ah
        je(Cont(x00402075), Cont(x0040208a))
    }
}

pub fn x00402075() -> Cont {
    unsafe {
        // 00402075 lea esi,[ecx+ecx*2]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.ecx * 2))
            .wrapping_add(0x0u32);
        // 00402078 lea esi,[esi*4+4070D0h]
        MACHINE.regs.esi = (MACHINE.regs.esi * 4).wrapping_add(0x4070d0u32);
        // 0040207f add eax,0Ch
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0xcu32);
        // 00402082 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00402084 jae short 0040208Ah
        jae(Cont(x00402086), Cont(x0040208a))
    }
}

pub fn x0040207f() -> Cont {
    unsafe {
        // 0040207f add eax,0Ch
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0xcu32);
        // 00402082 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00402084 jae short 0040208Ah
        jae(Cont(x00402086), Cont(x0040208a))
    }
}

pub fn x00402086() -> Cont {
    unsafe {
        // 00402086 cmp [eax],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.edx,
        );
        // 00402088 jne short 0040207Fh
        jne(Cont(x0040208a), Cont(x0040207f))
    }
}

pub fn x0040208a() -> Cont {
    unsafe {
        // 0040208a lea ecx,[ecx+ecx*2]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.ecx * 2))
            .wrapping_add(0x0u32);
        // 0040208d pop esi
        MACHINE.regs.esi = pop();
        // 0040208e lea ecx,[ecx*4+4070D0h]
        MACHINE.regs.ecx = (MACHINE.regs.ecx * 4).wrapping_add(0x4070d0u32);
        // 00402095 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00402097 jae short 0040209Dh
        jae(Cont(x00402099), Cont(x0040209d))
    }
}

pub fn x00402099() -> Cont {
    unsafe {
        // 00402099 cmp [eax],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.edx,
        );
        // 0040209b je short 0040209Fh
        je(Cont(x0040209d), Cont(x0040209f))
    }
}

pub fn x0040209d() -> Cont {
    unsafe {
        // 0040209d xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 0040209f ret
        ret(0)
    }
}

pub fn x0040209f() -> Cont {
    unsafe {
        // 0040209f ret
        ret(0)
    }
}

pub fn x004020a0() -> Cont {
    unsafe {
        // 004020a0 cmp dword ptr ds:[409AA8h],0
        sub(MACHINE.memory.read::<u32>(0x409aa8u32), 0x0u32);
        // 004020a7 jne short 004020AEh
        jne(Cont(x004020a9), Cont(x004020ae))
    }
}

pub fn x004020a9() -> Cont {
    unsafe {
        // 004020a9 call 0040457Bh
        call(0x4020ae, Cont(x0040457b))
    }
}

pub fn x004020ae() -> Cont {
    unsafe {
        // 004020ae push esi
        push(MACHINE.regs.esi);
        // 004020af mov esi,ds:[409AB8h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x409ab8u32);
        // 004020b5 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 004020b7 cmp al,22h
        sub(MACHINE.regs.get_al(), 0x22u8);
        // 004020b9 jne short 004020E0h
        jne(Cont(x004020bb), Cont(x004020e0))
    }
}

pub fn x004020bb() -> Cont {
    unsafe {
        // 004020bb mov al,[esi+1]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x1u32)),
        );
        // 004020be inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004020bf cmp al,22h
        sub(MACHINE.regs.get_al(), 0x22u8);
        // 004020c1 je short 004020D8h
        je(Cont(x004020c3), Cont(x004020d8))
    }
}

pub fn x004020c3() -> Cont {
    unsafe {
        // 004020c3 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004020c5 je short 004020D8h
        je(Cont(x004020c7), Cont(x004020d8))
    }
}

pub fn x004020c7() -> Cont {
    unsafe {
        // 004020c7 movzx eax,al
        MACHINE.regs.eax = MACHINE.regs.get_al() as _;
        // 004020ca push eax
        push(MACHINE.regs.eax);
        // 004020cb call 00404175h
        call(0x4020d0, Cont(x00404175))
    }
}

pub fn x004020d0() -> Cont {
    unsafe {
        // 004020d0 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004020d2 pop ecx
        MACHINE.regs.ecx = pop();
        // 004020d3 je short 004020BBh
        je(Cont(x004020d5), Cont(x004020bb))
    }
}

pub fn x004020d5() -> Cont {
    unsafe {
        // 004020d5 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004020d6 jmp short 004020BBh
        Cont(x004020bb)
    }
}

pub fn x004020d8() -> Cont {
    unsafe {
        // 004020d8 cmp byte ptr [esi],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 004020db jne short 004020EAh
        jne(Cont(x004020dd), Cont(x004020ea))
    }
}

pub fn x004020dd() -> Cont {
    unsafe {
        // 004020dd inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004020de jmp short 004020EAh
        Cont(x004020ea)
    }
}

pub fn x004020e0() -> Cont {
    unsafe {
        // 004020e0 cmp al,20h
        sub(MACHINE.regs.get_al(), 0x20u8);
        // 004020e2 jbe short 004020EAh
        jbe(Cont(x004020e4), Cont(x004020ea))
    }
}

pub fn x004020e4() -> Cont {
    unsafe {
        // 004020e4 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004020e5 cmp byte ptr [esi],20h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
            0x20u8,
        );
        // 004020e8 ja short 004020E4h
        ja(Cont(x004020ea), Cont(x004020e4))
    }
}

pub fn x004020ea() -> Cont {
    unsafe {
        // 004020ea mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 004020ec test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004020ee je short 004020F4h
        je(Cont(x004020f0), Cont(x004020f4))
    }
}

pub fn x004020f0() -> Cont {
    unsafe {
        // 004020f0 cmp al,20h
        sub(MACHINE.regs.get_al(), 0x20u8);
        // 004020f2 jbe short 004020DDh
        jbe(Cont(x004020f4), Cont(x004020dd))
    }
}

pub fn x004020f4() -> Cont {
    unsafe {
        // 004020f4 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 004020f6 pop esi
        MACHINE.regs.esi = pop();
        // 004020f7 ret
        ret(0)
    }
}

pub fn x004020f8() -> Cont {
    unsafe {
        // 004020f8 push ebx
        push(MACHINE.regs.ebx);
        // 004020f9 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 004020fb cmp ds:[409AA8h],ebx
        sub(MACHINE.memory.read::<u32>(0x409aa8u32), MACHINE.regs.ebx);
        // 00402101 push esi
        push(MACHINE.regs.esi);
        // 00402102 push edi
        push(MACHINE.regs.edi);
        // 00402103 jne short 0040210Ah
        jne(Cont(x00402105), Cont(x0040210a))
    }
}

pub fn x00402105() -> Cont {
    unsafe {
        // 00402105 call 0040457Bh
        call(0x40210a, Cont(x0040457b))
    }
}

pub fn x0040210a() -> Cont {
    unsafe {
        // 0040210a mov esi,ds:[40959Ch]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x40959cu32);
        // 00402110 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00402112 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00402114 cmp al,bl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_bl());
        // 00402116 je short 0040212Ah
        je(Cont(x00402118), Cont(x0040212a))
    }
}

pub fn x00402112() -> Cont {
    unsafe {
        // 00402112 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00402114 cmp al,bl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_bl());
        // 00402116 je short 0040212Ah
        je(Cont(x00402118), Cont(x0040212a))
    }
}

pub fn x00402118() -> Cont {
    unsafe {
        // 00402118 cmp al,3Dh
        sub(MACHINE.regs.get_al(), 0x3du8);
        // 0040211a je short 0040211Dh
        je(Cont(x0040211c), Cont(x0040211d))
    }
}

pub fn x0040211c() -> Cont {
    unsafe {
        // 0040211c inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 0040211d push esi
        push(MACHINE.regs.esi);
        // 0040211e call 00404690h
        call(0x402123, Cont(x00404690))
    }
}

pub fn x0040211d() -> Cont {
    unsafe {
        // 0040211d push esi
        push(MACHINE.regs.esi);
        // 0040211e call 00404690h
        call(0x402123, Cont(x00404690))
    }
}

pub fn x00402123() -> Cont {
    unsafe {
        // 00402123 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402124 lea esi,[esi+eax+1]
        MACHINE.regs.esi = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x1u32);
        // 00402128 jmp short 00402112h
        Cont(x00402112)
    }
}

pub fn x0040212a() -> Cont {
    unsafe {
        // 0040212a lea eax,[edi*4+4]
        MACHINE.regs.eax = (MACHINE.regs.edi * 4).wrapping_add(0x4u32);
        // 00402131 push eax
        push(MACHINE.regs.eax);
        // 00402132 call 00401E01h
        call(0x402137, Cont(x00401e01))
    }
}

pub fn x00402137() -> Cont {
    unsafe {
        // 00402137 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 00402139 pop ecx
        MACHINE.regs.ecx = pop();
        // 0040213a cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 0040213c mov ds:[4095D0h],esi
        MACHINE.memory.write::<u32>(0x4095d0u32, MACHINE.regs.esi);
        // 00402142 jne short 0040214Ch
        jne(Cont(x00402144), Cont(x0040214c))
    }
}

pub fn x00402144() -> Cont {
    unsafe {
        // 00402144 push 9
        push(0x9u32);
        // 00402146 call 004019B5h
        call(0x40214b, Cont(x004019b5))
    }
}

pub fn x0040214b() -> Cont {
    unsafe {
        // 0040214b pop ecx
        MACHINE.regs.ecx = pop();
        // 0040214c mov edi,ds:[40959Ch]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(0x40959cu32);
        // 00402152 cmp [edi],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 00402154 je short 0040218Fh
        je(Cont(x00402156), Cont(x0040218f))
    }
}

pub fn x0040214c() -> Cont {
    unsafe {
        // 0040214c mov edi,ds:[40959Ch]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(0x40959cu32);
        // 00402152 cmp [edi],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 00402154 je short 0040218Fh
        je(Cont(x00402156), Cont(x0040218f))
    }
}

pub fn x00402156() -> Cont {
    unsafe {
        // 00402156 push ebp
        push(MACHINE.regs.ebp);
        // 00402157 push edi
        push(MACHINE.regs.edi);
        // 00402158 call 00404690h
        call(0x40215d, Cont(x00404690))
    }
}

pub fn x00402157() -> Cont {
    unsafe {
        // 00402157 push edi
        push(MACHINE.regs.edi);
        // 00402158 call 00404690h
        call(0x40215d, Cont(x00404690))
    }
}

pub fn x0040215d() -> Cont {
    unsafe {
        // 0040215d mov ebp,eax
        MACHINE.regs.ebp = MACHINE.regs.eax;
        // 0040215f pop ecx
        MACHINE.regs.ecx = pop();
        // 00402160 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00402161 cmp byte ptr [edi],3Dh
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            0x3du8,
        );
        // 00402164 je short 00402188h
        je(Cont(x00402166), Cont(x00402188))
    }
}

pub fn x00402166() -> Cont {
    unsafe {
        // 00402166 push ebp
        push(MACHINE.regs.ebp);
        // 00402167 call 00401E01h
        call(0x40216c, Cont(x00401e01))
    }
}

pub fn x0040216c() -> Cont {
    unsafe {
        // 0040216c cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 0040216e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040216f mov [esi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00402171 jne short 0040217Bh
        jne(Cont(x00402173), Cont(x0040217b))
    }
}

pub fn x00402173() -> Cont {
    unsafe {
        // 00402173 push 9
        push(0x9u32);
        // 00402175 call 004019B5h
        call(0x40217a, Cont(x004019b5))
    }
}

pub fn x0040217a() -> Cont {
    unsafe {
        // 0040217a pop ecx
        MACHINE.regs.ecx = pop();
        // 0040217b push edi
        push(MACHINE.regs.edi);
        // 0040217c push dword ptr [esi]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 0040217e call 004045A0h
        call(0x402183, Cont(x004045a0))
    }
}

pub fn x0040217b() -> Cont {
    unsafe {
        // 0040217b push edi
        push(MACHINE.regs.edi);
        // 0040217c push dword ptr [esi]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 0040217e call 004045A0h
        call(0x402183, Cont(x004045a0))
    }
}

pub fn x00402183() -> Cont {
    unsafe {
        // 00402183 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402184 add esi,4
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x4u32);
        // 00402187 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402188 add edi,ebp
        MACHINE.regs.edi = add(MACHINE.regs.edi, MACHINE.regs.ebp);
        // 0040218a cmp [edi],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040218c jne short 00402157h
        jne(Cont(x0040218e), Cont(x00402157))
    }
}

pub fn x00402188() -> Cont {
    unsafe {
        // 00402188 add edi,ebp
        MACHINE.regs.edi = add(MACHINE.regs.edi, MACHINE.regs.ebp);
        // 0040218a cmp [edi],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040218c jne short 00402157h
        jne(Cont(x0040218e), Cont(x00402157))
    }
}

pub fn x0040218e() -> Cont {
    unsafe {
        // 0040218e pop ebp
        MACHINE.regs.ebp = pop();
        // 0040218f push dword ptr ds:[40959Ch]
        push(MACHINE.memory.read::<u32>(0x40959cu32));
        // 00402195 call 00401EB3h
        call(0x40219a, Cont(x00401eb3))
    }
}

pub fn x0040218f() -> Cont {
    unsafe {
        // 0040218f push dword ptr ds:[40959Ch]
        push(MACHINE.memory.read::<u32>(0x40959cu32));
        // 00402195 call 00401EB3h
        call(0x40219a, Cont(x00401eb3))
    }
}

pub fn x0040219a() -> Cont {
    unsafe {
        // 0040219a pop ecx
        MACHINE.regs.ecx = pop();
        // 0040219b mov ds:[40959Ch],ebx
        MACHINE.memory.write::<u32>(0x40959cu32, MACHINE.regs.ebx);
        // 004021a1 mov [esi],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.ebx);
        // 004021a3 pop edi
        MACHINE.regs.edi = pop();
        // 004021a4 pop esi
        MACHINE.regs.esi = pop();
        // 004021a5 mov dword ptr ds:[409AA4h],1
        MACHINE.memory.write::<u32>(0x409aa4u32, 0x1u32);
        // 004021af pop ebx
        MACHINE.regs.ebx = pop();
        // 004021b0 ret
        ret(0)
    }
}

pub fn x004021b1() -> Cont {
    unsafe {
        // 004021b1 push ebp
        push(MACHINE.regs.ebp);
        // 004021b2 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004021b4 push ecx
        push(MACHINE.regs.ecx);
        // 004021b5 push ecx
        push(MACHINE.regs.ecx);
        // 004021b6 push ebx
        push(MACHINE.regs.ebx);
        // 004021b7 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 004021b9 cmp ds:[409AA8h],ebx
        sub(MACHINE.memory.read::<u32>(0x409aa8u32), MACHINE.regs.ebx);
        // 004021bf push esi
        push(MACHINE.regs.esi);
        // 004021c0 push edi
        push(MACHINE.regs.edi);
        // 004021c1 jne short 004021C8h
        jne(Cont(x004021c3), Cont(x004021c8))
    }
}

pub fn x004021c3() -> Cont {
    unsafe {
        // 004021c3 call 0040457Bh
        call(0x4021c8, Cont(x0040457b))
    }
}

pub fn x004021c8() -> Cont {
    unsafe {
        // 004021c8 mov esi,4095F8h
        MACHINE.regs.esi = 0x4095f8u32;
        // 004021cd push 104h
        push(0x104u32);
        // 004021d2 push esi
        push(MACHINE.regs.esi);
        // 004021d3 push ebx
        push(MACHINE.regs.ebx);
        // 004021d4 call dword ptr ds:[40602Ch]
        call(0x4021da, Cont(kernel32::GetModuleFileNameA_stdcall))
    }
}

pub fn x004021da() -> Cont {
    unsafe {
        // 004021da mov eax,ds:[409AB8h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409ab8u32);
        // 004021df mov ds:[4095E0h],esi
        MACHINE.memory.write::<u32>(0x4095e0u32, MACHINE.regs.esi);
        // 004021e5 mov edi,esi
        MACHINE.regs.edi = MACHINE.regs.esi;
        // 004021e7 cmp [eax],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004021e9 je short 004021EDh
        je(Cont(x004021eb), Cont(x004021ed))
    }
}

pub fn x004021eb() -> Cont {
    unsafe {
        // 004021eb mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 004021ed lea eax,[ebp-8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffff8u32);
        // 004021f0 push eax
        push(MACHINE.regs.eax);
        // 004021f1 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 004021f4 push eax
        push(MACHINE.regs.eax);
        // 004021f5 push ebx
        push(MACHINE.regs.ebx);
        // 004021f6 push ebx
        push(MACHINE.regs.ebx);
        // 004021f7 push edi
        push(MACHINE.regs.edi);
        // 004021f8 call 0040224Ah
        call(0x4021fd, Cont(x0040224a))
    }
}

pub fn x004021ed() -> Cont {
    unsafe {
        // 004021ed lea eax,[ebp-8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffff8u32);
        // 004021f0 push eax
        push(MACHINE.regs.eax);
        // 004021f1 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 004021f4 push eax
        push(MACHINE.regs.eax);
        // 004021f5 push ebx
        push(MACHINE.regs.ebx);
        // 004021f6 push ebx
        push(MACHINE.regs.ebx);
        // 004021f7 push edi
        push(MACHINE.regs.edi);
        // 004021f8 call 0040224Ah
        call(0x4021fd, Cont(x0040224a))
    }
}

pub fn x004021fd() -> Cont {
    unsafe {
        // 004021fd mov eax,[ebp-8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00402200 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402203 lea eax,[eax+ecx*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0x0u32);
        // 00402206 push eax
        push(MACHINE.regs.eax);
        // 00402207 call 00401E01h
        call(0x40220c, Cont(x00401e01))
    }
}

pub fn x0040220c() -> Cont {
    unsafe {
        // 0040220c mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040220e add esp,18h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x18u32);
        // 00402211 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00402213 jne short 0040221Dh
        jne(Cont(x00402215), Cont(x0040221d))
    }
}

pub fn x00402215() -> Cont {
    unsafe {
        // 00402215 push 8
        push(0x8u32);
        // 00402217 call 004019B5h
        call(0x40221c, Cont(x004019b5))
    }
}

pub fn x0040221c() -> Cont {
    unsafe {
        // 0040221c pop ecx
        MACHINE.regs.ecx = pop();
        // 0040221d lea eax,[ebp-8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffff8u32);
        // 00402220 push eax
        push(MACHINE.regs.eax);
        // 00402221 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 00402224 push eax
        push(MACHINE.regs.eax);
        // 00402225 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402228 lea eax,[esi+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 0040222b push eax
        push(MACHINE.regs.eax);
        // 0040222c push esi
        push(MACHINE.regs.esi);
        // 0040222d push edi
        push(MACHINE.regs.edi);
        // 0040222e call 0040224Ah
        call(0x402233, Cont(x0040224a))
    }
}

pub fn x0040221d() -> Cont {
    unsafe {
        // 0040221d lea eax,[ebp-8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffff8u32);
        // 00402220 push eax
        push(MACHINE.regs.eax);
        // 00402221 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 00402224 push eax
        push(MACHINE.regs.eax);
        // 00402225 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402228 lea eax,[esi+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 0040222b push eax
        push(MACHINE.regs.eax);
        // 0040222c push esi
        push(MACHINE.regs.esi);
        // 0040222d push edi
        push(MACHINE.regs.edi);
        // 0040222e call 0040224Ah
        call(0x402233, Cont(x0040224a))
    }
}

pub fn x00402233() -> Cont {
    unsafe {
        // 00402233 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402236 add esp,14h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x14u32);
        // 00402239 dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 0040223a mov ds:[4095C8h],esi
        MACHINE.memory.write::<u32>(0x4095c8u32, MACHINE.regs.esi);
        // 00402240 pop edi
        MACHINE.regs.edi = pop();
        // 00402241 pop esi
        MACHINE.regs.esi = pop();
        // 00402242 mov ds:[4095C4h],eax
        MACHINE.memory.write::<u32>(0x4095c4u32, MACHINE.regs.eax);
        // 00402247 pop ebx
        MACHINE.regs.ebx = pop();
        // 00402248 leave
        leave();
        // 00402249 ret
        ret(0)
    }
}

pub fn x0040224a() -> Cont {
    unsafe {
        // 0040224a push ebp
        push(MACHINE.regs.ebp);
        // 0040224b mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 0040224d mov ecx,[ebp+18h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32));
        // 00402250 mov eax,[ebp+14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00402253 push ebx
        push(MACHINE.regs.ebx);
        // 00402254 push esi
        push(MACHINE.regs.esi);
        // 00402255 and dword ptr [ecx],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 00402258 mov esi,[ebp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 0040225b push edi
        push(MACHINE.regs.edi);
        // 0040225c mov edi,[ebp+0Ch]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040225f mov dword ptr [eax],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), 0x1u32);
        // 00402265 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402268 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 0040226a je short 00402274h
        je(Cont(x0040226c), Cont(x00402274))
    }
}

pub fn x0040226c() -> Cont {
    unsafe {
        // 0040226c mov [edi],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.esi);
        // 0040226e add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00402271 mov [ebp+0Ch],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32), MACHINE.regs.edi);
        // 00402274 cmp byte ptr [eax],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 00402277 jne short 004022BDh
        jne(Cont(x00402279), Cont(x004022bd))
    }
}

pub fn x00402274() -> Cont {
    unsafe {
        // 00402274 cmp byte ptr [eax],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 00402277 jne short 004022BDh
        jne(Cont(x00402279), Cont(x004022bd))
    }
}

pub fn x00402279() -> Cont {
    unsafe {
        // 00402279 mov dl,[eax+1]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x1u32)),
        );
        // 0040227c inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040227d cmp dl,22h
        sub(MACHINE.regs.get_dl(), 0x22u8);
        // 00402280 je short 004022ABh
        je(Cont(x00402282), Cont(x004022ab))
    }
}

pub fn x00402282() -> Cont {
    unsafe {
        // 00402282 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00402284 je short 004022ABh
        je(Cont(x00402286), Cont(x004022ab))
    }
}

pub fn x00402286() -> Cont {
    unsafe {
        // 00402286 movzx edx,dl
        MACHINE.regs.edx = MACHINE.regs.get_dl() as _;
        // 00402289 test byte ptr [edx+409861h],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0x409861u32)),
            0x4u8,
        );
        // 00402290 je short 0040229Eh
        je(Cont(x00402292), Cont(x0040229e))
    }
}

pub fn x00402292() -> Cont {
    unsafe {
        // 00402292 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 00402294 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402296 je short 0040229Eh
        je(Cont(x00402298), Cont(x0040229e))
    }
}

pub fn x00402298() -> Cont {
    unsafe {
        // 00402298 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 0040229a mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 0040229c inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 0040229d inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040229e inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004022a0 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022a2 je short 00402279h
        je(Cont(x004022a4), Cont(x00402279))
    }
}

pub fn x0040229e() -> Cont {
    unsafe {
        // 0040229e inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004022a0 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022a2 je short 00402279h
        je(Cont(x004022a4), Cont(x00402279))
    }
}

pub fn x004022a4() -> Cont {
    unsafe {
        // 004022a4 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004022a6 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 004022a8 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004022a9 jmp short 00402279h
        Cont(x00402279)
    }
}

pub fn x004022ab() -> Cont {
    unsafe {
        // 004022ab inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004022ad test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022af je short 004022B5h
        je(Cont(x004022b1), Cont(x004022b5))
    }
}

pub fn x004022b1() -> Cont {
    unsafe {
        // 004022b1 and byte ptr [esi],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0x0u8,
            ),
        );
        // 004022b4 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004022b5 cmp byte ptr [eax],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 004022b8 jne short 00402300h
        jne(Cont(x004022ba), Cont(x00402300))
    }
}

pub fn x004022b5() -> Cont {
    unsafe {
        // 004022b5 cmp byte ptr [eax],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 004022b8 jne short 00402300h
        jne(Cont(x004022ba), Cont(x00402300))
    }
}

pub fn x004022ba() -> Cont {
    unsafe {
        // 004022ba inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004022bb jmp short 00402300h
        Cont(x00402300)
    }
}

pub fn x004022bd() -> Cont {
    unsafe {
        // 004022bd inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004022bf test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022c1 je short 004022C8h
        je(Cont(x004022c3), Cont(x004022c8))
    }
}

pub fn x004022c3() -> Cont {
    unsafe {
        // 004022c3 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004022c5 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 004022c7 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004022c8 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004022ca inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004022cb movzx ebx,dl
        MACHINE.regs.ebx = MACHINE.regs.get_dl() as _;
        // 004022ce test byte ptr [ebx+409861h],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x409861u32)),
            0x4u8,
        );
        // 004022d5 je short 004022E3h
        je(Cont(x004022d7), Cont(x004022e3))
    }
}

pub fn x004022c8() -> Cont {
    unsafe {
        // 004022c8 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004022ca inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004022cb movzx ebx,dl
        MACHINE.regs.ebx = MACHINE.regs.get_dl() as _;
        // 004022ce test byte ptr [ebx+409861h],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x409861u32)),
            0x4u8,
        );
        // 004022d5 je short 004022E3h
        je(Cont(x004022d7), Cont(x004022e3))
    }
}

pub fn x004022d7() -> Cont {
    unsafe {
        // 004022d7 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004022d9 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022db je short 004022E2h
        je(Cont(x004022dd), Cont(x004022e2))
    }
}

pub fn x004022dd() -> Cont {
    unsafe {
        // 004022dd mov bl,[eax]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004022df mov [esi],bl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_bl());
        // 004022e1 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004022e2 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004022e3 cmp dl,20h
        sub(MACHINE.regs.get_dl(), 0x20u8);
        // 004022e6 je short 004022F1h
        je(Cont(x004022e8), Cont(x004022f1))
    }
}

pub fn x004022e2() -> Cont {
    unsafe {
        // 004022e2 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004022e3 cmp dl,20h
        sub(MACHINE.regs.get_dl(), 0x20u8);
        // 004022e6 je short 004022F1h
        je(Cont(x004022e8), Cont(x004022f1))
    }
}

pub fn x004022e3() -> Cont {
    unsafe {
        // 004022e3 cmp dl,20h
        sub(MACHINE.regs.get_dl(), 0x20u8);
        // 004022e6 je short 004022F1h
        je(Cont(x004022e8), Cont(x004022f1))
    }
}

pub fn x004022e8() -> Cont {
    unsafe {
        // 004022e8 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 004022ea je short 004022F5h
        je(Cont(x004022ec), Cont(x004022f5))
    }
}

pub fn x004022ec() -> Cont {
    unsafe {
        // 004022ec cmp dl,9
        sub(MACHINE.regs.get_dl(), 0x9u8);
        // 004022ef jne short 004022BDh
        jne(Cont(x004022f1), Cont(x004022bd))
    }
}

pub fn x004022f1() -> Cont {
    unsafe {
        // 004022f1 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 004022f3 jne short 004022F8h
        jne(Cont(x004022f5), Cont(x004022f8))
    }
}

pub fn x004022f5() -> Cont {
    unsafe {
        // 004022f5 dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 004022f6 jmp short 00402300h
        Cont(x00402300)
    }
}

pub fn x004022f8() -> Cont {
    unsafe {
        // 004022f8 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004022fa je short 00402300h
        je(Cont(x004022fc), Cont(x00402300))
    }
}

pub fn x004022fc() -> Cont {
    unsafe {
        // 004022fc and byte ptr [esi-1],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0xffffffffu32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0xffffffffu32)),
                0x0u8,
            ),
        );
        // 00402300 and dword ptr [ebp+18h],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x18u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
                0x0u32,
            ),
        );
        // 00402304 cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00402307 je near ptr 004023EDh
        je(Cont(x0040230d), Cont(x004023ed))
    }
}

pub fn x00402300() -> Cont {
    unsafe {
        // 00402300 and dword ptr [ebp+18h],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x18u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
                0x0u32,
            ),
        );
        // 00402304 cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00402307 je near ptr 004023EDh
        je(Cont(x0040230d), Cont(x004023ed))
    }
}

pub fn x00402304() -> Cont {
    unsafe {
        // 00402304 cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00402307 je near ptr 004023EDh
        je(Cont(x0040230d), Cont(x004023ed))
    }
}

pub fn x0040230d() -> Cont {
    unsafe {
        // 0040230d mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 0040230f cmp dl,20h
        sub(MACHINE.regs.get_dl(), 0x20u8);
        // 00402312 je short 00402319h
        je(Cont(x00402314), Cont(x00402319))
    }
}

pub fn x00402314() -> Cont {
    unsafe {
        // 00402314 cmp dl,9
        sub(MACHINE.regs.get_dl(), 0x9u8);
        // 00402317 jne short 0040231Ch
        jne(Cont(x00402319), Cont(x0040231c))
    }
}

pub fn x00402319() -> Cont {
    unsafe {
        // 00402319 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040231a jmp short 0040230Dh
        Cont(x0040230d)
    }
}

pub fn x0040231c() -> Cont {
    unsafe {
        // 0040231c cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 0040231f je near ptr 004023EDh
        je(Cont(x00402325), Cont(x004023ed))
    }
}

pub fn x00402325() -> Cont {
    unsafe {
        // 00402325 test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 00402327 je short 00402331h
        je(Cont(x00402329), Cont(x00402331))
    }
}

pub fn x00402329() -> Cont {
    unsafe {
        // 00402329 mov [edi],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.esi);
        // 0040232b add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 0040232e mov [ebp+0Ch],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32), MACHINE.regs.edi);
        // 00402331 mov edx,[ebp+14h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00402334 inc dword ptr [edx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32))),
        );
        // 00402336 mov dword ptr [ebp+8],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), 0x1u32);
        // 0040233d xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 0040233f cmp byte ptr [eax],5Ch
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x5cu8,
        );
        // 00402342 jne short 00402348h
        jne(Cont(x00402344), Cont(x00402348))
    }
}

pub fn x00402331() -> Cont {
    unsafe {
        // 00402331 mov edx,[ebp+14h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00402334 inc dword ptr [edx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32))),
        );
        // 00402336 mov dword ptr [ebp+8],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), 0x1u32);
        // 0040233d xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 0040233f cmp byte ptr [eax],5Ch
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x5cu8,
        );
        // 00402342 jne short 00402348h
        jne(Cont(x00402344), Cont(x00402348))
    }
}

pub fn x00402336() -> Cont {
    unsafe {
        // 00402336 mov dword ptr [ebp+8],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), 0x1u32);
        // 0040233d xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 0040233f cmp byte ptr [eax],5Ch
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x5cu8,
        );
        // 00402342 jne short 00402348h
        jne(Cont(x00402344), Cont(x00402348))
    }
}

pub fn x0040233f() -> Cont {
    unsafe {
        // 0040233f cmp byte ptr [eax],5Ch
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x5cu8,
        );
        // 00402342 jne short 00402348h
        jne(Cont(x00402344), Cont(x00402348))
    }
}

pub fn x00402344() -> Cont {
    unsafe {
        // 00402344 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00402345 inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 00402346 jmp short 0040233Fh
        Cont(x0040233f)
    }
}

pub fn x00402348() -> Cont {
    unsafe {
        // 00402348 cmp byte ptr [eax],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x22u8,
        );
        // 0040234b jne short 00402379h
        jne(Cont(x0040234d), Cont(x00402379))
    }
}

pub fn x0040234d() -> Cont {
    unsafe {
        // 0040234d test bl,1
        and(MACHINE.regs.get_bl(), 0x1u8);
        // 00402350 jne short 00402377h
        jne(Cont(x00402352), Cont(x00402377))
    }
}

pub fn x00402352() -> Cont {
    unsafe {
        // 00402352 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00402354 cmp [ebp+18h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
            MACHINE.regs.edi,
        );
        // 00402357 je short 00402366h
        je(Cont(x00402359), Cont(x00402366))
    }
}

pub fn x00402359() -> Cont {
    unsafe {
        // 00402359 cmp byte ptr [eax+1],22h
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x1u32)),
            0x22u8,
        );
        // 0040235d lea edx,[eax+1]
        MACHINE.regs.edx = MACHINE.regs.eax.wrapping_add(0x1u32);
        // 00402360 jne short 00402366h
        jne(Cont(x00402362), Cont(x00402366))
    }
}

pub fn x00402362() -> Cont {
    unsafe {
        // 00402362 mov eax,edx
        MACHINE.regs.eax = MACHINE.regs.edx;
        // 00402364 jmp short 00402369h
        Cont(x00402369)
    }
}

pub fn x00402366() -> Cont {
    unsafe {
        // 00402366 mov [ebp+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 00402369 mov edi,[ebp+0Ch]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040236c xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 0040236e cmp [ebp+18h],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
            MACHINE.regs.edx,
        );
        // 00402371 sete dl
        MACHINE.regs.set_dl(sete());
        // 00402374 mov [ebp+18h],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32), MACHINE.regs.edx);
        // 00402377 shr ebx,1
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, 0x1u8);
        // 00402379 mov edx,ebx
        MACHINE.regs.edx = MACHINE.regs.ebx;
        // 0040237b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040237c test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040237e je short 0040238Eh
        je(Cont(x00402380), Cont(x0040238e))
    }
}

pub fn x00402369() -> Cont {
    unsafe {
        // 00402369 mov edi,[ebp+0Ch]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040236c xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 0040236e cmp [ebp+18h],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
            MACHINE.regs.edx,
        );
        // 00402371 sete dl
        MACHINE.regs.set_dl(sete());
        // 00402374 mov [ebp+18h],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32), MACHINE.regs.edx);
        // 00402377 shr ebx,1
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, 0x1u8);
        // 00402379 mov edx,ebx
        MACHINE.regs.edx = MACHINE.regs.ebx;
        // 0040237b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040237c test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040237e je short 0040238Eh
        je(Cont(x00402380), Cont(x0040238e))
    }
}

pub fn x00402377() -> Cont {
    unsafe {
        // 00402377 shr ebx,1
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, 0x1u8);
        // 00402379 mov edx,ebx
        MACHINE.regs.edx = MACHINE.regs.ebx;
        // 0040237b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040237c test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040237e je short 0040238Eh
        je(Cont(x00402380), Cont(x0040238e))
    }
}

pub fn x00402379() -> Cont {
    unsafe {
        // 00402379 mov edx,ebx
        MACHINE.regs.edx = MACHINE.regs.ebx;
        // 0040237b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040237c test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 0040237e je short 0040238Eh
        je(Cont(x00402380), Cont(x0040238e))
    }
}

pub fn x00402380() -> Cont {
    unsafe {
        // 00402380 inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 00402381 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402383 je short 00402389h
        je(Cont(x00402385), Cont(x00402389))
    }
}

pub fn x00402381() -> Cont {
    unsafe {
        // 00402381 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402383 je short 00402389h
        je(Cont(x00402385), Cont(x00402389))
    }
}

pub fn x00402385() -> Cont {
    unsafe {
        // 00402385 mov byte ptr [esi],5Ch
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), 0x5cu8);
        // 00402388 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00402389 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 0040238b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040238c jne short 00402381h
        jne(Cont(x0040238e), Cont(x00402381))
    }
}

pub fn x00402389() -> Cont {
    unsafe {
        // 00402389 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 0040238b dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 0040238c jne short 00402381h
        jne(Cont(x0040238e), Cont(x00402381))
    }
}

pub fn x0040238e() -> Cont {
    unsafe {
        // 0040238e mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 00402390 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00402392 je short 004023DEh
        je(Cont(x00402394), Cont(x004023de))
    }
}

pub fn x00402394() -> Cont {
    unsafe {
        // 00402394 cmp dword ptr [ebp+18h],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
            0x0u32,
        );
        // 00402398 jne short 004023A4h
        jne(Cont(x0040239a), Cont(x004023a4))
    }
}

pub fn x0040239a() -> Cont {
    unsafe {
        // 0040239a cmp dl,20h
        sub(MACHINE.regs.get_dl(), 0x20u8);
        // 0040239d je short 004023DEh
        je(Cont(x0040239f), Cont(x004023de))
    }
}

pub fn x0040239f() -> Cont {
    unsafe {
        // 0040239f cmp dl,9
        sub(MACHINE.regs.get_dl(), 0x9u8);
        // 004023a2 je short 004023DEh
        je(Cont(x004023a4), Cont(x004023de))
    }
}

pub fn x004023a4() -> Cont {
    unsafe {
        // 004023a4 cmp dword ptr [ebp+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 004023a8 je short 004023D8h
        je(Cont(x004023aa), Cont(x004023d8))
    }
}

pub fn x004023aa() -> Cont {
    unsafe {
        // 004023aa test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004023ac je short 004023C7h
        je(Cont(x004023ae), Cont(x004023c7))
    }
}

pub fn x004023ae() -> Cont {
    unsafe {
        // 004023ae movzx ebx,dl
        MACHINE.regs.ebx = MACHINE.regs.get_dl() as _;
        // 004023b1 test byte ptr [ebx+409861h],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x409861u32)),
            0x4u8,
        );
        // 004023b8 je short 004023C0h
        je(Cont(x004023ba), Cont(x004023c0))
    }
}

pub fn x004023ba() -> Cont {
    unsafe {
        // 004023ba mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 004023bc inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004023bd inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004023be inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023c0 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004023c2 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 004023c4 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004023c5 jmp short 004023D6h
        Cont(x004023d6)
    }
}

pub fn x004023c0() -> Cont {
    unsafe {
        // 004023c0 mov dl,[eax]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 004023c2 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 004023c4 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004023c5 jmp short 004023D6h
        Cont(x004023d6)
    }
}

pub fn x004023c7() -> Cont {
    unsafe {
        // 004023c7 movzx edx,dl
        MACHINE.regs.edx = MACHINE.regs.get_dl() as _;
        // 004023ca test byte ptr [edx+409861h],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0x409861u32)),
            0x4u8,
        );
        // 004023d1 je short 004023D6h
        je(Cont(x004023d3), Cont(x004023d6))
    }
}

pub fn x004023d3() -> Cont {
    unsafe {
        // 004023d3 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004023d4 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023d6 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023d8 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004023d9 jmp near ptr 00402336h
        Cont(x00402336)
    }
}

pub fn x004023d6() -> Cont {
    unsafe {
        // 004023d6 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023d8 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004023d9 jmp near ptr 00402336h
        Cont(x00402336)
    }
}

pub fn x004023d8() -> Cont {
    unsafe {
        // 004023d8 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004023d9 jmp near ptr 00402336h
        Cont(x00402336)
    }
}

pub fn x004023de() -> Cont {
    unsafe {
        // 004023de test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 004023e0 je short 004023E6h
        je(Cont(x004023e2), Cont(x004023e6))
    }
}

pub fn x004023e2() -> Cont {
    unsafe {
        // 004023e2 and byte ptr [esi],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0x0u8,
            ),
        );
        // 004023e5 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004023e6 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023e8 jmp near ptr 00402304h
        Cont(x00402304)
    }
}

pub fn x004023e6() -> Cont {
    unsafe {
        // 004023e6 inc dword ptr [ecx]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004023e8 jmp near ptr 00402304h
        Cont(x00402304)
    }
}

pub fn x004023ed() -> Cont {
    unsafe {
        // 004023ed test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 004023ef je short 004023F4h
        je(Cont(x004023f1), Cont(x004023f4))
    }
}

pub fn x004023f1() -> Cont {
    unsafe {
        // 004023f1 and dword ptr [edi],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 004023f4 mov eax,[ebp+14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 004023f7 pop edi
        MACHINE.regs.edi = pop();
        // 004023f8 pop esi
        MACHINE.regs.esi = pop();
        // 004023f9 pop ebx
        MACHINE.regs.ebx = pop();
        // 004023fa inc dword ptr [eax]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32))),
        );
        // 004023fc pop ebp
        MACHINE.regs.ebp = pop();
        // 004023fd ret
        ret(0)
    }
}

pub fn x004023f4() -> Cont {
    unsafe {
        // 004023f4 mov eax,[ebp+14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 004023f7 pop edi
        MACHINE.regs.edi = pop();
        // 004023f8 pop esi
        MACHINE.regs.esi = pop();
        // 004023f9 pop ebx
        MACHINE.regs.ebx = pop();
        // 004023fa inc dword ptr [eax]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32))),
        );
        // 004023fc pop ebp
        MACHINE.regs.ebp = pop();
        // 004023fd ret
        ret(0)
    }
}

pub fn x004023fe() -> Cont {
    unsafe {
        // 004023fe push ecx
        push(MACHINE.regs.ecx);
        // 004023ff push ecx
        push(MACHINE.regs.ecx);
        // 00402400 mov eax,ds:[4096FCh]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4096fcu32);
        // 00402405 push ebx
        push(MACHINE.regs.ebx);
        // 00402406 push ebp
        push(MACHINE.regs.ebp);
        // 00402407 mov ebp,ds:[4060ACh]
        MACHINE.regs.ebp = MACHINE.memory.read::<u32>(0x4060acu32);
        // 0040240d push esi
        push(MACHINE.regs.esi);
        // 0040240e push edi
        push(MACHINE.regs.edi);
        // 0040240f xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00402411 xor esi,esi
        MACHINE.regs.esi ^= MACHINE.regs.esi;
        // 00402413 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00402415 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00402417 jne short 0040244Ch
        jne(Cont(x00402419), Cont(x0040244c))
    }
}

pub fn x00402419() -> Cont {
    unsafe {
        // 00402419 call ebp
        call(0x40241b, indirect(MACHINE.regs.ebp))
    }
}

pub fn x0040241b() -> Cont {
    unsafe {
        // 0040241b mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040241d cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 0040241f je short 0040242Dh
        je(Cont(x00402421), Cont(x0040242d))
    }
}

pub fn x00402421() -> Cont {
    unsafe {
        // 00402421 mov dword ptr ds:[4096FCh],1
        MACHINE.memory.write::<u32>(0x4096fcu32, 0x1u32);
        // 0040242b jmp short 00402455h
        Cont(x00402455)
    }
}

pub fn x0040242d() -> Cont {
    unsafe {
        // 0040242d call dword ptr ds:[4060A8h]
        call(0x402433, Cont(kernel32::GetEnvironmentStrings_stdcall))
    }
}

pub fn x00402433() -> Cont {
    unsafe {
        // 00402433 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00402435 cmp edi,ebx
        sub(MACHINE.regs.edi, MACHINE.regs.ebx);
        // 00402437 je near ptr 00402527h
        je(Cont(x0040243d), Cont(x00402527))
    }
}

pub fn x0040243d() -> Cont {
    unsafe {
        // 0040243d mov dword ptr ds:[4096FCh],2
        MACHINE.memory.write::<u32>(0x4096fcu32, 0x2u32);
        // 00402447 jmp near ptr 004024DBh
        Cont(x004024db)
    }
}

pub fn x0040244c() -> Cont {
    unsafe {
        // 0040244c cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 0040244f jne near ptr 004024D6h
        jne(Cont(x00402455), Cont(x004024d6))
    }
}

pub fn x00402455() -> Cont {
    unsafe {
        // 00402455 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00402457 jne short 00402465h
        jne(Cont(x00402459), Cont(x00402465))
    }
}

pub fn x00402459() -> Cont {
    unsafe {
        // 00402459 call ebp
        call(0x40245b, indirect(MACHINE.regs.ebp))
    }
}

pub fn x0040245b() -> Cont {
    unsafe {
        // 0040245b mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040245d cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 0040245f je near ptr 00402527h
        je(Cont(x00402465), Cont(x00402527))
    }
}

pub fn x00402465() -> Cont {
    unsafe {
        // 00402465 cmp [esi],bx
        sub(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.esi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bx(),
        );
        // 00402468 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 0040246a je short 0040247Ah
        je(Cont(x0040246c), Cont(x0040247a))
    }
}

pub fn x0040246c() -> Cont {
    unsafe {
        // 0040246c inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040246d inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040246e cmp [eax],bx
        sub(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bx(),
        );
        // 00402471 jne short 0040246Ch
        jne(Cont(x00402473), Cont(x0040246c))
    }
}

pub fn x00402473() -> Cont {
    unsafe {
        // 00402473 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00402474 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00402475 cmp [eax],bx
        sub(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bx(),
        );
        // 00402478 jne short 0040246Ch
        jne(Cont(x0040247a), Cont(x0040246c))
    }
}

pub fn x0040247a() -> Cont {
    unsafe {
        // 0040247a sub eax,esi
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 0040247c mov edi,ds:[4060A4h]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(0x4060a4u32);
        // 00402482 sar eax,1
        sar();
        // 00402484 push ebx
        push(MACHINE.regs.ebx);
        // 00402485 push ebx
        push(MACHINE.regs.ebx);
        // 00402486 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00402487 push ebx
        push(MACHINE.regs.ebx);
        // 00402488 push ebx
        push(MACHINE.regs.ebx);
        // 00402489 push eax
        push(MACHINE.regs.eax);
        // 0040248a push esi
        push(MACHINE.regs.esi);
        // 0040248b push ebx
        push(MACHINE.regs.ebx);
        // 0040248c push ebx
        push(MACHINE.regs.ebx);
        // 0040248d mov [esp+34h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x34u32), MACHINE.regs.eax);
        // 00402491 call edi
        call(0x402493, indirect(MACHINE.regs.edi))
    }
}

pub fn x00402493() -> Cont {
    unsafe {
        // 00402493 mov ebp,eax
        MACHINE.regs.ebp = MACHINE.regs.eax;
        // 00402495 cmp ebp,ebx
        sub(MACHINE.regs.ebp, MACHINE.regs.ebx);
        // 00402497 je short 004024CBh
        je(Cont(x00402499), Cont(x004024cb))
    }
}

pub fn x00402499() -> Cont {
    unsafe {
        // 00402499 push ebp
        push(MACHINE.regs.ebp);
        // 0040249a call 00401E01h
        call(0x40249f, Cont(x00401e01))
    }
}

pub fn x0040249f() -> Cont {
    unsafe {
        // 0040249f cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 004024a1 pop ecx
        MACHINE.regs.ecx = pop();
        // 004024a2 mov [esp+10h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.eax);
        // 004024a6 je short 004024CBh
        je(Cont(x004024a8), Cont(x004024cb))
    }
}

pub fn x004024a8() -> Cont {
    unsafe {
        // 004024a8 push ebx
        push(MACHINE.regs.ebx);
        // 004024a9 push ebx
        push(MACHINE.regs.ebx);
        // 004024aa push ebp
        push(MACHINE.regs.ebp);
        // 004024ab push eax
        push(MACHINE.regs.eax);
        // 004024ac push dword ptr [esp+24h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x24u32)),
        );
        // 004024b0 push esi
        push(MACHINE.regs.esi);
        // 004024b1 push ebx
        push(MACHINE.regs.ebx);
        // 004024b2 push ebx
        push(MACHINE.regs.ebx);
        // 004024b3 call edi
        call(0x4024b5, indirect(MACHINE.regs.edi))
    }
}

pub fn x004024b5() -> Cont {
    unsafe {
        // 004024b5 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004024b7 jne short 004024C7h
        jne(Cont(x004024b9), Cont(x004024c7))
    }
}

pub fn x004024b9() -> Cont {
    unsafe {
        // 004024b9 push dword ptr [esp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32)),
        );
        // 004024bd call 00401EB3h
        call(0x4024c2, Cont(x00401eb3))
    }
}

pub fn x004024c2() -> Cont {
    unsafe {
        // 004024c2 pop ecx
        MACHINE.regs.ecx = pop();
        // 004024c3 mov [esp+10h],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32), MACHINE.regs.ebx);
        // 004024c7 mov ebx,[esp+10h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 004024cb push esi
        push(MACHINE.regs.esi);
        // 004024cc call dword ptr ds:[4060A0h]
        call(0x4024d2, Cont(kernel32::FreeEnvironmentStringsW_stdcall))
    }
}

pub fn x004024c7() -> Cont {
    unsafe {
        // 004024c7 mov ebx,[esp+10h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 004024cb push esi
        push(MACHINE.regs.esi);
        // 004024cc call dword ptr ds:[4060A0h]
        call(0x4024d2, Cont(kernel32::FreeEnvironmentStringsW_stdcall))
    }
}

pub fn x004024cb() -> Cont {
    unsafe {
        // 004024cb push esi
        push(MACHINE.regs.esi);
        // 004024cc call dword ptr ds:[4060A0h]
        call(0x4024d2, Cont(kernel32::FreeEnvironmentStringsW_stdcall))
    }
}

pub fn x004024d2() -> Cont {
    unsafe {
        // 004024d2 mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 004024d4 jmp short 00402529h
        Cont(x00402529)
    }
}

pub fn x004024d6() -> Cont {
    unsafe {
        // 004024d6 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004024d9 jne short 00402527h
        jne(Cont(x004024db), Cont(x00402527))
    }
}

pub fn x004024db() -> Cont {
    unsafe {
        // 004024db cmp edi,ebx
        sub(MACHINE.regs.edi, MACHINE.regs.ebx);
        // 004024dd jne short 004024EBh
        jne(Cont(x004024df), Cont(x004024eb))
    }
}

pub fn x004024df() -> Cont {
    unsafe {
        // 004024df call dword ptr ds:[4060A8h]
        call(0x4024e5, Cont(kernel32::GetEnvironmentStrings_stdcall))
    }
}

pub fn x004024e5() -> Cont {
    unsafe {
        // 004024e5 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 004024e7 cmp edi,ebx
        sub(MACHINE.regs.edi, MACHINE.regs.ebx);
        // 004024e9 je short 00402527h
        je(Cont(x004024eb), Cont(x00402527))
    }
}

pub fn x004024eb() -> Cont {
    unsafe {
        // 004024eb cmp [edi],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004024ed mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 004024ef je short 004024FBh
        je(Cont(x004024f1), Cont(x004024fb))
    }
}

pub fn x004024f1() -> Cont {
    unsafe {
        // 004024f1 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004024f2 cmp [eax],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004024f4 jne short 004024F1h
        jne(Cont(x004024f6), Cont(x004024f1))
    }
}

pub fn x004024f6() -> Cont {
    unsafe {
        // 004024f6 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004024f7 cmp [eax],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004024f9 jne short 004024F1h
        jne(Cont(x004024fb), Cont(x004024f1))
    }
}

pub fn x004024fb() -> Cont {
    unsafe {
        // 004024fb sub eax,edi
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 004024fd inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004024fe mov ebp,eax
        MACHINE.regs.ebp = MACHINE.regs.eax;
        // 00402500 push ebp
        push(MACHINE.regs.ebp);
        // 00402501 call 00401E01h
        call(0x402506, Cont(x00401e01))
    }
}

pub fn x00402506() -> Cont {
    unsafe {
        // 00402506 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 00402508 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402509 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 0040250b jne short 00402511h
        jne(Cont(x0040250d), Cont(x00402511))
    }
}

pub fn x0040250d() -> Cont {
    unsafe {
        // 0040250d xor esi,esi
        MACHINE.regs.esi ^= MACHINE.regs.esi;
        // 0040250f jmp short 0040251Ch
        Cont(x0040251c)
    }
}

pub fn x00402511() -> Cont {
    unsafe {
        // 00402511 push ebp
        push(MACHINE.regs.ebp);
        // 00402512 push edi
        push(MACHINE.regs.edi);
        // 00402513 push esi
        push(MACHINE.regs.esi);
        // 00402514 call 00403E40h
        call(0x402519, Cont(x00403e40))
    }
}

pub fn x00402519() -> Cont {
    unsafe {
        // 00402519 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 0040251c push edi
        push(MACHINE.regs.edi);
        // 0040251d call dword ptr ds:[40609Ch]
        call(0x402523, Cont(kernel32::FreeEnvironmentStringsA_stdcall))
    }
}

pub fn x0040251c() -> Cont {
    unsafe {
        // 0040251c push edi
        push(MACHINE.regs.edi);
        // 0040251d call dword ptr ds:[40609Ch]
        call(0x402523, Cont(kernel32::FreeEnvironmentStringsA_stdcall))
    }
}

pub fn x00402523() -> Cont {
    unsafe {
        // 00402523 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00402525 jmp short 00402529h
        Cont(x00402529)
    }
}

pub fn x00402527() -> Cont {
    unsafe {
        // 00402527 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00402529 pop edi
        MACHINE.regs.edi = pop();
        // 0040252a pop esi
        MACHINE.regs.esi = pop();
        // 0040252b pop ebp
        MACHINE.regs.ebp = pop();
        // 0040252c pop ebx
        MACHINE.regs.ebx = pop();
        // 0040252d pop ecx
        MACHINE.regs.ecx = pop();
        // 0040252e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040252f ret
        ret(0)
    }
}

pub fn x00402529() -> Cont {
    unsafe {
        // 00402529 pop edi
        MACHINE.regs.edi = pop();
        // 0040252a pop esi
        MACHINE.regs.esi = pop();
        // 0040252b pop ebp
        MACHINE.regs.ebp = pop();
        // 0040252c pop ebx
        MACHINE.regs.ebx = pop();
        // 0040252d pop ecx
        MACHINE.regs.ecx = pop();
        // 0040252e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040252f ret
        ret(0)
    }
}

pub fn x00402530() -> Cont {
    unsafe {
        // 00402530 sub esp,44h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x44u32);
        // 00402533 push ebx
        push(MACHINE.regs.ebx);
        // 00402534 push ebp
        push(MACHINE.regs.ebp);
        // 00402535 push esi
        push(MACHINE.regs.esi);
        // 00402536 push edi
        push(MACHINE.regs.edi);
        // 00402537 push 100h
        push(0x100u32);
        // 0040253c call 00401E01h
        call(0x402541, Cont(x00401e01))
    }
}

pub fn x00402541() -> Cont {
    unsafe {
        // 00402541 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 00402543 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402544 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402546 jne short 00402550h
        jne(Cont(x00402548), Cont(x00402550))
    }
}

pub fn x00402548() -> Cont {
    unsafe {
        // 00402548 push 1Bh
        push(0x1bu32);
        // 0040254a call 004019B5h
        call(0x40254f, Cont(x004019b5))
    }
}

pub fn x0040254f() -> Cont {
    unsafe {
        // 0040254f pop ecx
        MACHINE.regs.ecx = pop();
        // 00402550 mov ds:[4099A0h],esi
        MACHINE.memory.write::<u32>(0x4099a0u32, MACHINE.regs.esi);
        // 00402556 mov dword ptr ds:[409AA0h],20h
        MACHINE.memory.write::<u32>(0x409aa0u32, 0x20u32);
        // 00402560 lea eax,[esi+100h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x100u32);
        // 00402566 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00402568 jae short 00402584h
        jae(Cont(x0040256a), Cont(x00402584))
    }
}

pub fn x00402550() -> Cont {
    unsafe {
        // 00402550 mov ds:[4099A0h],esi
        MACHINE.memory.write::<u32>(0x4099a0u32, MACHINE.regs.esi);
        // 00402556 mov dword ptr ds:[409AA0h],20h
        MACHINE.memory.write::<u32>(0x409aa0u32, 0x20u32);
        // 00402560 lea eax,[esi+100h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x100u32);
        // 00402566 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00402568 jae short 00402584h
        jae(Cont(x0040256a), Cont(x00402584))
    }
}

pub fn x00402566() -> Cont {
    unsafe {
        // 00402566 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00402568 jae short 00402584h
        jae(Cont(x0040256a), Cont(x00402584))
    }
}

pub fn x0040256a() -> Cont {
    unsafe {
        // 0040256a and byte ptr [esi+4],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                0x0u8,
            ),
        );
        // 0040256e or dword ptr [esi],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0xffffffffu32,
            ),
        );
        // 00402571 mov byte ptr [esi+5],0Ah
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x5u32), 0xau8);
        // 00402575 mov eax,ds:[4099A0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4099a0u32);
        // 0040257a add esi,8
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x8u32);
        // 0040257d add eax,100h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x100u32);
        // 00402582 jmp short 00402566h
        Cont(x00402566)
    }
}

pub fn x00402584() -> Cont {
    unsafe {
        // 00402584 lea eax,[esp+10h]
        MACHINE.regs.eax = MACHINE.regs.esp.wrapping_add(0x10u32);
        // 00402588 push eax
        push(MACHINE.regs.eax);
        // 00402589 call dword ptr ds:[40606Ch]
        call(0x40258f, Cont(kernel32::GetStartupInfoA_stdcall))
    }
}

pub fn x0040258f() -> Cont {
    unsafe {
        // 0040258f cmp word ptr [esp+42h],0
        sub(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.esp.wrapping_add(0x42u32)),
            0x0u16,
        );
        // 00402595 je near ptr 00402660h
        je(Cont(x0040259b), Cont(x00402660))
    }
}

pub fn x0040259b() -> Cont {
    unsafe {
        // 0040259b mov eax,[esp+44h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x44u32));
        // 0040259f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004025a1 je near ptr 00402660h
        je(Cont(x004025a7), Cont(x00402660))
    }
}

pub fn x004025a7() -> Cont {
    unsafe {
        // 004025a7 mov esi,[eax]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004025a9 lea ebp,[eax+4]
        MACHINE.regs.ebp = MACHINE.regs.eax.wrapping_add(0x4u32);
        // 004025ac mov eax,800h
        MACHINE.regs.eax = 0x800u32;
        // 004025b1 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 004025b3 lea ebx,[esi+ebp]
        MACHINE.regs.ebx = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.ebp * 1))
            .wrapping_add(0x0u32);
        // 004025b6 jl short 004025BAh
        jl(Cont(x004025b8), Cont(x004025ba))
    }
}

pub fn x004025b8() -> Cont {
    unsafe {
        // 004025b8 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 004025ba cmp ds:[409AA0h],esi
        sub(MACHINE.memory.read::<u32>(0x409aa0u32), MACHINE.regs.esi);
        // 004025c0 jge short 00402614h
        jge(Cont(x004025c2), Cont(x00402614))
    }
}

pub fn x004025ba() -> Cont {
    unsafe {
        // 004025ba cmp ds:[409AA0h],esi
        sub(MACHINE.memory.read::<u32>(0x409aa0u32), MACHINE.regs.esi);
        // 004025c0 jge short 00402614h
        jge(Cont(x004025c2), Cont(x00402614))
    }
}

pub fn x004025c2() -> Cont {
    unsafe {
        // 004025c2 mov edi,4099A4h
        MACHINE.regs.edi = 0x4099a4u32;
        // 004025c7 push 100h
        push(0x100u32);
        // 004025cc call 00401E01h
        call(0x4025d1, Cont(x00401e01))
    }
}

pub fn x004025c7() -> Cont {
    unsafe {
        // 004025c7 push 100h
        push(0x100u32);
        // 004025cc call 00401E01h
        call(0x4025d1, Cont(x00401e01))
    }
}

pub fn x004025d1() -> Cont {
    unsafe {
        // 004025d1 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004025d3 pop ecx
        MACHINE.regs.ecx = pop();
        // 004025d4 je short 0040260Eh
        je(Cont(x004025d6), Cont(x0040260e))
    }
}

pub fn x004025d6() -> Cont {
    unsafe {
        // 004025d6 add dword ptr ds:[409AA0h],20h
        MACHINE.memory.write::<u32>(
            0x409aa0u32,
            add(MACHINE.memory.read::<u32>(0x409aa0u32), 0x20u32),
        );
        // 004025dd mov [edi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 004025df lea ecx,[eax+100h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0x100u32);
        // 004025e5 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004025e7 jae short 00402601h
        jae(Cont(x004025e9), Cont(x00402601))
    }
}

pub fn x004025e5() -> Cont {
    unsafe {
        // 004025e5 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004025e7 jae short 00402601h
        jae(Cont(x004025e9), Cont(x00402601))
    }
}

pub fn x004025e9() -> Cont {
    unsafe {
        // 004025e9 and byte ptr [eax+4],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x4u32)),
                0x0u8,
            ),
        );
        // 004025ed or dword ptr [eax],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                0xffffffffu32,
            ),
        );
        // 004025f0 mov byte ptr [eax+5],0Ah
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.eax.wrapping_add(0x5u32), 0xau8);
        // 004025f4 mov ecx,[edi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 004025f6 add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 004025f9 add ecx,100h
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x100u32);
        // 004025ff jmp short 004025E5h
        Cont(x004025e5)
    }
}

pub fn x00402601() -> Cont {
    unsafe {
        // 00402601 add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00402604 cmp ds:[409AA0h],esi
        sub(MACHINE.memory.read::<u32>(0x409aa0u32), MACHINE.regs.esi);
        // 0040260a jl short 004025C7h
        jl(Cont(x0040260c), Cont(x004025c7))
    }
}

pub fn x0040260c() -> Cont {
    unsafe {
        // 0040260c jmp short 00402614h
        Cont(x00402614)
    }
}

pub fn x0040260e() -> Cont {
    unsafe {
        // 0040260e mov esi,ds:[409AA0h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x409aa0u32);
        // 00402614 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00402616 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402618 jle short 00402660h
        jle(Cont(x0040261a), Cont(x00402660))
    }
}

pub fn x00402614() -> Cont {
    unsafe {
        // 00402614 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00402616 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00402618 jle short 00402660h
        jle(Cont(x0040261a), Cont(x00402660))
    }
}

pub fn x0040261a() -> Cont {
    unsafe {
        // 0040261a mov eax,[ebx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32));
        // 0040261c cmp eax,0FFFFFFFFh
        sub(MACHINE.regs.eax, 0xffffffffu32);
        // 0040261f je short 00402657h
        je(Cont(x00402621), Cont(x00402657))
    }
}

pub fn x00402621() -> Cont {
    unsafe {
        // 00402621 mov cl,[ebp]
        MACHINE.regs.set_cl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x0u32)),
        );
        // 00402624 test cl,1
        and(MACHINE.regs.get_cl(), 0x1u8);
        // 00402627 je short 00402657h
        je(Cont(x00402629), Cont(x00402657))
    }
}

pub fn x00402629() -> Cont {
    unsafe {
        // 00402629 test cl,8
        and(MACHINE.regs.get_cl(), 0x8u8);
        // 0040262c jne short 00402639h
        jne(Cont(x0040262e), Cont(x00402639))
    }
}

pub fn x0040262e() -> Cont {
    unsafe {
        // 0040262e push eax
        push(MACHINE.regs.eax);
        // 0040262f call dword ptr ds:[4060B8h]
        call(0x402635, Cont(kernel32::GetFileType_stdcall))
    }
}

pub fn x00402635() -> Cont {
    unsafe {
        // 00402635 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402637 je short 00402657h
        je(Cont(x00402639), Cont(x00402657))
    }
}

pub fn x00402639() -> Cont {
    unsafe {
        // 00402639 mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 0040263b mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 0040263d sar eax,5
        sar();
        // 00402640 and ecx,1Fh
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x1fu32);
        // 00402643 mov eax,[eax*4+4099A0h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>((MACHINE.regs.eax * 4).wrapping_add(0x4099a0u32));
        // 0040264a lea eax,[eax+ecx*8]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 8))
            .wrapping_add(0x0u32);
        // 0040264d mov ecx,[ebx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32));
        // 0040264f mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00402651 mov cl,[ebp]
        MACHINE.regs.set_cl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x0u32)),
        );
        // 00402654 mov [eax+4],cl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.get_cl());
        // 00402657 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00402658 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00402659 add ebx,4
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x4u32);
        // 0040265c cmp edi,esi
        sub(MACHINE.regs.edi, MACHINE.regs.esi);
        // 0040265e jl short 0040261Ah
        jl(Cont(x00402660), Cont(x0040261a))
    }
}

pub fn x00402657() -> Cont {
    unsafe {
        // 00402657 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00402658 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00402659 add ebx,4
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x4u32);
        // 0040265c cmp edi,esi
        sub(MACHINE.regs.edi, MACHINE.regs.esi);
        // 0040265e jl short 0040261Ah
        jl(Cont(x00402660), Cont(x0040261a))
    }
}

pub fn x00402660() -> Cont {
    unsafe {
        // 00402660 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00402662 mov eax,ds:[4099A0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4099a0u32);
        // 00402667 cmp dword ptr [eax+ebx*8],0FFFFFFFFh
        sub(
            MACHINE.memory.read::<u32>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.ebx * 8))
                    .wrapping_add(0x0u32),
            ),
            0xffffffffu32,
        );
        // 0040266b lea esi,[eax+ebx*8]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ebx * 8))
            .wrapping_add(0x0u32);
        // 0040266e jne short 004026BDh
        jne(Cont(x00402670), Cont(x004026bd))
    }
}

pub fn x00402662() -> Cont {
    unsafe {
        // 00402662 mov eax,ds:[4099A0h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4099a0u32);
        // 00402667 cmp dword ptr [eax+ebx*8],0FFFFFFFFh
        sub(
            MACHINE.memory.read::<u32>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.ebx * 8))
                    .wrapping_add(0x0u32),
            ),
            0xffffffffu32,
        );
        // 0040266b lea esi,[eax+ebx*8]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ebx * 8))
            .wrapping_add(0x0u32);
        // 0040266e jne short 004026BDh
        jne(Cont(x00402670), Cont(x004026bd))
    }
}

pub fn x00402670() -> Cont {
    unsafe {
        // 00402670 test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00402672 mov byte ptr [esi+4],81h
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x4u32), 0x81u8);
        // 00402676 jne short 0040267Dh
        jne(Cont(x00402678), Cont(x0040267d))
    }
}

pub fn x00402678() -> Cont {
    unsafe {
        // 00402678 push 0FFFFFFF6h
        push(0xfffffff6u32);
        // 0040267a pop eax
        MACHINE.regs.eax = pop();
        // 0040267b jmp short 00402687h
        Cont(x00402687)
    }
}

pub fn x0040267d() -> Cont {
    unsafe {
        // 0040267d mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 0040267f dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 00402680 neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 00402682 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402684 add eax,0FFFFFFF5h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0xfffffff5u32);
        // 00402687 push eax
        push(MACHINE.regs.eax);
        // 00402688 call dword ptr ds:[4060B4h]
        call(0x40268e, Cont(kernel32::GetStdHandle_stdcall))
    }
}

pub fn x00402687() -> Cont {
    unsafe {
        // 00402687 push eax
        push(MACHINE.regs.eax);
        // 00402688 call dword ptr ds:[4060B4h]
        call(0x40268e, Cont(kernel32::GetStdHandle_stdcall))
    }
}

pub fn x0040268e() -> Cont {
    unsafe {
        // 0040268e mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00402690 cmp edi,0FFFFFFFFh
        sub(MACHINE.regs.edi, 0xffffffffu32);
        // 00402693 je short 004026ACh
        je(Cont(x00402695), Cont(x004026ac))
    }
}

pub fn x00402695() -> Cont {
    unsafe {
        // 00402695 push edi
        push(MACHINE.regs.edi);
        // 00402696 call dword ptr ds:[4060B8h]
        call(0x40269c, Cont(kernel32::GetFileType_stdcall))
    }
}

pub fn x0040269c() -> Cont {
    unsafe {
        // 0040269c test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040269e je short 004026ACh
        je(Cont(x004026a0), Cont(x004026ac))
    }
}

pub fn x004026a0() -> Cont {
    unsafe {
        // 004026a0 and eax,0FFh
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0xffu32);
        // 004026a5 mov [esi],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.edi);
        // 004026a7 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004026aa jne short 004026B2h
        jne(Cont(x004026ac), Cont(x004026b2))
    }
}

pub fn x004026ac() -> Cont {
    unsafe {
        // 004026ac or byte ptr [esi+4],40h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                0x40u8,
            ),
        );
        // 004026b0 jmp short 004026C1h
        Cont(x004026c1)
    }
}

pub fn x004026b2() -> Cont {
    unsafe {
        // 004026b2 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 004026b5 jne short 004026C1h
        jne(Cont(x004026b7), Cont(x004026c1))
    }
}

pub fn x004026b7() -> Cont {
    unsafe {
        // 004026b7 or byte ptr [esi+4],8
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                0x8u8,
            ),
        );
        // 004026bb jmp short 004026C1h
        Cont(x004026c1)
    }
}

pub fn x004026bd() -> Cont {
    unsafe {
        // 004026bd or byte ptr [esi+4],80h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                0x80u8,
            ),
        );
        // 004026c1 inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 004026c2 cmp ebx,3
        sub(MACHINE.regs.ebx, 0x3u32);
        // 004026c5 jl short 00402662h
        jl(Cont(x004026c7), Cont(x00402662))
    }
}

pub fn x004026c1() -> Cont {
    unsafe {
        // 004026c1 inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 004026c2 cmp ebx,3
        sub(MACHINE.regs.ebx, 0x3u32);
        // 004026c5 jl short 00402662h
        jl(Cont(x004026c7), Cont(x00402662))
    }
}

pub fn x004026c7() -> Cont {
    unsafe {
        // 004026c7 push dword ptr ds:[409AA0h]
        push(MACHINE.memory.read::<u32>(0x409aa0u32));
        // 004026cd call dword ptr ds:[4060B0h]
        call(0x4026d3, Cont(kernel32::SetHandleCount_stdcall))
    }
}

pub fn x004026d3() -> Cont {
    unsafe {
        // 004026d3 pop edi
        MACHINE.regs.edi = pop();
        // 004026d4 pop esi
        MACHINE.regs.esi = pop();
        // 004026d5 pop ebp
        MACHINE.regs.ebp = pop();
        // 004026d6 pop ebx
        MACHINE.regs.ebx = pop();
        // 004026d7 add esp,44h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x44u32);
        // 004026da ret
        ret(0)
    }
}

pub fn x004026db() -> Cont {
    unsafe {
        // 004026db push esi
        push(MACHINE.regs.esi);
        // 004026dc mov esi,[esp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 004026e0 push 0
        push(0x0u32);
        // 004026e2 and dword ptr [esi],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
                0x0u32,
            ),
        );
        // 004026e5 call dword ptr ds:[406068h]
        call(0x4026eb, Cont(kernel32::GetModuleHandleA_stdcall))
    }
}

pub fn x004026eb() -> Cont {
    unsafe {
        // 004026eb cmp word ptr [eax],5A4Dh
        sub(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x5a4du16,
        );
        // 004026f0 jne short 00402706h
        jne(Cont(x004026f2), Cont(x00402706))
    }
}

pub fn x004026f2() -> Cont {
    unsafe {
        // 004026f2 mov ecx,[eax+3Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x3cu32));
        // 004026f5 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 004026f7 je short 00402706h
        je(Cont(x004026f9), Cont(x00402706))
    }
}

pub fn x004026f9() -> Cont {
    unsafe {
        // 004026f9 add eax,ecx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004026fb mov cl,[eax+1Ah]
        MACHINE.regs.set_cl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x1au32)),
        );
        // 004026fe mov [esi],cl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_cl());
        // 00402700 mov al,[eax+1Bh]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x1bu32)),
        );
        // 00402703 mov [esi+1],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x1u32), MACHINE.regs.get_al());
        // 00402706 pop esi
        MACHINE.regs.esi = pop();
        // 00402707 ret
        ret(0)
    }
}

pub fn x00402706() -> Cont {
    unsafe {
        // 00402706 pop esi
        MACHINE.regs.esi = pop();
        // 00402707 ret
        ret(0)
    }
}

pub fn x00402708() -> Cont {
    unsafe {
        // 00402708 push ebp
        push(MACHINE.regs.ebp);
        // 00402709 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 0040270b mov eax,122Ch
        MACHINE.regs.eax = 0x122cu32;
        // 00402710 call 00404AC0h
        call(0x402715, Cont(x00404ac0))
    }
}

pub fn x00402715() -> Cont {
    unsafe {
        // 00402715 lea eax,[ebp-98h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff68u32);
        // 0040271b push ebx
        push(MACHINE.regs.ebx);
        // 0040271c push eax
        push(MACHINE.regs.eax);
        // 0040271d mov dword ptr [ebp-98h],94h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff68u32), 0x94u32);
        // 00402727 call dword ptr ds:[4060C0h]
        call(0x40272d, Cont(kernel32::GetVersionExA_stdcall))
    }
}

pub fn x0040272d() -> Cont {
    unsafe {
        // 0040272d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040272f je short 0040274Bh
        je(Cont(x00402731), Cont(x0040274b))
    }
}

pub fn x00402731() -> Cont {
    unsafe {
        // 00402731 cmp dword ptr [ebp-88h],2
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff78u32)),
            0x2u32,
        );
        // 00402738 jne short 0040274Bh
        jne(Cont(x0040273a), Cont(x0040274b))
    }
}

pub fn x0040273a() -> Cont {
    unsafe {
        // 0040273a cmp dword ptr [ebp-94h],5
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffff6cu32)),
            0x5u32,
        );
        // 00402741 jb short 0040274Bh
        jb(Cont(x00402743), Cont(x0040274b))
    }
}

pub fn x00402743() -> Cont {
    unsafe {
        // 00402743 push 1
        push(0x1u32);
        // 00402745 pop eax
        MACHINE.regs.eax = pop();
        // 00402746 jmp near ptr 0040284Dh
        Cont(x0040284d)
    }
}

pub fn x0040274b() -> Cont {
    unsafe {
        // 0040274b lea eax,[ebp-122Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffedd4u32);
        // 00402751 push 1090h
        push(0x1090u32);
        // 00402756 push eax
        push(MACHINE.regs.eax);
        // 00402757 push 40614Ch
        push(0x40614cu32);
        // 0040275c call dword ptr ds:[4060BCh]
        call(0x402762, Cont(kernel32::GetEnvironmentVariableA_stdcall))
    }
}

pub fn x00402762() -> Cont {
    unsafe {
        // 00402762 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402764 je near ptr 0040283Ah
        je(Cont(x0040276a), Cont(x0040283a))
    }
}

pub fn x0040276a() -> Cont {
    unsafe {
        // 0040276a xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 0040276c lea ecx,[ebp-122Ch]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xffffedd4u32);
        // 00402772 cmp [ebp-122Ch],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xffffedd4u32)),
            MACHINE.regs.get_bl(),
        );
        // 00402778 je short 0040278Dh
        je(Cont(x0040277a), Cont(x0040278d))
    }
}

pub fn x0040277a() -> Cont {
    unsafe {
        // 0040277a mov al,[ecx]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 0040277c cmp al,61h
        sub(MACHINE.regs.get_al(), 0x61u8);
        // 0040277e jl short 00402788h
        jl(Cont(x00402780), Cont(x00402788))
    }
}

pub fn x00402780() -> Cont {
    unsafe {
        // 00402780 cmp al,7Ah
        sub(MACHINE.regs.get_al(), 0x7au8);
        // 00402782 jg short 00402788h
        jg(Cont(x00402784), Cont(x00402788))
    }
}

pub fn x00402784() -> Cont {
    unsafe {
        // 00402784 sub al,20h
        MACHINE.regs.set_al(sub(MACHINE.regs.get_al(), 0x20u8));
        // 00402786 mov [ecx],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00402788 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00402789 cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040278b jne short 0040277Ah
        jne(Cont(x0040278d), Cont(x0040277a))
    }
}

pub fn x00402788() -> Cont {
    unsafe {
        // 00402788 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00402789 cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040278b jne short 0040277Ah
        jne(Cont(x0040278d), Cont(x0040277a))
    }
}

pub fn x0040278d() -> Cont {
    unsafe {
        // 0040278d lea eax,[ebp-122Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffedd4u32);
        // 00402793 push 16h
        push(0x16u32);
        // 00402795 push eax
        push(MACHINE.regs.eax);
        // 00402796 push 406134h
        push(0x406134u32);
        // 0040279b call 00404A80h
        call(0x4027a0, Cont(x00404a80))
    }
}

pub fn x004027a0() -> Cont {
    unsafe {
        // 004027a0 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 004027a3 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004027a5 jne short 004027AFh
        jne(Cont(x004027a7), Cont(x004027af))
    }
}

pub fn x004027a7() -> Cont {
    unsafe {
        // 004027a7 lea eax,[ebp-122Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffedd4u32);
        // 004027ad jmp short 004027F8h
        Cont(x004027f8)
    }
}

pub fn x004027af() -> Cont {
    unsafe {
        // 004027af lea eax,[ebp-19Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe64u32);
        // 004027b5 push 104h
        push(0x104u32);
        // 004027ba push eax
        push(MACHINE.regs.eax);
        // 004027bb push ebx
        push(MACHINE.regs.ebx);
        // 004027bc call dword ptr ds:[40602Ch]
        call(0x4027c2, Cont(kernel32::GetModuleFileNameA_stdcall))
    }
}

pub fn x004027c2() -> Cont {
    unsafe {
        // 004027c2 cmp [ebp-19Ch],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffe64u32)),
            MACHINE.regs.get_bl(),
        );
        // 004027c8 lea ecx,[ebp-19Ch]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xfffffe64u32);
        // 004027ce je short 004027E3h
        je(Cont(x004027d0), Cont(x004027e3))
    }
}

pub fn x004027d0() -> Cont {
    unsafe {
        // 004027d0 mov al,[ecx]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 004027d2 cmp al,61h
        sub(MACHINE.regs.get_al(), 0x61u8);
        // 004027d4 jl short 004027DEh
        jl(Cont(x004027d6), Cont(x004027de))
    }
}

pub fn x004027d6() -> Cont {
    unsafe {
        // 004027d6 cmp al,7Ah
        sub(MACHINE.regs.get_al(), 0x7au8);
        // 004027d8 jg short 004027DEh
        jg(Cont(x004027da), Cont(x004027de))
    }
}

pub fn x004027da() -> Cont {
    unsafe {
        // 004027da sub al,20h
        MACHINE.regs.set_al(sub(MACHINE.regs.get_al(), 0x20u8));
        // 004027dc mov [ecx],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 004027de inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004027df cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004027e1 jne short 004027D0h
        jne(Cont(x004027e3), Cont(x004027d0))
    }
}

pub fn x004027de() -> Cont {
    unsafe {
        // 004027de inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004027df cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 004027e1 jne short 004027D0h
        jne(Cont(x004027e3), Cont(x004027d0))
    }
}

pub fn x004027e3() -> Cont {
    unsafe {
        // 004027e3 lea eax,[ebp-19Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe64u32);
        // 004027e9 push eax
        push(MACHINE.regs.eax);
        // 004027ea lea eax,[ebp-122Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffedd4u32);
        // 004027f0 push eax
        push(MACHINE.regs.eax);
        // 004027f1 call 00404A00h
        call(0x4027f6, Cont(x00404a00))
    }
}

pub fn x004027f6() -> Cont {
    unsafe {
        // 004027f6 pop ecx
        MACHINE.regs.ecx = pop();
        // 004027f7 pop ecx
        MACHINE.regs.ecx = pop();
        // 004027f8 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 004027fa je short 0040283Ah
        je(Cont(x004027fc), Cont(x0040283a))
    }
}

pub fn x004027f8() -> Cont {
    unsafe {
        // 004027f8 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 004027fa je short 0040283Ah
        je(Cont(x004027fc), Cont(x0040283a))
    }
}

pub fn x004027fc() -> Cont {
    unsafe {
        // 004027fc push 2Ch
        push(0x2cu32);
        // 004027fe push eax
        push(MACHINE.regs.eax);
        // 004027ff call 00404940h
        call(0x402804, Cont(x00404940))
    }
}

pub fn x00402804() -> Cont {
    unsafe {
        // 00402804 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402805 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00402807 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402808 je short 0040283Ah
        je(Cont(x0040280a), Cont(x0040283a))
    }
}

pub fn x0040280a() -> Cont {
    unsafe {
        // 0040280a inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040280b mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 0040280d cmp [eax],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040280f je short 0040281Fh
        je(Cont(x00402811), Cont(x0040281f))
    }
}

pub fn x00402811() -> Cont {
    unsafe {
        // 00402811 cmp byte ptr [ecx],3Bh
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            0x3bu8,
        );
        // 00402814 jne short 0040281Ah
        jne(Cont(x00402816), Cont(x0040281a))
    }
}

pub fn x00402816() -> Cont {
    unsafe {
        // 00402816 mov [ecx],bl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.get_bl());
        // 00402818 jmp short 0040281Bh
        Cont(x0040281b)
    }
}

pub fn x0040281a() -> Cont {
    unsafe {
        // 0040281a inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 0040281b cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040281d jne short 00402811h
        jne(Cont(x0040281f), Cont(x00402811))
    }
}

pub fn x0040281b() -> Cont {
    unsafe {
        // 0040281b cmp [ecx],bl
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.get_bl(),
        );
        // 0040281d jne short 00402811h
        jne(Cont(x0040281f), Cont(x00402811))
    }
}

pub fn x0040281f() -> Cont {
    unsafe {
        // 0040281f push 0Ah
        push(0xau32);
        // 00402821 push ebx
        push(MACHINE.regs.ebx);
        // 00402822 push eax
        push(MACHINE.regs.eax);
        // 00402823 call 0040470Bh
        call(0x402828, Cont(x0040470b))
    }
}

pub fn x00402828() -> Cont {
    unsafe {
        // 00402828 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 0040282b cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 0040282e je short 0040284Dh
        je(Cont(x00402830), Cont(x0040284d))
    }
}

pub fn x00402830() -> Cont {
    unsafe {
        // 00402830 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00402833 je short 0040284Dh
        je(Cont(x00402835), Cont(x0040284d))
    }
}

pub fn x00402835() -> Cont {
    unsafe {
        // 00402835 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00402838 je short 0040284Dh
        je(Cont(x0040283a), Cont(x0040284d))
    }
}

pub fn x0040283a() -> Cont {
    unsafe {
        // 0040283a lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 0040283d push eax
        push(MACHINE.regs.eax);
        // 0040283e call 004026DBh
        call(0x402843, Cont(x004026db))
    }
}

pub fn x00402843() -> Cont {
    unsafe {
        // 00402843 cmp byte ptr [ebp-4],6
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x6u8,
        );
        // 00402847 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402848 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040284a add eax,3
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x3u32);
        // 0040284d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040284e leave
        leave();
        // 0040284f ret
        ret(0)
    }
}

pub fn x0040284d() -> Cont {
    unsafe {
        // 0040284d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040284e leave
        leave();
        // 0040284f ret
        ret(0)
    }
}

pub fn x00402850() -> Cont {
    unsafe {
        // 00402850 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00402852 push 0
        push(0x0u32);
        // 00402854 cmp [esp+8],eax
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
            MACHINE.regs.eax,
        );
        // 00402858 push 1000h
        push(0x1000u32);
        // 0040285d sete al
        MACHINE.regs.set_al(sete());
        // 00402860 push eax
        push(MACHINE.regs.eax);
        // 00402861 call dword ptr ds:[406094h]
        call(0x402867, Cont(kernel32::HeapCreate_stdcall))
    }
}

pub fn x00402867() -> Cont {
    unsafe {
        // 00402867 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402869 mov ds:[409984h],eax
        MACHINE.memory.write::<u32>(0x409984u32, MACHINE.regs.eax);
        // 0040286e je short 004028A6h
        je(Cont(x00402870), Cont(x004028a6))
    }
}

pub fn x00402870() -> Cont {
    unsafe {
        // 00402870 call 00402708h
        call(0x402875, Cont(x00402708))
    }
}

pub fn x00402875() -> Cont {
    unsafe {
        // 00402875 cmp eax,3
        sub(MACHINE.regs.eax, 0x3u32);
        // 00402878 mov ds:[409988h],eax
        MACHINE.memory.write::<u32>(0x409988u32, MACHINE.regs.eax);
        // 0040287d jne short 0040288Ch
        jne(Cont(x0040287f), Cont(x0040288c))
    }
}

pub fn x0040287f() -> Cont {
    unsafe {
        // 0040287f push 3F8h
        push(0x3f8u32);
        // 00402884 call 00402C0Ch
        call(0x402889, Cont(x00402c0c))
    }
}

pub fn x00402889() -> Cont {
    unsafe {
        // 00402889 pop ecx
        MACHINE.regs.ecx = pop();
        // 0040288a jmp short 00402896h
        Cont(x00402896)
    }
}

pub fn x0040288c() -> Cont {
    unsafe {
        // 0040288c cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 0040288f jne short 004028A9h
        jne(Cont(x00402891), Cont(x004028a9))
    }
}

pub fn x00402891() -> Cont {
    unsafe {
        // 00402891 call 00403753h
        call(0x402896, Cont(x00403753))
    }
}

pub fn x00402896() -> Cont {
    unsafe {
        // 00402896 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402898 jne short 004028A9h
        jne(Cont(x0040289a), Cont(x004028a9))
    }
}

pub fn x0040289a() -> Cont {
    unsafe {
        // 0040289a push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 004028a0 call dword ptr ds:[4060C4h]
        call(0x4028a6, Cont(kernel32::HeapDestroy_stdcall))
    }
}

pub fn x004028a6() -> Cont {
    unsafe {
        // 004028a6 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004028a8 ret
        ret(0)
    }
}

pub fn x004028a9() -> Cont {
    unsafe {
        // 004028a9 push 1
        push(0x1u32);
        // 004028ab pop eax
        MACHINE.regs.eax = pop();
        // 004028ac ret
        ret(0)
    }
}

pub fn x00402a80() -> Cont {
    unsafe {
        // 00402a80 mov eax,ds:[4095A4h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4095a4u32);
        // 00402a85 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00402a88 je short 00402A97h
        je(Cont(x00402a8a), Cont(x00402a97))
    }
}

pub fn x00402a8a() -> Cont {
    unsafe {
        // 00402a8a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402a8c jne short 00402AB8h
        jne(Cont(x00402a8e), Cont(x00402ab8))
    }
}

pub fn x00402a8e() -> Cont {
    unsafe {
        // 00402a8e cmp dword ptr ds:[4070C8h],1
        sub(MACHINE.memory.read::<u32>(0x4070c8u32), 0x1u32);
        // 00402a95 jne short 00402AB8h
        jne(Cont(x00402a97), Cont(x00402ab8))
    }
}

pub fn x00402a97() -> Cont {
    unsafe {
        // 00402a97 push 0FCh
        push(0xfcu32);
        // 00402a9c call 00402AB9h
        call(0x402aa1, Cont(x00402ab9))
    }
}

pub fn x00402aa1() -> Cont {
    unsafe {
        // 00402aa1 mov eax,ds:[409700h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409700u32);
        // 00402aa6 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402aa7 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402aa9 je short 00402AADh
        je(Cont(x00402aab), Cont(x00402aad))
    }
}

pub fn x00402aab() -> Cont {
    unsafe {
        // 00402aab call eax
        call(0x402aad, indirect(MACHINE.regs.eax))
    }
}

pub fn x00402aad() -> Cont {
    unsafe {
        // 00402aad push 0FFh
        push(0xffu32);
        // 00402ab2 call 00402AB9h
        call(0x402ab7, Cont(x00402ab9))
    }
}

pub fn x00402ab7() -> Cont {
    unsafe {
        // 00402ab7 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402ab8 ret
        ret(0)
    }
}

pub fn x00402ab8() -> Cont {
    unsafe {
        // 00402ab8 ret
        ret(0)
    }
}

pub fn x00402ab9() -> Cont {
    unsafe {
        // 00402ab9 push ebp
        push(MACHINE.regs.ebp);
        // 00402aba mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00402abc sub esp,1A4h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x1a4u32);
        // 00402ac2 mov edx,[ebp+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402ac5 xor ecx,ecx
        MACHINE.regs.ecx ^= MACHINE.regs.ecx;
        // 00402ac7 mov eax,407178h
        MACHINE.regs.eax = 0x407178u32;
        // 00402acc cmp edx,[eax]
        sub(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 00402ace je short 00402ADBh
        je(Cont(x00402ad0), Cont(x00402adb))
    }
}

pub fn x00402acc() -> Cont {
    unsafe {
        // 00402acc cmp edx,[eax]
        sub(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 00402ace je short 00402ADBh
        je(Cont(x00402ad0), Cont(x00402adb))
    }
}

pub fn x00402ad0() -> Cont {
    unsafe {
        // 00402ad0 add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 00402ad3 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00402ad4 cmp eax,407208h
        sub(MACHINE.regs.eax, 0x407208u32);
        // 00402ad9 jl short 00402ACCh
        jl(Cont(x00402adb), Cont(x00402acc))
    }
}

pub fn x00402adb() -> Cont {
    unsafe {
        // 00402adb push esi
        push(MACHINE.regs.esi);
        // 00402adc mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 00402ade shl esi,3
        MACHINE.regs.esi = shl(MACHINE.regs.esi, 0x3u8);
        // 00402ae1 cmp edx,[esi+407178h]
        sub(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x407178u32)),
        );
        // 00402ae7 jne near ptr 00402C09h
        jne(Cont(x00402aed), Cont(x00402c09))
    }
}

pub fn x00402aed() -> Cont {
    unsafe {
        // 00402aed mov eax,ds:[4095A4h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x4095a4u32);
        // 00402af2 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00402af5 je near ptr 00402BE3h
        je(Cont(x00402afb), Cont(x00402be3))
    }
}

pub fn x00402afb() -> Cont {
    unsafe {
        // 00402afb test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402afd jne short 00402B0Ch
        jne(Cont(x00402aff), Cont(x00402b0c))
    }
}

pub fn x00402aff() -> Cont {
    unsafe {
        // 00402aff cmp dword ptr ds:[4070C8h],1
        sub(MACHINE.memory.read::<u32>(0x4070c8u32), 0x1u32);
        // 00402b06 je near ptr 00402BE3h
        je(Cont(x00402b0c), Cont(x00402be3))
    }
}

pub fn x00402b0c() -> Cont {
    unsafe {
        // 00402b0c cmp edx,0FCh
        sub(MACHINE.regs.edx, 0xfcu32);
        // 00402b12 je near ptr 00402C09h
        je(Cont(x00402b18), Cont(x00402c09))
    }
}

pub fn x00402b18() -> Cont {
    unsafe {
        // 00402b18 lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b1e push 104h
        push(0x104u32);
        // 00402b23 push eax
        push(MACHINE.regs.eax);
        // 00402b24 push 0
        push(0x0u32);
        // 00402b26 call dword ptr ds:[40602Ch]
        call(0x402b2c, Cont(kernel32::GetModuleFileNameA_stdcall))
    }
}

pub fn x00402b2c() -> Cont {
    unsafe {
        // 00402b2c test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402b2e jne short 00402B43h
        jne(Cont(x00402b30), Cont(x00402b43))
    }
}

pub fn x00402b30() -> Cont {
    unsafe {
        // 00402b30 lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b36 push 40643Ch
        push(0x40643cu32);
        // 00402b3b push eax
        push(MACHINE.regs.eax);
        // 00402b3c call 004045A0h
        call(0x402b41, Cont(x004045a0))
    }
}

pub fn x00402b41() -> Cont {
    unsafe {
        // 00402b41 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402b42 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402b43 lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b49 push edi
        push(MACHINE.regs.edi);
        // 00402b4a push eax
        push(MACHINE.regs.eax);
        // 00402b4b lea edi,[ebp-1A4h]
        MACHINE.regs.edi = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b51 call 00404690h
        call(0x402b56, Cont(x00404690))
    }
}

pub fn x00402b43() -> Cont {
    unsafe {
        // 00402b43 lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b49 push edi
        push(MACHINE.regs.edi);
        // 00402b4a push eax
        push(MACHINE.regs.eax);
        // 00402b4b lea edi,[ebp-1A4h]
        MACHINE.regs.edi = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b51 call 00404690h
        call(0x402b56, Cont(x00404690))
    }
}

pub fn x00402b56() -> Cont {
    unsafe {
        // 00402b56 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00402b57 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402b58 cmp eax,3Ch
        sub(MACHINE.regs.eax, 0x3cu32);
        // 00402b5b jbe short 00402B86h
        jbe(Cont(x00402b5d), Cont(x00402b86))
    }
}

pub fn x00402b5d() -> Cont {
    unsafe {
        // 00402b5d lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b63 push eax
        push(MACHINE.regs.eax);
        // 00402b64 call 00404690h
        call(0x402b69, Cont(x00404690))
    }
}

pub fn x00402b69() -> Cont {
    unsafe {
        // 00402b69 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00402b6b lea eax,[ebp-1A4h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffe5cu32);
        // 00402b71 sub eax,3Bh
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x3bu32);
        // 00402b74 push 3
        push(0x3u32);
        // 00402b76 add edi,eax
        MACHINE.regs.edi = add(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00402b78 push 406438h
        push(0x406438u32);
        // 00402b7d push edi
        push(MACHINE.regs.edi);
        // 00402b7e call 00404B80h
        call(0x402b83, Cont(x00404b80))
    }
}

pub fn x00402b83() -> Cont {
    unsafe {
        // 00402b83 add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 00402b86 lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402b8c push 40641Ch
        push(0x40641cu32);
        // 00402b91 push eax
        push(MACHINE.regs.eax);
        // 00402b92 call 004045A0h
        call(0x402b97, Cont(x004045a0))
    }
}

pub fn x00402b86() -> Cont {
    unsafe {
        // 00402b86 lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402b8c push 40641Ch
        push(0x40641cu32);
        // 00402b91 push eax
        push(MACHINE.regs.eax);
        // 00402b92 call 004045A0h
        call(0x402b97, Cont(x004045a0))
    }
}

pub fn x00402b97() -> Cont {
    unsafe {
        // 00402b97 lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402b9d push edi
        push(MACHINE.regs.edi);
        // 00402b9e push eax
        push(MACHINE.regs.eax);
        // 00402b9f call 004045B0h
        call(0x402ba4, Cont(x004045b0))
    }
}

pub fn x00402ba4() -> Cont {
    unsafe {
        // 00402ba4 lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402baa push 406418h
        push(0x406418u32);
        // 00402baf push eax
        push(MACHINE.regs.eax);
        // 00402bb0 call 004045B0h
        call(0x402bb5, Cont(x004045b0))
    }
}

pub fn x00402bb5() -> Cont {
    unsafe {
        // 00402bb5 push dword ptr [esi+40717Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x40717cu32)),
        );
        // 00402bbb lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402bc1 push eax
        push(MACHINE.regs.eax);
        // 00402bc2 call 004045B0h
        call(0x402bc7, Cont(x004045b0))
    }
}

pub fn x00402bc7() -> Cont {
    unsafe {
        // 00402bc7 push 12010h
        push(0x12010u32);
        // 00402bcc lea eax,[ebp-0A0h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffff60u32);
        // 00402bd2 push 4063F0h
        push(0x4063f0u32);
        // 00402bd7 push eax
        push(MACHINE.regs.eax);
        // 00402bd8 call 00404AEFh
        call(0x402bdd, Cont(x00404aef))
    }
}

pub fn x00402bdd() -> Cont {
    unsafe {
        // 00402bdd add esp,2Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x2cu32);
        // 00402be0 pop edi
        MACHINE.regs.edi = pop();
        // 00402be1 jmp short 00402C09h
        Cont(x00402c09)
    }
}

pub fn x00402be3() -> Cont {
    unsafe {
        // 00402be3 lea eax,[ebp+8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0x8u32);
        // 00402be6 lea esi,[esi+40717Ch]
        MACHINE.regs.esi = MACHINE.regs.esi.wrapping_add(0x40717cu32);
        // 00402bec push 0
        push(0x0u32);
        // 00402bee push eax
        push(MACHINE.regs.eax);
        // 00402bef push dword ptr [esi]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00402bf1 call 00404690h
        call(0x402bf6, Cont(x00404690))
    }
}

pub fn x00402bf6() -> Cont {
    unsafe {
        // 00402bf6 pop ecx
        MACHINE.regs.ecx = pop();
        // 00402bf7 push eax
        push(MACHINE.regs.eax);
        // 00402bf8 push dword ptr [esi]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00402bfa push 0FFFFFFF4h
        push(0xfffffff4u32);
        // 00402bfc call dword ptr ds:[4060B4h]
        call(0x402c02, Cont(kernel32::GetStdHandle_stdcall))
    }
}

pub fn x00402c02() -> Cont {
    unsafe {
        // 00402c02 push eax
        push(MACHINE.regs.eax);
        // 00402c03 call dword ptr ds:[406060h]
        call(0x402c09, Cont(kernel32::WriteFile_stdcall))
    }
}

pub fn x00402c09() -> Cont {
    unsafe {
        // 00402c09 pop esi
        MACHINE.regs.esi = pop();
        // 00402c0a leave
        leave();
        // 00402c0b ret
        ret(0)
    }
}

pub fn x00402c0c() -> Cont {
    unsafe {
        // 00402c0c push 140h
        push(0x140u32);
        // 00402c11 push 0
        push(0x0u32);
        // 00402c13 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00402c19 call dword ptr ds:[406028h]
        call(0x402c1f, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00402c1f() -> Cont {
    unsafe {
        // 00402c1f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402c21 mov ds:[40997Ch],eax
        MACHINE.memory.write::<u32>(0x40997cu32, MACHINE.regs.eax);
        // 00402c26 jne short 00402C29h
        jne(Cont(x00402c28), Cont(x00402c29))
    }
}

pub fn x00402c28() -> Cont {
    unsafe {
        // 00402c28 ret
        ret(0)
    }
}

pub fn x00402c29() -> Cont {
    unsafe {
        // 00402c29 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00402c2d and dword ptr ds:[409974h],0
        MACHINE.memory.write::<u32>(
            0x409974u32,
            and(MACHINE.memory.read::<u32>(0x409974u32), 0x0u32),
        );
        // 00402c34 and dword ptr ds:[409978h],0
        MACHINE.memory.write::<u32>(
            0x409978u32,
            and(MACHINE.memory.read::<u32>(0x409978u32), 0x0u32),
        );
        // 00402c3b push 1
        push(0x1u32);
        // 00402c3d mov ds:[409970h],eax
        MACHINE.memory.write::<u32>(0x409970u32, MACHINE.regs.eax);
        // 00402c42 mov ds:[409980h],ecx
        MACHINE.memory.write::<u32>(0x409980u32, MACHINE.regs.ecx);
        // 00402c48 mov dword ptr ds:[409968h],10h
        MACHINE.memory.write::<u32>(0x409968u32, 0x10u32);
        // 00402c52 pop eax
        MACHINE.regs.eax = pop();
        // 00402c53 ret
        ret(0)
    }
}

pub fn x00402c54() -> Cont {
    unsafe {
        // 00402c54 mov eax,ds:[409978h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409978u32);
        // 00402c59 lea ecx,[eax+eax*4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00402c5c mov eax,ds:[40997Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40997cu32);
        // 00402c61 lea ecx,[eax+ecx*4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0x0u32);
        // 00402c64 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00402c66 jae short 00402C7Ch
        jae(Cont(x00402c68), Cont(x00402c7c))
    }
}

pub fn x00402c64() -> Cont {
    unsafe {
        // 00402c64 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00402c66 jae short 00402C7Ch
        jae(Cont(x00402c68), Cont(x00402c7c))
    }
}

pub fn x00402c68() -> Cont {
    unsafe {
        // 00402c68 mov edx,[esp+4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00402c6c sub edx,[eax+0Ch]
        MACHINE.regs.edx = sub(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0xcu32)),
        );
        // 00402c6f cmp edx,100000h
        sub(MACHINE.regs.edx, 0x100000u32);
        // 00402c75 jb short 00402C7Eh
        jb(Cont(x00402c77), Cont(x00402c7e))
    }
}

pub fn x00402c77() -> Cont {
    unsafe {
        // 00402c77 add eax,14h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x14u32);
        // 00402c7a jmp short 00402C64h
        Cont(x00402c64)
    }
}

pub fn x00402c7c() -> Cont {
    unsafe {
        // 00402c7c xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00402c7e ret
        ret(0)
    }
}

pub fn x00402c7e() -> Cont {
    unsafe {
        // 00402c7e ret
        ret(0)
    }
}

pub fn x00402c7f() -> Cont {
    unsafe {
        // 00402c7f push ebp
        push(MACHINE.regs.ebp);
        // 00402c80 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00402c82 sub esp,10h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x10u32);
        // 00402c85 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402c88 push ebx
        push(MACHINE.regs.ebx);
        // 00402c89 push esi
        push(MACHINE.regs.esi);
        // 00402c8a mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402c8d mov eax,[ecx+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x10u32));
        // 00402c90 push edi
        push(MACHINE.regs.edi);
        // 00402c91 mov edi,esi
        MACHINE.regs.edi = MACHINE.regs.esi;
        // 00402c93 add esi,0FFFFFFFCh
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0xfffffffcu32);
        // 00402c96 sub edi,[ecx+0Ch]
        MACHINE.regs.edi = sub(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xcu32)),
        );
        // 00402c99 shr edi,0Fh
        MACHINE.regs.edi = shr(MACHINE.regs.edi, 0xfu8);
        // 00402c9c mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 00402c9e imul ecx,204h
        imul();
        // 00402ca4 lea ecx,[ecx+eax+144h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x144u32);
        // 00402cab mov [ebp-10h],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff0u32),
            MACHINE.regs.ecx,
        );
        // 00402cae mov ecx,[esi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00402cb0 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00402cb1 test cl,1
        and(MACHINE.regs.get_cl(), 0x1u8);
        // 00402cb4 mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00402cb7 jne near ptr 00402FA3h
        jne(Cont(x00402cbd), Cont(x00402fa3))
    }
}

pub fn x00402cbd() -> Cont {
    unsafe {
        // 00402cbd mov edx,[ecx+esi]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0x0u32),
        );
        // 00402cc0 lea ebx,[ecx+esi]
        MACHINE.regs.ebx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0x0u32);
        // 00402cc3 mov [ebp-0Ch],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.edx,
        );
        // 00402cc6 mov edx,[esi-4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0xfffffffcu32));
        // 00402cc9 mov [ebp-8],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.edx,
        );
        // 00402ccc mov edx,[ebp-0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 00402ccf test dl,1
        and(MACHINE.regs.get_dl(), 0x1u8);
        // 00402cd2 mov [ebp+0Ch],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32), MACHINE.regs.ebx);
        // 00402cd5 jne short 00402D55h
        jne(Cont(x00402cd7), Cont(x00402d55))
    }
}

pub fn x00402cd7() -> Cont {
    unsafe {
        // 00402cd7 sar edx,4
        sar();
        // 00402cda dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402cdb cmp edx,3Fh
        sub(MACHINE.regs.edx, 0x3fu32);
        // 00402cde jbe short 00402CE3h
        jbe(Cont(x00402ce0), Cont(x00402ce3))
    }
}

pub fn x00402ce0() -> Cont {
    unsafe {
        // 00402ce0 push 3Fh
        push(0x3fu32);
        // 00402ce2 pop edx
        MACHINE.regs.edx = pop();
        // 00402ce3 mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00402ce6 cmp ecx,[ebx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
        );
        // 00402ce9 jne short 00402D37h
        jne(Cont(x00402ceb), Cont(x00402d37))
    }
}

pub fn x00402ce3() -> Cont {
    unsafe {
        // 00402ce3 mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00402ce6 cmp ecx,[ebx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
        );
        // 00402ce9 jne short 00402D37h
        jne(Cont(x00402ceb), Cont(x00402d37))
    }
}

pub fn x00402ceb() -> Cont {
    unsafe {
        // 00402ceb cmp edx,20h
        sub(MACHINE.regs.edx, 0x20u32);
        // 00402cee jae short 00402D0Eh
        jae(Cont(x00402cf0), Cont(x00402d0e))
    }
}

pub fn x00402cf0() -> Cont {
    unsafe {
        // 00402cf0 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402cf5 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00402cf7 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402cf9 lea ecx,[edx+eax+4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 00402cfd not ebx
        not();
        // 00402cff and [eax+edi*4+44h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edi * 4))
                .wrapping_add(0x44u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edi * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 00402d03 dec byte ptr [ecx]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 00402d05 jne short 00402D2Fh
        jne(Cont(x00402d07), Cont(x00402d2f))
    }
}

pub fn x00402d07() -> Cont {
    unsafe {
        // 00402d07 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402d0a and [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402d0c jmp short 00402D2Fh
        Cont(x00402d2f)
    }
}

pub fn x00402d0e() -> Cont {
    unsafe {
        // 00402d0e lea ecx,[edx-20h]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xffffffe0u32);
        // 00402d11 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402d16 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402d18 lea ecx,[edx+eax+4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 00402d1c not ebx
        not();
        // 00402d1e and [eax+edi*4+0C4h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edi * 4))
                .wrapping_add(0xc4u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edi * 4))
                        .wrapping_add(0xc4u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 00402d25 dec byte ptr [ecx]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 00402d27 jne short 00402D2Fh
        jne(Cont(x00402d29), Cont(x00402d2f))
    }
}

pub fn x00402d29() -> Cont {
    unsafe {
        // 00402d29 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402d2c and [ecx+4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402d2f mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402d32 mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402d35 jmp short 00402D3Ah
        Cont(x00402d3a)
    }
}

pub fn x00402d2f() -> Cont {
    unsafe {
        // 00402d2f mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402d32 mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402d35 jmp short 00402D3Ah
        Cont(x00402d3a)
    }
}

pub fn x00402d37() -> Cont {
    unsafe {
        // 00402d37 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402d3a mov edx,[ebx+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32));
        // 00402d3d mov ebx,[ebx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00402d40 add ecx,[ebp-0Ch]
        MACHINE.regs.ecx = add(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
        );
        // 00402d43 mov [edx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 00402d46 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402d49 mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00402d4c mov ebx,[edx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 00402d4f mov edx,[edx+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 00402d52 mov [ebx+8],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.edx);
        // 00402d55 mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00402d57 sar edx,4
        sar();
        // 00402d5a dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402d5b cmp edx,3Fh
        sub(MACHINE.regs.edx, 0x3fu32);
        // 00402d5e jbe short 00402D63h
        jbe(Cont(x00402d60), Cont(x00402d63))
    }
}

pub fn x00402d3a() -> Cont {
    unsafe {
        // 00402d3a mov edx,[ebx+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32));
        // 00402d3d mov ebx,[ebx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00402d40 add ecx,[ebp-0Ch]
        MACHINE.regs.ecx = add(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
        );
        // 00402d43 mov [edx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 00402d46 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402d49 mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00402d4c mov ebx,[edx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 00402d4f mov edx,[edx+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 00402d52 mov [ebx+8],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.edx);
        // 00402d55 mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00402d57 sar edx,4
        sar();
        // 00402d5a dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402d5b cmp edx,3Fh
        sub(MACHINE.regs.edx, 0x3fu32);
        // 00402d5e jbe short 00402D63h
        jbe(Cont(x00402d60), Cont(x00402d63))
    }
}

pub fn x00402d55() -> Cont {
    unsafe {
        // 00402d55 mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00402d57 sar edx,4
        sar();
        // 00402d5a dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402d5b cmp edx,3Fh
        sub(MACHINE.regs.edx, 0x3fu32);
        // 00402d5e jbe short 00402D63h
        jbe(Cont(x00402d60), Cont(x00402d63))
    }
}

pub fn x00402d60() -> Cont {
    unsafe {
        // 00402d60 push 3Fh
        push(0x3fu32);
        // 00402d62 pop edx
        MACHINE.regs.edx = pop();
        // 00402d63 mov ebx,[ebp-8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00402d66 and ebx,1
        MACHINE.regs.ebx = and(MACHINE.regs.ebx, 0x1u32);
        // 00402d69 mov [ebp-0Ch],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.ebx,
        );
        // 00402d6c jne near ptr 00402E06h
        jne(Cont(x00402d72), Cont(x00402e06))
    }
}

pub fn x00402d63() -> Cont {
    unsafe {
        // 00402d63 mov ebx,[ebp-8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00402d66 and ebx,1
        MACHINE.regs.ebx = and(MACHINE.regs.ebx, 0x1u32);
        // 00402d69 mov [ebp-0Ch],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.ebx,
        );
        // 00402d6c jne near ptr 00402E06h
        jne(Cont(x00402d72), Cont(x00402e06))
    }
}

pub fn x00402d72() -> Cont {
    unsafe {
        // 00402d72 sub esi,[ebp-8]
        MACHINE.regs.esi = sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00402d75 mov ebx,[ebp-8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00402d78 sar ebx,4
        sar();
        // 00402d7b push 3Fh
        push(0x3fu32);
        // 00402d7d mov [ebp+0Ch],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32), MACHINE.regs.esi);
        // 00402d80 dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 00402d81 pop esi
        MACHINE.regs.esi = pop();
        // 00402d82 cmp ebx,esi
        sub(MACHINE.regs.ebx, MACHINE.regs.esi);
        // 00402d84 jbe short 00402D88h
        jbe(Cont(x00402d86), Cont(x00402d88))
    }
}

pub fn x00402d86() -> Cont {
    unsafe {
        // 00402d86 mov ebx,esi
        MACHINE.regs.ebx = MACHINE.regs.esi;
        // 00402d88 add ecx,[ebp-8]
        MACHINE.regs.ecx = add(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00402d8b mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00402d8d mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00402d90 sar edx,4
        sar();
        // 00402d93 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402d94 cmp edx,esi
        sub(MACHINE.regs.edx, MACHINE.regs.esi);
        // 00402d96 jbe short 00402D9Ah
        jbe(Cont(x00402d98), Cont(x00402d9a))
    }
}

pub fn x00402d88() -> Cont {
    unsafe {
        // 00402d88 add ecx,[ebp-8]
        MACHINE.regs.ecx = add(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00402d8b mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00402d8d mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00402d90 sar edx,4
        sar();
        // 00402d93 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00402d94 cmp edx,esi
        sub(MACHINE.regs.edx, MACHINE.regs.esi);
        // 00402d96 jbe short 00402D9Ah
        jbe(Cont(x00402d98), Cont(x00402d9a))
    }
}

pub fn x00402d98() -> Cont {
    unsafe {
        // 00402d98 mov edx,esi
        MACHINE.regs.edx = MACHINE.regs.esi;
        // 00402d9a cmp ebx,edx
        sub(MACHINE.regs.ebx, MACHINE.regs.edx);
        // 00402d9c je short 00402E01h
        je(Cont(x00402d9e), Cont(x00402e01))
    }
}

pub fn x00402d9a() -> Cont {
    unsafe {
        // 00402d9a cmp ebx,edx
        sub(MACHINE.regs.ebx, MACHINE.regs.edx);
        // 00402d9c je short 00402E01h
        je(Cont(x00402d9e), Cont(x00402e01))
    }
}

pub fn x00402d9e() -> Cont {
    unsafe {
        // 00402d9e mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402da1 mov esi,[ecx+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00402da4 cmp esi,[ecx+8]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32)),
        );
        // 00402da7 jne short 00402DE9h
        jne(Cont(x00402da9), Cont(x00402de9))
    }
}

pub fn x00402da9() -> Cont {
    unsafe {
        // 00402da9 cmp ebx,20h
        sub(MACHINE.regs.ebx, 0x20u32);
        // 00402dac jae short 00402DCAh
        jae(Cont(x00402dae), Cont(x00402dca))
    }
}

pub fn x00402dae() -> Cont {
    unsafe {
        // 00402dae mov esi,80000000h
        MACHINE.regs.esi = 0x80000000u32;
        // 00402db3 mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00402db5 shr esi,cl
        MACHINE.regs.esi = shr(MACHINE.regs.esi, MACHINE.regs.get_cl());
        // 00402db7 not esi
        not();
        // 00402db9 and [eax+edi*4+44h],esi
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edi * 4))
                .wrapping_add(0x44u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edi * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.esi,
            ),
        );
        // 00402dbd dec byte ptr [ebx+eax+4]
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .ebx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            dec(MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .ebx
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            )),
        );
        // 00402dc1 jne short 00402DE9h
        jne(Cont(x00402dc3), Cont(x00402de9))
    }
}

pub fn x00402dc3() -> Cont {
    unsafe {
        // 00402dc3 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402dc6 and [ecx],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.esi,
            ),
        );
        // 00402dc8 jmp short 00402DE9h
        Cont(x00402de9)
    }
}

pub fn x00402dca() -> Cont {
    unsafe {
        // 00402dca lea ecx,[ebx-20h]
        MACHINE.regs.ecx = MACHINE.regs.ebx.wrapping_add(0xffffffe0u32);
        // 00402dcd mov esi,80000000h
        MACHINE.regs.esi = 0x80000000u32;
        // 00402dd2 shr esi,cl
        MACHINE.regs.esi = shr(MACHINE.regs.esi, MACHINE.regs.get_cl());
        // 00402dd4 not esi
        not();
        // 00402dd6 and [eax+edi*4+0C4h],esi
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edi * 4))
                .wrapping_add(0xc4u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edi * 4))
                        .wrapping_add(0xc4u32),
                ),
                MACHINE.regs.esi,
            ),
        );
        // 00402ddd dec byte ptr [ebx+eax+4]
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .ebx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            dec(MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .ebx
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            )),
        );
        // 00402de1 jne short 00402DE9h
        jne(Cont(x00402de3), Cont(x00402de9))
    }
}

pub fn x00402de3() -> Cont {
    unsafe {
        // 00402de3 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402de6 and [ecx+4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.esi,
            ),
        );
        // 00402de9 mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402dec mov esi,[ecx+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32));
        // 00402def mov ecx,[ecx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00402df2 mov [esi+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 00402df5 mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402df8 mov esi,[ecx+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00402dfb mov ecx,[ecx+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32));
        // 00402dfe mov [esi+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 00402e01 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402e04 jmp short 00402E09h
        Cont(x00402e09)
    }
}

pub fn x00402de9() -> Cont {
    unsafe {
        // 00402de9 mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402dec mov esi,[ecx+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32));
        // 00402def mov ecx,[ecx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00402df2 mov [esi+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 00402df5 mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402df8 mov esi,[ecx+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00402dfb mov ecx,[ecx+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32));
        // 00402dfe mov [esi+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 00402e01 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402e04 jmp short 00402E09h
        Cont(x00402e09)
    }
}

pub fn x00402e01() -> Cont {
    unsafe {
        // 00402e01 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00402e04 jmp short 00402E09h
        Cont(x00402e09)
    }
}

pub fn x00402e06() -> Cont {
    unsafe {
        // 00402e06 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402e09 cmp dword ptr [ebp-0Ch],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
            0x0u32,
        );
        // 00402e0d jne short 00402E17h
        jne(Cont(x00402e0f), Cont(x00402e17))
    }
}

pub fn x00402e09() -> Cont {
    unsafe {
        // 00402e09 cmp dword ptr [ebp-0Ch],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
            0x0u32,
        );
        // 00402e0d jne short 00402E17h
        jne(Cont(x00402e0f), Cont(x00402e17))
    }
}

pub fn x00402e0f() -> Cont {
    unsafe {
        // 00402e0f cmp ebx,edx
        sub(MACHINE.regs.ebx, MACHINE.regs.edx);
        // 00402e11 je near ptr 00402E98h
        je(Cont(x00402e17), Cont(x00402e98))
    }
}

pub fn x00402e17() -> Cont {
    unsafe {
        // 00402e17 mov ecx,[ebp-10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00402e1a mov ebx,[ecx+edx*8+4]
        MACHINE.regs.ebx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.edx * 8))
                .wrapping_add(0x4u32),
        );
        // 00402e1e lea ecx,[ecx+edx*8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edx * 8))
            .wrapping_add(0x0u32);
        // 00402e21 mov [esi+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 00402e24 mov [esi+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 00402e27 mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 00402e2a mov ecx,[esi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 00402e2d mov [ecx+8],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.esi);
        // 00402e30 mov ecx,[esi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 00402e33 cmp ecx,[esi+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32)),
        );
        // 00402e36 jne short 00402E98h
        jne(Cont(x00402e38), Cont(x00402e98))
    }
}

pub fn x00402e38() -> Cont {
    unsafe {
        // 00402e38 mov cl,[edx+eax+4]
        MACHINE.regs.set_cl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .edx
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            ),
        );
        // 00402e3c cmp edx,20h
        sub(MACHINE.regs.edx, 0x20u32);
        // 00402e3f mov [ebp+0Fh],cl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32), MACHINE.regs.get_cl());
        // 00402e42 inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 00402e44 mov [edx+eax+4],cl
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .edx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            MACHINE.regs.get_cl(),
        );
        // 00402e48 jae short 00402E6Fh
        jae(Cont(x00402e4a), Cont(x00402e6f))
    }
}

pub fn x00402e4a() -> Cont {
    unsafe {
        // 00402e4a cmp byte ptr [ebp+0Fh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32)),
            0x0u8,
        );
        // 00402e4e jne short 00402E5Eh
        jne(Cont(x00402e50), Cont(x00402e5e))
    }
}

pub fn x00402e50() -> Cont {
    unsafe {
        // 00402e50 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402e55 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00402e57 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402e59 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402e5c or [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402e5e mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402e63 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00402e65 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402e67 lea eax,[eax+edi*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 4))
            .wrapping_add(0x44u32);
        // 00402e6b or [eax],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402e6d jmp short 00402E98h
        Cont(x00402e98)
    }
}

pub fn x00402e5e() -> Cont {
    unsafe {
        // 00402e5e mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402e63 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00402e65 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402e67 lea eax,[eax+edi*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 4))
            .wrapping_add(0x44u32);
        // 00402e6b or [eax],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402e6d jmp short 00402E98h
        Cont(x00402e98)
    }
}

pub fn x00402e6f() -> Cont {
    unsafe {
        // 00402e6f cmp byte ptr [ebp+0Fh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32)),
            0x0u8,
        );
        // 00402e73 jne short 00402E85h
        jne(Cont(x00402e75), Cont(x00402e85))
    }
}

pub fn x00402e75() -> Cont {
    unsafe {
        // 00402e75 lea ecx,[edx-20h]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xffffffe0u32);
        // 00402e78 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00402e7d shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00402e7f mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402e82 or [ecx+4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00402e85 lea ecx,[edx-20h]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xffffffe0u32);
        // 00402e88 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 00402e8d shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00402e8f lea eax,[eax+edi*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 4))
            .wrapping_add(0xc4u32);
        // 00402e96 or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00402e98 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402e9b mov [esi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00402e9d mov [eax+esi-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00402ea1 mov eax,[ebp-10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00402ea4 dec dword ptr [eax]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32))),
        );
        // 00402ea6 jne near ptr 00402FA3h
        jne(Cont(x00402eac), Cont(x00402fa3))
    }
}

pub fn x00402e85() -> Cont {
    unsafe {
        // 00402e85 lea ecx,[edx-20h]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xffffffe0u32);
        // 00402e88 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 00402e8d shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00402e8f lea eax,[eax+edi*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 4))
            .wrapping_add(0xc4u32);
        // 00402e96 or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00402e98 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402e9b mov [esi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00402e9d mov [eax+esi-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00402ea1 mov eax,[ebp-10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00402ea4 dec dword ptr [eax]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32))),
        );
        // 00402ea6 jne near ptr 00402FA3h
        jne(Cont(x00402eac), Cont(x00402fa3))
    }
}

pub fn x00402e98() -> Cont {
    unsafe {
        // 00402e98 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00402e9b mov [esi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00402e9d mov [eax+esi-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00402ea1 mov eax,[ebp-10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00402ea4 dec dword ptr [eax]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32))),
        );
        // 00402ea6 jne near ptr 00402FA3h
        jne(Cont(x00402eac), Cont(x00402fa3))
    }
}

pub fn x00402eac() -> Cont {
    unsafe {
        // 00402eac mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402eb1 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00402eb3 je near ptr 00402F95h
        je(Cont(x00402eb9), Cont(x00402f95))
    }
}

pub fn x00402eb9() -> Cont {
    unsafe {
        // 00402eb9 mov ecx,ds:[40996Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40996cu32);
        // 00402ebf mov esi,ds:[406098h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x406098u32);
        // 00402ec5 shl ecx,0Fh
        MACHINE.regs.ecx = shl(MACHINE.regs.ecx, 0xfu8);
        // 00402ec8 add ecx,[eax+0Ch]
        MACHINE.regs.ecx = add(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0xcu32)),
        );
        // 00402ecb mov ebx,8000h
        MACHINE.regs.ebx = 0x8000u32;
        // 00402ed0 push 4000h
        push(0x4000u32);
        // 00402ed5 push ebx
        push(MACHINE.regs.ebx);
        // 00402ed6 push ecx
        push(MACHINE.regs.ecx);
        // 00402ed7 call esi
        call(0x402ed9, indirect(MACHINE.regs.esi))
    }
}

pub fn x00402ed9() -> Cont {
    unsafe {
        // 00402ed9 mov ecx,ds:[40996Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40996cu32);
        // 00402edf mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402ee4 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 00402ee9 shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00402eeb or [eax+8],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00402eee mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402ef3 mov ecx,ds:[40996Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40996cu32);
        // 00402ef9 mov eax,[eax+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00402efc and dword ptr [eax+ecx*4+0C4h],0
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ecx * 4))
                .wrapping_add(0xc4u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.ecx * 4))
                        .wrapping_add(0xc4u32),
                ),
                0x0u32,
            ),
        );
        // 00402f04 mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402f09 mov eax,[eax+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00402f0c dec byte ptr [eax+43h]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x43u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x43u32))),
        );
        // 00402f0f mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402f14 mov ecx,[eax+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00402f17 cmp byte ptr [ecx+43h],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x43u32)),
            0x0u8,
        );
        // 00402f1b jne short 00402F26h
        jne(Cont(x00402f1d), Cont(x00402f26))
    }
}

pub fn x00402f1d() -> Cont {
    unsafe {
        // 00402f1d and dword ptr [eax+4],0FFFFFFFEh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32)),
                0xfffffffeu32,
            ),
        );
        // 00402f21 mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402f26 cmp dword ptr [eax+8],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
            0xffffffffu32,
        );
        // 00402f2a jne short 00402F95h
        jne(Cont(x00402f2c), Cont(x00402f95))
    }
}

pub fn x00402f26() -> Cont {
    unsafe {
        // 00402f26 cmp dword ptr [eax+8],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
            0xffffffffu32,
        );
        // 00402f2a jne short 00402F95h
        jne(Cont(x00402f2c), Cont(x00402f95))
    }
}

pub fn x00402f2c() -> Cont {
    unsafe {
        // 00402f2c push ebx
        push(MACHINE.regs.ebx);
        // 00402f2d push 0
        push(0x0u32);
        // 00402f2f push dword ptr [eax+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0xcu32)),
        );
        // 00402f32 call esi
        call(0x402f34, indirect(MACHINE.regs.esi))
    }
}

pub fn x00402f34() -> Cont {
    unsafe {
        // 00402f34 mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402f39 push dword ptr [eax+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32)),
        );
        // 00402f3c push 0
        push(0x0u32);
        // 00402f3e push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00402f44 call dword ptr ds:[406090h]
        call(0x402f4a, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x00402f4a() -> Cont {
    unsafe {
        // 00402f4a mov eax,ds:[409978h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409978u32);
        // 00402f4f mov edx,ds:[40997Ch]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x40997cu32);
        // 00402f55 lea eax,[eax+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00402f58 shl eax,2
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x2u8);
        // 00402f5b mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 00402f5d mov eax,ds:[409974h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409974u32);
        // 00402f62 sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00402f64 lea ecx,[ecx+edx-14h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0xffffffecu32);
        // 00402f68 push ecx
        push(MACHINE.regs.ecx);
        // 00402f69 lea ecx,[eax+14h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0x14u32);
        // 00402f6c push ecx
        push(MACHINE.regs.ecx);
        // 00402f6d push eax
        push(MACHINE.regs.eax);
        // 00402f6e call 00404C80h
        call(0x402f73, Cont(x00404c80))
    }
}

pub fn x00402f73() -> Cont {
    unsafe {
        // 00402f73 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402f76 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00402f79 dec dword ptr ds:[409978h]
        MACHINE
            .memory
            .write::<u32>(0x409978u32, dec(MACHINE.memory.read::<u32>(0x409978u32)));
        // 00402f7f cmp eax,ds:[409974h]
        sub(MACHINE.regs.eax, MACHINE.memory.read::<u32>(0x409974u32));
        // 00402f85 jbe short 00402F8Bh
        jbe(Cont(x00402f87), Cont(x00402f8b))
    }
}

pub fn x00402f87() -> Cont {
    unsafe {
        // 00402f87 sub dword ptr [ebp+8],14h
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x8u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
                0x14u32,
            ),
        );
        // 00402f8b mov eax,ds:[40997Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40997cu32);
        // 00402f90 mov ds:[409970h],eax
        MACHINE.memory.write::<u32>(0x409970u32, MACHINE.regs.eax);
        // 00402f95 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402f98 mov ds:[40996Ch],edi
        MACHINE.memory.write::<u32>(0x40996cu32, MACHINE.regs.edi);
        // 00402f9e mov ds:[409974h],eax
        MACHINE.memory.write::<u32>(0x409974u32, MACHINE.regs.eax);
        // 00402fa3 pop edi
        MACHINE.regs.edi = pop();
        // 00402fa4 pop esi
        MACHINE.regs.esi = pop();
        // 00402fa5 pop ebx
        MACHINE.regs.ebx = pop();
        // 00402fa6 leave
        leave();
        // 00402fa7 ret
        ret(0)
    }
}

pub fn x00402f8b() -> Cont {
    unsafe {
        // 00402f8b mov eax,ds:[40997Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40997cu32);
        // 00402f90 mov ds:[409970h],eax
        MACHINE.memory.write::<u32>(0x409970u32, MACHINE.regs.eax);
        // 00402f95 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402f98 mov ds:[40996Ch],edi
        MACHINE.memory.write::<u32>(0x40996cu32, MACHINE.regs.edi);
        // 00402f9e mov ds:[409974h],eax
        MACHINE.memory.write::<u32>(0x409974u32, MACHINE.regs.eax);
        // 00402fa3 pop edi
        MACHINE.regs.edi = pop();
        // 00402fa4 pop esi
        MACHINE.regs.esi = pop();
        // 00402fa5 pop ebx
        MACHINE.regs.ebx = pop();
        // 00402fa6 leave
        leave();
        // 00402fa7 ret
        ret(0)
    }
}

pub fn x00402f95() -> Cont {
    unsafe {
        // 00402f95 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402f98 mov ds:[40996Ch],edi
        MACHINE.memory.write::<u32>(0x40996cu32, MACHINE.regs.edi);
        // 00402f9e mov ds:[409974h],eax
        MACHINE.memory.write::<u32>(0x409974u32, MACHINE.regs.eax);
        // 00402fa3 pop edi
        MACHINE.regs.edi = pop();
        // 00402fa4 pop esi
        MACHINE.regs.esi = pop();
        // 00402fa5 pop ebx
        MACHINE.regs.ebx = pop();
        // 00402fa6 leave
        leave();
        // 00402fa7 ret
        ret(0)
    }
}

pub fn x00402fa3() -> Cont {
    unsafe {
        // 00402fa3 pop edi
        MACHINE.regs.edi = pop();
        // 00402fa4 pop esi
        MACHINE.regs.esi = pop();
        // 00402fa5 pop ebx
        MACHINE.regs.ebx = pop();
        // 00402fa6 leave
        leave();
        // 00402fa7 ret
        ret(0)
    }
}

pub fn x00402fa8() -> Cont {
    unsafe {
        // 00402fa8 push ebp
        push(MACHINE.regs.ebp);
        // 00402fa9 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00402fab sub esp,14h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x14u32);
        // 00402fae mov eax,ds:[409978h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409978u32);
        // 00402fb3 mov edx,ds:[40997Ch]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x40997cu32);
        // 00402fb9 push ebx
        push(MACHINE.regs.ebx);
        // 00402fba push esi
        push(MACHINE.regs.esi);
        // 00402fbb lea eax,[eax+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00402fbe push edi
        push(MACHINE.regs.edi);
        // 00402fbf lea edi,[edx+eax*4]
        MACHINE.regs.edi = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00402fc2 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00402fc5 mov [ebp-4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edi,
        );
        // 00402fc8 lea ecx,[eax+17h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0x17u32);
        // 00402fcb and ecx,0FFFFFFF0h
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0xfffffff0u32);
        // 00402fce mov [ebp-10h],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff0u32),
            MACHINE.regs.ecx,
        );
        // 00402fd1 sar ecx,4
        sar();
        // 00402fd4 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00402fd5 cmp ecx,20h
        sub(MACHINE.regs.ecx, 0x20u32);
        // 00402fd8 jge short 00402FE8h
        jge(Cont(x00402fda), Cont(x00402fe8))
    }
}

pub fn x00402fda() -> Cont {
    unsafe {
        // 00402fda or esi,0FFFFFFFFh
        MACHINE.regs.esi = or(MACHINE.regs.esi, 0xffffffffu32);
        // 00402fdd shr esi,cl
        MACHINE.regs.esi = shr(MACHINE.regs.esi, MACHINE.regs.get_cl());
        // 00402fdf or dword ptr [ebp-8],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
                0xffffffffu32,
            ),
        );
        // 00402fe3 mov [ebp-0Ch],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.esi,
        );
        // 00402fe6 jmp short 00402FF8h
        Cont(x00402ff8)
    }
}

pub fn x00402fe8() -> Cont {
    unsafe {
        // 00402fe8 add ecx,0FFFFFFE0h
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0xffffffe0u32);
        // 00402feb or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 00402fee xor esi,esi
        MACHINE.regs.esi ^= MACHINE.regs.esi;
        // 00402ff0 shr eax,cl
        MACHINE.regs.eax = shr(MACHINE.regs.eax, MACHINE.regs.get_cl());
        // 00402ff2 mov [ebp-0Ch],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.esi,
        );
        // 00402ff5 mov [ebp-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 00402ff8 mov eax,ds:[409970h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409970u32);
        // 00402ffd mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00402fff cmp ebx,edi
        sub(MACHINE.regs.ebx, MACHINE.regs.edi);
        // 00403001 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403004 jae short 0040301Fh
        jae(Cont(x00403006), Cont(x0040301f))
    }
}

pub fn x00402ff8() -> Cont {
    unsafe {
        // 00402ff8 mov eax,ds:[409970h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409970u32);
        // 00402ffd mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00402fff cmp ebx,edi
        sub(MACHINE.regs.ebx, MACHINE.regs.edi);
        // 00403001 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403004 jae short 0040301Fh
        jae(Cont(x00403006), Cont(x0040301f))
    }
}

pub fn x00403006() -> Cont {
    unsafe {
        // 00403006 mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00403009 mov edi,[ebx]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32));
        // 0040300b and ecx,[ebp-8]
        MACHINE.regs.ecx = and(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 0040300e and edi,esi
        MACHINE.regs.edi = and(MACHINE.regs.edi, MACHINE.regs.esi);
        // 00403010 or ecx,edi
        MACHINE.regs.ecx = or(MACHINE.regs.ecx, MACHINE.regs.edi);
        // 00403012 jne short 0040301Fh
        jne(Cont(x00403014), Cont(x0040301f))
    }
}

pub fn x00403014() -> Cont {
    unsafe {
        // 00403014 add ebx,14h
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x14u32);
        // 00403017 cmp ebx,[ebp-4]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 0040301a mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 0040301d jb short 00403006h
        jb(Cont(x0040301f), Cont(x00403006))
    }
}

pub fn x0040301f() -> Cont {
    unsafe {
        // 0040301f cmp ebx,[ebp-4]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00403022 jne short 0040309Dh
        jne(Cont(x00403024), Cont(x0040309d))
    }
}

pub fn x00403024() -> Cont {
    unsafe {
        // 00403024 mov ebx,edx
        MACHINE.regs.ebx = MACHINE.regs.edx;
        // 00403026 cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403028 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 0040302b jae short 00403042h
        jae(Cont(x0040302d), Cont(x00403042))
    }
}

pub fn x00403026() -> Cont {
    unsafe {
        // 00403026 cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403028 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 0040302b jae short 00403042h
        jae(Cont(x0040302d), Cont(x00403042))
    }
}

pub fn x0040302d() -> Cont {
    unsafe {
        // 0040302d mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 00403030 mov edi,[ebx]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32));
        // 00403032 and ecx,[ebp-8]
        MACHINE.regs.ecx = and(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00403035 and edi,esi
        MACHINE.regs.edi = and(MACHINE.regs.edi, MACHINE.regs.esi);
        // 00403037 or ecx,edi
        MACHINE.regs.ecx = or(MACHINE.regs.ecx, MACHINE.regs.edi);
        // 00403039 jne short 00403040h
        jne(Cont(x0040303b), Cont(x00403040))
    }
}

pub fn x0040303b() -> Cont {
    unsafe {
        // 0040303b add ebx,14h
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x14u32);
        // 0040303e jmp short 00403026h
        Cont(x00403026)
    }
}

pub fn x00403040() -> Cont {
    unsafe {
        // 00403040 cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403042 jne short 0040309Dh
        jne(Cont(x00403044), Cont(x0040309d))
    }
}

pub fn x00403042() -> Cont {
    unsafe {
        // 00403042 jne short 0040309Dh
        jne(Cont(x00403044), Cont(x0040309d))
    }
}

pub fn x00403044() -> Cont {
    unsafe {
        // 00403044 cmp ebx,[ebp-4]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00403047 jae short 0040305Ah
        jae(Cont(x00403049), Cont(x0040305a))
    }
}

pub fn x00403049() -> Cont {
    unsafe {
        // 00403049 cmp dword ptr [ebx+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 0040304d jne short 00403057h
        jne(Cont(x0040304f), Cont(x00403057))
    }
}

pub fn x0040304f() -> Cont {
    unsafe {
        // 0040304f add ebx,14h
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x14u32);
        // 00403052 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403055 jmp short 00403044h
        Cont(x00403044)
    }
}

pub fn x00403057() -> Cont {
    unsafe {
        // 00403057 cmp ebx,[ebp-4]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 0040305a jne short 00403082h
        jne(Cont(x0040305c), Cont(x00403082))
    }
}

pub fn x0040305a() -> Cont {
    unsafe {
        // 0040305a jne short 00403082h
        jne(Cont(x0040305c), Cont(x00403082))
    }
}

pub fn x0040305c() -> Cont {
    unsafe {
        // 0040305c mov ebx,edx
        MACHINE.regs.ebx = MACHINE.regs.edx;
        // 0040305e cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403060 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403063 jae short 00403072h
        jae(Cont(x00403065), Cont(x00403072))
    }
}

pub fn x0040305e() -> Cont {
    unsafe {
        // 0040305e cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403060 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403063 jae short 00403072h
        jae(Cont(x00403065), Cont(x00403072))
    }
}

pub fn x00403065() -> Cont {
    unsafe {
        // 00403065 cmp dword ptr [ebx+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 00403069 jne short 00403070h
        jne(Cont(x0040306b), Cont(x00403070))
    }
}

pub fn x0040306b() -> Cont {
    unsafe {
        // 0040306b add ebx,14h
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x14u32);
        // 0040306e jmp short 0040305Eh
        Cont(x0040305e)
    }
}

pub fn x00403070() -> Cont {
    unsafe {
        // 00403070 cmp ebx,eax
        sub(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00403072 jne short 00403082h
        jne(Cont(x00403074), Cont(x00403082))
    }
}

pub fn x00403072() -> Cont {
    unsafe {
        // 00403072 jne short 00403082h
        jne(Cont(x00403074), Cont(x00403082))
    }
}

pub fn x00403074() -> Cont {
    unsafe {
        // 00403074 call 004032B1h
        call(0x403079, Cont(x004032b1))
    }
}

pub fn x00403079() -> Cont {
    unsafe {
        // 00403079 mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 0040307b test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 0040307d mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403080 je short 00403096h
        je(Cont(x00403082), Cont(x00403096))
    }
}

pub fn x00403082() -> Cont {
    unsafe {
        // 00403082 push ebx
        push(MACHINE.regs.ebx);
        // 00403083 call 00403362h
        call(0x403088, Cont(x00403362))
    }
}

pub fn x00403088() -> Cont {
    unsafe {
        // 00403088 pop ecx
        MACHINE.regs.ecx = pop();
        // 00403089 mov ecx,[ebx+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x10u32));
        // 0040308c mov [ecx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 0040308e mov eax,[ebx+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x10u32));
        // 00403091 cmp dword ptr [eax],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 00403094 jne short 0040309Dh
        jne(Cont(x00403096), Cont(x0040309d))
    }
}

pub fn x00403096() -> Cont {
    unsafe {
        // 00403096 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403098 jmp near ptr 004032ACh
        Cont(x004032ac)
    }
}

pub fn x0040309d() -> Cont {
    unsafe {
        // 0040309d mov ds:[409970h],ebx
        MACHINE.memory.write::<u32>(0x409970u32, MACHINE.regs.ebx);
        // 004030a3 mov eax,[ebx+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x10u32));
        // 004030a6 mov edx,[eax]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 004030a8 cmp edx,0FFFFFFFFh
        sub(MACHINE.regs.edx, 0xffffffffu32);
        // 004030ab mov [ebp-4],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edx,
        );
        // 004030ae je short 004030C4h
        je(Cont(x004030b0), Cont(x004030c4))
    }
}

pub fn x004030b0() -> Cont {
    unsafe {
        // 004030b0 mov ecx,[eax+edx*4+0C4h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0xc4u32),
        );
        // 004030b7 mov edi,[eax+edx*4+44h]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0x44u32),
        );
        // 004030bb and ecx,[ebp-8]
        MACHINE.regs.ecx = and(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 004030be and edi,esi
        MACHINE.regs.edi = and(MACHINE.regs.edi, MACHINE.regs.esi);
        // 004030c0 or ecx,edi
        MACHINE.regs.ecx = or(MACHINE.regs.ecx, MACHINE.regs.edi);
        // 004030c2 jne short 004030FBh
        jne(Cont(x004030c4), Cont(x004030fb))
    }
}

pub fn x004030c4() -> Cont {
    unsafe {
        // 004030c4 mov edx,[eax+0C4h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0xc4u32));
        // 004030ca mov esi,[eax+44h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x44u32));
        // 004030cd and edx,[ebp-8]
        MACHINE.regs.edx = and(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 004030d0 and esi,[ebp-0Ch]
        MACHINE.regs.esi = and(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
        );
        // 004030d3 and dword ptr [ebp-4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x0u32,
            ),
        );
        // 004030d7 lea ecx,[eax+44h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0x44u32);
        // 004030da or edx,esi
        MACHINE.regs.edx = or(MACHINE.regs.edx, MACHINE.regs.esi);
        // 004030dc mov esi,[ebp-0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 004030df jne short 004030F8h
        jne(Cont(x004030e1), Cont(x004030f8))
    }
}

pub fn x004030e1() -> Cont {
    unsafe {
        // 004030e1 mov edx,[ecx+84h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x84u32));
        // 004030e7 inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 004030ea and edx,[ebp-8]
        MACHINE.regs.edx = and(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 004030ed add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 004030f0 mov edi,esi
        MACHINE.regs.edi = MACHINE.regs.esi;
        // 004030f2 and edi,[ecx]
        MACHINE.regs.edi = and(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 004030f4 or edx,edi
        MACHINE.regs.edx = or(MACHINE.regs.edx, MACHINE.regs.edi);
        // 004030f6 je short 004030E1h
        je(Cont(x004030f8), Cont(x004030e1))
    }
}

pub fn x004030f8() -> Cont {
    unsafe {
        // 004030f8 mov edx,[ebp-4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004030fb mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 004030fd xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 004030ff imul ecx,204h
        imul();
        // 00403105 lea ecx,[ecx+eax+144h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x144u32);
        // 0040310c mov [ebp-0Ch],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.ecx,
        );
        // 0040310f mov ecx,[eax+edx*4+44h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0x44u32),
        );
        // 00403113 and ecx,esi
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, MACHINE.regs.esi);
        // 00403115 jne short 00403124h
        jne(Cont(x00403117), Cont(x00403124))
    }
}

pub fn x004030fb() -> Cont {
    unsafe {
        // 004030fb mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 004030fd xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 004030ff imul ecx,204h
        imul();
        // 00403105 lea ecx,[ecx+eax+144h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x144u32);
        // 0040310c mov [ebp-0Ch],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.ecx,
        );
        // 0040310f mov ecx,[eax+edx*4+44h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0x44u32),
        );
        // 00403113 and ecx,esi
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, MACHINE.regs.esi);
        // 00403115 jne short 00403124h
        jne(Cont(x00403117), Cont(x00403124))
    }
}

pub fn x00403117() -> Cont {
    unsafe {
        // 00403117 mov ecx,[eax+edx*4+0C4h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0xc4u32),
        );
        // 0040311e push 20h
        push(0x20u32);
        // 00403120 and ecx,[ebp-8]
        MACHINE.regs.ecx = and(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00403123 pop edi
        MACHINE.regs.edi = pop();
        // 00403124 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403126 jl short 0040312Dh
        jl(Cont(x00403128), Cont(x0040312d))
    }
}

pub fn x00403124() -> Cont {
    unsafe {
        // 00403124 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403126 jl short 0040312Dh
        jl(Cont(x00403128), Cont(x0040312d))
    }
}

pub fn x00403128() -> Cont {
    unsafe {
        // 00403128 shl ecx,1
        MACHINE.regs.ecx = shl(MACHINE.regs.ecx, 0x1u8);
        // 0040312a inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 0040312b jmp short 00403124h
        Cont(x00403124)
    }
}

pub fn x0040312d() -> Cont {
    unsafe {
        // 0040312d mov ecx,[ebp-0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 00403130 mov edx,[ecx+edi*8+4]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.edi * 8))
                .wrapping_add(0x4u32),
        );
        // 00403134 mov ecx,[edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32));
        // 00403136 sub ecx,[ebp-10h]
        MACHINE.regs.ecx = sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32)),
        );
        // 00403139 mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 0040313b mov [ebp-8],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.ecx,
        );
        // 0040313e sar esi,4
        sar();
        // 00403141 dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 00403142 cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 00403145 jle short 0040314Ah
        jle(Cont(x00403147), Cont(x0040314a))
    }
}

pub fn x00403147() -> Cont {
    unsafe {
        // 00403147 push 3Fh
        push(0x3fu32);
        // 00403149 pop esi
        MACHINE.regs.esi = pop();
        // 0040314a cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 0040314c je near ptr 0040325Fh
        je(Cont(x00403152), Cont(x0040325f))
    }
}

pub fn x0040314a() -> Cont {
    unsafe {
        // 0040314a cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 0040314c je near ptr 0040325Fh
        je(Cont(x00403152), Cont(x0040325f))
    }
}

pub fn x00403152() -> Cont {
    unsafe {
        // 00403152 mov ecx,[edx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 00403155 cmp ecx,[edx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32)),
        );
        // 00403158 jne short 004031BBh
        jne(Cont(x0040315a), Cont(x004031bb))
    }
}

pub fn x0040315a() -> Cont {
    unsafe {
        // 0040315a cmp edi,20h
        sub(MACHINE.regs.edi, 0x20u32);
        // 0040315d jge short 0040318Ah
        jge(Cont(x0040315f), Cont(x0040318a))
    }
}

pub fn x0040315f() -> Cont {
    unsafe {
        // 0040315f mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00403164 mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 00403166 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00403168 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 0040316b lea edi,[eax+edi+4]
        MACHINE.regs.edi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 1))
            .wrapping_add(0x4u32);
        // 0040316f not ebx
        not();
        // 00403171 mov [ebp-14h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffecu32),
            MACHINE.regs.ebx,
        );
        // 00403174 and ebx,[eax+ecx*4+44h]
        MACHINE.regs.ebx = and(
            MACHINE.regs.ebx,
            MACHINE.memory.read::<u32>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.ecx * 4))
                    .wrapping_add(0x44u32),
            ),
        );
        // 00403178 mov [eax+ecx*4+44h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ecx * 4))
                .wrapping_add(0x44u32),
            MACHINE.regs.ebx,
        );
        // 0040317c dec byte ptr [edi]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32))),
        );
        // 0040317e jne short 004031B8h
        jne(Cont(x00403180), Cont(x004031b8))
    }
}

pub fn x00403180() -> Cont {
    unsafe {
        // 00403180 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403183 mov ecx,[ebp-14h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffecu32));
        // 00403186 and [ebx],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
                MACHINE.regs.ecx,
            ),
        );
        // 00403188 jmp short 004031BBh
        Cont(x004031bb)
    }
}

pub fn x0040318a() -> Cont {
    unsafe {
        // 0040318a lea ecx,[edi-20h]
        MACHINE.regs.ecx = MACHINE.regs.edi.wrapping_add(0xffffffe0u32);
        // 0040318d mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00403192 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00403194 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403197 lea edi,[eax+edi+4]
        MACHINE.regs.edi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edi * 1))
            .wrapping_add(0x4u32);
        // 0040319b lea ecx,[eax+ecx*4+0C4h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0xc4u32);
        // 004031a2 not ebx
        not();
        // 004031a4 and [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 004031a6 dec byte ptr [edi]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32))),
        );
        // 004031a8 mov [ebp-14h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffecu32),
            MACHINE.regs.ebx,
        );
        // 004031ab jne short 004031B8h
        jne(Cont(x004031ad), Cont(x004031b8))
    }
}

pub fn x004031ad() -> Cont {
    unsafe {
        // 004031ad mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004031b0 mov ecx,[ebp-14h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffecu32));
        // 004031b3 and [ebx+4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32)),
                MACHINE.regs.ecx,
            ),
        );
        // 004031b6 jmp short 004031BBh
        Cont(x004031bb)
    }
}

pub fn x004031b8() -> Cont {
    unsafe {
        // 004031b8 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004031bb mov ecx,[edx+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 004031be mov edi,[edx+4]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031c1 cmp dword ptr [ebp-8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
            0x0u32,
        );
        // 004031c5 mov [ecx+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 004031c8 mov ecx,[edx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031cb mov edi,[edx+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 004031ce mov [ecx+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 004031d1 je near ptr 0040326Bh
        je(Cont(x004031d7), Cont(x0040326b))
    }
}

pub fn x004031bb() -> Cont {
    unsafe {
        // 004031bb mov ecx,[edx+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 004031be mov edi,[edx+4]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031c1 cmp dword ptr [ebp-8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
            0x0u32,
        );
        // 004031c5 mov [ecx+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 004031c8 mov ecx,[edx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031cb mov edi,[edx+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32));
        // 004031ce mov [ecx+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 004031d1 je near ptr 0040326Bh
        je(Cont(x004031d7), Cont(x0040326b))
    }
}

pub fn x004031d7() -> Cont {
    unsafe {
        // 004031d7 mov ecx,[ebp-0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 004031da mov edi,[ecx+esi*8+4]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.esi * 8))
                .wrapping_add(0x4u32),
        );
        // 004031de lea ecx,[ecx+esi*8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 8))
            .wrapping_add(0x0u32);
        // 004031e1 mov [edx+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 004031e4 mov [edx+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 004031e7 mov [ecx+4],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.edx);
        // 004031ea mov ecx,[edx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031ed mov [ecx+8],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.edx);
        // 004031f0 mov ecx,[edx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x4u32));
        // 004031f3 cmp ecx,[edx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edx.wrapping_add(0x8u32)),
        );
        // 004031f6 jne short 0040325Ch
        jne(Cont(x004031f8), Cont(x0040325c))
    }
}

pub fn x004031f8() -> Cont {
    unsafe {
        // 004031f8 mov cl,[esi+eax+4]
        MACHINE.regs.set_cl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .esi
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            ),
        );
        // 004031fc cmp esi,20h
        sub(MACHINE.regs.esi, 0x20u32);
        // 004031ff mov [ebp+0Bh],cl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0xbu32), MACHINE.regs.get_cl());
        // 00403202 jge short 0040322Dh
        jge(Cont(x00403204), Cont(x0040322d))
    }
}

pub fn x00403204() -> Cont {
    unsafe {
        // 00403204 inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 00403206 cmp byte ptr [ebp+0Bh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xbu32)),
            0x0u8,
        );
        // 0040320a mov [esi+eax+4],cl
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .esi
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            MACHINE.regs.get_cl(),
        );
        // 0040320e jne short 0040321Bh
        jne(Cont(x00403210), Cont(x0040321b))
    }
}

pub fn x00403210() -> Cont {
    unsafe {
        // 00403210 mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403215 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403217 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403219 or [ebx],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
                MACHINE.regs.edi,
            ),
        );
        // 0040321b mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403220 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403222 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403224 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403227 or [eax+ecx*4+44h],edi
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ecx * 4))
                .wrapping_add(0x44u32),
            or(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.ecx * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.edi,
            ),
        );
        // 0040322b jmp short 0040325Ch
        Cont(x0040325c)
    }
}

pub fn x0040321b() -> Cont {
    unsafe {
        // 0040321b mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403220 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403222 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403224 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403227 or [eax+ecx*4+44h],edi
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ecx * 4))
                .wrapping_add(0x44u32),
            or(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.ecx * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.edi,
            ),
        );
        // 0040322b jmp short 0040325Ch
        Cont(x0040325c)
    }
}

pub fn x0040322d() -> Cont {
    unsafe {
        // 0040322d inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 0040322f cmp byte ptr [ebp+0Bh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xbu32)),
            0x0u8,
        );
        // 00403233 mov [esi+eax+4],cl
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .esi
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            MACHINE.regs.get_cl(),
        );
        // 00403237 jne short 00403246h
        jne(Cont(x00403239), Cont(x00403246))
    }
}

pub fn x00403239() -> Cont {
    unsafe {
        // 00403239 lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 0040323c mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403241 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403243 or [ebx+4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32)),
                MACHINE.regs.edi,
            ),
        );
        // 00403246 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403249 lea edi,[eax+ecx*4+0C4h]
        MACHINE.regs.edi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0xc4u32);
        // 00403250 lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 00403253 mov esi,80000000h
        MACHINE.regs.esi = 0x80000000u32;
        // 00403258 shr esi,cl
        MACHINE.regs.esi = shr(MACHINE.regs.esi, MACHINE.regs.get_cl());
        // 0040325a or [edi],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.esi,
            ),
        );
        // 0040325c mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 0040325f test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403261 je short 0040326Eh
        je(Cont(x00403263), Cont(x0040326e))
    }
}

pub fn x00403246() -> Cont {
    unsafe {
        // 00403246 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403249 lea edi,[eax+ecx*4+0C4h]
        MACHINE.regs.edi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0xc4u32);
        // 00403250 lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 00403253 mov esi,80000000h
        MACHINE.regs.esi = 0x80000000u32;
        // 00403258 shr esi,cl
        MACHINE.regs.esi = shr(MACHINE.regs.esi, MACHINE.regs.get_cl());
        // 0040325a or [edi],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.esi,
            ),
        );
        // 0040325c mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 0040325f test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403261 je short 0040326Eh
        je(Cont(x00403263), Cont(x0040326e))
    }
}

pub fn x0040325c() -> Cont {
    unsafe {
        // 0040325c mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 0040325f test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403261 je short 0040326Eh
        je(Cont(x00403263), Cont(x0040326e))
    }
}

pub fn x0040325f() -> Cont {
    unsafe {
        // 0040325f test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403261 je short 0040326Eh
        je(Cont(x00403263), Cont(x0040326e))
    }
}

pub fn x00403263() -> Cont {
    unsafe {
        // 00403263 mov [edx],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00403265 mov [ecx+edx-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.edx * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00403269 jmp short 0040326Eh
        Cont(x0040326e)
    }
}

pub fn x0040326b() -> Cont {
    unsafe {
        // 0040326b mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 0040326e mov esi,[ebp-10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00403271 add edx,ecx
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00403273 lea ecx,[esi+1]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 00403276 mov [edx],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00403278 mov [edx+esi-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .edx
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 0040327c mov esi,[ebp-0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 0040327f mov ecx,[esi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00403281 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403283 lea edi,[ecx+1]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0x1u32);
        // 00403286 mov [esi],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.edi);
        // 00403288 jne short 004032A4h
        jne(Cont(x0040328a), Cont(x004032a4))
    }
}

pub fn x0040326e() -> Cont {
    unsafe {
        // 0040326e mov esi,[ebp-10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 00403271 add edx,ecx
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00403273 lea ecx,[esi+1]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 00403276 mov [edx],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00403278 mov [edx+esi-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .edx
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 0040327c mov esi,[ebp-0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 0040327f mov ecx,[esi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00403281 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00403283 lea edi,[ecx+1]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0x1u32);
        // 00403286 mov [esi],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.edi);
        // 00403288 jne short 004032A4h
        jne(Cont(x0040328a), Cont(x004032a4))
    }
}

pub fn x0040328a() -> Cont {
    unsafe {
        // 0040328a cmp ebx,ds:[409974h]
        sub(MACHINE.regs.ebx, MACHINE.memory.read::<u32>(0x409974u32));
        // 00403290 jne short 004032A4h
        jne(Cont(x00403292), Cont(x004032a4))
    }
}

pub fn x00403292() -> Cont {
    unsafe {
        // 00403292 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403295 cmp ecx,ds:[40996Ch]
        sub(MACHINE.regs.ecx, MACHINE.memory.read::<u32>(0x40996cu32));
        // 0040329b jne short 004032A4h
        jne(Cont(x0040329d), Cont(x004032a4))
    }
}

pub fn x0040329d() -> Cont {
    unsafe {
        // 0040329d and dword ptr ds:[409974h],0
        MACHINE.memory.write::<u32>(
            0x409974u32,
            and(MACHINE.memory.read::<u32>(0x409974u32), 0x0u32),
        );
        // 004032a4 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004032a7 mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004032a9 lea eax,[edx+4]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0x4u32);
        // 004032ac pop edi
        MACHINE.regs.edi = pop();
        // 004032ad pop esi
        MACHINE.regs.esi = pop();
        // 004032ae pop ebx
        MACHINE.regs.ebx = pop();
        // 004032af leave
        leave();
        // 004032b0 ret
        ret(0)
    }
}

pub fn x004032a4() -> Cont {
    unsafe {
        // 004032a4 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004032a7 mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004032a9 lea eax,[edx+4]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0x4u32);
        // 004032ac pop edi
        MACHINE.regs.edi = pop();
        // 004032ad pop esi
        MACHINE.regs.esi = pop();
        // 004032ae pop ebx
        MACHINE.regs.ebx = pop();
        // 004032af leave
        leave();
        // 004032b0 ret
        ret(0)
    }
}

pub fn x004032ac() -> Cont {
    unsafe {
        // 004032ac pop edi
        MACHINE.regs.edi = pop();
        // 004032ad pop esi
        MACHINE.regs.esi = pop();
        // 004032ae pop ebx
        MACHINE.regs.ebx = pop();
        // 004032af leave
        leave();
        // 004032b0 ret
        ret(0)
    }
}

pub fn x004032b1() -> Cont {
    unsafe {
        // 004032b1 mov eax,ds:[409978h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409978u32);
        // 004032b6 mov ecx,ds:[409968h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409968u32);
        // 004032bc push esi
        push(MACHINE.regs.esi);
        // 004032bd push edi
        push(MACHINE.regs.edi);
        // 004032be xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 004032c0 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004032c2 jne short 004032F4h
        jne(Cont(x004032c4), Cont(x004032f4))
    }
}

pub fn x004032c4() -> Cont {
    unsafe {
        // 004032c4 lea eax,[ecx+ecx*4+50h]
        MACHINE.regs.eax = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.ecx * 4))
            .wrapping_add(0x50u32);
        // 004032c8 shl eax,2
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x2u8);
        // 004032cb push eax
        push(MACHINE.regs.eax);
        // 004032cc push dword ptr ds:[40997Ch]
        push(MACHINE.memory.read::<u32>(0x40997cu32));
        // 004032d2 push edi
        push(MACHINE.regs.edi);
        // 004032d3 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 004032d9 call dword ptr ds:[40607Ch]
        call(0x4032df, Cont(kernel32::HeapReAlloc_stdcall))
    }
}

pub fn x004032df() -> Cont {
    unsafe {
        // 004032df cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 004032e1 je short 00403344h
        je(Cont(x004032e3), Cont(x00403344))
    }
}

pub fn x004032e3() -> Cont {
    unsafe {
        // 004032e3 add dword ptr ds:[409968h],10h
        MACHINE.memory.write::<u32>(
            0x409968u32,
            add(MACHINE.memory.read::<u32>(0x409968u32), 0x10u32),
        );
        // 004032ea mov ds:[40997Ch],eax
        MACHINE.memory.write::<u32>(0x40997cu32, MACHINE.regs.eax);
        // 004032ef mov eax,ds:[409978h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409978u32);
        // 004032f4 mov ecx,ds:[40997Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40997cu32);
        // 004032fa push 41C4h
        push(0x41c4u32);
        // 004032ff push 8
        push(0x8u32);
        // 00403301 lea eax,[eax+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00403304 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 0040330a lea esi,[ecx+eax*4]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 0040330d call dword ptr ds:[406028h]
        call(0x403313, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x004032f4() -> Cont {
    unsafe {
        // 004032f4 mov ecx,ds:[40997Ch]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x40997cu32);
        // 004032fa push 41C4h
        push(0x41c4u32);
        // 004032ff push 8
        push(0x8u32);
        // 00403301 lea eax,[eax+eax*4]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 00403304 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 0040330a lea esi,[ecx+eax*4]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 4))
            .wrapping_add(0x0u32);
        // 0040330d call dword ptr ds:[406028h]
        call(0x403313, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x00403313() -> Cont {
    unsafe {
        // 00403313 cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 00403315 mov [esi+10h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32), MACHINE.regs.eax);
        // 00403318 je short 00403344h
        je(Cont(x0040331a), Cont(x00403344))
    }
}

pub fn x0040331a() -> Cont {
    unsafe {
        // 0040331a push 4
        push(0x4u32);
        // 0040331c push 2000h
        push(0x2000u32);
        // 00403321 push 100000h
        push(0x100000u32);
        // 00403326 push edi
        push(MACHINE.regs.edi);
        // 00403327 call dword ptr ds:[40605Ch]
        call(0x40332d, Cont(kernel32::VirtualAlloc_stdcall))
    }
}

pub fn x0040332d() -> Cont {
    unsafe {
        // 0040332d cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 0040332f mov [esi+0Ch],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32), MACHINE.regs.eax);
        // 00403332 jne short 00403348h
        jne(Cont(x00403334), Cont(x00403348))
    }
}

pub fn x00403334() -> Cont {
    unsafe {
        // 00403334 push dword ptr [esi+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32)),
        );
        // 00403337 push edi
        push(MACHINE.regs.edi);
        // 00403338 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 0040333e call dword ptr ds:[406090h]
        call(0x403344, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x00403344() -> Cont {
    unsafe {
        // 00403344 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403346 jmp short 0040335Fh
        Cont(x0040335f)
    }
}

pub fn x00403348() -> Cont {
    unsafe {
        // 00403348 or dword ptr [esi+8],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32)),
                0xffffffffu32,
            ),
        );
        // 0040334c mov [esi],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.edi);
        // 0040334e mov [esi+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 00403351 inc dword ptr ds:[409978h]
        MACHINE
            .memory
            .write::<u32>(0x409978u32, inc(MACHINE.memory.read::<u32>(0x409978u32)));
        // 00403357 mov eax,[esi+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 0040335a or dword ptr [eax],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                0xffffffffu32,
            ),
        );
        // 0040335d mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 0040335f pop edi
        MACHINE.regs.edi = pop();
        // 00403360 pop esi
        MACHINE.regs.esi = pop();
        // 00403361 ret
        ret(0)
    }
}

pub fn x0040335f() -> Cont {
    unsafe {
        // 0040335f pop edi
        MACHINE.regs.edi = pop();
        // 00403360 pop esi
        MACHINE.regs.esi = pop();
        // 00403361 ret
        ret(0)
    }
}

pub fn x00403362() -> Cont {
    unsafe {
        // 00403362 push ebp
        push(MACHINE.regs.ebp);
        // 00403363 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403365 push ecx
        push(MACHINE.regs.ecx);
        // 00403366 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403369 push ebx
        push(MACHINE.regs.ebx);
        // 0040336a push esi
        push(MACHINE.regs.esi);
        // 0040336b push edi
        push(MACHINE.regs.edi);
        // 0040336c mov esi,[ecx+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x10u32));
        // 0040336f mov eax,[ecx+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32));
        // 00403372 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00403374 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403376 jl short 0040337Dh
        jl(Cont(x00403378), Cont(x0040337d))
    }
}

pub fn x00403374() -> Cont {
    unsafe {
        // 00403374 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403376 jl short 0040337Dh
        jl(Cont(x00403378), Cont(x0040337d))
    }
}

pub fn x00403378() -> Cont {
    unsafe {
        // 00403378 shl eax,1
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x1u8);
        // 0040337a inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 0040337b jmp short 00403374h
        Cont(x00403374)
    }
}

pub fn x0040337d() -> Cont {
    unsafe {
        // 0040337d mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 0040337f push 3Fh
        push(0x3fu32);
        // 00403381 imul eax,204h
        imul();
        // 00403387 pop edx
        MACHINE.regs.edx = pop();
        // 00403388 lea eax,[eax+esi+144h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0x144u32);
        // 0040338f mov [ebp-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00403392 mov [eax+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 00403395 mov [eax+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 00403398 add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 0040339b dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 0040339c jne short 00403392h
        jne(Cont(x0040339e), Cont(x00403392))
    }
}

pub fn x00403392() -> Cont {
    unsafe {
        // 00403392 mov [eax+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 00403395 mov [eax+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 00403398 add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 0040339b dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 0040339c jne short 00403392h
        jne(Cont(x0040339e), Cont(x00403392))
    }
}

pub fn x0040339e() -> Cont {
    unsafe {
        // 0040339e mov edi,ebx
        MACHINE.regs.edi = MACHINE.regs.ebx;
        // 004033a0 push 4
        push(0x4u32);
        // 004033a2 shl edi,0Fh
        MACHINE.regs.edi = shl(MACHINE.regs.edi, 0xfu8);
        // 004033a5 add edi,[ecx+0Ch]
        MACHINE.regs.edi = add(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xcu32)),
        );
        // 004033a8 push 1000h
        push(0x1000u32);
        // 004033ad push 8000h
        push(0x8000u32);
        // 004033b2 push edi
        push(MACHINE.regs.edi);
        // 004033b3 call dword ptr ds:[40605Ch]
        call(0x4033b9, Cont(kernel32::VirtualAlloc_stdcall))
    }
}

pub fn x004033b9() -> Cont {
    unsafe {
        // 004033b9 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004033bb jne short 004033C5h
        jne(Cont(x004033bd), Cont(x004033c5))
    }
}

pub fn x004033bd() -> Cont {
    unsafe {
        // 004033bd or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 004033c0 jmp near ptr 00403458h
        Cont(x00403458)
    }
}

pub fn x004033c5() -> Cont {
    unsafe {
        // 004033c5 lea edx,[edi+7000h]
        MACHINE.regs.edx = MACHINE.regs.edi.wrapping_add(0x7000u32);
        // 004033cb cmp edi,edx
        sub(MACHINE.regs.edi, MACHINE.regs.edx);
        // 004033cd ja short 0040340Bh
        ja(Cont(x004033cf), Cont(x0040340b))
    }
}

pub fn x004033cf() -> Cont {
    unsafe {
        // 004033cf lea eax,[edi+10h]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x10u32);
        // 004033d2 or dword ptr [eax-8],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfffffff8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0xfffffff8u32)),
                0xffffffffu32,
            ),
        );
        // 004033d6 or dword ptr [eax+0FECh],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfecu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0xfecu32)),
                0xffffffffu32,
            ),
        );
        // 004033dd lea ecx,[eax+0FFCh]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xffcu32);
        // 004033e3 mov dword ptr [eax-4],0FF0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0xfffffffcu32), 0xff0u32);
        // 004033ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004033ec lea ecx,[eax-1004h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xffffeffcu32);
        // 004033f2 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 004033f5 mov dword ptr [eax+0FE8h],0FF0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0xfe8u32), 0xff0u32);
        // 004033ff add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403404 lea ecx,[eax-10h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xfffffff0u32);
        // 00403407 cmp ecx,edx
        sub(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 00403409 jbe short 004033D2h
        jbe(Cont(x0040340b), Cont(x004033d2))
    }
}

pub fn x004033d2() -> Cont {
    unsafe {
        // 004033d2 or dword ptr [eax-8],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfffffff8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0xfffffff8u32)),
                0xffffffffu32,
            ),
        );
        // 004033d6 or dword ptr [eax+0FECh],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfecu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0xfecu32)),
                0xffffffffu32,
            ),
        );
        // 004033dd lea ecx,[eax+0FFCh]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xffcu32);
        // 004033e3 mov dword ptr [eax-4],0FF0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0xfffffffcu32), 0xff0u32);
        // 004033ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004033ec lea ecx,[eax-1004h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xffffeffcu32);
        // 004033f2 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 004033f5 mov dword ptr [eax+0FE8h],0FF0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0xfe8u32), 0xff0u32);
        // 004033ff add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403404 lea ecx,[eax-10h]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0xfffffff0u32);
        // 00403407 cmp ecx,edx
        sub(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 00403409 jbe short 004033D2h
        jbe(Cont(x0040340b), Cont(x004033d2))
    }
}

pub fn x0040340b() -> Cont {
    unsafe {
        // 0040340b mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 0040340e lea ecx,[edi+0Ch]
        MACHINE.regs.ecx = MACHINE.regs.edi.wrapping_add(0xcu32);
        // 00403411 add eax,1F8h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1f8u32);
        // 00403416 push 1
        push(0x1u32);
        // 00403418 pop edi
        MACHINE.regs.edi = pop();
        // 00403419 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 0040341c mov [ecx+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 0040341f lea ecx,[edx+0Ch]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xcu32);
        // 00403422 mov [eax+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 00403425 mov [ecx+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 00403428 and dword ptr [esi+ebx*4+44h],0
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .esi
                .wrapping_add((MACHINE.regs.ebx * 4))
                .wrapping_add(0x44u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .esi
                        .wrapping_add((MACHINE.regs.ebx * 4))
                        .wrapping_add(0x44u32),
                ),
                0x0u32,
            ),
        );
        // 0040342d mov [esi+ebx*4+0C4h],edi
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .esi
                .wrapping_add((MACHINE.regs.ebx * 4))
                .wrapping_add(0xc4u32),
            MACHINE.regs.edi,
        );
        // 00403434 mov al,[esi+43h]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x43u32)),
        );
        // 00403437 mov cl,al
        MACHINE.regs.set_cl(MACHINE.regs.get_al());
        // 00403439 inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 0040343b test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 0040343d mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403440 mov [esi+43h],cl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x43u32),
            MACHINE.regs.get_cl(),
        );
        // 00403443 jne short 00403448h
        jne(Cont(x00403445), Cont(x00403448))
    }
}

pub fn x00403445() -> Cont {
    unsafe {
        // 00403445 or [eax+4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32)),
                MACHINE.regs.edi,
            ),
        );
        // 00403448 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 0040344d mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 0040344f shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00403451 not edx
        not();
        // 00403453 and [eax+8],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403456 mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00403458 pop edi
        MACHINE.regs.edi = pop();
        // 00403459 pop esi
        MACHINE.regs.esi = pop();
        // 0040345a pop ebx
        MACHINE.regs.ebx = pop();
        // 0040345b leave
        leave();
        // 0040345c ret
        ret(0)
    }
}

pub fn x00403448() -> Cont {
    unsafe {
        // 00403448 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 0040344d mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 0040344f shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00403451 not edx
        not();
        // 00403453 and [eax+8],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x8u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403456 mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00403458 pop edi
        MACHINE.regs.edi = pop();
        // 00403459 pop esi
        MACHINE.regs.esi = pop();
        // 0040345a pop ebx
        MACHINE.regs.ebx = pop();
        // 0040345b leave
        leave();
        // 0040345c ret
        ret(0)
    }
}

pub fn x00403458() -> Cont {
    unsafe {
        // 00403458 pop edi
        MACHINE.regs.edi = pop();
        // 00403459 pop esi
        MACHINE.regs.esi = pop();
        // 0040345a pop ebx
        MACHINE.regs.ebx = pop();
        // 0040345b leave
        leave();
        // 0040345c ret
        ret(0)
    }
}

pub fn x0040345d() -> Cont {
    unsafe {
        // 0040345d push ebp
        push(MACHINE.regs.ebp);
        // 0040345e mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403460 sub esp,0Ch
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0xcu32);
        // 00403463 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403466 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403469 push ebx
        push(MACHINE.regs.ebx);
        // 0040346a push esi
        push(MACHINE.regs.esi);
        // 0040346b push edi
        push(MACHINE.regs.edi);
        // 0040346c mov edi,[ebp+0Ch]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040346f mov edx,edi
        MACHINE.regs.edx = MACHINE.regs.edi;
        // 00403471 lea esi,[eax+17h]
        MACHINE.regs.esi = MACHINE.regs.eax.wrapping_add(0x17u32);
        // 00403474 sub edx,[ecx+0Ch]
        MACHINE.regs.edx = sub(
            MACHINE.regs.edx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xcu32)),
        );
        // 00403477 mov eax,[ecx+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x10u32));
        // 0040347a and esi,0FFFFFFF0h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffffff0u32);
        // 0040347d shr edx,0Fh
        MACHINE.regs.edx = shr(MACHINE.regs.edx, 0xfu8);
        // 00403480 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00403482 imul ecx,204h
        imul();
        // 00403488 lea ecx,[ecx+eax+144h]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x144u32);
        // 0040348f mov [ebp-0Ch],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.ecx,
        );
        // 00403492 mov ecx,[edi-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0xfffffffcu32));
        // 00403495 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00403496 cmp esi,ecx
        sub(MACHINE.regs.esi, MACHINE.regs.ecx);
        // 00403498 mov [ebp+10h],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.ecx);
        // 0040349b mov ebx,[ecx+edi-4]
        MACHINE.regs.ebx = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.edi * 1))
                .wrapping_add(0xfffffffcu32),
        );
        // 0040349f lea edi,[ecx+edi-4]
        MACHINE.regs.edi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edi * 1))
            .wrapping_add(0xfffffffcu32);
        // 004034a3 mov [ebp-4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ebx,
        );
        // 004034a6 jle near ptr 0040360Bh
        jle(Cont(x004034ac), Cont(x0040360b))
    }
}

pub fn x004034ac() -> Cont {
    unsafe {
        // 004034ac test bl,1
        and(MACHINE.regs.get_bl(), 0x1u8);
        // 004034af jne near ptr 00403604h
        jne(Cont(x004034b5), Cont(x00403604))
    }
}

pub fn x004034b5() -> Cont {
    unsafe {
        // 004034b5 add ebx,ecx
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, MACHINE.regs.ecx);
        // 004034b7 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 004034b9 jg near ptr 00403604h
        jg(Cont(x004034bf), Cont(x00403604))
    }
}

pub fn x004034bf() -> Cont {
    unsafe {
        // 004034bf mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004034c2 sar ecx,4
        sar();
        // 004034c5 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 004034c6 cmp ecx,3Fh
        sub(MACHINE.regs.ecx, 0x3fu32);
        // 004034c9 mov [ebp-8],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.ecx,
        );
        // 004034cc jbe short 004034D4h
        jbe(Cont(x004034ce), Cont(x004034d4))
    }
}

pub fn x004034ce() -> Cont {
    unsafe {
        // 004034ce push 3Fh
        push(0x3fu32);
        // 004034d0 pop ecx
        MACHINE.regs.ecx = pop();
        // 004034d1 mov [ebp-8],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.ecx,
        );
        // 004034d4 mov ebx,[edi+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004034d7 cmp ebx,[edi+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32)),
        );
        // 004034da jne short 00403524h
        jne(Cont(x004034dc), Cont(x00403524))
    }
}

pub fn x004034d4() -> Cont {
    unsafe {
        // 004034d4 mov ebx,[edi+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004034d7 cmp ebx,[edi+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32)),
        );
        // 004034da jne short 00403524h
        jne(Cont(x004034dc), Cont(x00403524))
    }
}

pub fn x004034dc() -> Cont {
    unsafe {
        // 004034dc cmp ecx,20h
        sub(MACHINE.regs.ecx, 0x20u32);
        // 004034df jae short 00403500h
        jae(Cont(x004034e1), Cont(x00403500))
    }
}

pub fn x004034e1() -> Cont {
    unsafe {
        // 004034e1 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 004034e6 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 004034e8 mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 004034eb lea ecx,[ecx+eax+4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 004034ef not ebx
        not();
        // 004034f1 and [eax+edx*4+44h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0x44u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edx * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 004034f5 dec byte ptr [ecx]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 004034f7 jne short 00403524h
        jne(Cont(x004034f9), Cont(x00403524))
    }
}

pub fn x004034f9() -> Cont {
    unsafe {
        // 004034f9 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004034fc and [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 004034fe jmp short 00403524h
        Cont(x00403524)
    }
}

pub fn x00403500() -> Cont {
    unsafe {
        // 00403500 add ecx,0FFFFFFE0h
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0xffffffe0u32);
        // 00403503 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00403508 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 0040350a mov ecx,[ebp-8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 0040350d lea ecx,[ecx+eax+4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 00403511 not ebx
        not();
        // 00403513 and [eax+edx*4+0C4h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0xc4u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edx * 4))
                        .wrapping_add(0xc4u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 0040351a dec byte ptr [ecx]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 0040351c jne short 00403524h
        jne(Cont(x0040351e), Cont(x00403524))
    }
}

pub fn x0040351e() -> Cont {
    unsafe {
        // 0040351e mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403521 and [ecx+4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00403524 mov ecx,[edi+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 00403527 mov ebx,[edi+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 0040352a mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 0040352d mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 00403530 mov edi,[edi+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 00403533 mov [ecx+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 00403536 mov ecx,[ebp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403539 sub ecx,esi
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.esi);
        // 0040353b add [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                MACHINE.regs.ecx,
            ),
        );
        // 0040353e cmp dword ptr [ebp-4],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x0u32,
        );
        // 00403542 jle near ptr 004035F2h
        jle(Cont(x00403548), Cont(x004035f2))
    }
}

pub fn x00403524() -> Cont {
    unsafe {
        // 00403524 mov ecx,[edi+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 00403527 mov ebx,[edi+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 0040352a mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 0040352d mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 00403530 mov edi,[edi+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 00403533 mov [ecx+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 00403536 mov ecx,[ebp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403539 sub ecx,esi
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.esi);
        // 0040353b add [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                MACHINE.regs.ecx,
            ),
        );
        // 0040353e cmp dword ptr [ebp-4],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x0u32,
        );
        // 00403542 jle near ptr 004035F2h
        jle(Cont(x00403548), Cont(x004035f2))
    }
}

pub fn x00403548() -> Cont {
    unsafe {
        // 00403548 mov edi,[ebp-4]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 0040354b mov ecx,[ebp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040354e sar edi,4
        sar();
        // 00403551 dec edi
        MACHINE.regs.edi = dec(MACHINE.regs.edi);
        // 00403552 lea ecx,[ecx+esi-4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00403556 cmp edi,3Fh
        sub(MACHINE.regs.edi, 0x3fu32);
        // 00403559 jbe short 0040355Eh
        jbe(Cont(x0040355b), Cont(x0040355e))
    }
}

pub fn x0040355b() -> Cont {
    unsafe {
        // 0040355b push 3Fh
        push(0x3fu32);
        // 0040355d pop edi
        MACHINE.regs.edi = pop();
        // 0040355e mov ebx,[ebp-0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 00403561 lea ebx,[ebx+edi*8]
        MACHINE.regs.ebx = MACHINE
            .regs
            .ebx
            .wrapping_add((MACHINE.regs.edi * 8))
            .wrapping_add(0x0u32);
        // 00403564 mov [ebp+10h],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.ebx);
        // 00403567 mov ebx,[ebx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 0040356a mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 0040356d mov ebx,[ebp+10h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403570 mov [ecx+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403573 mov [ebx+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 00403576 mov ebx,[ecx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00403579 mov [ebx+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 0040357c mov ebx,[ecx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 0040357f cmp ebx,[ecx+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32)),
        );
        // 00403582 jne short 004035E0h
        jne(Cont(x00403584), Cont(x004035e0))
    }
}

pub fn x0040355e() -> Cont {
    unsafe {
        // 0040355e mov ebx,[ebp-0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 00403561 lea ebx,[ebx+edi*8]
        MACHINE.regs.ebx = MACHINE
            .regs
            .ebx
            .wrapping_add((MACHINE.regs.edi * 8))
            .wrapping_add(0x0u32);
        // 00403564 mov [ebp+10h],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.ebx);
        // 00403567 mov ebx,[ebx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 0040356a mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 0040356d mov ebx,[ebp+10h]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403570 mov [ecx+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403573 mov [ebx+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 00403576 mov ebx,[ecx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00403579 mov [ebx+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 0040357c mov ebx,[ecx+4]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 0040357f cmp ebx,[ecx+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32)),
        );
        // 00403582 jne short 004035E0h
        jne(Cont(x00403584), Cont(x004035e0))
    }
}

pub fn x00403584() -> Cont {
    unsafe {
        // 00403584 mov cl,[edi+eax+4]
        MACHINE.regs.set_cl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .edi
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            ),
        );
        // 00403588 cmp edi,20h
        sub(MACHINE.regs.edi, 0x20u32);
        // 0040358b mov [ebp+13h],cl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0x13u32),
            MACHINE.regs.get_cl(),
        );
        // 0040358e inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 00403590 mov [edi+eax+4],cl
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .edi
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            MACHINE.regs.get_cl(),
        );
        // 00403594 jae short 004035B7h
        jae(Cont(x00403596), Cont(x004035b7))
    }
}

pub fn x00403596() -> Cont {
    unsafe {
        // 00403596 cmp byte ptr [ebp+13h],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x13u32)),
            0x0u8,
        );
        // 0040359a jne short 004035AAh
        jne(Cont(x0040359c), Cont(x004035aa))
    }
}

pub fn x0040359c() -> Cont {
    unsafe {
        // 0040359c mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 004035a1 mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 004035a3 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 004035a5 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004035a8 or [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 004035aa lea eax,[eax+edx*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0x44u32);
        // 004035ae mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 004035b3 mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 004035b5 jmp short 004035DCh
        Cont(x004035dc)
    }
}

pub fn x004035aa() -> Cont {
    unsafe {
        // 004035aa lea eax,[eax+edx*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0x44u32);
        // 004035ae mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 004035b3 mov ecx,edi
        MACHINE.regs.ecx = MACHINE.regs.edi;
        // 004035b5 jmp short 004035DCh
        Cont(x004035dc)
    }
}

pub fn x004035b7() -> Cont {
    unsafe {
        // 004035b7 cmp byte ptr [ebp+13h],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x13u32)),
            0x0u8,
        );
        // 004035bb jne short 004035CDh
        jne(Cont(x004035bd), Cont(x004035cd))
    }
}

pub fn x004035bd() -> Cont {
    unsafe {
        // 004035bd lea ecx,[edi-20h]
        MACHINE.regs.ecx = MACHINE.regs.edi.wrapping_add(0xffffffe0u32);
        // 004035c0 mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 004035c5 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 004035c7 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004035ca or [ecx+4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 004035cd lea eax,[eax+edx*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0xc4u32);
        // 004035d4 lea ecx,[edi-20h]
        MACHINE.regs.ecx = MACHINE.regs.edi.wrapping_add(0xffffffe0u32);
        // 004035d7 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 004035dc shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 004035de or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 004035e0 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 004035e3 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004035e6 lea eax,[edx+esi-4]
        MACHINE.regs.eax = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 004035ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004035ec mov [ecx+eax-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 004035f0 jmp short 004035F5h
        Cont(x004035f5)
    }
}

pub fn x004035cd() -> Cont {
    unsafe {
        // 004035cd lea eax,[eax+edx*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0xc4u32);
        // 004035d4 lea ecx,[edi-20h]
        MACHINE.regs.ecx = MACHINE.regs.edi.wrapping_add(0xffffffe0u32);
        // 004035d7 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 004035dc shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 004035de or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 004035e0 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 004035e3 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004035e6 lea eax,[edx+esi-4]
        MACHINE.regs.eax = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 004035ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004035ec mov [ecx+eax-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 004035f0 jmp short 004035F5h
        Cont(x004035f5)
    }
}

pub fn x004035dc() -> Cont {
    unsafe {
        // 004035dc shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 004035de or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 004035e0 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 004035e3 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004035e6 lea eax,[edx+esi-4]
        MACHINE.regs.eax = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 004035ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004035ec mov [ecx+eax-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 004035f0 jmp short 004035F5h
        Cont(x004035f5)
    }
}

pub fn x004035e0() -> Cont {
    unsafe {
        // 004035e0 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 004035e3 mov ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 004035e6 lea eax,[edx+esi-4]
        MACHINE.regs.eax = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 004035ea mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004035ec mov [ecx+eax-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 004035f0 jmp short 004035F5h
        Cont(x004035f5)
    }
}

pub fn x004035f2() -> Cont {
    unsafe {
        // 004035f2 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 004035f5 lea eax,[esi+1]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 004035f8 mov [edx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edx.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 004035fb mov [edx+esi-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .edx
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 004035ff jmp near ptr 0040374Bh
        Cont(x0040374b)
    }
}

pub fn x004035f5() -> Cont {
    unsafe {
        // 004035f5 lea eax,[esi+1]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 004035f8 mov [edx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edx.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 004035fb mov [edx+esi-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .edx
                .wrapping_add((MACHINE.regs.esi * 1))
                .wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 004035ff jmp near ptr 0040374Bh
        Cont(x0040374b)
    }
}

pub fn x00403604() -> Cont {
    unsafe {
        // 00403604 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403606 jmp near ptr 0040374Eh
        Cont(x0040374e)
    }
}

pub fn x0040360b() -> Cont {
    unsafe {
        // 0040360b jge near ptr 0040374Bh
        jge(Cont(x00403611), Cont(x0040374b))
    }
}

pub fn x00403611() -> Cont {
    unsafe {
        // 00403611 mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00403614 sub [ebp+10h],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x10u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
                MACHINE.regs.esi,
            ),
        );
        // 00403617 lea ecx,[esi+1]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 0040361a mov [ebx-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 0040361d lea ebx,[ebx+esi-4]
        MACHINE.regs.ebx = MACHINE
            .regs
            .ebx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00403621 mov esi,[ebp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403624 mov [ebp+0Ch],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32), MACHINE.regs.ebx);
        // 00403627 sar esi,4
        sar();
        // 0040362a dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 0040362b mov [ebx-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 0040362e cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 00403631 jbe short 00403636h
        jbe(Cont(x00403633), Cont(x00403636))
    }
}

pub fn x00403633() -> Cont {
    unsafe {
        // 00403633 push 3Fh
        push(0x3fu32);
        // 00403635 pop esi
        MACHINE.regs.esi = pop();
        // 00403636 test byte ptr [ebp-4],1
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x1u8,
        );
        // 0040363a jne near ptr 004036C5h
        jne(Cont(x00403640), Cont(x004036c5))
    }
}

pub fn x00403636() -> Cont {
    unsafe {
        // 00403636 test byte ptr [ebp-4],1
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x1u8,
        );
        // 0040363a jne near ptr 004036C5h
        jne(Cont(x00403640), Cont(x004036c5))
    }
}

pub fn x00403640() -> Cont {
    unsafe {
        // 00403640 mov esi,[ebp-4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403643 sar esi,4
        sar();
        // 00403646 dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 00403647 cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 0040364a jbe short 0040364Fh
        jbe(Cont(x0040364c), Cont(x0040364f))
    }
}

pub fn x0040364c() -> Cont {
    unsafe {
        // 0040364c push 3Fh
        push(0x3fu32);
        // 0040364e pop esi
        MACHINE.regs.esi = pop();
        // 0040364f mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 00403652 cmp ecx,[edi+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32)),
        );
        // 00403655 jne short 0040369Eh
        jne(Cont(x00403657), Cont(x0040369e))
    }
}

pub fn x0040364f() -> Cont {
    unsafe {
        // 0040364f mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 00403652 cmp ecx,[edi+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32)),
        );
        // 00403655 jne short 0040369Eh
        jne(Cont(x00403657), Cont(x0040369e))
    }
}

pub fn x00403657() -> Cont {
    unsafe {
        // 00403657 cmp esi,20h
        sub(MACHINE.regs.esi, 0x20u32);
        // 0040365a jae short 0040367Ah
        jae(Cont(x0040365c), Cont(x0040367a))
    }
}

pub fn x0040365c() -> Cont {
    unsafe {
        // 0040365c mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00403661 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403663 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00403665 lea esi,[esi+eax+4]
        MACHINE.regs.esi = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 00403669 not ebx
        not();
        // 0040366b and [eax+edx*4+44h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0x44u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edx * 4))
                        .wrapping_add(0x44u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 0040366f dec byte ptr [esi]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.esi.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32))),
        );
        // 00403671 jne short 0040369Bh
        jne(Cont(x00403673), Cont(x0040369b))
    }
}

pub fn x00403673() -> Cont {
    unsafe {
        // 00403673 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403676 and [ecx],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00403678 jmp short 0040369Bh
        Cont(x0040369b)
    }
}

pub fn x0040367a() -> Cont {
    unsafe {
        // 0040367a lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 0040367d mov ebx,80000000h
        MACHINE.regs.ebx = 0x80000000u32;
        // 00403682 shr ebx,cl
        MACHINE.regs.ebx = shr(MACHINE.regs.ebx, MACHINE.regs.get_cl());
        // 00403684 lea ecx,[esi+eax+4]
        MACHINE.regs.ecx = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x4u32);
        // 00403688 not ebx
        not();
        // 0040368a and [eax+edx*4+0C4h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.edx * 4))
                .wrapping_add(0xc4u32),
            and(
                MACHINE.memory.read::<u32>(
                    MACHINE
                        .regs
                        .eax
                        .wrapping_add((MACHINE.regs.edx * 4))
                        .wrapping_add(0xc4u32),
                ),
                MACHINE.regs.ebx,
            ),
        );
        // 00403691 dec byte ptr [ecx]
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            dec(MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32))),
        );
        // 00403693 jne short 0040369Bh
        jne(Cont(x00403695), Cont(x0040369b))
    }
}

pub fn x00403695() -> Cont {
    unsafe {
        // 00403695 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403698 and [ecx+4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 0040369b mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040369e mov ecx,[edi+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036a1 mov esi,[edi+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036a4 mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 004036a7 mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036aa mov esi,[edi+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036ad mov [ecx+8],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.esi);
        // 004036b0 mov esi,[ebp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 004036b3 add esi,[ebp-4]
        MACHINE.regs.esi = add(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 004036b6 mov [ebp+10h],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.esi);
        // 004036b9 sar esi,4
        sar();
        // 004036bc dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 004036bd cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 004036c0 jbe short 004036C5h
        jbe(Cont(x004036c2), Cont(x004036c5))
    }
}

pub fn x0040369b() -> Cont {
    unsafe {
        // 0040369b mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040369e mov ecx,[edi+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036a1 mov esi,[edi+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036a4 mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 004036a7 mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036aa mov esi,[edi+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036ad mov [ecx+8],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.esi);
        // 004036b0 mov esi,[ebp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 004036b3 add esi,[ebp-4]
        MACHINE.regs.esi = add(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 004036b6 mov [ebp+10h],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.esi);
        // 004036b9 sar esi,4
        sar();
        // 004036bc dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 004036bd cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 004036c0 jbe short 004036C5h
        jbe(Cont(x004036c2), Cont(x004036c5))
    }
}

pub fn x0040369e() -> Cont {
    unsafe {
        // 0040369e mov ecx,[edi+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036a1 mov esi,[edi+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036a4 mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 004036a7 mov ecx,[edi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32));
        // 004036aa mov esi,[edi+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32));
        // 004036ad mov [ecx+8],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.esi);
        // 004036b0 mov esi,[ebp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 004036b3 add esi,[ebp-4]
        MACHINE.regs.esi = add(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 004036b6 mov [ebp+10h],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.esi);
        // 004036b9 sar esi,4
        sar();
        // 004036bc dec esi
        MACHINE.regs.esi = dec(MACHINE.regs.esi);
        // 004036bd cmp esi,3Fh
        sub(MACHINE.regs.esi, 0x3fu32);
        // 004036c0 jbe short 004036C5h
        jbe(Cont(x004036c2), Cont(x004036c5))
    }
}

pub fn x004036c2() -> Cont {
    unsafe {
        // 004036c2 push 3Fh
        push(0x3fu32);
        // 004036c4 pop esi
        MACHINE.regs.esi = pop();
        // 004036c5 mov ecx,[ebp-0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 004036c8 mov edi,[ecx+esi*8+4]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.esi * 8))
                .wrapping_add(0x4u32),
        );
        // 004036cc lea ecx,[ecx+esi*8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 8))
            .wrapping_add(0x0u32);
        // 004036cf mov [ebx+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 004036d2 mov [ebx+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 004036d5 mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 004036d8 mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 004036db mov [ecx+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 004036de mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 004036e1 cmp ecx,[ebx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
        );
        // 004036e4 jne short 00403742h
        jne(Cont(x004036e6), Cont(x00403742))
    }
}

pub fn x004036c5() -> Cont {
    unsafe {
        // 004036c5 mov ecx,[ebp-0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32));
        // 004036c8 mov edi,[ecx+esi*8+4]
        MACHINE.regs.edi = MACHINE.memory.read::<u32>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.esi * 8))
                .wrapping_add(0x4u32),
        );
        // 004036cc lea ecx,[ecx+esi*8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 8))
            .wrapping_add(0x0u32);
        // 004036cf mov [ebx+4],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32), MACHINE.regs.edi);
        // 004036d2 mov [ebx+8],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32), MACHINE.regs.ecx);
        // 004036d5 mov [ecx+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 004036d8 mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 004036db mov [ecx+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 004036de mov ecx,[ebx+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32));
        // 004036e1 cmp ecx,[ebx+8]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x8u32)),
        );
        // 004036e4 jne short 00403742h
        jne(Cont(x004036e6), Cont(x00403742))
    }
}

pub fn x004036e6() -> Cont {
    unsafe {
        // 004036e6 mov cl,[esi+eax+4]
        MACHINE.regs.set_cl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .esi
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x4u32),
            ),
        );
        // 004036ea cmp esi,20h
        sub(MACHINE.regs.esi, 0x20u32);
        // 004036ed mov [ebp+0Fh],cl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32), MACHINE.regs.get_cl());
        // 004036f0 inc cl
        MACHINE.regs.set_cl(inc(MACHINE.regs.get_cl()));
        // 004036f2 mov [esi+eax+4],cl
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .esi
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0x4u32),
            MACHINE.regs.get_cl(),
        );
        // 004036f6 jae short 00403719h
        jae(Cont(x004036f8), Cont(x00403719))
    }
}

pub fn x004036f8() -> Cont {
    unsafe {
        // 004036f8 cmp byte ptr [ebp+0Fh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32)),
            0x0u8,
        );
        // 004036fc jne short 0040370Ch
        jne(Cont(x004036fe), Cont(x0040370c))
    }
}

pub fn x004036fe() -> Cont {
    unsafe {
        // 004036fe mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403703 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403705 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403707 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 0040370a or [ecx],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.edi,
            ),
        );
        // 0040370c lea eax,[eax+edx*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0x44u32);
        // 00403710 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 00403715 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403717 jmp short 0040373Eh
        Cont(x0040373e)
    }
}

pub fn x0040370c() -> Cont {
    unsafe {
        // 0040370c lea eax,[eax+edx*4+44h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0x44u32);
        // 00403710 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 00403715 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403717 jmp short 0040373Eh
        Cont(x0040373e)
    }
}

pub fn x00403719() -> Cont {
    unsafe {
        // 00403719 cmp byte ptr [ebp+0Fh],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfu32)),
            0x0u8,
        );
        // 0040371d jne short 0040372Fh
        jne(Cont(x0040371f), Cont(x0040372f))
    }
}

pub fn x0040371f() -> Cont {
    unsafe {
        // 0040371f lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 00403722 mov edi,80000000h
        MACHINE.regs.edi = 0x80000000u32;
        // 00403727 shr edi,cl
        MACHINE.regs.edi = shr(MACHINE.regs.edi, MACHINE.regs.get_cl());
        // 00403729 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 0040372c or [ecx+4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.edi,
            ),
        );
        // 0040372f lea eax,[eax+edx*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0xc4u32);
        // 00403736 lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 00403739 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 0040373e shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00403740 or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403742 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403745 mov [ebx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403747 mov [eax+ebx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ebx * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 0040374b push 1
        push(0x1u32);
        // 0040374d pop eax
        MACHINE.regs.eax = pop();
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x0040372f() -> Cont {
    unsafe {
        // 0040372f lea eax,[eax+edx*4+0C4h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 4))
            .wrapping_add(0xc4u32);
        // 00403736 lea ecx,[esi-20h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0xffffffe0u32);
        // 00403739 mov edx,80000000h
        MACHINE.regs.edx = 0x80000000u32;
        // 0040373e shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00403740 or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403742 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403745 mov [ebx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403747 mov [eax+ebx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ebx * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 0040374b push 1
        push(0x1u32);
        // 0040374d pop eax
        MACHINE.regs.eax = pop();
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x0040373e() -> Cont {
    unsafe {
        // 0040373e shr edx,cl
        MACHINE.regs.edx = shr(MACHINE.regs.edx, MACHINE.regs.get_cl());
        // 00403740 or [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403742 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403745 mov [ebx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403747 mov [eax+ebx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ebx * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 0040374b push 1
        push(0x1u32);
        // 0040374d pop eax
        MACHINE.regs.eax = pop();
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x00403742() -> Cont {
    unsafe {
        // 00403742 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403745 mov [ebx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403747 mov [eax+ebx-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE
                .regs
                .eax
                .wrapping_add((MACHINE.regs.ebx * 1))
                .wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 0040374b push 1
        push(0x1u32);
        // 0040374d pop eax
        MACHINE.regs.eax = pop();
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x0040374b() -> Cont {
    unsafe {
        // 0040374b push 1
        push(0x1u32);
        // 0040374d pop eax
        MACHINE.regs.eax = pop();
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x0040374e() -> Cont {
    unsafe {
        // 0040374e pop edi
        MACHINE.regs.edi = pop();
        // 0040374f pop esi
        MACHINE.regs.esi = pop();
        // 00403750 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403751 leave
        leave();
        // 00403752 ret
        ret(0)
    }
}

pub fn x00403753() -> Cont {
    unsafe {
        // 00403753 cmp dword ptr ds:[407218h],0FFFFFFFFh
        sub(MACHINE.memory.read::<u32>(0x407218u32), 0xffffffffu32);
        // 0040375a push ebx
        push(MACHINE.regs.ebx);
        // 0040375b push ebp
        push(MACHINE.regs.ebp);
        // 0040375c push esi
        push(MACHINE.regs.esi);
        // 0040375d push edi
        push(MACHINE.regs.edi);
        // 0040375e jne short 00403767h
        jne(Cont(x00403760), Cont(x00403767))
    }
}

pub fn x00403760() -> Cont {
    unsafe {
        // 00403760 mov esi,407208h
        MACHINE.regs.esi = 0x407208u32;
        // 00403765 jmp short 00403784h
        Cont(x00403784)
    }
}

pub fn x00403767() -> Cont {
    unsafe {
        // 00403767 push 2020h
        push(0x2020u32);
        // 0040376c push 0
        push(0x0u32);
        // 0040376e push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 00403774 call dword ptr ds:[406028h]
        call(0x40377a, Cont(kernel32::HeapAlloc_stdcall))
    }
}

pub fn x0040377a() -> Cont {
    unsafe {
        // 0040377a mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040377c test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 0040377e je near ptr 00403890h
        je(Cont(x00403784), Cont(x00403890))
    }
}

pub fn x00403784() -> Cont {
    unsafe {
        // 00403784 mov ebp,ds:[40605Ch]
        MACHINE.regs.ebp = MACHINE.memory.read::<u32>(0x40605cu32);
        // 0040378a push 4
        push(0x4u32);
        // 0040378c push 2000h
        push(0x2000u32);
        // 00403791 push 400000h
        push(0x400000u32);
        // 00403796 push 0
        push(0x0u32);
        // 00403798 call ebp
        call(0x40379a, indirect(MACHINE.regs.ebp))
    }
}

pub fn x0040379a() -> Cont {
    unsafe {
        // 0040379a mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 0040379c test edi,edi
        and(MACHINE.regs.edi, MACHINE.regs.edi);
        // 0040379e je near ptr 00403879h
        je(Cont(x004037a4), Cont(x00403879))
    }
}

pub fn x004037a4() -> Cont {
    unsafe {
        // 004037a4 push 4
        push(0x4u32);
        // 004037a6 mov ebx,10000h
        MACHINE.regs.ebx = 0x10000u32;
        // 004037ab push 1000h
        push(0x1000u32);
        // 004037b0 push ebx
        push(MACHINE.regs.ebx);
        // 004037b1 push edi
        push(MACHINE.regs.edi);
        // 004037b2 call ebp
        call(0x4037b4, indirect(MACHINE.regs.ebp))
    }
}

pub fn x004037b4() -> Cont {
    unsafe {
        // 004037b4 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004037b6 je near ptr 0040386Bh
        je(Cont(x004037bc), Cont(x0040386b))
    }
}

pub fn x004037bc() -> Cont {
    unsafe {
        // 004037bc mov eax,407208h
        MACHINE.regs.eax = 0x407208u32;
        // 004037c1 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 004037c3 jne short 004037E3h
        jne(Cont(x004037c5), Cont(x004037e3))
    }
}

pub fn x004037c5() -> Cont {
    unsafe {
        // 004037c5 cmp dword ptr ds:[407208h],0
        sub(MACHINE.memory.read::<u32>(0x407208u32), 0x0u32);
        // 004037cc jne short 004037D3h
        jne(Cont(x004037ce), Cont(x004037d3))
    }
}

pub fn x004037ce() -> Cont {
    unsafe {
        // 004037ce mov ds:[407208h],eax
        MACHINE.memory.write::<u32>(0x407208u32, MACHINE.regs.eax);
        // 004037d3 cmp dword ptr ds:[40720Ch],0
        sub(MACHINE.memory.read::<u32>(0x40720cu32), 0x0u32);
        // 004037da jne short 004037F8h
        jne(Cont(x004037dc), Cont(x004037f8))
    }
}

pub fn x004037d3() -> Cont {
    unsafe {
        // 004037d3 cmp dword ptr ds:[40720Ch],0
        sub(MACHINE.memory.read::<u32>(0x40720cu32), 0x0u32);
        // 004037da jne short 004037F8h
        jne(Cont(x004037dc), Cont(x004037f8))
    }
}

pub fn x004037dc() -> Cont {
    unsafe {
        // 004037dc mov ds:[40720Ch],eax
        MACHINE.memory.write::<u32>(0x40720cu32, MACHINE.regs.eax);
        // 004037e1 jmp short 004037F8h
        Cont(x004037f8)
    }
}

pub fn x004037e3() -> Cont {
    unsafe {
        // 004037e3 mov [esi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 004037e5 mov eax,ds:[40720Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40720cu32);
        // 004037ea mov [esi+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 004037ed mov ds:[40720Ch],esi
        MACHINE.memory.write::<u32>(0x40720cu32, MACHINE.regs.esi);
        // 004037f3 mov eax,[esi+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 004037f6 mov [eax],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.esi);
        // 004037f8 lea eax,[edi+400000h]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x400000u32);
        // 004037fe lea ecx,[esi+98h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x98u32);
        // 00403804 mov [esi+14h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x14u32), MACHINE.regs.eax);
        // 00403807 lea eax,[esi+18h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x18u32);
        // 0040380a mov [esi+0Ch],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32), MACHINE.regs.ecx);
        // 0040380d mov [esi+10h],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32), MACHINE.regs.edi);
        // 00403810 mov [esi+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 00403813 xor ebp,ebp
        MACHINE.regs.ebp ^= MACHINE.regs.ebp;
        // 00403815 mov ecx,0F1h
        MACHINE.regs.ecx = 0xf1u32;
        // 0040381a xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 0040381c cmp ebp,10h
        sub(MACHINE.regs.ebp, 0x10u32);
        // 0040381f setge dl
        setge();
        // 00403822 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403823 and edx,ecx
        MACHINE.regs.edx = and(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00403825 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403826 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00403827 mov [eax],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403829 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 0040382c add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 0040382f cmp ebp,400h
        sub(MACHINE.regs.ebp, 0x400u32);
        // 00403835 jl short 0040381Ah
        jl(Cont(x00403837), Cont(x0040381a))
    }
}

pub fn x004037f8() -> Cont {
    unsafe {
        // 004037f8 lea eax,[edi+400000h]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x400000u32);
        // 004037fe lea ecx,[esi+98h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x98u32);
        // 00403804 mov [esi+14h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x14u32), MACHINE.regs.eax);
        // 00403807 lea eax,[esi+18h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x18u32);
        // 0040380a mov [esi+0Ch],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32), MACHINE.regs.ecx);
        // 0040380d mov [esi+10h],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32), MACHINE.regs.edi);
        // 00403810 mov [esi+8],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.eax);
        // 00403813 xor ebp,ebp
        MACHINE.regs.ebp ^= MACHINE.regs.ebp;
        // 00403815 mov ecx,0F1h
        MACHINE.regs.ecx = 0xf1u32;
        // 0040381a xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 0040381c cmp ebp,10h
        sub(MACHINE.regs.ebp, 0x10u32);
        // 0040381f setge dl
        setge();
        // 00403822 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403823 and edx,ecx
        MACHINE.regs.edx = and(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00403825 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403826 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00403827 mov [eax],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403829 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 0040382c add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 0040382f cmp ebp,400h
        sub(MACHINE.regs.ebp, 0x400u32);
        // 00403835 jl short 0040381Ah
        jl(Cont(x00403837), Cont(x0040381a))
    }
}

pub fn x0040381a() -> Cont {
    unsafe {
        // 0040381a xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 0040381c cmp ebp,10h
        sub(MACHINE.regs.ebp, 0x10u32);
        // 0040381f setge dl
        setge();
        // 00403822 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403823 and edx,ecx
        MACHINE.regs.edx = and(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00403825 dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 00403826 inc ebp
        MACHINE.regs.ebp = inc(MACHINE.regs.ebp);
        // 00403827 mov [eax],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403829 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 0040382c add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 0040382f cmp ebp,400h
        sub(MACHINE.regs.ebp, 0x400u32);
        // 00403835 jl short 0040381Ah
        jl(Cont(x00403837), Cont(x0040381a))
    }
}

pub fn x00403837() -> Cont {
    unsafe {
        // 00403837 push ebx
        push(MACHINE.regs.ebx);
        // 00403838 push 0
        push(0x0u32);
        // 0040383a push edi
        push(MACHINE.regs.edi);
        // 0040383b call 00404FC0h
        call(0x403840, Cont(x00404fc0))
    }
}

pub fn x00403840() -> Cont {
    unsafe {
        // 00403840 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00403843 mov eax,[esi+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403846 add eax,ebx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00403848 cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 0040384a jae short 00403867h
        jae(Cont(x0040384c), Cont(x00403867))
    }
}

pub fn x00403843() -> Cont {
    unsafe {
        // 00403843 mov eax,[esi+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403846 add eax,ebx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00403848 cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 0040384a jae short 00403867h
        jae(Cont(x0040384c), Cont(x00403867))
    }
}

pub fn x0040384c() -> Cont {
    unsafe {
        // 0040384c or byte ptr [edi+0F8h],0FFh
        MACHINE.memory.write::<u8>(
            MACHINE.regs.edi.wrapping_add(0xf8u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.edi.wrapping_add(0xf8u32)),
                0xffu8,
            ),
        );
        // 00403853 lea eax,[edi+8]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x8u32);
        // 00403856 mov [edi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403858 mov dword ptr [edi+4],0F0h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32), 0xf0u32);
        // 0040385f add edi,1000h
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x1000u32);
        // 00403865 jmp short 00403843h
        Cont(x00403843)
    }
}

pub fn x00403867() -> Cont {
    unsafe {
        // 00403867 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00403869 jmp short 00403892h
        Cont(x00403892)
    }
}

pub fn x0040386b() -> Cont {
    unsafe {
        // 0040386b push 8000h
        push(0x8000u32);
        // 00403870 push 0
        push(0x0u32);
        // 00403872 push edi
        push(MACHINE.regs.edi);
        // 00403873 call dword ptr ds:[406098h]
        call(0x403879, Cont(kernel32::VirtualFree_stdcall))
    }
}

pub fn x00403879() -> Cont {
    unsafe {
        // 00403879 cmp esi,407208h
        sub(MACHINE.regs.esi, 0x407208u32);
        // 0040387f je short 00403890h
        je(Cont(x00403881), Cont(x00403890))
    }
}

pub fn x00403881() -> Cont {
    unsafe {
        // 00403881 push esi
        push(MACHINE.regs.esi);
        // 00403882 push 0
        push(0x0u32);
        // 00403884 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 0040388a call dword ptr ds:[406090h]
        call(0x403890, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x00403890() -> Cont {
    unsafe {
        // 00403890 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403892 pop edi
        MACHINE.regs.edi = pop();
        // 00403893 pop esi
        MACHINE.regs.esi = pop();
        // 00403894 pop ebp
        MACHINE.regs.ebp = pop();
        // 00403895 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403896 ret
        ret(0)
    }
}

pub fn x00403892() -> Cont {
    unsafe {
        // 00403892 pop edi
        MACHINE.regs.edi = pop();
        // 00403893 pop esi
        MACHINE.regs.esi = pop();
        // 00403894 pop ebp
        MACHINE.regs.ebp = pop();
        // 00403895 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403896 ret
        ret(0)
    }
}

pub fn x00403897() -> Cont {
    unsafe {
        // 00403897 push esi
        push(MACHINE.regs.esi);
        // 00403898 mov esi,[esp+8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 0040389c push 8000h
        push(0x8000u32);
        // 004038a1 push 0
        push(0x0u32);
        // 004038a3 push dword ptr [esi+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32)),
        );
        // 004038a6 call dword ptr ds:[406098h]
        call(0x4038ac, Cont(kernel32::VirtualFree_stdcall))
    }
}

pub fn x004038ac() -> Cont {
    unsafe {
        // 004038ac cmp ds:[409228h],esi
        sub(MACHINE.memory.read::<u32>(0x409228u32), MACHINE.regs.esi);
        // 004038b2 jne short 004038BCh
        jne(Cont(x004038b4), Cont(x004038bc))
    }
}

pub fn x004038b4() -> Cont {
    unsafe {
        // 004038b4 mov eax,[esi+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 004038b7 mov ds:[409228h],eax
        MACHINE.memory.write::<u32>(0x409228u32, MACHINE.regs.eax);
        // 004038bc cmp esi,407208h
        sub(MACHINE.regs.esi, 0x407208u32);
        // 004038c2 je short 004038E4h
        je(Cont(x004038c4), Cont(x004038e4))
    }
}

pub fn x004038bc() -> Cont {
    unsafe {
        // 004038bc cmp esi,407208h
        sub(MACHINE.regs.esi, 0x407208u32);
        // 004038c2 je short 004038E4h
        je(Cont(x004038c4), Cont(x004038e4))
    }
}

pub fn x004038c4() -> Cont {
    unsafe {
        // 004038c4 mov eax,[esi+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 004038c7 mov ecx,[esi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 004038c9 push esi
        push(MACHINE.regs.esi);
        // 004038ca push 0
        push(0x0u32);
        // 004038cc mov [eax],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004038ce mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 004038d0 mov ecx,[esi+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 004038d3 mov [eax+4],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), MACHINE.regs.ecx);
        // 004038d6 push dword ptr ds:[409984h]
        push(MACHINE.memory.read::<u32>(0x409984u32));
        // 004038dc call dword ptr ds:[406090h]
        call(0x4038e2, Cont(kernel32::HeapFree_stdcall))
    }
}

pub fn x004038e2() -> Cont {
    unsafe {
        // 004038e2 pop esi
        MACHINE.regs.esi = pop();
        // 004038e3 ret
        ret(0)
    }
}

pub fn x004038e4() -> Cont {
    unsafe {
        // 004038e4 or dword ptr ds:[407218h],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            0x407218u32,
            or(MACHINE.memory.read::<u32>(0x407218u32), 0xffffffffu32),
        );
        // 004038eb pop esi
        MACHINE.regs.esi = pop();
        // 004038ec ret
        ret(0)
    }
}

pub fn x004038ed() -> Cont {
    unsafe {
        // 004038ed push ebp
        push(MACHINE.regs.ebp);
        // 004038ee mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004038f0 push ecx
        push(MACHINE.regs.ecx);
        // 004038f1 push ebx
        push(MACHINE.regs.ebx);
        // 004038f2 push esi
        push(MACHINE.regs.esi);
        // 004038f3 mov esi,ds:[40720Ch]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x40720cu32);
        // 004038f9 push edi
        push(MACHINE.regs.edi);
        // 004038fa cmp dword ptr [esi+10h],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32)),
            0xffffffffu32,
        );
        // 004038fe je near ptr 00403998h
        je(Cont(x00403904), Cont(x00403998))
    }
}

pub fn x004038fa() -> Cont {
    unsafe {
        // 004038fa cmp dword ptr [esi+10h],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32)),
            0xffffffffu32,
        );
        // 004038fe je near ptr 00403998h
        je(Cont(x00403904), Cont(x00403998))
    }
}

pub fn x00403904() -> Cont {
    unsafe {
        // 00403904 and dword ptr [ebp-4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x0u32,
            ),
        );
        // 00403908 lea edi,[esi+2010h]
        MACHINE.regs.edi = MACHINE.regs.esi.wrapping_add(0x2010u32);
        // 0040390e mov ebx,3FF000h
        MACHINE.regs.ebx = 0x3ff000u32;
        // 00403913 cmp dword ptr [edi],0F0h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            0xf0u32,
        );
        // 00403919 jne short 00403954h
        jne(Cont(x0040391b), Cont(x00403954))
    }
}

pub fn x00403913() -> Cont {
    unsafe {
        // 00403913 cmp dword ptr [edi],0F0h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
            0xf0u32,
        );
        // 00403919 jne short 00403954h
        jne(Cont(x0040391b), Cont(x00403954))
    }
}

pub fn x0040391b() -> Cont {
    unsafe {
        // 0040391b mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 0040391d push 4000h
        push(0x4000u32);
        // 00403922 add eax,[esi+10h]
        MACHINE.regs.eax = add(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32)),
        );
        // 00403925 push 1000h
        push(0x1000u32);
        // 0040392a push eax
        push(MACHINE.regs.eax);
        // 0040392b call dword ptr ds:[406098h]
        call(0x403931, Cont(kernel32::VirtualFree_stdcall))
    }
}

pub fn x00403931() -> Cont {
    unsafe {
        // 00403931 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403933 je short 00403954h
        je(Cont(x00403935), Cont(x00403954))
    }
}

pub fn x00403935() -> Cont {
    unsafe {
        // 00403935 or dword ptr [edi],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                0xffffffffu32,
            ),
        );
        // 00403938 dec dword ptr ds:[409704h]
        MACHINE
            .memory
            .write::<u32>(0x409704u32, dec(MACHINE.memory.read::<u32>(0x409704u32)));
        // 0040393e mov eax,[esi+0Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32));
        // 00403941 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403943 je short 00403949h
        je(Cont(x00403945), Cont(x00403949))
    }
}

pub fn x00403945() -> Cont {
    unsafe {
        // 00403945 cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 00403947 jbe short 0040394Ch
        jbe(Cont(x00403949), Cont(x0040394c))
    }
}

pub fn x00403949() -> Cont {
    unsafe {
        // 00403949 mov [esi+0Ch],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0xcu32), MACHINE.regs.edi);
        // 0040394c inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 0040394f dec dword ptr [ebp+8]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x8u32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32))),
        );
        // 00403952 je short 00403961h
        je(Cont(x00403954), Cont(x00403961))
    }
}

pub fn x0040394c() -> Cont {
    unsafe {
        // 0040394c inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 0040394f dec dword ptr [ebp+8]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x8u32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32))),
        );
        // 00403952 je short 00403961h
        je(Cont(x00403954), Cont(x00403961))
    }
}

pub fn x00403954() -> Cont {
    unsafe {
        // 00403954 sub ebx,1000h
        MACHINE.regs.ebx = sub(MACHINE.regs.ebx, 0x1000u32);
        // 0040395a sub edi,8
        MACHINE.regs.edi = sub(MACHINE.regs.edi, 0x8u32);
        // 0040395d test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 0040395f jge short 00403913h
        jge(Cont(x00403961), Cont(x00403913))
    }
}

pub fn x00403961() -> Cont {
    unsafe {
        // 00403961 cmp dword ptr [ebp-4],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x0u32,
        );
        // 00403965 mov ecx,esi
        MACHINE.regs.ecx = MACHINE.regs.esi;
        // 00403967 mov esi,[esi+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32));
        // 0040396a je short 00403998h
        je(Cont(x0040396c), Cont(x00403998))
    }
}

pub fn x0040396c() -> Cont {
    unsafe {
        // 0040396c cmp dword ptr [ecx+18h],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x18u32)),
            0xffffffffu32,
        );
        // 00403970 jne short 00403998h
        jne(Cont(x00403972), Cont(x00403998))
    }
}

pub fn x00403972() -> Cont {
    unsafe {
        // 00403972 push 1
        push(0x1u32);
        // 00403974 lea eax,[ecx+20h]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0x20u32);
        // 00403977 pop edx
        MACHINE.regs.edx = pop();
        // 00403978 cmp dword ptr [eax],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 0040397b jne short 00403989h
        jne(Cont(x0040397d), Cont(x00403989))
    }
}

pub fn x00403978() -> Cont {
    unsafe {
        // 00403978 cmp dword ptr [eax],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 0040397b jne short 00403989h
        jne(Cont(x0040397d), Cont(x00403989))
    }
}

pub fn x0040397d() -> Cont {
    unsafe {
        // 0040397d inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 0040397e add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 00403981 cmp edx,400h
        sub(MACHINE.regs.edx, 0x400u32);
        // 00403987 jl short 00403978h
        jl(Cont(x00403989), Cont(x00403978))
    }
}

pub fn x00403989() -> Cont {
    unsafe {
        // 00403989 cmp edx,400h
        sub(MACHINE.regs.edx, 0x400u32);
        // 0040398f jne short 00403998h
        jne(Cont(x00403991), Cont(x00403998))
    }
}

pub fn x00403991() -> Cont {
    unsafe {
        // 00403991 push ecx
        push(MACHINE.regs.ecx);
        // 00403992 call 00403897h
        call(0x403997, Cont(x00403897))
    }
}

pub fn x00403997() -> Cont {
    unsafe {
        // 00403997 pop ecx
        MACHINE.regs.ecx = pop();
        // 00403998 cmp esi,ds:[40720Ch]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x40720cu32));
        // 0040399e je short 004039AAh
        je(Cont(x004039a0), Cont(x004039aa))
    }
}

pub fn x00403998() -> Cont {
    unsafe {
        // 00403998 cmp esi,ds:[40720Ch]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x40720cu32));
        // 0040399e je short 004039AAh
        je(Cont(x004039a0), Cont(x004039aa))
    }
}

pub fn x004039a0() -> Cont {
    unsafe {
        // 004039a0 cmp dword ptr [ebp+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 004039a4 jg near ptr 004038FAh
        jg(Cont(x004039aa), Cont(x004038fa))
    }
}

pub fn x004039aa() -> Cont {
    unsafe {
        // 004039aa pop edi
        MACHINE.regs.edi = pop();
        // 004039ab pop esi
        MACHINE.regs.esi = pop();
        // 004039ac pop ebx
        MACHINE.regs.ebx = pop();
        // 004039ad leave
        leave();
        // 004039ae ret
        ret(0)
    }
}

pub fn x004039af() -> Cont {
    unsafe {
        // 004039af mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 004039b3 mov edx,407208h
        MACHINE.regs.edx = 0x407208u32;
        // 004039b8 push esi
        push(MACHINE.regs.esi);
        // 004039b9 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 004039bb cmp eax,[ecx+10h]
        sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x10u32)),
        );
        // 004039be jbe short 004039C5h
        jbe(Cont(x004039c0), Cont(x004039c5))
    }
}

pub fn x004039bb() -> Cont {
    unsafe {
        // 004039bb cmp eax,[ecx+10h]
        sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x10u32)),
        );
        // 004039be jbe short 004039C5h
        jbe(Cont(x004039c0), Cont(x004039c5))
    }
}

pub fn x004039c0() -> Cont {
    unsafe {
        // 004039c0 cmp eax,[ecx+14h]
        sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x14u32)),
        );
        // 004039c3 jb short 004039CDh
        jb(Cont(x004039c5), Cont(x004039cd))
    }
}

pub fn x004039c5() -> Cont {
    unsafe {
        // 004039c5 mov ecx,[ecx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 004039c7 cmp ecx,edx
        sub(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 004039c9 je short 00403A02h
        je(Cont(x004039cb), Cont(x00403a02))
    }
}

pub fn x004039cb() -> Cont {
    unsafe {
        // 004039cb jmp short 004039BBh
        Cont(x004039bb)
    }
}

pub fn x004039cd() -> Cont {
    unsafe {
        // 004039cd test al,0Fh
        and(MACHINE.regs.get_al(), 0xfu8);
        // 004039cf jne short 00403A02h
        jne(Cont(x004039d1), Cont(x00403a02))
    }
}

pub fn x004039d1() -> Cont {
    unsafe {
        // 004039d1 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 004039d3 mov edx,100h
        MACHINE.regs.edx = 0x100u32;
        // 004039d8 and esi,0FFFh
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0xfffu32);
        // 004039de cmp esi,edx
        sub(MACHINE.regs.esi, MACHINE.regs.edx);
        // 004039e0 jb short 00403A02h
        jb(Cont(x004039e2), Cont(x00403a02))
    }
}

pub fn x004039e2() -> Cont {
    unsafe {
        // 004039e2 mov esi,[esp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 004039e6 mov [esi],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004039e8 mov esi,[esp+10h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 004039ec mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 004039ee and cx,0F000h
        MACHINE.regs.set_cx(and(MACHINE.regs.get_cx(), 0xf000u16));
        // 004039f3 sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004039f5 mov [esi],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004039f7 sub eax,edx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.edx);
        // 004039f9 pop esi
        MACHINE.regs.esi = pop();
        // 004039fa sar eax,4
        sar();
        // 004039fd lea eax,[eax+ecx+8]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 1))
            .wrapping_add(0x8u32);
        // 00403a01 ret
        ret(0)
    }
}

pub fn x00403a02() -> Cont {
    unsafe {
        // 00403a02 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403a04 pop esi
        MACHINE.regs.esi = pop();
        // 00403a05 ret
        ret(0)
    }
}

pub fn x00403a06() -> Cont {
    unsafe {
        // 00403a06 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00403a0a mov ecx,[esp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00403a0e sub ecx,[eax+10h]
        MACHINE.regs.ecx = sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32)),
        );
        // 00403a11 sar ecx,0Ch
        sar();
        // 00403a14 lea eax,[eax+ecx*8+18h]
        MACHINE.regs.eax = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.ecx * 8))
            .wrapping_add(0x18u32);
        // 00403a18 mov ecx,[esp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 00403a1c movzx edx,byte ptr [ecx]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)) as _;
        // 00403a1f add [eax],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x0u32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403a21 and byte ptr [ecx],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                0x0u8,
            ),
        );
        // 00403a24 cmp dword ptr [eax],0F0h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0xf0u32,
        );
        // 00403a2a mov dword ptr [eax+4],0F1h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32), 0xf1u32);
        // 00403a31 jne short 00403A4Ah
        jne(Cont(x00403a33), Cont(x00403a4a))
    }
}

pub fn x00403a33() -> Cont {
    unsafe {
        // 00403a33 inc dword ptr ds:[409704h]
        MACHINE
            .memory
            .write::<u32>(0x409704u32, inc(MACHINE.memory.read::<u32>(0x409704u32)));
        // 00403a39 cmp dword ptr ds:[409704h],20h
        sub(MACHINE.memory.read::<u32>(0x409704u32), 0x20u32);
        // 00403a40 jne short 00403A4Ah
        jne(Cont(x00403a42), Cont(x00403a4a))
    }
}

pub fn x00403a42() -> Cont {
    unsafe {
        // 00403a42 push 10h
        push(0x10u32);
        // 00403a44 call 004038EDh
        call(0x403a49, Cont(x004038ed))
    }
}

pub fn x00403a49() -> Cont {
    unsafe {
        // 00403a49 pop ecx
        MACHINE.regs.ecx = pop();
        // 00403a4a ret
        ret(0)
    }
}

pub fn x00403a4a() -> Cont {
    unsafe {
        // 00403a4a ret
        ret(0)
    }
}

pub fn x00403a4b() -> Cont {
    unsafe {
        // 00403a4b push ebp
        push(MACHINE.regs.ebp);
        // 00403a4c mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403a4e push ecx
        push(MACHINE.regs.ecx);
        // 00403a4f push ecx
        push(MACHINE.regs.ecx);
        // 00403a50 push ebx
        push(MACHINE.regs.ebx);
        // 00403a51 push esi
        push(MACHINE.regs.esi);
        // 00403a52 mov esi,ds:[409228h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x409228u32);
        // 00403a58 push edi
        push(MACHINE.regs.edi);
        // 00403a59 mov edx,[esi+10h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403a5c cmp edx,0FFFFFFFFh
        sub(MACHINE.regs.edx, 0xffffffffu32);
        // 00403a5f je near ptr 00403B04h
        je(Cont(x00403a65), Cont(x00403b04))
    }
}

pub fn x00403a59() -> Cont {
    unsafe {
        // 00403a59 mov edx,[esi+10h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403a5c cmp edx,0FFFFFFFFh
        sub(MACHINE.regs.edx, 0xffffffffu32);
        // 00403a5f je near ptr 00403B04h
        je(Cont(x00403a65), Cont(x00403b04))
    }
}

pub fn x00403a65() -> Cont {
    unsafe {
        // 00403a65 mov edi,[esi+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32));
        // 00403a68 lea ecx,[esi+2018h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x2018u32);
        // 00403a6e mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00403a70 sub eax,esi
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403a72 sub eax,18h
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x18u32);
        // 00403a75 sar eax,3
        sar();
        // 00403a78 shl eax,0Ch
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0xcu8);
        // 00403a7b add eax,edx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.edx);
        // 00403a7d cmp edi,ecx
        sub(MACHINE.regs.edi, MACHINE.regs.ecx);
        // 00403a7f mov [ebp-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00403a82 jae short 00403ABEh
        jae(Cont(x00403a84), Cont(x00403abe))
    }
}

pub fn x00403a84() -> Cont {
    unsafe {
        // 00403a84 mov ecx,[edi]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 00403a86 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403a89 cmp ecx,ebx
        sub(MACHINE.regs.ecx, MACHINE.regs.ebx);
        // 00403a8b jl short 00403AA7h
        jl(Cont(x00403a8d), Cont(x00403aa7))
    }
}

pub fn x00403a8d() -> Cont {
    unsafe {
        // 00403a8d cmp [edi+4],ebx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32)),
            MACHINE.regs.ebx,
        );
        // 00403a90 jbe short 00403AA7h
        jbe(Cont(x00403a92), Cont(x00403aa7))
    }
}

pub fn x00403a92() -> Cont {
    unsafe {
        // 00403a92 push ebx
        push(MACHINE.regs.ebx);
        // 00403a93 push ecx
        push(MACHINE.regs.ecx);
        // 00403a94 push eax
        push(MACHINE.regs.eax);
        // 00403a95 call 00403C53h
        call(0x403a9a, Cont(x00403c53))
    }
}

pub fn x00403a9a() -> Cont {
    unsafe {
        // 00403a9a add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00403a9d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403a9f jne short 00403B16h
        jne(Cont(x00403aa1), Cont(x00403b16))
    }
}

pub fn x00403aa1() -> Cont {
    unsafe {
        // 00403aa1 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403aa4 mov [edi+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 00403aa7 add edi,8
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x8u32);
        // 00403aaa lea ecx,[esi+2018h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x2018u32);
        // 00403ab0 add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403ab5 cmp edi,ecx
        sub(MACHINE.regs.edi, MACHINE.regs.ecx);
        // 00403ab7 mov [ebp-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00403aba jb short 00403A84h
        jb(Cont(x00403abc), Cont(x00403a84))
    }
}

pub fn x00403aa7() -> Cont {
    unsafe {
        // 00403aa7 add edi,8
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x8u32);
        // 00403aaa lea ecx,[esi+2018h]
        MACHINE.regs.ecx = MACHINE.regs.esi.wrapping_add(0x2018u32);
        // 00403ab0 add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403ab5 cmp edi,ecx
        sub(MACHINE.regs.edi, MACHINE.regs.ecx);
        // 00403ab7 mov [ebp-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 00403aba jb short 00403A84h
        jb(Cont(x00403abc), Cont(x00403a84))
    }
}

pub fn x00403abc() -> Cont {
    unsafe {
        // 00403abc jmp short 00403AC1h
        Cont(x00403ac1)
    }
}

pub fn x00403abe() -> Cont {
    unsafe {
        // 00403abe mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403ac1 mov eax,[esi+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32));
        // 00403ac4 mov ecx,[esi+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403ac7 lea edi,[esi+18h]
        MACHINE.regs.edi = MACHINE.regs.esi.wrapping_add(0x18u32);
        // 00403aca mov [ebp-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 00403acd cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00403acf mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00403ad2 jae short 00403B07h
        jae(Cont(x00403ad4), Cont(x00403b07))
    }
}

pub fn x00403ac1() -> Cont {
    unsafe {
        // 00403ac1 mov eax,[esi+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32));
        // 00403ac4 mov ecx,[esi+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x10u32));
        // 00403ac7 lea edi,[esi+18h]
        MACHINE.regs.edi = MACHINE.regs.esi.wrapping_add(0x18u32);
        // 00403aca mov [ebp-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 00403acd cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00403acf mov [ebp-4],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ecx,
        );
        // 00403ad2 jae short 00403B07h
        jae(Cont(x00403ad4), Cont(x00403b07))
    }
}

pub fn x00403ad4() -> Cont {
    unsafe {
        // 00403ad4 mov eax,[edi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 00403ad6 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00403ad8 jl short 00403AF3h
        jl(Cont(x00403ada), Cont(x00403af3))
    }
}

pub fn x00403ada() -> Cont {
    unsafe {
        // 00403ada cmp [edi+4],ebx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32)),
            MACHINE.regs.ebx,
        );
        // 00403add jbe short 00403AF3h
        jbe(Cont(x00403adf), Cont(x00403af3))
    }
}

pub fn x00403adf() -> Cont {
    unsafe {
        // 00403adf push ebx
        push(MACHINE.regs.ebx);
        // 00403ae0 push eax
        push(MACHINE.regs.eax);
        // 00403ae1 push dword ptr [ebp-4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00403ae4 call 00403C53h
        call(0x403ae9, Cont(x00403c53))
    }
}

pub fn x00403ae9() -> Cont {
    unsafe {
        // 00403ae9 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00403aec test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403aee jne short 00403B16h
        jne(Cont(x00403af0), Cont(x00403b16))
    }
}

pub fn x00403af0() -> Cont {
    unsafe {
        // 00403af0 mov [edi+4],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32), MACHINE.regs.ebx);
        // 00403af3 add dword ptr [ebp-4],1000h
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x1000u32,
            ),
        );
        // 00403afa add edi,8
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x8u32);
        // 00403afd cmp edi,[ebp-8]
        sub(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00403b00 jb short 00403AD4h
        jb(Cont(x00403b02), Cont(x00403ad4))
    }
}

pub fn x00403af3() -> Cont {
    unsafe {
        // 00403af3 add dword ptr [ebp-4],1000h
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x1000u32,
            ),
        );
        // 00403afa add edi,8
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x8u32);
        // 00403afd cmp edi,[ebp-8]
        sub(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00403b00 jb short 00403AD4h
        jb(Cont(x00403b02), Cont(x00403ad4))
    }
}

pub fn x00403b02() -> Cont {
    unsafe {
        // 00403b02 jmp short 00403B07h
        Cont(x00403b07)
    }
}

pub fn x00403b04() -> Cont {
    unsafe {
        // 00403b04 mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403b07 mov esi,[esi]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00403b09 cmp esi,ds:[409228h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409228u32));
        // 00403b0f je short 00403B26h
        je(Cont(x00403b11), Cont(x00403b26))
    }
}

pub fn x00403b07() -> Cont {
    unsafe {
        // 00403b07 mov esi,[esi]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00403b09 cmp esi,ds:[409228h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409228u32));
        // 00403b0f je short 00403B26h
        je(Cont(x00403b11), Cont(x00403b26))
    }
}

pub fn x00403b11() -> Cont {
    unsafe {
        // 00403b11 jmp near ptr 00403A59h
        Cont(x00403a59)
    }
}

pub fn x00403b16() -> Cont {
    unsafe {
        // 00403b16 mov ds:[409228h],esi
        MACHINE.memory.write::<u32>(0x409228u32, MACHINE.regs.esi);
        // 00403b1c sub [edi],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.ebx,
            ),
        );
        // 00403b1e mov [esi+8],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.edi);
        // 00403b21 jmp near ptr 00403C4Eh
        Cont(x00403c4e)
    }
}

pub fn x00403b26() -> Cont {
    unsafe {
        // 00403b26 mov eax,407208h
        MACHINE.regs.eax = 0x407208u32;
        // 00403b2b mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00403b2d cmp dword ptr [edi+10h],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x10u32)),
            0xffffffffu32,
        );
        // 00403b31 je short 00403B39h
        je(Cont(x00403b33), Cont(x00403b39))
    }
}

pub fn x00403b2d() -> Cont {
    unsafe {
        // 00403b2d cmp dword ptr [edi+10h],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x10u32)),
            0xffffffffu32,
        );
        // 00403b31 je short 00403B39h
        je(Cont(x00403b33), Cont(x00403b39))
    }
}

pub fn x00403b33() -> Cont {
    unsafe {
        // 00403b33 cmp dword ptr [edi+0Ch],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0xcu32)),
            0x0u32,
        );
        // 00403b37 jne short 00403B45h
        jne(Cont(x00403b39), Cont(x00403b45))
    }
}

pub fn x00403b39() -> Cont {
    unsafe {
        // 00403b39 mov edi,[edi]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32));
        // 00403b3b cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00403b3d je near ptr 00403C1Ah
        je(Cont(x00403b43), Cont(x00403c1a))
    }
}

pub fn x00403b43() -> Cont {
    unsafe {
        // 00403b43 jmp short 00403B2Dh
        Cont(x00403b2d)
    }
}

pub fn x00403b45() -> Cont {
    unsafe {
        // 00403b45 mov ebx,[edi+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edi.wrapping_add(0xcu32));
        // 00403b48 and dword ptr [ebp-4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x0u32,
            ),
        );
        // 00403b4c mov esi,ebx
        MACHINE.regs.esi = MACHINE.regs.ebx;
        // 00403b4e mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00403b50 sub esi,edi
        MACHINE.regs.esi = sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 00403b52 sub esi,18h
        MACHINE.regs.esi = sub(MACHINE.regs.esi, 0x18u32);
        // 00403b55 sar esi,3
        sar();
        // 00403b58 shl esi,0Ch
        MACHINE.regs.esi = shl(MACHINE.regs.esi, 0xcu8);
        // 00403b5b add esi,[edi+10h]
        MACHINE.regs.esi = add(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x10u32)),
        );
        // 00403b5e cmp dword ptr [ebx],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 00403b61 jne short 00403B74h
        jne(Cont(x00403b63), Cont(x00403b74))
    }
}

pub fn x00403b63() -> Cont {
    unsafe {
        // 00403b63 cmp dword ptr [ebp-4],10h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x10u32,
        );
        // 00403b67 jge short 00403B74h
        jge(Cont(x00403b69), Cont(x00403b74))
    }
}

pub fn x00403b69() -> Cont {
    unsafe {
        // 00403b69 add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 00403b6c inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 00403b6f cmp dword ptr [eax],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 00403b72 je short 00403B63h
        je(Cont(x00403b74), Cont(x00403b63))
    }
}

pub fn x00403b74() -> Cont {
    unsafe {
        // 00403b74 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403b77 push 4
        push(0x4u32);
        // 00403b79 shl eax,0Ch
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0xcu8);
        // 00403b7c push 1000h
        push(0x1000u32);
        // 00403b81 push eax
        push(MACHINE.regs.eax);
        // 00403b82 push esi
        push(MACHINE.regs.esi);
        // 00403b83 mov [ebp-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 00403b86 call dword ptr ds:[40605Ch]
        call(0x403b8c, Cont(kernel32::VirtualAlloc_stdcall))
    }
}

pub fn x00403b8c() -> Cont {
    unsafe {
        // 00403b8c cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403b8e jne near ptr 00403C4Ch
        jne(Cont(x00403b94), Cont(x00403c4c))
    }
}

pub fn x00403b94() -> Cont {
    unsafe {
        // 00403b94 push 0
        push(0x0u32);
        // 00403b96 push dword ptr [ebp-8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
        );
        // 00403b99 push esi
        push(MACHINE.regs.esi);
        // 00403b9a call 00404FC0h
        call(0x403b9f, Cont(x00404fc0))
    }
}

pub fn x00403b9f() -> Cont {
    unsafe {
        // 00403b9f mov edx,[ebp-4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403ba2 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00403ba5 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00403ba7 mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00403ba9 jle short 00403BDBh
        jle(Cont(x00403bab), Cont(x00403bdb))
    }
}

pub fn x00403bab() -> Cont {
    unsafe {
        // 00403bab lea eax,[esi+4]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x4u32);
        // 00403bae mov [ebp-4],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edx,
        );
        // 00403bb1 or byte ptr [eax+0F4h],0FFh
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0xf4u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0xf4u32)),
                0xffu8,
            ),
        );
        // 00403bb8 lea edx,[eax+4]
        MACHINE.regs.edx = MACHINE.regs.eax.wrapping_add(0x4u32);
        // 00403bbb mov [eax-4],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edx,
        );
        // 00403bbe mov edx,0F0h
        MACHINE.regs.edx = 0xf0u32;
        // 00403bc3 mov [eax],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403bc5 mov [ecx],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403bc7 mov dword ptr [ecx+4],0F1h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), 0xf1u32);
        // 00403bce add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403bd3 add ecx,8
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x8u32);
        // 00403bd6 dec dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 00403bd9 jne short 00403BB1h
        jne(Cont(x00403bdb), Cont(x00403bb1))
    }
}

pub fn x00403bb1() -> Cont {
    unsafe {
        // 00403bb1 or byte ptr [eax+0F4h],0FFh
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0xf4u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0xf4u32)),
                0xffu8,
            ),
        );
        // 00403bb8 lea edx,[eax+4]
        MACHINE.regs.edx = MACHINE.regs.eax.wrapping_add(0x4u32);
        // 00403bbb mov [eax-4],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edx,
        );
        // 00403bbe mov edx,0F0h
        MACHINE.regs.edx = 0xf0u32;
        // 00403bc3 mov [eax],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403bc5 mov [ecx],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403bc7 mov dword ptr [ecx+4],0F1h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), 0xf1u32);
        // 00403bce add eax,1000h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x1000u32);
        // 00403bd3 add ecx,8
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x8u32);
        // 00403bd6 dec dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 00403bd9 jne short 00403BB1h
        jne(Cont(x00403bdb), Cont(x00403bb1))
    }
}

pub fn x00403bdb() -> Cont {
    unsafe {
        // 00403bdb mov ds:[409228h],edi
        MACHINE.memory.write::<u32>(0x409228u32, MACHINE.regs.edi);
        // 00403be1 lea eax,[edi+2018h]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x2018u32);
        // 00403be7 cmp ecx,eax
        sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00403be9 jae short 00403BF7h
        jae(Cont(x00403beb), Cont(x00403bf7))
    }
}

pub fn x00403be7() -> Cont {
    unsafe {
        // 00403be7 cmp ecx,eax
        sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00403be9 jae short 00403BF7h
        jae(Cont(x00403beb), Cont(x00403bf7))
    }
}

pub fn x00403beb() -> Cont {
    unsafe {
        // 00403beb cmp dword ptr [ecx],0FFFFFFFFh
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            0xffffffffu32,
        );
        // 00403bee je short 00403BF5h
        je(Cont(x00403bf0), Cont(x00403bf5))
    }
}

pub fn x00403bf0() -> Cont {
    unsafe {
        // 00403bf0 add ecx,8
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x8u32);
        // 00403bf3 jmp short 00403BE7h
        Cont(x00403be7)
    }
}

pub fn x00403bf5() -> Cont {
    unsafe {
        // 00403bf5 cmp ecx,eax
        sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00403bf7 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403bf9 and eax,ecx
        MACHINE.regs.eax = and(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00403bfb mov [edi+0Ch],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0xcu32), MACHINE.regs.eax);
        // 00403bfe mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403c01 mov [esi+8],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.get_al());
        // 00403c04 mov [edi+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403c07 sub [ebx],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x0u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
                MACHINE.regs.eax,
            ),
        );
        // 00403c09 sub [esi+4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                MACHINE.regs.eax,
            ),
        );
        // 00403c0c lea ecx,[esi+eax+8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x8u32);
        // 00403c10 lea eax,[esi+100h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x100u32);
        // 00403c16 mov [esi],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00403c18 jmp short 00403C4Eh
        Cont(x00403c4e)
    }
}

pub fn x00403bf7() -> Cont {
    unsafe {
        // 00403bf7 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403bf9 and eax,ecx
        MACHINE.regs.eax = and(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00403bfb mov [edi+0Ch],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0xcu32), MACHINE.regs.eax);
        // 00403bfe mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403c01 mov [esi+8],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x8u32), MACHINE.regs.get_al());
        // 00403c04 mov [edi+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403c07 sub [ebx],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x0u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
                MACHINE.regs.eax,
            ),
        );
        // 00403c09 sub [esi+4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.esi.wrapping_add(0x4u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.esi.wrapping_add(0x4u32)),
                MACHINE.regs.eax,
            ),
        );
        // 00403c0c lea ecx,[esi+eax+8]
        MACHINE.regs.ecx = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x8u32);
        // 00403c10 lea eax,[esi+100h]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x100u32);
        // 00403c16 mov [esi],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 00403c18 jmp short 00403C4Eh
        Cont(x00403c4e)
    }
}

pub fn x00403c1a() -> Cont {
    unsafe {
        // 00403c1a call 00403753h
        call(0x403c1f, Cont(x00403753))
    }
}

pub fn x00403c1f() -> Cont {
    unsafe {
        // 00403c1f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403c21 je short 00403C4Ch
        je(Cont(x00403c23), Cont(x00403c4c))
    }
}

pub fn x00403c23() -> Cont {
    unsafe {
        // 00403c23 mov ecx,[eax+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00403c26 mov [ecx+8],bl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ecx.wrapping_add(0x8u32), MACHINE.regs.get_bl());
        // 00403c29 lea edx,[ecx+ebx+8]
        MACHINE.regs.edx = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.ebx * 1))
            .wrapping_add(0x8u32);
        // 00403c2d mov ds:[409228h],eax
        MACHINE.memory.write::<u32>(0x409228u32, MACHINE.regs.eax);
        // 00403c32 mov [ecx],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00403c34 mov edx,0F0h
        MACHINE.regs.edx = 0xf0u32;
        // 00403c39 sub edx,ebx
        MACHINE.regs.edx = sub(MACHINE.regs.edx, MACHINE.regs.ebx);
        // 00403c3b mov [ecx+4],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.edx);
        // 00403c3e movzx edx,bl
        MACHINE.regs.edx = MACHINE.regs.get_bl() as _;
        // 00403c41 sub [eax+18h],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.eax.wrapping_add(0x18u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.eax.wrapping_add(0x18u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403c44 lea eax,[ecx+100h]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0x100u32);
        // 00403c4a jmp short 00403C4Eh
        Cont(x00403c4e)
    }
}

pub fn x00403c4c() -> Cont {
    unsafe {
        // 00403c4c xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403c4e pop edi
        MACHINE.regs.edi = pop();
        // 00403c4f pop esi
        MACHINE.regs.esi = pop();
        // 00403c50 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403c51 leave
        leave();
        // 00403c52 ret
        ret(0)
    }
}

pub fn x00403c4e() -> Cont {
    unsafe {
        // 00403c4e pop edi
        MACHINE.regs.edi = pop();
        // 00403c4f pop esi
        MACHINE.regs.esi = pop();
        // 00403c50 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403c51 leave
        leave();
        // 00403c52 ret
        ret(0)
    }
}

pub fn x00403c53() -> Cont {
    unsafe {
        // 00403c53 push ebp
        push(MACHINE.regs.ebp);
        // 00403c54 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403c56 push ecx
        push(MACHINE.regs.ecx);
        // 00403c57 mov ecx,[ebp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403c5a mov edx,[ebp+10h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403c5d push ebx
        push(MACHINE.regs.ebx);
        // 00403c5e push esi
        push(MACHINE.regs.esi);
        // 00403c5f mov esi,[ecx+4]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32));
        // 00403c62 push edi
        push(MACHINE.regs.edi);
        // 00403c63 mov edi,[ecx]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 00403c65 lea ebx,[ecx+0F8h]
        MACHINE.regs.ebx = MACHINE.regs.ecx.wrapping_add(0xf8u32);
        // 00403c6b cmp esi,edx
        sub(MACHINE.regs.esi, MACHINE.regs.edx);
        // 00403c6d mov [ebp-4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edi,
        );
        // 00403c70 mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00403c72 mov [ebp+8],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.ebx);
        // 00403c75 jb short 00403C98h
        jb(Cont(x00403c77), Cont(x00403c98))
    }
}

pub fn x00403c77() -> Cont {
    unsafe {
        // 00403c77 lea eax,[edi+edx]
        MACHINE.regs.eax = MACHINE
            .regs
            .edi
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403c7a mov [edi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00403c7c cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00403c7e jae short 00403C87h
        jae(Cont(x00403c80), Cont(x00403c87))
    }
}

pub fn x00403c80() -> Cont {
    unsafe {
        // 00403c80 add [ecx],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x0u32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403c82 sub [ecx+4],edx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                MACHINE.regs.edx,
            ),
        );
        // 00403c85 jmp short 00403C90h
        Cont(x00403c90)
    }
}

pub fn x00403c87() -> Cont {
    unsafe {
        // 00403c87 and dword ptr [ecx+4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                0x0u32,
            ),
        );
        // 00403c8b lea eax,[ecx+8]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0x8u32);
        // 00403c8e mov [ecx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403c90 lea eax,[edi+8]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x8u32);
        // 00403c93 jmp near ptr 00403D66h
        Cont(x00403d66)
    }
}

pub fn x00403c90() -> Cont {
    unsafe {
        // 00403c90 lea eax,[edi+8]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x8u32);
        // 00403c93 jmp near ptr 00403D66h
        Cont(x00403d66)
    }
}

pub fn x00403c98() -> Cont {
    unsafe {
        // 00403c98 add esi,edi
        MACHINE.regs.esi = add(MACHINE.regs.esi, MACHINE.regs.edi);
        // 00403c9a cmp byte ptr [esi],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403c9d je short 00403CA1h
        je(Cont(x00403c9f), Cont(x00403ca1))
    }
}

pub fn x00403c9f() -> Cont {
    unsafe {
        // 00403c9f mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00403ca1 lea esi,[eax+edx]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403ca4 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00403ca6 jae short 00403CEBh
        jae(Cont(x00403ca8), Cont(x00403ceb))
    }
}

pub fn x00403ca1() -> Cont {
    unsafe {
        // 00403ca1 lea esi,[eax+edx]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403ca4 cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00403ca6 jae short 00403CEBh
        jae(Cont(x00403ca8), Cont(x00403ceb))
    }
}

pub fn x00403ca8() -> Cont {
    unsafe {
        // 00403ca8 mov bl,[eax]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 00403caa test bl,bl
        and(MACHINE.regs.get_bl(), MACHINE.regs.get_bl());
        // 00403cac jne short 00403CDEh
        jne(Cont(x00403cae), Cont(x00403cde))
    }
}

pub fn x00403cae() -> Cont {
    unsafe {
        // 00403cae push 1
        push(0x1u32);
        // 00403cb0 lea ebx,[eax+1]
        MACHINE.regs.ebx = MACHINE.regs.eax.wrapping_add(0x1u32);
        // 00403cb3 pop esi
        MACHINE.regs.esi = pop();
        // 00403cb4 cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403cb7 jne short 00403CBDh
        jne(Cont(x00403cb9), Cont(x00403cbd))
    }
}

pub fn x00403cb4() -> Cont {
    unsafe {
        // 00403cb4 cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403cb7 jne short 00403CBDh
        jne(Cont(x00403cb9), Cont(x00403cbd))
    }
}

pub fn x00403cb9() -> Cont {
    unsafe {
        // 00403cb9 inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 00403cba inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00403cbb jmp short 00403CB4h
        Cont(x00403cb4)
    }
}

pub fn x00403cbd() -> Cont {
    unsafe {
        // 00403cbd cmp esi,edx
        sub(MACHINE.regs.esi, MACHINE.regs.edx);
        // 00403cbf jae short 00403D0Fh
        jae(Cont(x00403cc1), Cont(x00403d0f))
    }
}

pub fn x00403cc1() -> Cont {
    unsafe {
        // 00403cc1 cmp eax,[ebp-4]
        sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
        );
        // 00403cc4 jne short 00403CCBh
        jne(Cont(x00403cc6), Cont(x00403ccb))
    }
}

pub fn x00403cc6() -> Cont {
    unsafe {
        // 00403cc6 mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 00403cc9 jmp short 00403CD7h
        Cont(x00403cd7)
    }
}

pub fn x00403ccb() -> Cont {
    unsafe {
        // 00403ccb sub [ebp+0Ch],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xcu32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
                MACHINE.regs.esi,
            ),
        );
        // 00403cce cmp [ebp+0Ch],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
            MACHINE.regs.edx,
        );
        // 00403cd1 jb near ptr 00403D70h
        jb(Cont(x00403cd7), Cont(x00403d70))
    }
}

pub fn x00403cd7() -> Cont {
    unsafe {
        // 00403cd7 mov edi,[ebp-4]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403cda mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00403cdc jmp short 00403CE3h
        Cont(x00403ce3)
    }
}

pub fn x00403cde() -> Cont {
    unsafe {
        // 00403cde movzx esi,bl
        MACHINE.regs.esi = MACHINE.regs.get_bl() as _;
        // 00403ce1 add eax,esi
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403ce3 lea esi,[eax+edx]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403ce6 cmp esi,[ebp+8]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00403ce9 jb short 00403CA8h
        jb(Cont(x00403ceb), Cont(x00403ca8))
    }
}

pub fn x00403ce3() -> Cont {
    unsafe {
        // 00403ce3 lea esi,[eax+edx]
        MACHINE.regs.esi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403ce6 cmp esi,[ebp+8]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00403ce9 jb short 00403CA8h
        jb(Cont(x00403ceb), Cont(x00403ca8))
    }
}

pub fn x00403ceb() -> Cont {
    unsafe {
        // 00403ceb lea esi,[ecx+8]
        MACHINE.regs.esi = MACHINE.regs.ecx.wrapping_add(0x8u32);
        // 00403cee cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 00403cf0 jae short 00403D70h
        jae(Cont(x00403cf2), Cont(x00403d70))
    }
}

pub fn x00403cee() -> Cont {
    unsafe {
        // 00403cee cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 00403cf0 jae short 00403D70h
        jae(Cont(x00403cf2), Cont(x00403d70))
    }
}

pub fn x00403cf2() -> Cont {
    unsafe {
        // 00403cf2 lea eax,[esi+edx]
        MACHINE.regs.eax = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403cf5 cmp eax,[ebp+8]
        sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00403cf8 jae short 00403D70h
        jae(Cont(x00403cfa), Cont(x00403d70))
    }
}

pub fn x00403cfa() -> Cont {
    unsafe {
        // 00403cfa mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00403cfc test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00403cfe jne short 00403D40h
        jne(Cont(x00403d00), Cont(x00403d40))
    }
}

pub fn x00403d00() -> Cont {
    unsafe {
        // 00403d00 push 1
        push(0x1u32);
        // 00403d02 lea ebx,[esi+1]
        MACHINE.regs.ebx = MACHINE.regs.esi.wrapping_add(0x1u32);
        // 00403d05 pop eax
        MACHINE.regs.eax = pop();
        // 00403d06 cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403d09 jne short 00403D30h
        jne(Cont(x00403d0b), Cont(x00403d30))
    }
}

pub fn x00403d06() -> Cont {
    unsafe {
        // 00403d06 cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403d09 jne short 00403D30h
        jne(Cont(x00403d0b), Cont(x00403d30))
    }
}

pub fn x00403d0b() -> Cont {
    unsafe {
        // 00403d0b inc ebx
        MACHINE.regs.ebx = inc(MACHINE.regs.ebx);
        // 00403d0c inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00403d0d jmp short 00403D06h
        Cont(x00403d06)
    }
}

pub fn x00403d0f() -> Cont {
    unsafe {
        // 00403d0f lea ebx,[eax+edx]
        MACHINE.regs.ebx = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403d12 cmp ebx,[ebp+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00403d15 jae short 00403D20h
        jae(Cont(x00403d17), Cont(x00403d20))
    }
}

pub fn x00403d17() -> Cont {
    unsafe {
        // 00403d17 sub esi,edx
        MACHINE.regs.esi = sub(MACHINE.regs.esi, MACHINE.regs.edx);
        // 00403d19 mov [ecx],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.ebx);
        // 00403d1b mov [ecx+4],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.esi);
        // 00403d1e jmp short 00403D29h
        Cont(x00403d29)
    }
}

pub fn x00403d20() -> Cont {
    unsafe {
        // 00403d20 and dword ptr [ecx+4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                0x0u32,
            ),
        );
        // 00403d24 lea esi,[ecx+8]
        MACHINE.regs.esi = MACHINE.regs.ecx.wrapping_add(0x8u32);
        // 00403d27 mov [ecx],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.esi);
        // 00403d29 mov [eax],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00403d2b add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 00403d2e jmp short 00403D66h
        Cont(x00403d66)
    }
}

pub fn x00403d29() -> Cont {
    unsafe {
        // 00403d29 mov [eax],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00403d2b add eax,8
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x8u32);
        // 00403d2e jmp short 00403D66h
        Cont(x00403d66)
    }
}

pub fn x00403d30() -> Cont {
    unsafe {
        // 00403d30 cmp eax,edx
        sub(MACHINE.regs.eax, MACHINE.regs.edx);
        // 00403d32 jae short 00403D47h
        jae(Cont(x00403d34), Cont(x00403d47))
    }
}

pub fn x00403d34() -> Cont {
    unsafe {
        // 00403d34 sub [ebp+0Ch],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xcu32),
            sub(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
                MACHINE.regs.eax,
            ),
        );
        // 00403d37 cmp [ebp+0Ch],edx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
            MACHINE.regs.edx,
        );
        // 00403d3a jb short 00403D70h
        jb(Cont(x00403d3c), Cont(x00403d70))
    }
}

pub fn x00403d3c() -> Cont {
    unsafe {
        // 00403d3c mov esi,ebx
        MACHINE.regs.esi = MACHINE.regs.ebx;
        // 00403d3e jmp short 00403CEEh
        Cont(x00403cee)
    }
}

pub fn x00403d40() -> Cont {
    unsafe {
        // 00403d40 movzx eax,al
        MACHINE.regs.eax = MACHINE.regs.get_al() as _;
        // 00403d43 add esi,eax
        MACHINE.regs.esi = add(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00403d45 jmp short 00403CEEh
        Cont(x00403cee)
    }
}

pub fn x00403d47() -> Cont {
    unsafe {
        // 00403d47 lea ebx,[esi+edx]
        MACHINE.regs.ebx = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403d4a cmp ebx,[ebp+8]
        sub(
            MACHINE.regs.ebx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00403d4d jae short 00403D58h
        jae(Cont(x00403d4f), Cont(x00403d58))
    }
}

pub fn x00403d4f() -> Cont {
    unsafe {
        // 00403d4f sub eax,edx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.edx);
        // 00403d51 mov [ecx],ebx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.ebx);
        // 00403d53 mov [ecx+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 00403d56 jmp short 00403D61h
        Cont(x00403d61)
    }
}

pub fn x00403d58() -> Cont {
    unsafe {
        // 00403d58 and dword ptr [ecx+4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ecx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x4u32)),
                0x0u32,
            ),
        );
        // 00403d5c lea eax,[ecx+8]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0x8u32);
        // 00403d5f mov [ecx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403d61 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00403d63 lea eax,[esi+8]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x8u32);
        // 00403d66 imul ecx,0Fh
        imul();
        // 00403d69 shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00403d6c sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00403d6e jmp short 00403D72h
        Cont(x00403d72)
    }
}

pub fn x00403d61() -> Cont {
    unsafe {
        // 00403d61 mov [esi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00403d63 lea eax,[esi+8]
        MACHINE.regs.eax = MACHINE.regs.esi.wrapping_add(0x8u32);
        // 00403d66 imul ecx,0Fh
        imul();
        // 00403d69 shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00403d6c sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00403d6e jmp short 00403D72h
        Cont(x00403d72)
    }
}

pub fn x00403d66() -> Cont {
    unsafe {
        // 00403d66 imul ecx,0Fh
        imul();
        // 00403d69 shl eax,4
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x4u8);
        // 00403d6c sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00403d6e jmp short 00403D72h
        Cont(x00403d72)
    }
}

pub fn x00403d70() -> Cont {
    unsafe {
        // 00403d70 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403d72 pop edi
        MACHINE.regs.edi = pop();
        // 00403d73 pop esi
        MACHINE.regs.esi = pop();
        // 00403d74 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403d75 leave
        leave();
        // 00403d76 ret
        ret(0)
    }
}

pub fn x00403d72() -> Cont {
    unsafe {
        // 00403d72 pop edi
        MACHINE.regs.edi = pop();
        // 00403d73 pop esi
        MACHINE.regs.esi = pop();
        // 00403d74 pop ebx
        MACHINE.regs.ebx = pop();
        // 00403d75 leave
        leave();
        // 00403d76 ret
        ret(0)
    }
}

pub fn x00403d77() -> Cont {
    unsafe {
        // 00403d77 push ebp
        push(MACHINE.regs.ebp);
        // 00403d78 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403d7a push ecx
        push(MACHINE.regs.ecx);
        // 00403d7b mov edx,[ebp+10h]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403d7e push ebx
        push(MACHINE.regs.ebx);
        // 00403d7f mov ebx,[ebp+0Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00403d82 push esi
        push(MACHINE.regs.esi);
        // 00403d83 movzx ecx,byte ptr [edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32)) as _;
        // 00403d86 push edi
        push(MACHINE.regs.edi);
        // 00403d87 mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403d8a and dword ptr [ebp-4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0x0u32,
            ),
        );
        // 00403d8e mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00403d90 sub eax,[edi+10h]
        MACHINE.regs.eax = sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.edi.wrapping_add(0x10u32)),
        );
        // 00403d93 sar eax,0Ch
        sar();
        // 00403d96 cmp ecx,[ebp+14h]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00403d99 lea edi,[edi+eax*8+18h]
        MACHINE.regs.edi = MACHINE
            .regs
            .edi
            .wrapping_add((MACHINE.regs.eax * 8))
            .wrapping_add(0x18u32);
        // 00403d9d jbe short 00403DB1h
        jbe(Cont(x00403d9f), Cont(x00403db1))
    }
}

pub fn x00403d9f() -> Cont {
    unsafe {
        // 00403d9f mov eax,[ebp+14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00403da2 sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00403da4 mov [edx],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00403da6 add [edi],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.ecx,
            ),
        );
        // 00403da8 mov dword ptr [edi+4],0F1h
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x4u32), 0xf1u32);
        // 00403daf jmp short 00403E11h
        Cont(x00403e11)
    }
}

pub fn x00403db1() -> Cont {
    unsafe {
        // 00403db1 jae short 00403E18h
        jae(Cont(x00403db3), Cont(x00403e18))
    }
}

pub fn x00403db3() -> Cont {
    unsafe {
        // 00403db3 mov eax,[ebp+14h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00403db6 lea esi,[edx+eax]
        MACHINE.regs.esi = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x0u32);
        // 00403db9 lea eax,[ebx+0F8h]
        MACHINE.regs.eax = MACHINE.regs.ebx.wrapping_add(0xf8u32);
        // 00403dbf cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403dc1 jb short 00403E18h
        jb(Cont(x00403dc3), Cont(x00403e18))
    }
}

pub fn x00403dc3() -> Cont {
    unsafe {
        // 00403dc3 lea eax,[ecx+edx]
        MACHINE.regs.eax = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edx * 1))
            .wrapping_add(0x0u32);
        // 00403dc6 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403dc8 jae short 00403DD4h
        jae(Cont(x00403dca), Cont(x00403dd4))
    }
}

pub fn x00403dc6() -> Cont {
    unsafe {
        // 00403dc6 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403dc8 jae short 00403DD4h
        jae(Cont(x00403dca), Cont(x00403dd4))
    }
}

pub fn x00403dca() -> Cont {
    unsafe {
        // 00403dca cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00403dcd jne short 00403DD2h
        jne(Cont(x00403dcf), Cont(x00403dd2))
    }
}

pub fn x00403dcf() -> Cont {
    unsafe {
        // 00403dcf inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00403dd0 jmp short 00403DC6h
        Cont(x00403dc6)
    }
}

pub fn x00403dd2() -> Cont {
    unsafe {
        // 00403dd2 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403dd4 jne short 00403E18h
        jne(Cont(x00403dd6), Cont(x00403e18))
    }
}

pub fn x00403dd4() -> Cont {
    unsafe {
        // 00403dd4 jne short 00403E18h
        jne(Cont(x00403dd6), Cont(x00403e18))
    }
}

pub fn x00403dd6() -> Cont {
    unsafe {
        // 00403dd6 mov al,[ebp+14h]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00403dd9 mov [edx],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00403ddb mov eax,[ebx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32));
        // 00403ddd cmp edx,eax
        sub(MACHINE.regs.edx, MACHINE.regs.eax);
        // 00403ddf ja short 00403E0Ch
        ja(Cont(x00403de1), Cont(x00403e0c))
    }
}

pub fn x00403de1() -> Cont {
    unsafe {
        // 00403de1 cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00403de3 jbe short 00403E0Ch
        jbe(Cont(x00403de5), Cont(x00403e0c))
    }
}

pub fn x00403de5() -> Cont {
    unsafe {
        // 00403de5 lea eax,[ebx+0F8h]
        MACHINE.regs.eax = MACHINE.regs.ebx.wrapping_add(0xf8u32);
        // 00403deb cmp esi,eax
        sub(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00403ded jae short 00403E03h
        jae(Cont(x00403def), Cont(x00403e03))
    }
}

pub fn x00403def() -> Cont {
    unsafe {
        // 00403def xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403df1 mov [ebx],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.esi);
        // 00403df3 cmp [esi],al
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
            MACHINE.regs.get_al(),
        );
        // 00403df5 jne short 00403DFEh
        jne(Cont(x00403df7), Cont(x00403dfe))
    }
}

pub fn x00403df7() -> Cont {
    unsafe {
        // 00403df7 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00403df8 cmp byte ptr [esi+eax],0
        sub(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .esi
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0x0u32),
            ),
            0x0u8,
        );
        // 00403dfc je short 00403DF7h
        je(Cont(x00403dfe), Cont(x00403df7))
    }
}

pub fn x00403dfe() -> Cont {
    unsafe {
        // 00403dfe mov [ebx+4],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32), MACHINE.regs.eax);
        // 00403e01 jmp short 00403E0Ch
        Cont(x00403e0c)
    }
}

pub fn x00403e03() -> Cont {
    unsafe {
        // 00403e03 and dword ptr [ebx+4],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebx.wrapping_add(0x4u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebx.wrapping_add(0x4u32)),
                0x0u32,
            ),
        );
        // 00403e07 lea eax,[ebx+8]
        MACHINE.regs.eax = MACHINE.regs.ebx.wrapping_add(0x8u32);
        // 00403e0a mov [ebx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00403e0c sub ecx,[ebp+14h]
        MACHINE.regs.ecx = sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00403e0f add [edi],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.ecx,
            ),
        );
        // 00403e11 mov dword ptr [ebp-4],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32), 0x1u32);
        // 00403e18 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403e1b pop edi
        MACHINE.regs.edi = pop();
        // 00403e1c pop esi
        MACHINE.regs.esi = pop();
        // 00403e1d pop ebx
        MACHINE.regs.ebx = pop();
        // 00403e1e leave
        leave();
        // 00403e1f ret
        ret(0)
    }
}

pub fn x00403e0c() -> Cont {
    unsafe {
        // 00403e0c sub ecx,[ebp+14h]
        MACHINE.regs.ecx = sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00403e0f add [edi],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.edi.wrapping_add(0x0u32),
            add(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32)),
                MACHINE.regs.ecx,
            ),
        );
        // 00403e11 mov dword ptr [ebp-4],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32), 0x1u32);
        // 00403e18 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403e1b pop edi
        MACHINE.regs.edi = pop();
        // 00403e1c pop esi
        MACHINE.regs.esi = pop();
        // 00403e1d pop ebx
        MACHINE.regs.ebx = pop();
        // 00403e1e leave
        leave();
        // 00403e1f ret
        ret(0)
    }
}

pub fn x00403e11() -> Cont {
    unsafe {
        // 00403e11 mov dword ptr [ebp-4],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32), 0x1u32);
        // 00403e18 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403e1b pop edi
        MACHINE.regs.edi = pop();
        // 00403e1c pop esi
        MACHINE.regs.esi = pop();
        // 00403e1d pop ebx
        MACHINE.regs.ebx = pop();
        // 00403e1e leave
        leave();
        // 00403e1f ret
        ret(0)
    }
}

pub fn x00403e18() -> Cont {
    unsafe {
        // 00403e18 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00403e1b pop edi
        MACHINE.regs.edi = pop();
        // 00403e1c pop esi
        MACHINE.regs.esi = pop();
        // 00403e1d pop ebx
        MACHINE.regs.ebx = pop();
        // 00403e1e leave
        leave();
        // 00403e1f ret
        ret(0)
    }
}

pub fn x00403e20() -> Cont {
    unsafe {
        // 00403e20 mov eax,ds:[409708h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409708u32);
        // 00403e25 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403e27 je short 00403E38h
        je(Cont(x00403e29), Cont(x00403e38))
    }
}

pub fn x00403e29() -> Cont {
    unsafe {
        // 00403e29 push dword ptr [esp+4]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 00403e2d call eax
        call(0x403e2f, indirect(MACHINE.regs.eax))
    }
}

pub fn x00403e2f() -> Cont {
    unsafe {
        // 00403e2f test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00403e31 pop ecx
        MACHINE.regs.ecx = pop();
        // 00403e32 je short 00403E38h
        je(Cont(x00403e34), Cont(x00403e38))
    }
}

pub fn x00403e34() -> Cont {
    unsafe {
        // 00403e34 push 1
        push(0x1u32);
        // 00403e36 pop eax
        MACHINE.regs.eax = pop();
        // 00403e37 ret
        ret(0)
    }
}

pub fn x00403e38() -> Cont {
    unsafe {
        // 00403e38 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00403e3a ret
        ret(0)
    }
}

pub fn x00403e40() -> Cont {
    unsafe {
        // 00403e40 push ebp
        push(MACHINE.regs.ebp);
        // 00403e41 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00403e43 push edi
        push(MACHINE.regs.edi);
        // 00403e44 push esi
        push(MACHINE.regs.esi);
        // 00403e45 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00403e48 mov ecx,[ebp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00403e4b mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00403e4e mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00403e50 mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00403e52 add eax,esi
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00403e54 cmp edi,esi
        sub(MACHINE.regs.edi, MACHINE.regs.esi);
        // 00403e56 jbe short 00403E60h
        jbe(Cont(x00403e58), Cont(x00403e60))
    }
}

pub fn x00403e58() -> Cont {
    unsafe {
        // 00403e58 cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00403e5a jb near ptr 00403FD8h
        jb(Cont(x00403e60), Cont(x00403fd8))
    }
}

pub fn x00403e60() -> Cont {
    unsafe {
        // 00403e60 test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00403e66 jne short 00403E7Ch
        jne(Cont(x00403e68), Cont(x00403e7c))
    }
}

pub fn x00403e68() -> Cont {
    unsafe {
        // 00403e68 shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00403e6b and edx,3
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0x3u32);
        // 00403e6e cmp ecx,8
        sub(MACHINE.regs.ecx, 0x8u32);
        // 00403e71 jb short 00403E9Ch
        jb(Cont(x00403e73), Cont(x00403e9c))
    }
}

pub fn x00403e73() -> Cont {
    unsafe {
        // 00403e73 rep movsd
        movsd();
        // 00403e75 jmp dword ptr [edx*4+403F88h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.edx * 4).wrapping_add(0x403f88u32)),
        )
    }
}

pub fn x00403e7c() -> Cont {
    unsafe {
        // 00403e7c mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00403e7e mov edx,3
        MACHINE.regs.edx = 0x3u32;
        // 00403e83 sub ecx,4
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, 0x4u32);
        // 00403e86 jb short 00403E94h
        jb(Cont(x00403e88), Cont(x00403e94))
    }
}

pub fn x00403e88() -> Cont {
    unsafe {
        // 00403e88 and eax,3
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x3u32);
        // 00403e8b add ecx,eax
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00403e8d jmp dword ptr [eax*4+403EA0h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.eax * 4).wrapping_add(0x403ea0u32)),
        )
    }
}

pub fn x00403e94() -> Cont {
    unsafe {
        // 00403e94 jmp dword ptr [ecx*4+403F98h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x403f98u32)),
        )
    }
}

pub fn x00403e9c() -> Cont {
    unsafe {
        // 00403e9c jmp dword ptr [ecx*4+403F1Ch]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x403f1cu32)),
        )
    }
}

pub fn x00403fd8() -> Cont {
    unsafe {
        // 00403fd8 lea esi,[ecx+esi-4]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00403fdc lea edi,[ecx+edi-4]
        MACHINE.regs.edi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00403fe0 test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00403fe6 jne short 0040400Ch
        jne(Cont(x00403fe8), Cont(x0040400c))
    }
}

pub fn x00403fe8() -> Cont {
    unsafe {
        // 00403fe8 shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00403feb and edx,3
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0x3u32);
        // 00403fee cmp ecx,8
        sub(MACHINE.regs.ecx, 0x8u32);
        // 00403ff1 jb short 00404000h
        jb(Cont(x00403ff3), Cont(x00404000))
    }
}

pub fn x00403ff3() -> Cont {
    unsafe {
        // 00403ff3 std
        std();
        // 00403ff4 rep movsd
        movsd();
        // 00403ff6 cld
        cld();
        // 00403ff7 jmp dword ptr [edx*4+404120h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.edx * 4).wrapping_add(0x404120u32)),
        )
    }
}

pub fn x00404000() -> Cont {
    unsafe {
        // 00404000 neg ecx
        MACHINE.regs.ecx = neg(MACHINE.regs.ecx);
        // 00404002 jmp dword ptr [ecx*4+4040D0h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x4040d0u32)),
        )
    }
}

pub fn x0040400c() -> Cont {
    unsafe {
        // 0040400c mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 0040400e mov edx,3
        MACHINE.regs.edx = 0x3u32;
        // 00404013 cmp ecx,4
        sub(MACHINE.regs.ecx, 0x4u32);
        // 00404016 jb short 00404024h
        jb(Cont(x00404018), Cont(x00404024))
    }
}

pub fn x00404018() -> Cont {
    unsafe {
        // 00404018 and eax,3
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x3u32);
        // 0040401b sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 0040401d jmp dword ptr [eax*4+404028h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.eax * 4).wrapping_add(0x404028u32)),
        )
    }
}

pub fn x00404024() -> Cont {
    unsafe {
        // 00404024 jmp dword ptr [ecx*4+404120h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x404120u32)),
        )
    }
}

pub fn x00404175() -> Cont {
    unsafe {
        // 00404175 push 4
        push(0x4u32);
        // 00404177 push 0
        push(0x0u32);
        // 00404179 push dword ptr [esp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 0040417d call 00404186h
        call(0x404182, Cont(x00404186))
    }
}

pub fn x00404182() -> Cont {
    unsafe {
        // 00404182 add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 00404185 ret
        ret(0)
    }
}

pub fn x00404186() -> Cont {
    unsafe {
        // 00404186 movzx eax,byte ptr [esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.esp.wrapping_add(0x4u32)) as _;
        // 0040418b mov cl,[esp+0Ch]
        MACHINE.regs.set_cl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esp.wrapping_add(0xcu32)),
        );
        // 0040418f test [eax+409861h],cl
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
            MACHINE.regs.get_cl(),
        );
        // 00404195 jne short 004041B3h
        jne(Cont(x00404197), Cont(x004041b3))
    }
}

pub fn x00404197() -> Cont {
    unsafe {
        // 00404197 cmp dword ptr [esp+8],0
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
            0x0u32,
        );
        // 0040419c je short 004041ACh
        je(Cont(x0040419e), Cont(x004041ac))
    }
}

pub fn x0040419e() -> Cont {
    unsafe {
        // 0040419e movzx eax,word ptr [eax*2+40933Ah]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u16>((MACHINE.regs.eax * 2).wrapping_add(0x40933au32))
            as _;
        // 004041a6 and eax,[esp+8]
        MACHINE.regs.eax = and(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 004041aa jmp short 004041AEh
        Cont(x004041ae)
    }
}

pub fn x004041ac() -> Cont {
    unsafe {
        // 004041ac xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004041ae test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004041b0 jne short 004041B3h
        jne(Cont(x004041b2), Cont(x004041b3))
    }
}

pub fn x004041ae() -> Cont {
    unsafe {
        // 004041ae test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004041b0 jne short 004041B3h
        jne(Cont(x004041b2), Cont(x004041b3))
    }
}

pub fn x004041b2() -> Cont {
    unsafe {
        // 004041b2 ret
        ret(0)
    }
}

pub fn x004041b3() -> Cont {
    unsafe {
        // 004041b3 push 1
        push(0x1u32);
        // 004041b5 pop eax
        MACHINE.regs.eax = pop();
        // 004041b6 ret
        ret(0)
    }
}

pub fn x004041b7() -> Cont {
    unsafe {
        // 004041b7 push ebp
        push(MACHINE.regs.ebp);
        // 004041b8 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004041ba sub esp,18h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x18u32);
        // 004041bd push ebx
        push(MACHINE.regs.ebx);
        // 004041be push esi
        push(MACHINE.regs.esi);
        // 004041bf push edi
        push(MACHINE.regs.edi);
        // 004041c0 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 004041c3 call 00404350h
        call(0x4041c8, Cont(x00404350))
    }
}

pub fn x004041c8() -> Cont {
    unsafe {
        // 004041c8 mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 004041ca pop ecx
        MACHINE.regs.ecx = pop();
        // 004041cb cmp esi,ds:[409748h]
        sub(MACHINE.regs.esi, MACHINE.memory.read::<u32>(0x409748u32));
        // 004041d1 mov [ebp+8],esi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.esi);
        // 004041d4 je near ptr 00404344h
        je(Cont(x004041da), Cont(x00404344))
    }
}

pub fn x004041da() -> Cont {
    unsafe {
        // 004041da xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 004041dc cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 004041de je near ptr 0040433Ah
        je(Cont(x004041e4), Cont(x0040433a))
    }
}

pub fn x004041e4() -> Cont {
    unsafe {
        // 004041e4 xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 004041e6 mov eax,409238h
        MACHINE.regs.eax = 0x409238u32;
        // 004041eb cmp [eax],esi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.esi,
        );
        // 004041ed je short 00404261h
        je(Cont(x004041ef), Cont(x00404261))
    }
}

pub fn x004041eb() -> Cont {
    unsafe {
        // 004041eb cmp [eax],esi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            MACHINE.regs.esi,
        );
        // 004041ed je short 00404261h
        je(Cont(x004041ef), Cont(x00404261))
    }
}

pub fn x004041ef() -> Cont {
    unsafe {
        // 004041ef add eax,30h
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x30u32);
        // 004041f2 inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 004041f3 cmp eax,409328h
        sub(MACHINE.regs.eax, 0x409328u32);
        // 004041f8 jl short 004041EBh
        jl(Cont(x004041fa), Cont(x004041eb))
    }
}

pub fn x004041fa() -> Cont {
    unsafe {
        // 004041fa lea eax,[ebp-18h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffffe8u32);
        // 004041fd push eax
        push(MACHINE.regs.eax);
        // 004041fe push esi
        push(MACHINE.regs.esi);
        // 004041ff call dword ptr ds:[406058h]
        call(0x404205, Cont(kernel32::GetCPInfo_stdcall))
    }
}

pub fn x00404205() -> Cont {
    unsafe {
        // 00404205 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00404208 jne near ptr 00404332h
        jne(Cont(x0040420e), Cont(x00404332))
    }
}

pub fn x0040420e() -> Cont {
    unsafe {
        // 0040420e push 40h
        push(0x40u32);
        // 00404210 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404212 pop ecx
        MACHINE.regs.ecx = pop();
        // 00404213 mov edi,409860h
        MACHINE.regs.edi = 0x409860u32;
        // 00404218 cmp dword ptr [ebp-18h],1
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe8u32)),
            0x1u32,
        );
        // 0040421c mov ds:[409748h],esi
        MACHINE.memory.write::<u32>(0x409748u32, MACHINE.regs.esi);
        // 00404222 rep stosd
        stosd();
        // 00404224 stosb
        stosb();
        // 00404225 mov ds:[409964h],ebx
        MACHINE.memory.write::<u32>(0x409964u32, MACHINE.regs.ebx);
        // 0040422b jbe near ptr 00404320h
        jbe(Cont(x00404231), Cont(x00404320))
    }
}

pub fn x00404231() -> Cont {
    unsafe {
        // 00404231 cmp byte ptr [ebp-12h],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xffffffeeu32)),
            0x0u8,
        );
        // 00404235 je near ptr 004042F6h
        je(Cont(x0040423b), Cont(x004042f6))
    }
}

pub fn x0040423b() -> Cont {
    unsafe {
        // 0040423b lea ecx,[ebp-11h]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xffffffefu32);
        // 0040423e mov dl,[ecx]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 00404240 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404242 je near ptr 004042F6h
        je(Cont(x00404248), Cont(x004042f6))
    }
}

pub fn x0040423e() -> Cont {
    unsafe {
        // 0040423e mov dl,[ecx]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 00404240 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404242 je near ptr 004042F6h
        je(Cont(x00404248), Cont(x004042f6))
    }
}

pub fn x00404248() -> Cont {
    unsafe {
        // 00404248 movzx eax,byte ptr [ecx-1]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ecx.wrapping_add(0xffffffffu32))
            as _;
        // 0040424c movzx edx,dl
        MACHINE.regs.edx = MACHINE.regs.get_dl() as _;
        // 0040424f cmp eax,edx
        sub(MACHINE.regs.eax, MACHINE.regs.edx);
        // 00404251 ja near ptr 004042EAh
        ja(Cont(x00404257), Cont(x004042ea))
    }
}

pub fn x0040424f() -> Cont {
    unsafe {
        // 0040424f cmp eax,edx
        sub(MACHINE.regs.eax, MACHINE.regs.edx);
        // 00404251 ja near ptr 004042EAh
        ja(Cont(x00404257), Cont(x004042ea))
    }
}

pub fn x00404257() -> Cont {
    unsafe {
        // 00404257 or byte ptr [eax+409861h],4
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x4u8,
            ),
        );
        // 0040425e inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 0040425f jmp short 0040424Fh
        Cont(x0040424f)
    }
}

pub fn x00404261() -> Cont {
    unsafe {
        // 00404261 push 40h
        push(0x40u32);
        // 00404263 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404265 pop ecx
        MACHINE.regs.ecx = pop();
        // 00404266 mov edi,409860h
        MACHINE.regs.edi = 0x409860u32;
        // 0040426b rep stosd
        stosd();
        // 0040426d lea esi,[edx+edx*2]
        MACHINE.regs.esi = MACHINE
            .regs
            .edx
            .wrapping_add((MACHINE.regs.edx * 2))
            .wrapping_add(0x0u32);
        // 00404270 mov [ebp-4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ebx,
        );
        // 00404273 shl esi,4
        MACHINE.regs.esi = shl(MACHINE.regs.esi, 0x4u8);
        // 00404276 stosb
        stosb();
        // 00404277 lea ebx,[esi+409248h]
        MACHINE.regs.ebx = MACHINE.regs.esi.wrapping_add(0x409248u32);
        // 0040427d cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00404280 mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00404282 je short 004042B0h
        je(Cont(x00404284), Cont(x004042b0))
    }
}

pub fn x0040427d() -> Cont {
    unsafe {
        // 0040427d cmp byte ptr [ebx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 00404280 mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00404282 je short 004042B0h
        je(Cont(x00404284), Cont(x004042b0))
    }
}

pub fn x00404284() -> Cont {
    unsafe {
        // 00404284 mov dl,[ecx+1]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x1u32)),
        );
        // 00404287 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404289 je short 004042B0h
        je(Cont(x0040428b), Cont(x004042b0))
    }
}

pub fn x0040428b() -> Cont {
    unsafe {
        // 0040428b movzx eax,byte ptr [ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)) as _;
        // 0040428e movzx edi,dl
        MACHINE.regs.edi = MACHINE.regs.get_dl() as _;
        // 00404291 cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 00404293 ja short 004042A9h
        ja(Cont(x00404295), Cont(x004042a9))
    }
}

pub fn x00404295() -> Cont {
    unsafe {
        // 00404295 mov edx,[ebp-4]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00404298 mov dl,[edx+409230h]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0x409230u32)),
        );
        // 0040429e or [eax+409861h],dl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                MACHINE.regs.get_dl(),
            ),
        );
        // 004042a4 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004042a5 cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 004042a7 jbe short 0040429Eh
        jbe(Cont(x004042a9), Cont(x0040429e))
    }
}

pub fn x0040429e() -> Cont {
    unsafe {
        // 0040429e or [eax+409861h],dl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                MACHINE.regs.get_dl(),
            ),
        );
        // 004042a4 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004042a5 cmp eax,edi
        sub(MACHINE.regs.eax, MACHINE.regs.edi);
        // 004042a7 jbe short 0040429Eh
        jbe(Cont(x004042a9), Cont(x0040429e))
    }
}

pub fn x004042a9() -> Cont {
    unsafe {
        // 004042a9 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004042aa inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004042ab cmp byte ptr [ecx],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 004042ae jne short 00404284h
        jne(Cont(x004042b0), Cont(x00404284))
    }
}

pub fn x004042b0() -> Cont {
    unsafe {
        // 004042b0 inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 004042b3 add ebx,8
        MACHINE.regs.ebx = add(MACHINE.regs.ebx, 0x8u32);
        // 004042b6 cmp dword ptr [ebp-4],4
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
            0x4u32,
        );
        // 004042ba jb short 0040427Dh
        jb(Cont(x004042bc), Cont(x0040427d))
    }
}

pub fn x004042bc() -> Cont {
    unsafe {
        // 004042bc mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004042bf mov dword ptr ds:[40975Ch],1
        MACHINE.memory.write::<u32>(0x40975cu32, 0x1u32);
        // 004042c9 push eax
        push(MACHINE.regs.eax);
        // 004042ca mov ds:[409748h],eax
        MACHINE.memory.write::<u32>(0x409748u32, MACHINE.regs.eax);
        // 004042cf call 0040439Ah
        call(0x4042d4, Cont(x0040439a))
    }
}

pub fn x004042d4() -> Cont {
    unsafe {
        // 004042d4 lea esi,[esi+40923Ch]
        MACHINE.regs.esi = MACHINE.regs.esi.wrapping_add(0x40923cu32);
        // 004042da mov edi,409750h
        MACHINE.regs.edi = 0x409750u32;
        // 004042df movsd
        movsd();
        // 004042e0 movsd
        movsd();
        // 004042e1 pop ecx
        MACHINE.regs.ecx = pop();
        // 004042e2 mov ds:[409964h],eax
        MACHINE.memory.write::<u32>(0x409964u32, MACHINE.regs.eax);
        // 004042e7 movsd
        movsd();
        // 004042e8 jmp short 0040433Fh
        Cont(x0040433f)
    }
}

pub fn x004042ea() -> Cont {
    unsafe {
        // 004042ea inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004042eb inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004042ec cmp byte ptr [ecx-1],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0xffffffffu32)),
            0x0u8,
        );
        // 004042f0 jne near ptr 0040423Eh
        jne(Cont(x004042f6), Cont(x0040423e))
    }
}

pub fn x004042f6() -> Cont {
    unsafe {
        // 004042f6 push 1
        push(0x1u32);
        // 004042f8 pop eax
        MACHINE.regs.eax = pop();
        // 004042f9 or byte ptr [eax+409861h],8
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x8u8,
            ),
        );
        // 00404300 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404301 cmp eax,0FFh
        sub(MACHINE.regs.eax, 0xffu32);
        // 00404306 jb short 004042F9h
        jb(Cont(x00404308), Cont(x004042f9))
    }
}

pub fn x004042f9() -> Cont {
    unsafe {
        // 004042f9 or byte ptr [eax+409861h],8
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x8u8,
            ),
        );
        // 00404300 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404301 cmp eax,0FFh
        sub(MACHINE.regs.eax, 0xffu32);
        // 00404306 jb short 004042F9h
        jb(Cont(x00404308), Cont(x004042f9))
    }
}

pub fn x00404308() -> Cont {
    unsafe {
        // 00404308 push esi
        push(MACHINE.regs.esi);
        // 00404309 call 0040439Ah
        call(0x40430e, Cont(x0040439a))
    }
}

pub fn x0040430e() -> Cont {
    unsafe {
        // 0040430e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040430f mov ds:[409964h],eax
        MACHINE.memory.write::<u32>(0x409964u32, MACHINE.regs.eax);
        // 00404314 mov dword ptr ds:[40975Ch],1
        MACHINE.memory.write::<u32>(0x40975cu32, 0x1u32);
        // 0040431e jmp short 00404326h
        Cont(x00404326)
    }
}

pub fn x00404320() -> Cont {
    unsafe {
        // 00404320 mov ds:[40975Ch],ebx
        MACHINE.memory.write::<u32>(0x40975cu32, MACHINE.regs.ebx);
        // 00404326 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404328 mov edi,409750h
        MACHINE.regs.edi = 0x409750u32;
        // 0040432d stosd
        stosd();
        // 0040432e stosd
        stosd();
        // 0040432f stosd
        stosd();
        // 00404330 jmp short 0040433Fh
        Cont(x0040433f)
    }
}

pub fn x00404326() -> Cont {
    unsafe {
        // 00404326 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404328 mov edi,409750h
        MACHINE.regs.edi = 0x409750u32;
        // 0040432d stosd
        stosd();
        // 0040432e stosd
        stosd();
        // 0040432f stosd
        stosd();
        // 00404330 jmp short 0040433Fh
        Cont(x0040433f)
    }
}

pub fn x00404332() -> Cont {
    unsafe {
        // 00404332 cmp ds:[409710h],ebx
        sub(MACHINE.memory.read::<u32>(0x409710u32), MACHINE.regs.ebx);
        // 00404338 je short 00404348h
        je(Cont(x0040433a), Cont(x00404348))
    }
}

pub fn x0040433a() -> Cont {
    unsafe {
        // 0040433a call 004043CDh
        call(0x40433f, Cont(x004043cd))
    }
}

pub fn x0040433f() -> Cont {
    unsafe {
        // 0040433f call 004043F6h
        call(0x404344, Cont(x004043f6))
    }
}

pub fn x00404344() -> Cont {
    unsafe {
        // 00404344 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404346 jmp short 0040434Bh
        Cont(x0040434b)
    }
}

pub fn x00404348() -> Cont {
    unsafe {
        // 00404348 or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 0040434b pop edi
        MACHINE.regs.edi = pop();
        // 0040434c pop esi
        MACHINE.regs.esi = pop();
        // 0040434d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040434e leave
        leave();
        // 0040434f ret
        ret(0)
    }
}

pub fn x0040434b() -> Cont {
    unsafe {
        // 0040434b pop edi
        MACHINE.regs.edi = pop();
        // 0040434c pop esi
        MACHINE.regs.esi = pop();
        // 0040434d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040434e leave
        leave();
        // 0040434f ret
        ret(0)
    }
}

pub fn x00404350() -> Cont {
    unsafe {
        // 00404350 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00404354 and dword ptr ds:[409710h],0
        MACHINE.memory.write::<u32>(
            0x409710u32,
            and(MACHINE.memory.read::<u32>(0x409710u32), 0x0u32),
        );
        // 0040435b cmp eax,0FFFFFFFEh
        sub(MACHINE.regs.eax, 0xfffffffeu32);
        // 0040435e jne short 00404370h
        jne(Cont(x00404360), Cont(x00404370))
    }
}

pub fn x00404360() -> Cont {
    unsafe {
        // 00404360 mov dword ptr ds:[409710h],1
        MACHINE.memory.write::<u32>(0x409710u32, 0x1u32);
        // 0040436a jmp dword ptr ds:[406050h]
        Cont(kernel32::GetOEMCP_stdcall)
    }
}

pub fn x00404370() -> Cont {
    unsafe {
        // 00404370 cmp eax,0FFFFFFFDh
        sub(MACHINE.regs.eax, 0xfffffffdu32);
        // 00404373 jne short 00404385h
        jne(Cont(x00404375), Cont(x00404385))
    }
}

pub fn x00404375() -> Cont {
    unsafe {
        // 00404375 mov dword ptr ds:[409710h],1
        MACHINE.memory.write::<u32>(0x409710u32, 0x1u32);
        // 0040437f jmp dword ptr ds:[406054h]
        Cont(kernel32::GetACP_stdcall)
    }
}

pub fn x00404385() -> Cont {
    unsafe {
        // 00404385 cmp eax,0FFFFFFFCh
        sub(MACHINE.regs.eax, 0xfffffffcu32);
        // 00404388 jne short 00404399h
        jne(Cont(x0040438a), Cont(x00404399))
    }
}

pub fn x0040438a() -> Cont {
    unsafe {
        // 0040438a mov eax,ds:[409738h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409738u32);
        // 0040438f mov dword ptr ds:[409710h],1
        MACHINE.memory.write::<u32>(0x409710u32, 0x1u32);
        // 00404399 ret
        ret(0)
    }
}

pub fn x00404399() -> Cont {
    unsafe {
        // 00404399 ret
        ret(0)
    }
}

pub fn x0040439a() -> Cont {
    unsafe {
        // 0040439a mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 0040439e sub eax,3A4h
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x3a4u32);
        // 004043a3 je short 004043C7h
        je(Cont(x004043a5), Cont(x004043c7))
    }
}

pub fn x004043a5() -> Cont {
    unsafe {
        // 004043a5 sub eax,4
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x4u32);
        // 004043a8 je short 004043C1h
        je(Cont(x004043aa), Cont(x004043c1))
    }
}

pub fn x004043aa() -> Cont {
    unsafe {
        // 004043aa sub eax,0Dh
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0xdu32);
        // 004043ad je short 004043BBh
        je(Cont(x004043af), Cont(x004043bb))
    }
}

pub fn x004043af() -> Cont {
    unsafe {
        // 004043af dec eax
        MACHINE.regs.eax = dec(MACHINE.regs.eax);
        // 004043b0 je short 004043B5h
        je(Cont(x004043b2), Cont(x004043b5))
    }
}

pub fn x004043b2() -> Cont {
    unsafe {
        // 004043b2 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004043b4 ret
        ret(0)
    }
}

pub fn x004043b5() -> Cont {
    unsafe {
        // 004043b5 mov eax,404h
        MACHINE.regs.eax = 0x404u32;
        // 004043ba ret
        ret(0)
    }
}

pub fn x004043bb() -> Cont {
    unsafe {
        // 004043bb mov eax,412h
        MACHINE.regs.eax = 0x412u32;
        // 004043c0 ret
        ret(0)
    }
}

pub fn x004043c1() -> Cont {
    unsafe {
        // 004043c1 mov eax,804h
        MACHINE.regs.eax = 0x804u32;
        // 004043c6 ret
        ret(0)
    }
}

pub fn x004043c7() -> Cont {
    unsafe {
        // 004043c7 mov eax,411h
        MACHINE.regs.eax = 0x411u32;
        // 004043cc ret
        ret(0)
    }
}

pub fn x004043cd() -> Cont {
    unsafe {
        // 004043cd push edi
        push(MACHINE.regs.edi);
        // 004043ce push 40h
        push(0x40u32);
        // 004043d0 pop ecx
        MACHINE.regs.ecx = pop();
        // 004043d1 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004043d3 mov edi,409860h
        MACHINE.regs.edi = 0x409860u32;
        // 004043d8 rep stosd
        stosd();
        // 004043da stosb
        stosb();
        // 004043db xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004043dd mov edi,409750h
        MACHINE.regs.edi = 0x409750u32;
        // 004043e2 mov ds:[409748h],eax
        MACHINE.memory.write::<u32>(0x409748u32, MACHINE.regs.eax);
        // 004043e7 mov ds:[40975Ch],eax
        MACHINE.memory.write::<u32>(0x40975cu32, MACHINE.regs.eax);
        // 004043ec mov ds:[409964h],eax
        MACHINE.memory.write::<u32>(0x409964u32, MACHINE.regs.eax);
        // 004043f1 stosd
        stosd();
        // 004043f2 stosd
        stosd();
        // 004043f3 stosd
        stosd();
        // 004043f4 pop edi
        MACHINE.regs.edi = pop();
        // 004043f5 ret
        ret(0)
    }
}

pub fn x004043f6() -> Cont {
    unsafe {
        // 004043f6 push ebp
        push(MACHINE.regs.ebp);
        // 004043f7 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004043f9 sub esp,514h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x514u32);
        // 004043ff lea eax,[ebp-14h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffffecu32);
        // 00404402 push esi
        push(MACHINE.regs.esi);
        // 00404403 push eax
        push(MACHINE.regs.eax);
        // 00404404 push dword ptr ds:[409748h]
        push(MACHINE.memory.read::<u32>(0x409748u32));
        // 0040440a call dword ptr ds:[406058h]
        call(0x404410, Cont(kernel32::GetCPInfo_stdcall))
    }
}

pub fn x00404410() -> Cont {
    unsafe {
        // 00404410 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00404413 jne near ptr 0040452Fh
        jne(Cont(x00404419), Cont(x0040452f))
    }
}

pub fn x00404419() -> Cont {
    unsafe {
        // 00404419 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 0040441b mov esi,100h
        MACHINE.regs.esi = 0x100u32;
        // 00404420 mov [ebp+eax-114h],al
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .ebp
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffeecu32),
            MACHINE.regs.get_al(),
        );
        // 00404427 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404428 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 0040442a jb short 00404420h
        jb(Cont(x0040442c), Cont(x00404420))
    }
}

pub fn x00404420() -> Cont {
    unsafe {
        // 00404420 mov [ebp+eax-114h],al
        MACHINE.memory.write::<u8>(
            MACHINE
                .regs
                .ebp
                .wrapping_add((MACHINE.regs.eax * 1))
                .wrapping_add(0xfffffeecu32),
            MACHINE.regs.get_al(),
        );
        // 00404427 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404428 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 0040442a jb short 00404420h
        jb(Cont(x0040442c), Cont(x00404420))
    }
}

pub fn x0040442c() -> Cont {
    unsafe {
        // 0040442c mov al,[ebp-0Eh]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffff2u32)),
        );
        // 0040442f mov byte ptr [ebp-114h],20h
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffeecu32), 0x20u8);
        // 00404436 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404438 je short 00404471h
        je(Cont(x0040443a), Cont(x00404471))
    }
}

pub fn x0040443a() -> Cont {
    unsafe {
        // 0040443a push ebx
        push(MACHINE.regs.ebx);
        // 0040443b push edi
        push(MACHINE.regs.edi);
        // 0040443c lea edx,[ebp-0Dh]
        MACHINE.regs.edx = MACHINE.regs.ebp.wrapping_add(0xfffffff3u32);
        // 0040443f movzx ecx,byte ptr [edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32)) as _;
        // 00404442 movzx eax,al
        MACHINE.regs.eax = MACHINE.regs.get_al() as _;
        // 00404445 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00404447 ja short 00404466h
        ja(Cont(x00404449), Cont(x00404466))
    }
}

pub fn x0040443f() -> Cont {
    unsafe {
        // 0040443f movzx ecx,byte ptr [edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32)) as _;
        // 00404442 movzx eax,al
        MACHINE.regs.eax = MACHINE.regs.get_al() as _;
        // 00404445 cmp eax,ecx
        sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00404447 ja short 00404466h
        ja(Cont(x00404449), Cont(x00404466))
    }
}

pub fn x00404449() -> Cont {
    unsafe {
        // 00404449 sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 0040444b lea edi,[ebp+eax-114h]
        MACHINE.regs.edi = MACHINE
            .regs
            .ebp
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0xfffffeecu32);
        // 00404452 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404453 mov eax,20202020h
        MACHINE.regs.eax = 0x20202020u32;
        // 00404458 mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 0040445a shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 0040445d rep stosd
        stosd();
        // 0040445f mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00404461 and ecx,3
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x3u32);
        // 00404464 rep stosb
        rep(Rep::REP, stosb);
        // 00404466 inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 00404467 inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 00404468 mov al,[edx-1]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0xffffffffu32)),
        );
        // 0040446b test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 0040446d jne short 0040443Fh
        jne(Cont(x0040446f), Cont(x0040443f))
    }
}

pub fn x00404466() -> Cont {
    unsafe {
        // 00404466 inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 00404467 inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 00404468 mov al,[edx-1]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0xffffffffu32)),
        );
        // 0040446b test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 0040446d jne short 0040443Fh
        jne(Cont(x0040446f), Cont(x0040443f))
    }
}

pub fn x0040446f() -> Cont {
    unsafe {
        // 0040446f pop edi
        MACHINE.regs.edi = pop();
        // 00404470 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404471 push 0
        push(0x0u32);
        // 00404473 lea eax,[ebp-514h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffaecu32);
        // 00404479 push dword ptr ds:[409964h]
        push(MACHINE.memory.read::<u32>(0x409964u32));
        // 0040447f push dword ptr ds:[409748h]
        push(MACHINE.memory.read::<u32>(0x409748u32));
        // 00404485 push eax
        push(MACHINE.regs.eax);
        // 00404486 lea eax,[ebp-114h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffeecu32);
        // 0040448c push esi
        push(MACHINE.regs.esi);
        // 0040448d push eax
        push(MACHINE.regs.eax);
        // 0040448e push 1
        push(0x1u32);
        // 00404490 call 00405267h
        call(0x404495, Cont(x00405267))
    }
}

pub fn x00404471() -> Cont {
    unsafe {
        // 00404471 push 0
        push(0x0u32);
        // 00404473 lea eax,[ebp-514h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffaecu32);
        // 00404479 push dword ptr ds:[409964h]
        push(MACHINE.memory.read::<u32>(0x409964u32));
        // 0040447f push dword ptr ds:[409748h]
        push(MACHINE.memory.read::<u32>(0x409748u32));
        // 00404485 push eax
        push(MACHINE.regs.eax);
        // 00404486 lea eax,[ebp-114h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffeecu32);
        // 0040448c push esi
        push(MACHINE.regs.esi);
        // 0040448d push eax
        push(MACHINE.regs.eax);
        // 0040448e push 1
        push(0x1u32);
        // 00404490 call 00405267h
        call(0x404495, Cont(x00405267))
    }
}

pub fn x00404495() -> Cont {
    unsafe {
        // 00404495 push 0
        push(0x0u32);
        // 00404497 lea eax,[ebp-214h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffdecu32);
        // 0040449d push dword ptr ds:[409748h]
        push(MACHINE.memory.read::<u32>(0x409748u32));
        // 004044a3 push esi
        push(MACHINE.regs.esi);
        // 004044a4 push eax
        push(MACHINE.regs.eax);
        // 004044a5 lea eax,[ebp-114h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffeecu32);
        // 004044ab push esi
        push(MACHINE.regs.esi);
        // 004044ac push eax
        push(MACHINE.regs.eax);
        // 004044ad push esi
        push(MACHINE.regs.esi);
        // 004044ae push dword ptr ds:[409964h]
        push(MACHINE.memory.read::<u32>(0x409964u32));
        // 004044b4 call 00405018h
        call(0x4044b9, Cont(x00405018))
    }
}

pub fn x004044b9() -> Cont {
    unsafe {
        // 004044b9 push 0
        push(0x0u32);
        // 004044bb lea eax,[ebp-314h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffcecu32);
        // 004044c1 push dword ptr ds:[409748h]
        push(MACHINE.memory.read::<u32>(0x409748u32));
        // 004044c7 push esi
        push(MACHINE.regs.esi);
        // 004044c8 push eax
        push(MACHINE.regs.eax);
        // 004044c9 lea eax,[ebp-114h]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffeecu32);
        // 004044cf push esi
        push(MACHINE.regs.esi);
        // 004044d0 push eax
        push(MACHINE.regs.eax);
        // 004044d1 push 200h
        push(0x200u32);
        // 004044d6 push dword ptr ds:[409964h]
        push(MACHINE.memory.read::<u32>(0x409964u32));
        // 004044dc call 00405018h
        call(0x4044e1, Cont(x00405018))
    }
}

pub fn x004044e1() -> Cont {
    unsafe {
        // 004044e1 add esp,5Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x5cu32);
        // 004044e4 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004044e6 lea ecx,[ebp-514h]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xfffffaecu32);
        // 004044ec mov dx,[ecx]
        MACHINE.regs.set_dx(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 004044ef test dl,1
        and(MACHINE.regs.get_dl(), 0x1u8);
        // 004044f2 je short 0040450Ah
        je(Cont(x004044f4), Cont(x0040450a))
    }
}

pub fn x004044ec() -> Cont {
    unsafe {
        // 004044ec mov dx,[ecx]
        MACHINE.regs.set_dx(
            MACHINE
                .memory
                .read::<u16>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 004044ef test dl,1
        and(MACHINE.regs.get_dl(), 0x1u8);
        // 004044f2 je short 0040450Ah
        je(Cont(x004044f4), Cont(x0040450a))
    }
}

pub fn x004044f4() -> Cont {
    unsafe {
        // 004044f4 or byte ptr [eax+409861h],10h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x10u8,
            ),
        );
        // 004044fb mov dl,[ebp+eax-214h]
        MACHINE.regs.set_dl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .ebp
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0xfffffdecu32),
            ),
        );
        // 00404502 mov [eax+409760h],dl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            MACHINE.regs.get_dl(),
        );
        // 00404508 jmp short 00404526h
        Cont(x00404526)
    }
}

pub fn x00404502() -> Cont {
    unsafe {
        // 00404502 mov [eax+409760h],dl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            MACHINE.regs.get_dl(),
        );
        // 00404508 jmp short 00404526h
        Cont(x00404526)
    }
}

pub fn x0040450a() -> Cont {
    unsafe {
        // 0040450a test dl,2
        and(MACHINE.regs.get_dl(), 0x2u8);
        // 0040450d je short 0040451Fh
        je(Cont(x0040450f), Cont(x0040451f))
    }
}

pub fn x0040450f() -> Cont {
    unsafe {
        // 0040450f or byte ptr [eax+409861h],20h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x20u8,
            ),
        );
        // 00404516 mov dl,[ebp+eax-314h]
        MACHINE.regs.set_dl(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .ebp
                    .wrapping_add((MACHINE.regs.eax * 1))
                    .wrapping_add(0xfffffcecu32),
            ),
        );
        // 0040451d jmp short 00404502h
        Cont(x00404502)
    }
}

pub fn x0040451f() -> Cont {
    unsafe {
        // 0040451f and byte ptr [eax+409760h],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409760u32)),
                0x0u8,
            ),
        );
        // 00404526 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404527 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404528 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404529 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 0040452b jb short 004044ECh
        jb(Cont(x0040452d), Cont(x004044ec))
    }
}

pub fn x00404526() -> Cont {
    unsafe {
        // 00404526 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404527 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404528 inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404529 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 0040452b jb short 004044ECh
        jb(Cont(x0040452d), Cont(x004044ec))
    }
}

pub fn x0040452d() -> Cont {
    unsafe {
        // 0040452d jmp short 00404578h
        Cont(x00404578)
    }
}

pub fn x0040452f() -> Cont {
    unsafe {
        // 0040452f xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404531 mov esi,100h
        MACHINE.regs.esi = 0x100u32;
        // 00404536 cmp eax,41h
        sub(MACHINE.regs.eax, 0x41u32);
        // 00404539 jb short 00404554h
        jb(Cont(x0040453b), Cont(x00404554))
    }
}

pub fn x00404536() -> Cont {
    unsafe {
        // 00404536 cmp eax,41h
        sub(MACHINE.regs.eax, 0x41u32);
        // 00404539 jb short 00404554h
        jb(Cont(x0040453b), Cont(x00404554))
    }
}

pub fn x0040453b() -> Cont {
    unsafe {
        // 0040453b cmp eax,5Ah
        sub(MACHINE.regs.eax, 0x5au32);
        // 0040453e ja short 00404554h
        ja(Cont(x00404540), Cont(x00404554))
    }
}

pub fn x00404540() -> Cont {
    unsafe {
        // 00404540 or byte ptr [eax+409861h],10h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x10u8,
            ),
        );
        // 00404547 mov cl,al
        MACHINE.regs.set_cl(MACHINE.regs.get_al());
        // 00404549 add cl,20h
        MACHINE.regs.set_cl(add(MACHINE.regs.get_cl(), 0x20u8));
        // 0040454c mov [eax+409760h],cl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            MACHINE.regs.get_cl(),
        );
        // 00404552 jmp short 00404573h
        Cont(x00404573)
    }
}

pub fn x0040454c() -> Cont {
    unsafe {
        // 0040454c mov [eax+409760h],cl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            MACHINE.regs.get_cl(),
        );
        // 00404552 jmp short 00404573h
        Cont(x00404573)
    }
}

pub fn x00404554() -> Cont {
    unsafe {
        // 00404554 cmp eax,61h
        sub(MACHINE.regs.eax, 0x61u32);
        // 00404557 jb short 0040456Ch
        jb(Cont(x00404559), Cont(x0040456c))
    }
}

pub fn x00404559() -> Cont {
    unsafe {
        // 00404559 cmp eax,7Ah
        sub(MACHINE.regs.eax, 0x7au32);
        // 0040455c ja short 0040456Ch
        ja(Cont(x0040455e), Cont(x0040456c))
    }
}

pub fn x0040455e() -> Cont {
    unsafe {
        // 0040455e or byte ptr [eax+409861h],20h
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409861u32),
            or(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409861u32)),
                0x20u8,
            ),
        );
        // 00404565 mov cl,al
        MACHINE.regs.set_cl(MACHINE.regs.get_al());
        // 00404567 sub cl,20h
        MACHINE.regs.set_cl(sub(MACHINE.regs.get_cl(), 0x20u8));
        // 0040456a jmp short 0040454Ch
        Cont(x0040454c)
    }
}

pub fn x0040456c() -> Cont {
    unsafe {
        // 0040456c and byte ptr [eax+409760h],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.eax.wrapping_add(0x409760u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.eax.wrapping_add(0x409760u32)),
                0x0u8,
            ),
        );
        // 00404573 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404574 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00404576 jb short 00404536h
        jb(Cont(x00404578), Cont(x00404536))
    }
}

pub fn x00404573() -> Cont {
    unsafe {
        // 00404573 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00404574 cmp eax,esi
        sub(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00404576 jb short 00404536h
        jb(Cont(x00404578), Cont(x00404536))
    }
}

pub fn x00404578() -> Cont {
    unsafe {
        // 00404578 pop esi
        MACHINE.regs.esi = pop();
        // 00404579 leave
        leave();
        // 0040457a ret
        ret(0)
    }
}

pub fn x0040457b() -> Cont {
    unsafe {
        // 0040457b cmp dword ptr ds:[409AA8h],0
        sub(MACHINE.memory.read::<u32>(0x409aa8u32), 0x0u32);
        // 00404582 jne short 00404596h
        jne(Cont(x00404584), Cont(x00404596))
    }
}

pub fn x00404584() -> Cont {
    unsafe {
        // 00404584 push 0FFFFFFFDh
        push(0xfffffffdu32);
        // 00404586 call 004041B7h
        call(0x40458b, Cont(x004041b7))
    }
}

pub fn x0040458b() -> Cont {
    unsafe {
        // 0040458b pop ecx
        MACHINE.regs.ecx = pop();
        // 0040458c mov dword ptr ds:[409AA8h],1
        MACHINE.memory.write::<u32>(0x409aa8u32, 0x1u32);
        // 00404596 ret
        ret(0)
    }
}

pub fn x00404596() -> Cont {
    unsafe {
        // 00404596 ret
        ret(0)
    }
}

pub fn x004045a0() -> Cont {
    unsafe {
        // 004045a0 push edi
        push(MACHINE.regs.edi);
        // 004045a1 mov edi,[esp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 004045a5 jmp short 00404611h
        Cont(x00404611)
    }
}

pub fn x004045b0() -> Cont {
    unsafe {
        // 004045b0 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 004045b4 push edi
        push(MACHINE.regs.edi);
        // 004045b5 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 004045bb je short 004045CCh
        je(Cont(x004045bd), Cont(x004045cc))
    }
}

pub fn x004045bd() -> Cont {
    unsafe {
        // 004045bd mov al,[ecx]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 004045bf inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 004045c0 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004045c2 je short 004045FFh
        je(Cont(x004045c4), Cont(x004045ff))
    }
}

pub fn x004045c4() -> Cont {
    unsafe {
        // 004045c4 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 004045ca jne short 004045BDh
        jne(Cont(x004045cc), Cont(x004045bd))
    }
}

pub fn x004045cc() -> Cont {
    unsafe {
        // 004045cc mov eax,[ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 004045ce mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 004045d3 add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 004045d5 xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 004045d8 xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 004045da add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 004045dd test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 004045e2 je short 004045CCh
        je(Cont(x004045e4), Cont(x004045cc))
    }
}

pub fn x004045e4() -> Cont {
    unsafe {
        // 004045e4 mov eax,[ecx-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xfffffffcu32));
        // 004045e7 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004045e9 je short 0040460Eh
        je(Cont(x004045eb), Cont(x0040460e))
    }
}

pub fn x004045eb() -> Cont {
    unsafe {
        // 004045eb test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 004045ed je short 00404609h
        je(Cont(x004045ef), Cont(x00404609))
    }
}

pub fn x004045ef() -> Cont {
    unsafe {
        // 004045ef test eax,0FF0000h
        and(MACHINE.regs.eax, 0xff0000u32);
        // 004045f4 je short 00404604h
        je(Cont(x004045f6), Cont(x00404604))
    }
}

pub fn x004045f6() -> Cont {
    unsafe {
        // 004045f6 test eax,0FF000000h
        and(MACHINE.regs.eax, 0xff000000u32);
        // 004045fb je short 004045FFh
        je(Cont(x004045fd), Cont(x004045ff))
    }
}

pub fn x004045fd() -> Cont {
    unsafe {
        // 004045fd jmp short 004045CCh
        Cont(x004045cc)
    }
}

pub fn x004045ff() -> Cont {
    unsafe {
        // 004045ff lea edi,[ecx-1]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0xffffffffu32);
        // 00404602 jmp short 00404611h
        Cont(x00404611)
    }
}

pub fn x00404604() -> Cont {
    unsafe {
        // 00404604 lea edi,[ecx-2]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0xfffffffeu32);
        // 00404607 jmp short 00404611h
        Cont(x00404611)
    }
}

pub fn x00404609() -> Cont {
    unsafe {
        // 00404609 lea edi,[ecx-3]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0xfffffffdu32);
        // 0040460c jmp short 00404611h
        Cont(x00404611)
    }
}

pub fn x0040460e() -> Cont {
    unsafe {
        // 0040460e lea edi,[ecx-4]
        MACHINE.regs.edi = MACHINE.regs.ecx.wrapping_add(0xfffffffcu32);
        // 00404611 mov ecx,[esp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 00404615 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 0040461b je short 00404636h
        je(Cont(x0040461d), Cont(x00404636))
    }
}

pub fn x00404611() -> Cont {
    unsafe {
        // 00404611 mov ecx,[esp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 00404615 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 0040461b je short 00404636h
        je(Cont(x0040461d), Cont(x00404636))
    }
}

pub fn x0040461d() -> Cont {
    unsafe {
        // 0040461d mov dl,[ecx]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 0040461f inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 00404620 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404622 je short 00404688h
        je(Cont(x00404624), Cont(x00404688))
    }
}

pub fn x00404624() -> Cont {
    unsafe {
        // 00404624 mov [edi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 00404626 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404627 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 0040462d jne short 0040461Dh
        jne(Cont(x0040462f), Cont(x0040461d))
    }
}

pub fn x0040462f() -> Cont {
    unsafe {
        // 0040462f jmp short 00404636h
        Cont(x00404636)
    }
}

pub fn x00404631() -> Cont {
    unsafe {
        // 00404631 mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404633 add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404636 mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 0040463b mov eax,[ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 0040463d add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 0040463f xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 00404642 xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 00404644 mov edx,[ecx]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 00404646 add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 00404649 test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 0040464e je short 00404631h
        je(Cont(x00404650), Cont(x00404631))
    }
}

pub fn x00404636() -> Cont {
    unsafe {
        // 00404636 mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 0040463b mov eax,[ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 0040463d add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 0040463f xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 00404642 xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 00404644 mov edx,[ecx]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 00404646 add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 00404649 test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 0040464e je short 00404631h
        je(Cont(x00404650), Cont(x00404631))
    }
}

pub fn x00404650() -> Cont {
    unsafe {
        // 00404650 test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404652 je short 00404688h
        je(Cont(x00404654), Cont(x00404688))
    }
}

pub fn x00404654() -> Cont {
    unsafe {
        // 00404654 test dh,dh
        and(MACHINE.regs.get_dh(), MACHINE.regs.get_dh());
        // 00404656 je short 0040467Fh
        je(Cont(x00404658), Cont(x0040467f))
    }
}

pub fn x00404658() -> Cont {
    unsafe {
        // 00404658 test edx,0FF0000h
        and(MACHINE.regs.edx, 0xff0000u32);
        // 0040465e je short 00404672h
        je(Cont(x00404660), Cont(x00404672))
    }
}

pub fn x00404660() -> Cont {
    unsafe {
        // 00404660 test edx,0FF000000h
        and(MACHINE.regs.edx, 0xff000000u32);
        // 00404666 je short 0040466Ah
        je(Cont(x00404668), Cont(x0040466a))
    }
}

pub fn x00404668() -> Cont {
    unsafe {
        // 00404668 jmp short 00404631h
        Cont(x00404631)
    }
}

pub fn x0040466a() -> Cont {
    unsafe {
        // 0040466a mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 0040466c mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404670 pop edi
        MACHINE.regs.edi = pop();
        // 00404671 ret
        ret(0)
    }
}

pub fn x00404672() -> Cont {
    unsafe {
        // 00404672 mov [edi],dx
        MACHINE
            .memory
            .write::<u16>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_dx());
        // 00404675 mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404679 mov byte ptr [edi+2],0
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x2u32), 0x0u8);
        // 0040467d pop edi
        MACHINE.regs.edi = pop();
        // 0040467e ret
        ret(0)
    }
}

pub fn x0040467f() -> Cont {
    unsafe {
        // 0040467f mov [edi],dx
        MACHINE
            .memory
            .write::<u16>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_dx());
        // 00404682 mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404686 pop edi
        MACHINE.regs.edi = pop();
        // 00404687 ret
        ret(0)
    }
}

pub fn x00404688() -> Cont {
    unsafe {
        // 00404688 mov [edi],dl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_dl());
        // 0040468a mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 0040468e pop edi
        MACHINE.regs.edi = pop();
        // 0040468f ret
        ret(0)
    }
}

pub fn x00404690() -> Cont {
    unsafe {
        // 00404690 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00404694 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 0040469a je short 004046B0h
        je(Cont(x0040469c), Cont(x004046b0))
    }
}

pub fn x0040469c() -> Cont {
    unsafe {
        // 0040469c mov al,[ecx]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 0040469e inc ecx
        MACHINE.regs.ecx = inc(MACHINE.regs.ecx);
        // 0040469f test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004046a1 je short 004046E3h
        je(Cont(x004046a3), Cont(x004046e3))
    }
}

pub fn x004046a3() -> Cont {
    unsafe {
        // 004046a3 test ecx,3
        and(MACHINE.regs.ecx, 0x3u32);
        // 004046a9 jne short 0040469Ch
        jne(Cont(x004046ab), Cont(x0040469c))
    }
}

pub fn x004046ab() -> Cont {
    unsafe {
        // 004046ab add eax,0
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x0u32);
        // 004046b0 mov eax,[ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 004046b2 mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 004046b7 add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 004046b9 xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 004046bc xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 004046be add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 004046c1 test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 004046c6 je short 004046B0h
        je(Cont(x004046c8), Cont(x004046b0))
    }
}

pub fn x004046b0() -> Cont {
    unsafe {
        // 004046b0 mov eax,[ecx]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32));
        // 004046b2 mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 004046b7 add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 004046b9 xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 004046bc xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 004046be add ecx,4
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x4u32);
        // 004046c1 test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 004046c6 je short 004046B0h
        je(Cont(x004046c8), Cont(x004046b0))
    }
}

pub fn x004046c8() -> Cont {
    unsafe {
        // 004046c8 mov eax,[ecx-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ecx.wrapping_add(0xfffffffcu32));
        // 004046cb test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004046cd je short 00404701h
        je(Cont(x004046cf), Cont(x00404701))
    }
}

pub fn x004046cf() -> Cont {
    unsafe {
        // 004046cf test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 004046d1 je short 004046F7h
        je(Cont(x004046d3), Cont(x004046f7))
    }
}

pub fn x004046d3() -> Cont {
    unsafe {
        // 004046d3 test eax,0FF0000h
        and(MACHINE.regs.eax, 0xff0000u32);
        // 004046d8 je short 004046EDh
        je(Cont(x004046da), Cont(x004046ed))
    }
}

pub fn x004046da() -> Cont {
    unsafe {
        // 004046da test eax,0FF000000h
        and(MACHINE.regs.eax, 0xff000000u32);
        // 004046df je short 004046E3h
        je(Cont(x004046e1), Cont(x004046e3))
    }
}

pub fn x004046e1() -> Cont {
    unsafe {
        // 004046e1 jmp short 004046B0h
        Cont(x004046b0)
    }
}

pub fn x004046e3() -> Cont {
    unsafe {
        // 004046e3 lea eax,[ecx-1]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0xffffffffu32);
        // 004046e6 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 004046ea sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004046ec ret
        ret(0)
    }
}

pub fn x004046ed() -> Cont {
    unsafe {
        // 004046ed lea eax,[ecx-2]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0xfffffffeu32);
        // 004046f0 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 004046f4 sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 004046f6 ret
        ret(0)
    }
}

pub fn x004046f7() -> Cont {
    unsafe {
        // 004046f7 lea eax,[ecx-3]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0xfffffffdu32);
        // 004046fa mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 004046fe sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00404700 ret
        ret(0)
    }
}

pub fn x00404701() -> Cont {
    unsafe {
        // 00404701 lea eax,[ecx-4]
        MACHINE.regs.eax = MACHINE.regs.ecx.wrapping_add(0xfffffffcu32);
        // 00404704 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00404708 sub eax,ecx
        MACHINE.regs.eax = sub(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 0040470a ret
        ret(0)
    }
}

pub fn x0040470b() -> Cont {
    unsafe {
        // 0040470b push 0
        push(0x0u32);
        // 0040470d push dword ptr [esp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32)),
        );
        // 00404711 push dword ptr [esp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32)),
        );
        // 00404715 push dword ptr [esp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32)),
        );
        // 00404719 call 00404722h
        call(0x40471e, Cont(x00404722))
    }
}

pub fn x0040471e() -> Cont {
    unsafe {
        // 0040471e add esp,10h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x10u32);
        // 00404721 ret
        ret(0)
    }
}

pub fn x00404722() -> Cont {
    unsafe {
        // 00404722 push ebp
        push(MACHINE.regs.ebp);
        // 00404723 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00404725 sub esp,0Ch
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0xcu32);
        // 00404728 push ebx
        push(MACHINE.regs.ebx);
        // 00404729 and dword ptr [ebp-8],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
                0x0u32,
            ),
        );
        // 0040472d push esi
        push(MACHINE.regs.esi);
        // 0040472e push edi
        push(MACHINE.regs.edi);
        // 0040472f mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00404732 mov bl,[edi]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
        );
        // 00404734 lea esi,[edi+1]
        MACHINE.regs.esi = MACHINE.regs.edi.wrapping_add(0x1u32);
        // 00404737 mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 0040473a cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 00404741 jle short 00404752h
        jle(Cont(x00404743), Cont(x00404752))
    }
}

pub fn x0040473a() -> Cont {
    unsafe {
        // 0040473a cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 00404741 jle short 00404752h
        jle(Cont(x00404743), Cont(x00404752))
    }
}

pub fn x00404743() -> Cont {
    unsafe {
        // 00404743 movzx eax,bl
        MACHINE.regs.eax = MACHINE.regs.get_bl() as _;
        // 00404746 push 8
        push(0x8u32);
        // 00404748 push eax
        push(MACHINE.regs.eax);
        // 00404749 call 0040547Ch
        call(0x40474e, Cont(x0040547c))
    }
}

pub fn x0040474e() -> Cont {
    unsafe {
        // 0040474e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040474f pop ecx
        MACHINE.regs.ecx = pop();
        // 00404750 jmp short 00404761h
        Cont(x00404761)
    }
}

pub fn x00404752() -> Cont {
    unsafe {
        // 00404752 mov ecx,ds:[409330h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409330u32);
        // 00404758 movzx eax,bl
        MACHINE.regs.eax = MACHINE.regs.get_bl() as _;
        // 0040475b mov al,[ecx+eax*2]
        MACHINE.regs.set_al(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .ecx
                    .wrapping_add((MACHINE.regs.eax * 2))
                    .wrapping_add(0x0u32),
            ),
        );
        // 0040475e and eax,8
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x8u32);
        // 00404761 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404763 je short 0040476Ah
        je(Cont(x00404765), Cont(x0040476a))
    }
}

pub fn x00404761() -> Cont {
    unsafe {
        // 00404761 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404763 je short 0040476Ah
        je(Cont(x00404765), Cont(x0040476a))
    }
}

pub fn x00404765() -> Cont {
    unsafe {
        // 00404765 mov bl,[esi]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404767 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404768 jmp short 0040473Ah
        Cont(x0040473a)
    }
}

pub fn x0040476a() -> Cont {
    unsafe {
        // 0040476a cmp bl,2Dh
        sub(MACHINE.regs.get_bl(), 0x2du8);
        // 0040476d mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 00404770 jne short 00404778h
        jne(Cont(x00404772), Cont(x00404778))
    }
}

pub fn x00404772() -> Cont {
    unsafe {
        // 00404772 or dword ptr [ebp+14h],2
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x14u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
                0x2u32,
            ),
        );
        // 00404776 jmp short 0040477Dh
        Cont(x0040477d)
    }
}

pub fn x00404778() -> Cont {
    unsafe {
        // 00404778 cmp bl,2Bh
        sub(MACHINE.regs.get_bl(), 0x2bu8);
        // 0040477b jne short 00404783h
        jne(Cont(x0040477d), Cont(x00404783))
    }
}

pub fn x0040477d() -> Cont {
    unsafe {
        // 0040477d mov bl,[esi]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 0040477f inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404780 mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 00404783 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00404786 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404788 jl near ptr 0040491Ah
        jl(Cont(x0040478e), Cont(x0040491a))
    }
}

pub fn x00404783() -> Cont {
    unsafe {
        // 00404783 mov eax,[ebp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00404786 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404788 jl near ptr 0040491Ah
        jl(Cont(x0040478e), Cont(x0040491a))
    }
}

pub fn x0040478e() -> Cont {
    unsafe {
        // 0040478e cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00404791 je near ptr 0040491Ah
        je(Cont(x00404797), Cont(x0040491a))
    }
}

pub fn x00404797() -> Cont {
    unsafe {
        // 00404797 cmp eax,24h
        sub(MACHINE.regs.eax, 0x24u32);
        // 0040479a jg near ptr 0040491Ah
        jg(Cont(x004047a0), Cont(x0040491a))
    }
}

pub fn x004047a0() -> Cont {
    unsafe {
        // 004047a0 push 10h
        push(0x10u32);
        // 004047a2 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004047a4 pop ecx
        MACHINE.regs.ecx = pop();
        // 004047a5 jne short 004047CBh
        jne(Cont(x004047a7), Cont(x004047cb))
    }
}

pub fn x004047a7() -> Cont {
    unsafe {
        // 004047a7 cmp bl,30h
        sub(MACHINE.regs.get_bl(), 0x30u8);
        // 004047aa je short 004047B5h
        je(Cont(x004047ac), Cont(x004047b5))
    }
}

pub fn x004047ac() -> Cont {
    unsafe {
        // 004047ac mov dword ptr [ebp+10h],0Ah
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), 0xau32);
        // 004047b3 jmp short 004047E7h
        Cont(x004047e7)
    }
}

pub fn x004047b5() -> Cont {
    unsafe {
        // 004047b5 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 004047b7 cmp al,78h
        sub(MACHINE.regs.get_al(), 0x78u8);
        // 004047b9 je short 004047C8h
        je(Cont(x004047bb), Cont(x004047c8))
    }
}

pub fn x004047bb() -> Cont {
    unsafe {
        // 004047bb cmp al,58h
        sub(MACHINE.regs.get_al(), 0x58u8);
        // 004047bd je short 004047C8h
        je(Cont(x004047bf), Cont(x004047c8))
    }
}

pub fn x004047bf() -> Cont {
    unsafe {
        // 004047bf mov dword ptr [ebp+10h],8
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), 0x8u32);
        // 004047c6 jmp short 004047E7h
        Cont(x004047e7)
    }
}

pub fn x004047c8() -> Cont {
    unsafe {
        // 004047c8 mov [ebp+10h],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32), MACHINE.regs.ecx);
        // 004047cb cmp [ebp+10h],ecx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
            MACHINE.regs.ecx,
        );
        // 004047ce jne short 004047E7h
        jne(Cont(x004047d0), Cont(x004047e7))
    }
}

pub fn x004047cb() -> Cont {
    unsafe {
        // 004047cb cmp [ebp+10h],ecx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
            MACHINE.regs.ecx,
        );
        // 004047ce jne short 004047E7h
        jne(Cont(x004047d0), Cont(x004047e7))
    }
}

pub fn x004047d0() -> Cont {
    unsafe {
        // 004047d0 cmp bl,30h
        sub(MACHINE.regs.get_bl(), 0x30u8);
        // 004047d3 jne short 004047E7h
        jne(Cont(x004047d5), Cont(x004047e7))
    }
}

pub fn x004047d5() -> Cont {
    unsafe {
        // 004047d5 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 004047d7 cmp al,78h
        sub(MACHINE.regs.get_al(), 0x78u8);
        // 004047d9 je short 004047DFh
        je(Cont(x004047db), Cont(x004047df))
    }
}

pub fn x004047db() -> Cont {
    unsafe {
        // 004047db cmp al,58h
        sub(MACHINE.regs.get_al(), 0x58u8);
        // 004047dd jne short 004047E7h
        jne(Cont(x004047df), Cont(x004047e7))
    }
}

pub fn x004047df() -> Cont {
    unsafe {
        // 004047df mov bl,[esi+1]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x1u32)),
        );
        // 004047e2 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004047e3 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 004047e4 mov [ebp-4],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.esi,
        );
        // 004047e7 or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 004047ea xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 004047ec div dword ptr [ebp+10h]
        div();
        // 004047ef mov edi,103h
        MACHINE.regs.edi = 0x103u32;
        // 004047f4 mov [ebp-0Ch],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.eax,
        );
        // 004047f7 cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 004047fe movzx esi,bl
        MACHINE.regs.esi = MACHINE.regs.get_bl() as _;
        // 00404801 jle short 0040480Fh
        jle(Cont(x00404803), Cont(x0040480f))
    }
}

pub fn x004047e7() -> Cont {
    unsafe {
        // 004047e7 or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 004047ea xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 004047ec div dword ptr [ebp+10h]
        div();
        // 004047ef mov edi,103h
        MACHINE.regs.edi = 0x103u32;
        // 004047f4 mov [ebp-0Ch],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff4u32),
            MACHINE.regs.eax,
        );
        // 004047f7 cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 004047fe movzx esi,bl
        MACHINE.regs.esi = MACHINE.regs.get_bl() as _;
        // 00404801 jle short 0040480Fh
        jle(Cont(x00404803), Cont(x0040480f))
    }
}

pub fn x004047f7() -> Cont {
    unsafe {
        // 004047f7 cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 004047fe movzx esi,bl
        MACHINE.regs.esi = MACHINE.regs.get_bl() as _;
        // 00404801 jle short 0040480Fh
        jle(Cont(x00404803), Cont(x0040480f))
    }
}

pub fn x00404803() -> Cont {
    unsafe {
        // 00404803 push 4
        push(0x4u32);
        // 00404805 push esi
        push(MACHINE.regs.esi);
        // 00404806 call 0040547Ch
        call(0x40480b, Cont(x0040547c))
    }
}

pub fn x0040480b() -> Cont {
    unsafe {
        // 0040480b pop ecx
        MACHINE.regs.ecx = pop();
        // 0040480c pop ecx
        MACHINE.regs.ecx = pop();
        // 0040480d jmp short 0040481Ah
        Cont(x0040481a)
    }
}

pub fn x0040480f() -> Cont {
    unsafe {
        // 0040480f mov eax,ds:[409330h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409330u32);
        // 00404814 mov al,[eax+esi*2]
        MACHINE.regs.set_al(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.esi * 2))
                    .wrapping_add(0x0u32),
            ),
        );
        // 00404817 and eax,4
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x4u32);
        // 0040481a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040481c je short 00404826h
        je(Cont(x0040481e), Cont(x00404826))
    }
}

pub fn x0040481a() -> Cont {
    unsafe {
        // 0040481a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040481c je short 00404826h
        je(Cont(x0040481e), Cont(x00404826))
    }
}

pub fn x0040481e() -> Cont {
    unsafe {
        // 0040481e movsx ecx,bl
        movsx();
        // 00404821 sub ecx,30h
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, 0x30u32);
        // 00404824 jmp short 00404858h
        Cont(x00404858)
    }
}

pub fn x00404826() -> Cont {
    unsafe {
        // 00404826 cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 0040482d jle short 0040483Ah
        jle(Cont(x0040482f), Cont(x0040483a))
    }
}

pub fn x0040482f() -> Cont {
    unsafe {
        // 0040482f push edi
        push(MACHINE.regs.edi);
        // 00404830 push esi
        push(MACHINE.regs.esi);
        // 00404831 call 0040547Ch
        call(0x404836, Cont(x0040547c))
    }
}

pub fn x00404836() -> Cont {
    unsafe {
        // 00404836 pop ecx
        MACHINE.regs.ecx = pop();
        // 00404837 pop ecx
        MACHINE.regs.ecx = pop();
        // 00404838 jmp short 00404845h
        Cont(x00404845)
    }
}

pub fn x0040483a() -> Cont {
    unsafe {
        // 0040483a mov eax,ds:[409330h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409330u32);
        // 0040483f mov ax,[eax+esi*2]
        MACHINE.regs.set_ax(
            MACHINE.memory.read::<u16>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.esi * 2))
                    .wrapping_add(0x0u32),
            ),
        );
        // 00404843 and eax,edi
        MACHINE.regs.eax = and(MACHINE.regs.eax, MACHINE.regs.edi);
        // 00404845 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404847 je short 00404893h
        je(Cont(x00404849), Cont(x00404893))
    }
}

pub fn x00404845() -> Cont {
    unsafe {
        // 00404845 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404847 je short 00404893h
        je(Cont(x00404849), Cont(x00404893))
    }
}

pub fn x00404849() -> Cont {
    unsafe {
        // 00404849 movsx eax,bl
        movsx();
        // 0040484c push eax
        push(MACHINE.regs.eax);
        // 0040484d call 004053B0h
        call(0x404852, Cont(x004053b0))
    }
}

pub fn x00404852() -> Cont {
    unsafe {
        // 00404852 pop ecx
        MACHINE.regs.ecx = pop();
        // 00404853 mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 00404855 sub ecx,37h
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, 0x37u32);
        // 00404858 cmp ecx,[ebp+10h]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040485b jae short 00404893h
        jae(Cont(x0040485d), Cont(x00404893))
    }
}

pub fn x00404858() -> Cont {
    unsafe {
        // 00404858 cmp ecx,[ebp+10h]
        sub(
            MACHINE.regs.ecx,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040485b jae short 00404893h
        jae(Cont(x0040485d), Cont(x00404893))
    }
}

pub fn x0040485d() -> Cont {
    unsafe {
        // 0040485d mov esi,[ebp-8]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00404860 or dword ptr [ebp+14h],8
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x14u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
                0x8u32,
            ),
        );
        // 00404864 cmp esi,[ebp-0Ch]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff4u32)),
        );
        // 00404867 jb short 0040487Dh
        jb(Cont(x00404869), Cont(x0040487d))
    }
}

pub fn x00404869() -> Cont {
    unsafe {
        // 00404869 jne short 00404877h
        jne(Cont(x0040486b), Cont(x00404877))
    }
}

pub fn x0040486b() -> Cont {
    unsafe {
        // 0040486b or eax,0FFFFFFFFh
        MACHINE.regs.eax = or(MACHINE.regs.eax, 0xffffffffu32);
        // 0040486e xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 00404870 div dword ptr [ebp+10h]
        div();
        // 00404873 cmp ecx,edx
        sub(MACHINE.regs.ecx, MACHINE.regs.edx);
        // 00404875 jbe short 0040487Dh
        jbe(Cont(x00404877), Cont(x0040487d))
    }
}

pub fn x00404877() -> Cont {
    unsafe {
        // 00404877 or dword ptr [ebp+14h],4
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0x14u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
                0x4u32,
            ),
        );
        // 0040487b jmp short 00404886h
        Cont(x00404886)
    }
}

pub fn x0040487d() -> Cont {
    unsafe {
        // 0040487d imul esi,[ebp+10h]
        imul();
        // 00404881 add esi,ecx
        MACHINE.regs.esi = add(MACHINE.regs.esi, MACHINE.regs.ecx);
        // 00404883 mov [ebp-8],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.esi,
        );
        // 00404886 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00404889 inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 0040488c mov bl,[eax]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 0040488e jmp near ptr 004047F7h
        Cont(x004047f7)
    }
}

pub fn x00404886() -> Cont {
    unsafe {
        // 00404886 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00404889 inc dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            inc(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 0040488c mov bl,[eax]
        MACHINE.regs.set_bl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
        );
        // 0040488e jmp near ptr 004047F7h
        Cont(x004047f7)
    }
}

pub fn x00404893() -> Cont {
    unsafe {
        // 00404893 mov ecx,[ebp+14h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 00404896 dec dword ptr [ebp-4]
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            dec(MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))),
        );
        // 00404899 mov edx,[ebp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040489c test cl,8
        and(MACHINE.regs.get_cl(), 0x8u8);
        // 0040489f jne short 004048B1h
        jne(Cont(x004048a1), Cont(x004048b1))
    }
}

pub fn x004048a1() -> Cont {
    unsafe {
        // 004048a1 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 004048a3 je short 004048ABh
        je(Cont(x004048a5), Cont(x004048ab))
    }
}

pub fn x004048a5() -> Cont {
    unsafe {
        // 004048a5 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004048a8 mov [ebp-4],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.eax,
        );
        // 004048ab and dword ptr [ebp-8],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
                0x0u32,
            ),
        );
        // 004048af jmp short 004048FEh
        Cont(x004048fe)
    }
}

pub fn x004048ab() -> Cont {
    unsafe {
        // 004048ab and dword ptr [ebp-8],0
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            and(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
                0x0u32,
            ),
        );
        // 004048af jmp short 004048FEh
        Cont(x004048fe)
    }
}

pub fn x004048b1() -> Cont {
    unsafe {
        // 004048b1 test cl,4
        and(MACHINE.regs.get_cl(), 0x4u8);
        // 004048b4 mov eax,7FFFFFFFh
        MACHINE.regs.eax = 0x7fffffffu32;
        // 004048b9 jne short 004048D7h
        jne(Cont(x004048bb), Cont(x004048d7))
    }
}

pub fn x004048bb() -> Cont {
    unsafe {
        // 004048bb test cl,1
        and(MACHINE.regs.get_cl(), 0x1u8);
        // 004048be jne short 004048FEh
        jne(Cont(x004048c0), Cont(x004048fe))
    }
}

pub fn x004048c0() -> Cont {
    unsafe {
        // 004048c0 and ecx,2
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x2u32);
        // 004048c3 je short 004048CEh
        je(Cont(x004048c5), Cont(x004048ce))
    }
}

pub fn x004048c5() -> Cont {
    unsafe {
        // 004048c5 cmp dword ptr [ebp-8],80000000h
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
            0x80000000u32,
        );
        // 004048cc ja short 004048D7h
        ja(Cont(x004048ce), Cont(x004048d7))
    }
}

pub fn x004048ce() -> Cont {
    unsafe {
        // 004048ce test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 004048d0 jne short 004048FEh
        jne(Cont(x004048d2), Cont(x004048fe))
    }
}

pub fn x004048d2() -> Cont {
    unsafe {
        // 004048d2 cmp [ebp-8],eax
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
            MACHINE.regs.eax,
        );
        // 004048d5 jbe short 004048FEh
        jbe(Cont(x004048d7), Cont(x004048fe))
    }
}

pub fn x004048d7() -> Cont {
    unsafe {
        // 004048d7 test byte ptr [ebp+14h],1
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
            0x1u8,
        );
        // 004048db mov dword ptr ds:[4095A8h],22h
        MACHINE.memory.write::<u32>(0x4095a8u32, 0x22u32);
        // 004048e5 je short 004048EDh
        je(Cont(x004048e7), Cont(x004048ed))
    }
}

pub fn x004048e7() -> Cont {
    unsafe {
        // 004048e7 or dword ptr [ebp-8],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32)),
                0xffffffffu32,
            ),
        );
        // 004048eb jmp short 004048FEh
        Cont(x004048fe)
    }
}

pub fn x004048ed() -> Cont {
    unsafe {
        // 004048ed mov ecx,[ebp+14h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32));
        // 004048f0 and cl,2
        MACHINE.regs.set_cl(and(MACHINE.regs.get_cl(), 0x2u8));
        // 004048f3 neg cl
        MACHINE.regs.set_cl(neg(MACHINE.regs.get_cl()));
        // 004048f5 sbb ecx,ecx
        MACHINE.regs.ecx = sbb(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 004048f7 neg ecx
        MACHINE.regs.ecx = neg(MACHINE.regs.ecx);
        // 004048f9 add ecx,eax
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 004048fb mov [ebp-8],ecx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.ecx,
        );
        // 004048fe test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00404900 je short 00404907h
        je(Cont(x00404902), Cont(x00404907))
    }
}

pub fn x004048fe() -> Cont {
    unsafe {
        // 004048fe test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00404900 je short 00404907h
        je(Cont(x00404902), Cont(x00404907))
    }
}

pub fn x00404902() -> Cont {
    unsafe {
        // 00404902 mov eax,[ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32));
        // 00404905 mov [edx],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00404907 test byte ptr [ebp+14h],2
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
            0x2u8,
        );
        // 0040490b je short 00404915h
        je(Cont(x0040490d), Cont(x00404915))
    }
}

pub fn x00404907() -> Cont {
    unsafe {
        // 00404907 test byte ptr [ebp+14h],2
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
            0x2u8,
        );
        // 0040490b je short 00404915h
        je(Cont(x0040490d), Cont(x00404915))
    }
}

pub fn x0040490d() -> Cont {
    unsafe {
        // 0040490d mov eax,[ebp-8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00404910 neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 00404912 mov [ebp-8],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffff8u32),
            MACHINE.regs.eax,
        );
        // 00404915 mov eax,[ebp-8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00404918 jmp short 00404925h
        Cont(x00404925)
    }
}

pub fn x00404915() -> Cont {
    unsafe {
        // 00404915 mov eax,[ebp-8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff8u32));
        // 00404918 jmp short 00404925h
        Cont(x00404925)
    }
}

pub fn x0040491a() -> Cont {
    unsafe {
        // 0040491a mov eax,[ebp+0Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 0040491d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040491f je short 00404923h
        je(Cont(x00404921), Cont(x00404923))
    }
}

pub fn x00404921() -> Cont {
    unsafe {
        // 00404921 mov [eax],edi
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32), MACHINE.regs.edi);
        // 00404923 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404925 pop edi
        MACHINE.regs.edi = pop();
        // 00404926 pop esi
        MACHINE.regs.esi = pop();
        // 00404927 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404928 leave
        leave();
        // 00404929 ret
        ret(0)
    }
}

pub fn x00404923() -> Cont {
    unsafe {
        // 00404923 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404925 pop edi
        MACHINE.regs.edi = pop();
        // 00404926 pop esi
        MACHINE.regs.esi = pop();
        // 00404927 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404928 leave
        leave();
        // 00404929 ret
        ret(0)
    }
}

pub fn x00404925() -> Cont {
    unsafe {
        // 00404925 pop edi
        MACHINE.regs.edi = pop();
        // 00404926 pop esi
        MACHINE.regs.esi = pop();
        // 00404927 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404928 leave
        leave();
        // 00404929 ret
        ret(0)
    }
}

pub fn x00404930() -> Cont {
    unsafe {
        // 00404930 lea eax,[edx-1]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0xffffffffu32);
        // 00404933 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404934 ret
        ret(0)
    }
}

pub fn x00404940() -> Cont {
    unsafe {
        // 00404940 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404942 mov al,[esp+8]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 00404946 push ebx
        push(MACHINE.regs.ebx);
        // 00404947 mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00404949 shl eax,8
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x8u8);
        // 0040494c mov edx,[esp+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404950 test edx,3
        and(MACHINE.regs.edx, 0x3u32);
        // 00404956 je short 0040496Bh
        je(Cont(x00404958), Cont(x0040496b))
    }
}

pub fn x00404946() -> Cont {
    unsafe {
        // 00404946 push ebx
        push(MACHINE.regs.ebx);
        // 00404947 mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00404949 shl eax,8
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x8u8);
        // 0040494c mov edx,[esp+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404950 test edx,3
        and(MACHINE.regs.edx, 0x3u32);
        // 00404956 je short 0040496Bh
        je(Cont(x00404958), Cont(x0040496b))
    }
}

pub fn x00404958() -> Cont {
    unsafe {
        // 00404958 mov cl,[edx]
        MACHINE.regs.set_cl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edx.wrapping_add(0x0u32)),
        );
        // 0040495a inc edx
        MACHINE.regs.edx = inc(MACHINE.regs.edx);
        // 0040495b cmp cl,bl
        sub(MACHINE.regs.get_cl(), MACHINE.regs.get_bl());
        // 0040495d je short 00404930h
        je(Cont(x0040495f), Cont(x00404930))
    }
}

pub fn x0040495f() -> Cont {
    unsafe {
        // 0040495f test cl,cl
        and(MACHINE.regs.get_cl(), MACHINE.regs.get_cl());
        // 00404961 je short 004049B4h
        je(Cont(x00404963), Cont(x004049b4))
    }
}

pub fn x00404963() -> Cont {
    unsafe {
        // 00404963 test edx,3
        and(MACHINE.regs.edx, 0x3u32);
        // 00404969 jne short 00404958h
        jne(Cont(x0040496b), Cont(x00404958))
    }
}

pub fn x0040496b() -> Cont {
    unsafe {
        // 0040496b or ebx,eax
        MACHINE.regs.ebx = or(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 0040496d push edi
        push(MACHINE.regs.edi);
        // 0040496e mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00404970 shl ebx,10h
        MACHINE.regs.ebx = shl(MACHINE.regs.ebx, 0x10u8);
        // 00404973 push esi
        push(MACHINE.regs.esi);
        // 00404974 or ebx,eax
        MACHINE.regs.ebx = or(MACHINE.regs.ebx, MACHINE.regs.eax);
        // 00404976 mov ecx,[edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32));
        // 00404978 mov edi,7EFEFEFFh
        MACHINE.regs.edi = 0x7efefeffu32;
        // 0040497d mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 0040497f mov esi,edi
        MACHINE.regs.esi = MACHINE.regs.edi;
        // 00404981 xor ecx,ebx
        MACHINE.regs.ecx ^= MACHINE.regs.ebx;
        // 00404983 add esi,eax
        MACHINE.regs.esi = add(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00404985 add edi,ecx
        MACHINE.regs.edi = add(MACHINE.regs.edi, MACHINE.regs.ecx);
        // 00404987 xor ecx,0FFFFFFFFh
        MACHINE.regs.ecx ^= 0xffffffffu32;
        // 0040498a xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 0040498d xor ecx,edi
        MACHINE.regs.ecx ^= MACHINE.regs.edi;
        // 0040498f xor eax,esi
        MACHINE.regs.eax ^= MACHINE.regs.esi;
        // 00404991 add edx,4
        MACHINE.regs.edx = add(MACHINE.regs.edx, 0x4u32);
        // 00404994 and ecx,81010100h
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x81010100u32);
        // 0040499a jne short 004049B8h
        jne(Cont(x0040499c), Cont(x004049b8))
    }
}

pub fn x00404976() -> Cont {
    unsafe {
        // 00404976 mov ecx,[edx]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0x0u32));
        // 00404978 mov edi,7EFEFEFFh
        MACHINE.regs.edi = 0x7efefeffu32;
        // 0040497d mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 0040497f mov esi,edi
        MACHINE.regs.esi = MACHINE.regs.edi;
        // 00404981 xor ecx,ebx
        MACHINE.regs.ecx ^= MACHINE.regs.ebx;
        // 00404983 add esi,eax
        MACHINE.regs.esi = add(MACHINE.regs.esi, MACHINE.regs.eax);
        // 00404985 add edi,ecx
        MACHINE.regs.edi = add(MACHINE.regs.edi, MACHINE.regs.ecx);
        // 00404987 xor ecx,0FFFFFFFFh
        MACHINE.regs.ecx ^= 0xffffffffu32;
        // 0040498a xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 0040498d xor ecx,edi
        MACHINE.regs.ecx ^= MACHINE.regs.edi;
        // 0040498f xor eax,esi
        MACHINE.regs.eax ^= MACHINE.regs.esi;
        // 00404991 add edx,4
        MACHINE.regs.edx = add(MACHINE.regs.edx, 0x4u32);
        // 00404994 and ecx,81010100h
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x81010100u32);
        // 0040499a jne short 004049B8h
        jne(Cont(x0040499c), Cont(x004049b8))
    }
}

pub fn x0040499c() -> Cont {
    unsafe {
        // 0040499c and eax,81010100h
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x81010100u32);
        // 004049a1 je short 00404976h
        je(Cont(x004049a3), Cont(x00404976))
    }
}

pub fn x004049a3() -> Cont {
    unsafe {
        // 004049a3 and eax,1010100h
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x1010100u32);
        // 004049a8 jne short 004049B2h
        jne(Cont(x004049aa), Cont(x004049b2))
    }
}

pub fn x004049aa() -> Cont {
    unsafe {
        // 004049aa and esi,80000000h
        MACHINE.regs.esi = and(MACHINE.regs.esi, 0x80000000u32);
        // 004049b0 jne short 00404976h
        jne(Cont(x004049b2), Cont(x00404976))
    }
}

pub fn x004049b2() -> Cont {
    unsafe {
        // 004049b2 pop esi
        MACHINE.regs.esi = pop();
        // 004049b3 pop edi
        MACHINE.regs.edi = pop();
        // 004049b4 pop ebx
        MACHINE.regs.ebx = pop();
        // 004049b5 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004049b7 ret
        ret(0)
    }
}

pub fn x004049b4() -> Cont {
    unsafe {
        // 004049b4 pop ebx
        MACHINE.regs.ebx = pop();
        // 004049b5 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004049b7 ret
        ret(0)
    }
}

pub fn x004049b8() -> Cont {
    unsafe {
        // 004049b8 mov eax,[edx-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.edx.wrapping_add(0xfffffffcu32));
        // 004049bb cmp al,bl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_bl());
        // 004049bd je short 004049F5h
        je(Cont(x004049bf), Cont(x004049f5))
    }
}

pub fn x004049bf() -> Cont {
    unsafe {
        // 004049bf test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004049c1 je short 004049B2h
        je(Cont(x004049c3), Cont(x004049b2))
    }
}

pub fn x004049c3() -> Cont {
    unsafe {
        // 004049c3 cmp ah,bl
        sub(MACHINE.regs.get_ah(), MACHINE.regs.get_bl());
        // 004049c5 je short 004049EEh
        je(Cont(x004049c7), Cont(x004049ee))
    }
}

pub fn x004049c7() -> Cont {
    unsafe {
        // 004049c7 test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 004049c9 je short 004049B2h
        je(Cont(x004049cb), Cont(x004049b2))
    }
}

pub fn x004049cb() -> Cont {
    unsafe {
        // 004049cb shr eax,10h
        MACHINE.regs.eax = shr(MACHINE.regs.eax, 0x10u8);
        // 004049ce cmp al,bl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_bl());
        // 004049d0 je short 004049E7h
        je(Cont(x004049d2), Cont(x004049e7))
    }
}

pub fn x004049d2() -> Cont {
    unsafe {
        // 004049d2 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 004049d4 je short 004049B2h
        je(Cont(x004049d6), Cont(x004049b2))
    }
}

pub fn x004049d6() -> Cont {
    unsafe {
        // 004049d6 cmp ah,bl
        sub(MACHINE.regs.get_ah(), MACHINE.regs.get_bl());
        // 004049d8 je short 004049E0h
        je(Cont(x004049da), Cont(x004049e0))
    }
}

pub fn x004049da() -> Cont {
    unsafe {
        // 004049da test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 004049dc je short 004049B2h
        je(Cont(x004049de), Cont(x004049b2))
    }
}

pub fn x004049de() -> Cont {
    unsafe {
        // 004049de jmp short 00404976h
        Cont(x00404976)
    }
}

pub fn x004049e0() -> Cont {
    unsafe {
        // 004049e0 pop esi
        MACHINE.regs.esi = pop();
        // 004049e1 pop edi
        MACHINE.regs.edi = pop();
        // 004049e2 lea eax,[edx-1]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0xffffffffu32);
        // 004049e5 pop ebx
        MACHINE.regs.ebx = pop();
        // 004049e6 ret
        ret(0)
    }
}

pub fn x004049e7() -> Cont {
    unsafe {
        // 004049e7 lea eax,[edx-2]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0xfffffffeu32);
        // 004049ea pop esi
        MACHINE.regs.esi = pop();
        // 004049eb pop edi
        MACHINE.regs.edi = pop();
        // 004049ec pop ebx
        MACHINE.regs.ebx = pop();
        // 004049ed ret
        ret(0)
    }
}

pub fn x004049ee() -> Cont {
    unsafe {
        // 004049ee lea eax,[edx-3]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0xfffffffdu32);
        // 004049f1 pop esi
        MACHINE.regs.esi = pop();
        // 004049f2 pop edi
        MACHINE.regs.edi = pop();
        // 004049f3 pop ebx
        MACHINE.regs.ebx = pop();
        // 004049f4 ret
        ret(0)
    }
}

pub fn x004049f5() -> Cont {
    unsafe {
        // 004049f5 lea eax,[edx-4]
        MACHINE.regs.eax = MACHINE.regs.edx.wrapping_add(0xfffffffcu32);
        // 004049f8 pop esi
        MACHINE.regs.esi = pop();
        // 004049f9 pop edi
        MACHINE.regs.edi = pop();
        // 004049fa pop ebx
        MACHINE.regs.ebx = pop();
        // 004049fb ret
        ret(0)
    }
}

pub fn x00404a00() -> Cont {
    unsafe {
        // 00404a00 mov ecx,[esp+8]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404a04 push edi
        push(MACHINE.regs.edi);
        // 00404a05 push ebx
        push(MACHINE.regs.ebx);
        // 00404a06 push esi
        push(MACHINE.regs.esi);
        // 00404a07 mov dl,[ecx]
        MACHINE.regs.set_dl(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
        );
        // 00404a09 mov edi,[esp+10h]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 00404a0d test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404a0f je short 00404A7Ah
        je(Cont(x00404a11), Cont(x00404a7a))
    }
}

pub fn x00404a11() -> Cont {
    unsafe {
        // 00404a11 mov dh,[ecx+1]
        MACHINE.regs.set_dh(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x1u32)),
        );
        // 00404a14 test dh,dh
        and(MACHINE.regs.get_dh(), MACHINE.regs.get_dh());
        // 00404a16 je short 00404A67h
        je(Cont(x00404a18), Cont(x00404a67))
    }
}

pub fn x00404a18() -> Cont {
    unsafe {
        // 00404a18 mov esi,edi
        MACHINE.regs.esi = MACHINE.regs.edi;
        // 00404a1a mov ecx,[esp+14h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32));
        // 00404a1e mov al,[edi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32)),
        );
        // 00404a20 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404a21 cmp al,dl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_dl());
        // 00404a23 je short 00404A3Ah
        je(Cont(x00404a25), Cont(x00404a3a))
    }
}

pub fn x00404a25() -> Cont {
    unsafe {
        // 00404a25 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404a27 je short 00404A34h
        je(Cont(x00404a29), Cont(x00404a34))
    }
}

pub fn x00404a29() -> Cont {
    unsafe {
        // 00404a29 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404a2b inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404a2c cmp al,dl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_dl());
        // 00404a2e je short 00404A3Ah
        je(Cont(x00404a30), Cont(x00404a3a))
    }
}

pub fn x00404a2c() -> Cont {
    unsafe {
        // 00404a2c cmp al,dl
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_dl());
        // 00404a2e je short 00404A3Ah
        je(Cont(x00404a30), Cont(x00404a3a))
    }
}

pub fn x00404a30() -> Cont {
    unsafe {
        // 00404a30 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404a32 jne short 00404A29h
        jne(Cont(x00404a34), Cont(x00404a29))
    }
}

pub fn x00404a34() -> Cont {
    unsafe {
        // 00404a34 pop esi
        MACHINE.regs.esi = pop();
        // 00404a35 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404a36 pop edi
        MACHINE.regs.edi = pop();
        // 00404a37 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404a39 ret
        ret(0)
    }
}

pub fn x00404a3a() -> Cont {
    unsafe {
        // 00404a3a mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404a3c inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404a3d cmp al,dh
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_dh());
        // 00404a3f jne short 00404A2Ch
        jne(Cont(x00404a41), Cont(x00404a2c))
    }
}

pub fn x00404a41() -> Cont {
    unsafe {
        // 00404a41 lea edi,[esi-1]
        MACHINE.regs.edi = MACHINE.regs.esi.wrapping_add(0xffffffffu32);
        // 00404a44 mov ah,[ecx+2]
        MACHINE.regs.set_ah(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x2u32)),
        );
        // 00404a47 test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 00404a49 je short 00404A73h
        je(Cont(x00404a4b), Cont(x00404a73))
    }
}

pub fn x00404a44() -> Cont {
    unsafe {
        // 00404a44 mov ah,[ecx+2]
        MACHINE.regs.set_ah(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x2u32)),
        );
        // 00404a47 test ah,ah
        and(MACHINE.regs.get_ah(), MACHINE.regs.get_ah());
        // 00404a49 je short 00404A73h
        je(Cont(x00404a4b), Cont(x00404a73))
    }
}

pub fn x00404a4b() -> Cont {
    unsafe {
        // 00404a4b mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404a4d add esi,2
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x2u32);
        // 00404a50 cmp al,ah
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_ah());
        // 00404a52 jne short 00404A18h
        jne(Cont(x00404a54), Cont(x00404a18))
    }
}

pub fn x00404a54() -> Cont {
    unsafe {
        // 00404a54 mov al,[ecx+3]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ecx.wrapping_add(0x3u32)),
        );
        // 00404a57 test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404a59 je short 00404A73h
        je(Cont(x00404a5b), Cont(x00404a73))
    }
}

pub fn x00404a5b() -> Cont {
    unsafe {
        // 00404a5b mov ah,[esi-1]
        MACHINE.regs.set_ah(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0xffffffffu32)),
        );
        // 00404a5e add ecx,2
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, 0x2u32);
        // 00404a61 cmp al,ah
        sub(MACHINE.regs.get_al(), MACHINE.regs.get_ah());
        // 00404a63 je short 00404A44h
        je(Cont(x00404a65), Cont(x00404a44))
    }
}

pub fn x00404a65() -> Cont {
    unsafe {
        // 00404a65 jmp short 00404A18h
        Cont(x00404a18)
    }
}

pub fn x00404a67() -> Cont {
    unsafe {
        // 00404a67 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404a69 pop esi
        MACHINE.regs.esi = pop();
        // 00404a6a pop ebx
        MACHINE.regs.ebx = pop();
        // 00404a6b pop edi
        MACHINE.regs.edi = pop();
        // 00404a6c mov al,dl
        MACHINE.regs.set_al(MACHINE.regs.get_dl());
        // 00404a6e jmp near ptr 00404946h
        Cont(x00404946)
    }
}

pub fn x00404a73() -> Cont {
    unsafe {
        // 00404a73 lea eax,[edi-1]
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0xffffffffu32);
        // 00404a76 pop esi
        MACHINE.regs.esi = pop();
        // 00404a77 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404a78 pop edi
        MACHINE.regs.edi = pop();
        // 00404a79 ret
        ret(0)
    }
}

pub fn x00404a7a() -> Cont {
    unsafe {
        // 00404a7a mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00404a7c pop esi
        MACHINE.regs.esi = pop();
        // 00404a7d pop ebx
        MACHINE.regs.ebx = pop();
        // 00404a7e pop edi
        MACHINE.regs.edi = pop();
        // 00404a7f ret
        ret(0)
    }
}

pub fn x00404a80() -> Cont {
    unsafe {
        // 00404a80 push ebp
        push(MACHINE.regs.ebp);
        // 00404a81 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00404a83 push edi
        push(MACHINE.regs.edi);
        // 00404a84 push esi
        push(MACHINE.regs.esi);
        // 00404a85 push ebx
        push(MACHINE.regs.ebx);
        // 00404a86 mov ecx,[ebp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00404a89 jecxz 00404AB1h
        jecxz(Cont(x00404a8b), Cont(x00404ab1))
    }
}

pub fn x00404a8b() -> Cont {
    unsafe {
        // 00404a8b mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 00404a8d mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00404a90 mov esi,edi
        MACHINE.regs.esi = MACHINE.regs.edi;
        // 00404a92 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404a94 repne scasb
        scasb();
        // 00404a96 neg ecx
        MACHINE.regs.ecx = neg(MACHINE.regs.ecx);
        // 00404a98 add ecx,ebx
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.ebx);
        // 00404a9a mov edi,esi
        MACHINE.regs.edi = MACHINE.regs.esi;
        // 00404a9c mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00404a9f repe cmpsb
        cmpsb();
        // 00404aa1 mov al,[esi-1]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0xffffffffu32)),
        );
        // 00404aa4 xor ecx,ecx
        MACHINE.regs.ecx ^= MACHINE.regs.ecx;
        // 00404aa6 cmp al,[edi-1]
        sub(
            MACHINE.regs.get_al(),
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.edi.wrapping_add(0xffffffffu32)),
        );
        // 00404aa9 ja short 00404AAFh
        ja(Cont(x00404aab), Cont(x00404aaf))
    }
}

pub fn x00404aab() -> Cont {
    unsafe {
        // 00404aab je short 00404AB1h
        je(Cont(x00404aad), Cont(x00404ab1))
    }
}

pub fn x00404aad() -> Cont {
    unsafe {
        // 00404aad dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404aae dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404aaf not ecx
        not();
        // 00404ab1 mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00404ab3 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404ab4 pop esi
        MACHINE.regs.esi = pop();
        // 00404ab5 pop edi
        MACHINE.regs.edi = pop();
        // 00404ab6 leave
        leave();
        // 00404ab7 ret
        ret(0)
    }
}

pub fn x00404aaf() -> Cont {
    unsafe {
        // 00404aaf not ecx
        not();
        // 00404ab1 mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00404ab3 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404ab4 pop esi
        MACHINE.regs.esi = pop();
        // 00404ab5 pop edi
        MACHINE.regs.edi = pop();
        // 00404ab6 leave
        leave();
        // 00404ab7 ret
        ret(0)
    }
}

pub fn x00404ab1() -> Cont {
    unsafe {
        // 00404ab1 mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00404ab3 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404ab4 pop esi
        MACHINE.regs.esi = pop();
        // 00404ab5 pop edi
        MACHINE.regs.edi = pop();
        // 00404ab6 leave
        leave();
        // 00404ab7 ret
        ret(0)
    }
}

pub fn x00404ac0() -> Cont {
    unsafe {
        // 00404ac0 push ecx
        push(MACHINE.regs.ecx);
        // 00404ac1 cmp eax,1000h
        sub(MACHINE.regs.eax, 0x1000u32);
        // 00404ac6 lea ecx,[esp+8]
        MACHINE.regs.ecx = MACHINE.regs.esp.wrapping_add(0x8u32);
        // 00404aca jb short 00404AE0h
        jb(Cont(x00404acc), Cont(x00404ae0))
    }
}

pub fn x00404acc() -> Cont {
    unsafe {
        // 00404acc sub ecx,1000h
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, 0x1000u32);
        // 00404ad2 sub eax,1000h
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x1000u32);
        // 00404ad7 test [ecx],eax
        and(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.eax,
        );
        // 00404ad9 cmp eax,1000h
        sub(MACHINE.regs.eax, 0x1000u32);
        // 00404ade jae short 00404ACCh
        jae(Cont(x00404ae0), Cont(x00404acc))
    }
}

pub fn x00404ae0() -> Cont {
    unsafe {
        // 00404ae0 sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00404ae2 mov eax,esp
        MACHINE.regs.eax = MACHINE.regs.esp;
        // 00404ae4 test [ecx],eax
        and(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ecx.wrapping_add(0x0u32)),
            MACHINE.regs.eax,
        );
        // 00404ae6 mov esp,ecx
        MACHINE.regs.esp = MACHINE.regs.ecx;
        // 00404ae8 mov ecx,[eax]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x0u32));
        // 00404aea mov eax,[eax+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x4u32));
        // 00404aed push eax
        push(MACHINE.regs.eax);
        // 00404aee ret
        ret(0)
    }
}

pub fn x00404aef() -> Cont {
    unsafe {
        // 00404aef push ebx
        push(MACHINE.regs.ebx);
        // 00404af0 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00404af2 cmp ds:[409714h],ebx
        sub(MACHINE.memory.read::<u32>(0x409714u32), MACHINE.regs.ebx);
        // 00404af8 push esi
        push(MACHINE.regs.esi);
        // 00404af9 push edi
        push(MACHINE.regs.edi);
        // 00404afa jne short 00404B3Eh
        jne(Cont(x00404afc), Cont(x00404b3e))
    }
}

pub fn x00404afc() -> Cont {
    unsafe {
        // 00404afc push 406484h
        push(0x406484u32);
        // 00404b01 call dword ptr ds:[406048h]
        call(0x404b07, Cont(kernel32::LoadLibraryA_stdcall))
    }
}

pub fn x00404b07() -> Cont {
    unsafe {
        // 00404b07 mov edi,eax
        MACHINE.regs.edi = MACHINE.regs.eax;
        // 00404b09 cmp edi,ebx
        sub(MACHINE.regs.edi, MACHINE.regs.ebx);
        // 00404b0b je short 00404B74h
        je(Cont(x00404b0d), Cont(x00404b74))
    }
}

pub fn x00404b0d() -> Cont {
    unsafe {
        // 00404b0d mov esi,ds:[40604Ch]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x40604cu32);
        // 00404b13 push 406478h
        push(0x406478u32);
        // 00404b18 push edi
        push(MACHINE.regs.edi);
        // 00404b19 call esi
        call(0x404b1b, indirect(MACHINE.regs.esi))
    }
}

pub fn x00404b1b() -> Cont {
    unsafe {
        // 00404b1b test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404b1d mov ds:[409714h],eax
        MACHINE.memory.write::<u32>(0x409714u32, MACHINE.regs.eax);
        // 00404b22 je short 00404B74h
        je(Cont(x00404b24), Cont(x00404b74))
    }
}

pub fn x00404b24() -> Cont {
    unsafe {
        // 00404b24 push 406468h
        push(0x406468u32);
        // 00404b29 push edi
        push(MACHINE.regs.edi);
        // 00404b2a call esi
        call(0x404b2c, indirect(MACHINE.regs.esi))
    }
}

pub fn x00404b2c() -> Cont {
    unsafe {
        // 00404b2c push 406454h
        push(0x406454u32);
        // 00404b31 push edi
        push(MACHINE.regs.edi);
        // 00404b32 mov ds:[409718h],eax
        MACHINE.memory.write::<u32>(0x409718u32, MACHINE.regs.eax);
        // 00404b37 call esi
        call(0x404b39, indirect(MACHINE.regs.esi))
    }
}

pub fn x00404b39() -> Cont {
    unsafe {
        // 00404b39 mov ds:[40971Ch],eax
        MACHINE.memory.write::<u32>(0x40971cu32, MACHINE.regs.eax);
        // 00404b3e mov eax,ds:[409718h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409718u32);
        // 00404b43 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404b45 je short 00404B5Dh
        je(Cont(x00404b47), Cont(x00404b5d))
    }
}

pub fn x00404b3e() -> Cont {
    unsafe {
        // 00404b3e mov eax,ds:[409718h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409718u32);
        // 00404b43 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404b45 je short 00404B5Dh
        je(Cont(x00404b47), Cont(x00404b5d))
    }
}

pub fn x00404b47() -> Cont {
    unsafe {
        // 00404b47 call eax
        call(0x404b49, indirect(MACHINE.regs.eax))
    }
}

pub fn x00404b49() -> Cont {
    unsafe {
        // 00404b49 mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00404b4b test ebx,ebx
        and(MACHINE.regs.ebx, MACHINE.regs.ebx);
        // 00404b4d je short 00404B5Dh
        je(Cont(x00404b4f), Cont(x00404b5d))
    }
}

pub fn x00404b4f() -> Cont {
    unsafe {
        // 00404b4f mov eax,ds:[40971Ch]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x40971cu32);
        // 00404b54 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00404b56 je short 00404B5Dh
        je(Cont(x00404b58), Cont(x00404b5d))
    }
}

pub fn x00404b58() -> Cont {
    unsafe {
        // 00404b58 push ebx
        push(MACHINE.regs.ebx);
        // 00404b59 call eax
        call(0x404b5b, indirect(MACHINE.regs.eax))
    }
}

pub fn x00404b5b() -> Cont {
    unsafe {
        // 00404b5b mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 00404b5d push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b61 push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b65 push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b69 push ebx
        push(MACHINE.regs.ebx);
        // 00404b6a call dword ptr ds:[409714h]
        call(0x404b70, indirect(MACHINE.memory.read(0x409714u32)))
    }
}

pub fn x00404b5d() -> Cont {
    unsafe {
        // 00404b5d push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b61 push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b65 push dword ptr [esp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x18u32)),
        );
        // 00404b69 push ebx
        push(MACHINE.regs.ebx);
        // 00404b6a call dword ptr ds:[409714h]
        call(0x404b70, indirect(MACHINE.memory.read(0x409714u32)))
    }
}

pub fn x00404b70() -> Cont {
    unsafe {
        // 00404b70 pop edi
        MACHINE.regs.edi = pop();
        // 00404b71 pop esi
        MACHINE.regs.esi = pop();
        // 00404b72 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404b73 ret
        ret(0)
    }
}

pub fn x00404b74() -> Cont {
    unsafe {
        // 00404b74 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404b76 jmp short 00404B70h
        Cont(x00404b70)
    }
}

pub fn x00404b80() -> Cont {
    unsafe {
        // 00404b80 mov ecx,[esp+0Ch]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 00404b84 push edi
        push(MACHINE.regs.edi);
        // 00404b85 test ecx,ecx
        and(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 00404b87 je short 00404C03h
        je(Cont(x00404b89), Cont(x00404c03))
    }
}

pub fn x00404b89() -> Cont {
    unsafe {
        // 00404b89 push esi
        push(MACHINE.regs.esi);
        // 00404b8a push ebx
        push(MACHINE.regs.ebx);
        // 00404b8b mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 00404b8d mov esi,[esp+14h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x14u32));
        // 00404b91 test esi,3
        and(MACHINE.regs.esi, 0x3u32);
        // 00404b97 mov edi,[esp+10h]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 00404b9b jne short 00404BA4h
        jne(Cont(x00404b9d), Cont(x00404ba4))
    }
}

pub fn x00404b9d() -> Cont {
    unsafe {
        // 00404b9d shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404ba0 jne short 00404C11h
        jne(Cont(x00404ba2), Cont(x00404c11))
    }
}

pub fn x00404ba2() -> Cont {
    unsafe {
        // 00404ba2 jmp short 00404BC5h
        Cont(x00404bc5)
    }
}

pub fn x00404ba4() -> Cont {
    unsafe {
        // 00404ba4 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404ba6 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404ba7 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404ba9 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404baa dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404bab je short 00404BD2h
        je(Cont(x00404bad), Cont(x00404bd2))
    }
}

pub fn x00404bad() -> Cont {
    unsafe {
        // 00404bad test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404baf je short 00404BDAh
        je(Cont(x00404bb1), Cont(x00404bda))
    }
}

pub fn x00404bb1() -> Cont {
    unsafe {
        // 00404bb1 test esi,3
        and(MACHINE.regs.esi, 0x3u32);
        // 00404bb7 jne short 00404BA4h
        jne(Cont(x00404bb9), Cont(x00404ba4))
    }
}

pub fn x00404bb9() -> Cont {
    unsafe {
        // 00404bb9 mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 00404bbb shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404bbe jne short 00404C11h
        jne(Cont(x00404bc0), Cont(x00404c11))
    }
}

pub fn x00404bc0() -> Cont {
    unsafe {
        // 00404bc0 and ebx,3
        MACHINE.regs.ebx = and(MACHINE.regs.ebx, 0x3u32);
        // 00404bc3 je short 00404BD2h
        je(Cont(x00404bc5), Cont(x00404bd2))
    }
}

pub fn x00404bc5() -> Cont {
    unsafe {
        // 00404bc5 mov al,[esi]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esi.wrapping_add(0x0u32)),
        );
        // 00404bc7 inc esi
        MACHINE.regs.esi = inc(MACHINE.regs.esi);
        // 00404bc8 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404bca inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404bcb test al,al
        and(MACHINE.regs.get_al(), MACHINE.regs.get_al());
        // 00404bcd je short 00404BFEh
        je(Cont(x00404bcf), Cont(x00404bfe))
    }
}

pub fn x00404bcf() -> Cont {
    unsafe {
        // 00404bcf dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 00404bd0 jne short 00404BC5h
        jne(Cont(x00404bd2), Cont(x00404bc5))
    }
}

pub fn x00404bd2() -> Cont {
    unsafe {
        // 00404bd2 mov eax,[esp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 00404bd6 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404bd7 pop esi
        MACHINE.regs.esi = pop();
        // 00404bd8 pop edi
        MACHINE.regs.edi = pop();
        // 00404bd9 ret
        ret(0)
    }
}

pub fn x00404bda() -> Cont {
    unsafe {
        // 00404bda test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00404be0 je short 00404BF4h
        je(Cont(x00404be2), Cont(x00404bf4))
    }
}

pub fn x00404be2() -> Cont {
    unsafe {
        // 00404be2 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404be4 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404be5 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404be6 je near ptr 00404C76h
        je(Cont(x00404bec), Cont(x00404c76))
    }
}

pub fn x00404bec() -> Cont {
    unsafe {
        // 00404bec test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00404bf2 jne short 00404BE2h
        jne(Cont(x00404bf4), Cont(x00404be2))
    }
}

pub fn x00404bf4() -> Cont {
    unsafe {
        // 00404bf4 mov ebx,ecx
        MACHINE.regs.ebx = MACHINE.regs.ecx;
        // 00404bf6 shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404bf9 jne short 00404C67h
        jne(Cont(x00404bfb), Cont(x00404c67))
    }
}

pub fn x00404bfb() -> Cont {
    unsafe {
        // 00404bfb mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404bfd inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404bfe dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 00404bff jne short 00404BFBh
        jne(Cont(x00404c01), Cont(x00404bfb))
    }
}

pub fn x00404bfe() -> Cont {
    unsafe {
        // 00404bfe dec ebx
        MACHINE.regs.ebx = dec(MACHINE.regs.ebx);
        // 00404bff jne short 00404BFBh
        jne(Cont(x00404c01), Cont(x00404bfb))
    }
}

pub fn x00404c01() -> Cont {
    unsafe {
        // 00404c01 pop ebx
        MACHINE.regs.ebx = pop();
        // 00404c02 pop esi
        MACHINE.regs.esi = pop();
        // 00404c03 mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404c07 pop edi
        MACHINE.regs.edi = pop();
        // 00404c08 ret
        ret(0)
    }
}

pub fn x00404c03() -> Cont {
    unsafe {
        // 00404c03 mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00404c07 pop edi
        MACHINE.regs.edi = pop();
        // 00404c08 ret
        ret(0)
    }
}

pub fn x00404c09() -> Cont {
    unsafe {
        // 00404c09 mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404c0b add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404c0e dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404c0f je short 00404BC0h
        je(Cont(x00404c11), Cont(x00404bc0))
    }
}

pub fn x00404c11() -> Cont {
    unsafe {
        // 00404c11 mov edx,7EFEFEFFh
        MACHINE.regs.edx = 0x7efefeffu32;
        // 00404c16 mov eax,[esi]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00404c18 add edx,eax
        MACHINE.regs.edx = add(MACHINE.regs.edx, MACHINE.regs.eax);
        // 00404c1a xor eax,0FFFFFFFFh
        MACHINE.regs.eax ^= 0xffffffffu32;
        // 00404c1d xor eax,edx
        MACHINE.regs.eax ^= MACHINE.regs.edx;
        // 00404c1f mov edx,[esi]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esi.wrapping_add(0x0u32));
        // 00404c21 add esi,4
        MACHINE.regs.esi = add(MACHINE.regs.esi, 0x4u32);
        // 00404c24 test eax,81010100h
        and(MACHINE.regs.eax, 0x81010100u32);
        // 00404c29 je short 00404C09h
        je(Cont(x00404c2b), Cont(x00404c09))
    }
}

pub fn x00404c2b() -> Cont {
    unsafe {
        // 00404c2b test dl,dl
        and(MACHINE.regs.get_dl(), MACHINE.regs.get_dl());
        // 00404c2d je short 00404C5Bh
        je(Cont(x00404c2f), Cont(x00404c5b))
    }
}

pub fn x00404c2f() -> Cont {
    unsafe {
        // 00404c2f test dh,dh
        and(MACHINE.regs.get_dh(), MACHINE.regs.get_dh());
        // 00404c31 je short 00404C51h
        je(Cont(x00404c33), Cont(x00404c51))
    }
}

pub fn x00404c33() -> Cont {
    unsafe {
        // 00404c33 test edx,0FF0000h
        and(MACHINE.regs.edx, 0xff0000u32);
        // 00404c39 je short 00404C47h
        je(Cont(x00404c3b), Cont(x00404c47))
    }
}

pub fn x00404c3b() -> Cont {
    unsafe {
        // 00404c3b test edx,0FF000000h
        and(MACHINE.regs.edx, 0xff000000u32);
        // 00404c41 jne short 00404C09h
        jne(Cont(x00404c43), Cont(x00404c09))
    }
}

pub fn x00404c43() -> Cont {
    unsafe {
        // 00404c43 mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404c45 jmp short 00404C5Fh
        Cont(x00404c5f)
    }
}

pub fn x00404c47() -> Cont {
    unsafe {
        // 00404c47 and edx,0FFFFh
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0xffffu32);
        // 00404c4d mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404c4f jmp short 00404C5Fh
        Cont(x00404c5f)
    }
}

pub fn x00404c51() -> Cont {
    unsafe {
        // 00404c51 and edx,0FFh
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0xffu32);
        // 00404c57 mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404c59 jmp short 00404C5Fh
        Cont(x00404c5f)
    }
}

pub fn x00404c5b() -> Cont {
    unsafe {
        // 00404c5b xor edx,edx
        MACHINE.regs.edx ^= MACHINE.regs.edx;
        // 00404c5d mov [edi],edx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.edx);
        // 00404c5f add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404c62 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404c64 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404c65 je short 00404C71h
        je(Cont(x00404c67), Cont(x00404c71))
    }
}

pub fn x00404c5f() -> Cont {
    unsafe {
        // 00404c5f add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404c62 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404c64 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404c65 je short 00404C71h
        je(Cont(x00404c67), Cont(x00404c71))
    }
}

pub fn x00404c67() -> Cont {
    unsafe {
        // 00404c67 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404c69 mov [edi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00404c6b add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404c6e dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404c6f jne short 00404C69h
        jne(Cont(x00404c71), Cont(x00404c69))
    }
}

pub fn x00404c69() -> Cont {
    unsafe {
        // 00404c69 mov [edi],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.eax);
        // 00404c6b add edi,4
        MACHINE.regs.edi = add(MACHINE.regs.edi, 0x4u32);
        // 00404c6e dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404c6f jne short 00404C69h
        jne(Cont(x00404c71), Cont(x00404c69))
    }
}

pub fn x00404c71() -> Cont {
    unsafe {
        // 00404c71 and ebx,3
        MACHINE.regs.ebx = and(MACHINE.regs.ebx, 0x3u32);
        // 00404c74 jne short 00404BFBh
        jne(Cont(x00404c76), Cont(x00404bfb))
    }
}

pub fn x00404c76() -> Cont {
    unsafe {
        // 00404c76 mov eax,[esp+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x10u32));
        // 00404c7a pop ebx
        MACHINE.regs.ebx = pop();
        // 00404c7b pop esi
        MACHINE.regs.esi = pop();
        // 00404c7c pop edi
        MACHINE.regs.edi = pop();
        // 00404c7d ret
        ret(0)
    }
}

pub fn x00404c80() -> Cont {
    unsafe {
        // 00404c80 push ebp
        push(MACHINE.regs.ebp);
        // 00404c81 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00404c83 push edi
        push(MACHINE.regs.edi);
        // 00404c84 push esi
        push(MACHINE.regs.esi);
        // 00404c85 mov esi,[ebp+0Ch]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32));
        // 00404c88 mov ecx,[ebp+10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32));
        // 00404c8b mov edi,[ebp+8]
        MACHINE.regs.edi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00404c8e mov eax,ecx
        MACHINE.regs.eax = MACHINE.regs.ecx;
        // 00404c90 mov edx,ecx
        MACHINE.regs.edx = MACHINE.regs.ecx;
        // 00404c92 add eax,esi
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.esi);
        // 00404c94 cmp edi,esi
        sub(MACHINE.regs.edi, MACHINE.regs.esi);
        // 00404c96 jbe short 00404CA0h
        jbe(Cont(x00404c98), Cont(x00404ca0))
    }
}

pub fn x00404c98() -> Cont {
    unsafe {
        // 00404c98 cmp edi,eax
        sub(MACHINE.regs.edi, MACHINE.regs.eax);
        // 00404c9a jb near ptr 00404E18h
        jb(Cont(x00404ca0), Cont(x00404e18))
    }
}

pub fn x00404ca0() -> Cont {
    unsafe {
        // 00404ca0 test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00404ca6 jne short 00404CBCh
        jne(Cont(x00404ca8), Cont(x00404cbc))
    }
}

pub fn x00404ca8() -> Cont {
    unsafe {
        // 00404ca8 shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404cab and edx,3
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0x3u32);
        // 00404cae cmp ecx,8
        sub(MACHINE.regs.ecx, 0x8u32);
        // 00404cb1 jb short 00404CDCh
        jb(Cont(x00404cb3), Cont(x00404cdc))
    }
}

pub fn x00404cb3() -> Cont {
    unsafe {
        // 00404cb3 rep movsd
        movsd();
        // 00404cb5 jmp dword ptr [edx*4+404DC8h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.edx * 4).wrapping_add(0x404dc8u32)),
        )
    }
}

pub fn x00404cbc() -> Cont {
    unsafe {
        // 00404cbc mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00404cbe mov edx,3
        MACHINE.regs.edx = 0x3u32;
        // 00404cc3 sub ecx,4
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, 0x4u32);
        // 00404cc6 jb short 00404CD4h
        jb(Cont(x00404cc8), Cont(x00404cd4))
    }
}

pub fn x00404cc8() -> Cont {
    unsafe {
        // 00404cc8 and eax,3
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x3u32);
        // 00404ccb add ecx,eax
        MACHINE.regs.ecx = add(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00404ccd jmp dword ptr [eax*4+404CE0h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.eax * 4).wrapping_add(0x404ce0u32)),
        )
    }
}

pub fn x00404cd4() -> Cont {
    unsafe {
        // 00404cd4 jmp dword ptr [ecx*4+404DD8h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x404dd8u32)),
        )
    }
}

pub fn x00404cdc() -> Cont {
    unsafe {
        // 00404cdc jmp dword ptr [ecx*4+404D5Ch]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x404d5cu32)),
        )
    }
}

pub fn x00404e18() -> Cont {
    unsafe {
        // 00404e18 lea esi,[ecx+esi-4]
        MACHINE.regs.esi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00404e1c lea edi,[ecx+edi-4]
        MACHINE.regs.edi = MACHINE
            .regs
            .ecx
            .wrapping_add((MACHINE.regs.edi * 1))
            .wrapping_add(0xfffffffcu32);
        // 00404e20 test edi,3
        and(MACHINE.regs.edi, 0x3u32);
        // 00404e26 jne short 00404E4Ch
        jne(Cont(x00404e28), Cont(x00404e4c))
    }
}

pub fn x00404e28() -> Cont {
    unsafe {
        // 00404e28 shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404e2b and edx,3
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0x3u32);
        // 00404e2e cmp ecx,8
        sub(MACHINE.regs.ecx, 0x8u32);
        // 00404e31 jb short 00404E40h
        jb(Cont(x00404e33), Cont(x00404e40))
    }
}

pub fn x00404e33() -> Cont {
    unsafe {
        // 00404e33 std
        std();
        // 00404e34 rep movsd
        movsd();
        // 00404e36 cld
        cld();
        // 00404e37 jmp dword ptr [edx*4+404F60h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.edx * 4).wrapping_add(0x404f60u32)),
        )
    }
}

pub fn x00404e40() -> Cont {
    unsafe {
        // 00404e40 neg ecx
        MACHINE.regs.ecx = neg(MACHINE.regs.ecx);
        // 00404e42 jmp dword ptr [ecx*4+404F10h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x404f10u32)),
        )
    }
}

pub fn x00404e4c() -> Cont {
    unsafe {
        // 00404e4c mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00404e4e mov edx,3
        MACHINE.regs.edx = 0x3u32;
        // 00404e53 cmp ecx,4
        sub(MACHINE.regs.ecx, 0x4u32);
        // 00404e56 jb short 00404E64h
        jb(Cont(x00404e58), Cont(x00404e64))
    }
}

pub fn x00404e58() -> Cont {
    unsafe {
        // 00404e58 and eax,3
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x3u32);
        // 00404e5b sub ecx,eax
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.eax);
        // 00404e5d jmp dword ptr [eax*4+404E68h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.eax * 4).wrapping_add(0x404e68u32)),
        )
    }
}

pub fn x00404e64() -> Cont {
    unsafe {
        // 00404e64 jmp dword ptr [ecx*4+404F60h]
        indirect(
            MACHINE
                .memory
                .read((MACHINE.regs.ecx * 4).wrapping_add(0x404f60u32)),
        )
    }
}

pub fn x00404fc0() -> Cont {
    unsafe {
        // 00404fc0 mov edx,[esp+0Ch]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0xcu32));
        // 00404fc4 mov ecx,[esp+4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00404fc8 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00404fca je short 00405013h
        je(Cont(x00404fcc), Cont(x00405013))
    }
}

pub fn x00404fcc() -> Cont {
    unsafe {
        // 00404fcc xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 00404fce mov al,[esp+8]
        MACHINE.regs.set_al(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.esp.wrapping_add(0x8u32)),
        );
        // 00404fd2 push edi
        push(MACHINE.regs.edi);
        // 00404fd3 mov edi,ecx
        MACHINE.regs.edi = MACHINE.regs.ecx;
        // 00404fd5 cmp edx,4
        sub(MACHINE.regs.edx, 0x4u32);
        // 00404fd8 jb short 00405007h
        jb(Cont(x00404fda), Cont(x00405007))
    }
}

pub fn x00404fda() -> Cont {
    unsafe {
        // 00404fda neg ecx
        MACHINE.regs.ecx = neg(MACHINE.regs.ecx);
        // 00404fdc and ecx,3
        MACHINE.regs.ecx = and(MACHINE.regs.ecx, 0x3u32);
        // 00404fdf je short 00404FE9h
        je(Cont(x00404fe1), Cont(x00404fe9))
    }
}

pub fn x00404fe1() -> Cont {
    unsafe {
        // 00404fe1 sub edx,ecx
        MACHINE.regs.edx = sub(MACHINE.regs.edx, MACHINE.regs.ecx);
        // 00404fe3 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404fe5 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404fe6 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404fe7 jne short 00404FE3h
        jne(Cont(x00404fe9), Cont(x00404fe3))
    }
}

pub fn x00404fe3() -> Cont {
    unsafe {
        // 00404fe3 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00404fe5 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 00404fe6 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00404fe7 jne short 00404FE3h
        jne(Cont(x00404fe9), Cont(x00404fe3))
    }
}

pub fn x00404fe9() -> Cont {
    unsafe {
        // 00404fe9 mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 00404feb shl eax,8
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x8u8);
        // 00404fee add eax,ecx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00404ff0 mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 00404ff2 shl eax,10h
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x10u8);
        // 00404ff5 add eax,ecx
        MACHINE.regs.eax = add(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00404ff7 mov ecx,edx
        MACHINE.regs.ecx = MACHINE.regs.edx;
        // 00404ff9 and edx,3
        MACHINE.regs.edx = and(MACHINE.regs.edx, 0x3u32);
        // 00404ffc shr ecx,2
        MACHINE.regs.ecx = shr(MACHINE.regs.ecx, 0x2u8);
        // 00404fff je short 00405007h
        je(Cont(x00405001), Cont(x00405007))
    }
}

pub fn x00405001() -> Cont {
    unsafe {
        // 00405001 rep stosd
        stosd();
        // 00405003 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00405005 je short 0040500Dh
        je(Cont(x00405007), Cont(x0040500d))
    }
}

pub fn x00405007() -> Cont {
    unsafe {
        // 00405007 mov [edi],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.edi.wrapping_add(0x0u32), MACHINE.regs.get_al());
        // 00405009 inc edi
        MACHINE.regs.edi = inc(MACHINE.regs.edi);
        // 0040500a dec edx
        MACHINE.regs.edx = dec(MACHINE.regs.edx);
        // 0040500b jne short 00405007h
        jne(Cont(x0040500d), Cont(x00405007))
    }
}

pub fn x0040500d() -> Cont {
    unsafe {
        // 0040500d mov eax,[esp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00405011 pop edi
        MACHINE.regs.edi = pop();
        // 00405012 ret
        ret(0)
    }
}

pub fn x00405013() -> Cont {
    unsafe {
        // 00405013 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00405017 ret
        ret(0)
    }
}

pub fn x00405018() -> Cont {
    unsafe {
        // 00405018 push ebp
        push(MACHINE.regs.ebp);
        // 00405019 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 0040501b push 0FFFFFFFFh
        push(0xffffffffu32);
        // 0040501d push 406498h
        push(0x406498u32);
        // 00405022 push 4029A8h
        push(0x4029a8u32);
        // 00405027 mov eax,fs:[0]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32));
        // 0040502d push eax
        push(MACHINE.regs.eax);
        // 0040502e mov fs:[0],esp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.esp);
        // 00405035 sub esp,1Ch
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x1cu32);
        // 00405038 push ebx
        push(MACHINE.regs.ebx);
        // 00405039 push esi
        push(MACHINE.regs.esi);
        // 0040503a push edi
        push(MACHINE.regs.edi);
        // 0040503b mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 0040503e xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00405040 cmp ds:[409740h],edi
        sub(MACHINE.memory.read::<u32>(0x409740u32), MACHINE.regs.edi);
        // 00405046 jne short 0040508Eh
        jne(Cont(x00405048), Cont(x0040508e))
    }
}

pub fn x00405048() -> Cont {
    unsafe {
        // 00405048 push edi
        push(MACHINE.regs.edi);
        // 00405049 push edi
        push(MACHINE.regs.edi);
        // 0040504a push 1
        push(0x1u32);
        // 0040504c pop ebx
        MACHINE.regs.ebx = pop();
        // 0040504d push ebx
        push(MACHINE.regs.ebx);
        // 0040504e push 406494h
        push(0x406494u32);
        // 00405053 mov esi,100h
        MACHINE.regs.esi = 0x100u32;
        // 00405058 push esi
        push(MACHINE.regs.esi);
        // 00405059 push edi
        push(MACHINE.regs.edi);
        // 0040505a call dword ptr ds:[40603Ch]
        call(0x405060, Cont(kernel32::LCMapStringW_stdcall))
    }
}

pub fn x00405060() -> Cont {
    unsafe {
        // 00405060 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405062 je short 0040506Ch
        je(Cont(x00405064), Cont(x0040506c))
    }
}

pub fn x00405064() -> Cont {
    unsafe {
        // 00405064 mov ds:[409740h],ebx
        MACHINE.memory.write::<u32>(0x409740u32, MACHINE.regs.ebx);
        // 0040506a jmp short 0040508Eh
        Cont(x0040508e)
    }
}

pub fn x0040506c() -> Cont {
    unsafe {
        // 0040506c push edi
        push(MACHINE.regs.edi);
        // 0040506d push edi
        push(MACHINE.regs.edi);
        // 0040506e push ebx
        push(MACHINE.regs.ebx);
        // 0040506f push 406490h
        push(0x406490u32);
        // 00405074 push esi
        push(MACHINE.regs.esi);
        // 00405075 push edi
        push(MACHINE.regs.edi);
        // 00405076 call dword ptr ds:[406040h]
        call(0x40507c, Cont(kernel32::LCMapStringA_stdcall))
    }
}

pub fn x0040507c() -> Cont {
    unsafe {
        // 0040507c test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040507e je near ptr 004051A6h
        je(Cont(x00405084), Cont(x004051a6))
    }
}

pub fn x00405084() -> Cont {
    unsafe {
        // 00405084 mov dword ptr ds:[409740h],2
        MACHINE.memory.write::<u32>(0x409740u32, 0x2u32);
        // 0040508e cmp [ebp+14h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
            MACHINE.regs.edi,
        );
        // 00405091 jle short 004050A3h
        jle(Cont(x00405093), Cont(x004050a3))
    }
}

pub fn x0040508e() -> Cont {
    unsafe {
        // 0040508e cmp [ebp+14h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
            MACHINE.regs.edi,
        );
        // 00405091 jle short 004050A3h
        jle(Cont(x00405093), Cont(x004050a3))
    }
}

pub fn x00405093() -> Cont {
    unsafe {
        // 00405093 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00405096 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 00405099 call 0040523Ch
        call(0x40509e, Cont(x0040523c))
    }
}

pub fn x0040509e() -> Cont {
    unsafe {
        // 0040509e pop ecx
        MACHINE.regs.ecx = pop();
        // 0040509f pop ecx
        MACHINE.regs.ecx = pop();
        // 004050a0 mov [ebp+14h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32), MACHINE.regs.eax);
        // 004050a3 mov eax,ds:[409740h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409740u32);
        // 004050a8 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004050ab jne short 004050CAh
        jne(Cont(x004050ad), Cont(x004050ca))
    }
}

pub fn x004050a3() -> Cont {
    unsafe {
        // 004050a3 mov eax,ds:[409740h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409740u32);
        // 004050a8 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004050ab jne short 004050CAh
        jne(Cont(x004050ad), Cont(x004050ca))
    }
}

pub fn x004050ad() -> Cont {
    unsafe {
        // 004050ad push dword ptr [ebp+1Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
        );
        // 004050b0 push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 004050b3 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 004050b6 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 004050b9 push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004050bc push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 004050bf call dword ptr ds:[406040h]
        call(0x4050c5, Cont(kernel32::LCMapStringA_stdcall))
    }
}

pub fn x004050c5() -> Cont {
    unsafe {
        // 004050c5 jmp near ptr 004051A8h
        Cont(x004051a8)
    }
}

pub fn x004050ca() -> Cont {
    unsafe {
        // 004050ca cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 004050cd jne near ptr 004051A6h
        jne(Cont(x004050d3), Cont(x004051a6))
    }
}

pub fn x004050d3() -> Cont {
    unsafe {
        // 004050d3 cmp [ebp+20h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
            MACHINE.regs.edi,
        );
        // 004050d6 jne short 004050E0h
        jne(Cont(x004050d8), Cont(x004050e0))
    }
}

pub fn x004050d8() -> Cont {
    unsafe {
        // 004050d8 mov eax,ds:[409738h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409738u32);
        // 004050dd mov [ebp+20h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32), MACHINE.regs.eax);
        // 004050e0 push edi
        push(MACHINE.regs.edi);
        // 004050e1 push edi
        push(MACHINE.regs.edi);
        // 004050e2 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 004050e5 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 004050e8 mov eax,[ebp+24h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x24u32));
        // 004050eb neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 004050ed sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004050ef and eax,8
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x8u32);
        // 004050f2 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004050f3 push eax
        push(MACHINE.regs.eax);
        // 004050f4 push dword ptr [ebp+20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
        );
        // 004050f7 call dword ptr ds:[406044h]
        call(0x4050fd, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x004050e0() -> Cont {
    unsafe {
        // 004050e0 push edi
        push(MACHINE.regs.edi);
        // 004050e1 push edi
        push(MACHINE.regs.edi);
        // 004050e2 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 004050e5 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 004050e8 mov eax,[ebp+24h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x24u32));
        // 004050eb neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 004050ed sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004050ef and eax,8
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x8u32);
        // 004050f2 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 004050f3 push eax
        push(MACHINE.regs.eax);
        // 004050f4 push dword ptr [ebp+20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
        );
        // 004050f7 call dword ptr ds:[406044h]
        call(0x4050fd, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x004050fd() -> Cont {
    unsafe {
        // 004050fd mov ebx,eax
        MACHINE.regs.ebx = MACHINE.regs.eax;
        // 004050ff mov [ebp-1Ch],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe4u32),
            MACHINE.regs.ebx,
        );
        // 00405102 cmp ebx,edi
        sub(MACHINE.regs.ebx, MACHINE.regs.edi);
        // 00405104 je near ptr 004051A6h
        je(Cont(x0040510a), Cont(x004051a6))
    }
}

pub fn x0040510a() -> Cont {
    unsafe {
        // 0040510a mov [ebp-4],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.edi,
        );
        // 0040510d lea eax,[ebx+ebx]
        MACHINE.regs.eax = MACHINE
            .regs
            .ebx
            .wrapping_add((MACHINE.regs.ebx * 1))
            .wrapping_add(0x0u32);
        // 00405110 add eax,3
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x3u32);
        // 00405113 and al,0FCh
        MACHINE.regs.set_al(and(MACHINE.regs.get_al(), 0xfcu8));
        // 00405115 call 00404AC0h
        call(0x40511a, Cont(x00404ac0))
    }
}

pub fn x0040511a() -> Cont {
    unsafe {
        // 0040511a mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 0040511d mov eax,esp
        MACHINE.regs.eax = MACHINE.regs.esp;
        // 0040511f mov [ebp-24h],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffdcu32),
            MACHINE.regs.eax,
        );
        // 00405122 or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 00405126 jmp short 0040513Bh
        Cont(x0040513b)
    }
}

pub fn x00405128() -> Cont {
    unsafe {
        // 00405128 push 1
        push(0x1u32);
        // 0040512a pop eax
        MACHINE.regs.eax = pop();
        // 0040512b ret
        ret(0)
    }
}

pub fn x0040512c() -> Cont {
    unsafe {
        // 0040512c mov esp,[ebp-18h]
        MACHINE.regs.esp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe8u32));
        // 0040512f xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00405131 mov [ebp-24h],edi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffdcu32),
            MACHINE.regs.edi,
        );
        // 00405134 or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 00405138 mov ebx,[ebp-1Ch]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe4u32));
        // 0040513b cmp [ebp-24h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
            MACHINE.regs.edi,
        );
        // 0040513e je short 004051A6h
        je(Cont(x00405140), Cont(x004051a6))
    }
}

pub fn x0040513b() -> Cont {
    unsafe {
        // 0040513b cmp [ebp-24h],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
            MACHINE.regs.edi,
        );
        // 0040513e je short 004051A6h
        je(Cont(x00405140), Cont(x004051a6))
    }
}

pub fn x00405140() -> Cont {
    unsafe {
        // 00405140 push ebx
        push(MACHINE.regs.ebx);
        // 00405141 push dword ptr [ebp-24h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
        );
        // 00405144 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 00405147 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040514a push 1
        push(0x1u32);
        // 0040514c push dword ptr [ebp+20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
        );
        // 0040514f call dword ptr ds:[406044h]
        call(0x405155, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x00405155() -> Cont {
    unsafe {
        // 00405155 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405157 je short 004051A6h
        je(Cont(x00405159), Cont(x004051a6))
    }
}

pub fn x00405159() -> Cont {
    unsafe {
        // 00405159 push edi
        push(MACHINE.regs.edi);
        // 0040515a push edi
        push(MACHINE.regs.edi);
        // 0040515b push ebx
        push(MACHINE.regs.ebx);
        // 0040515c push dword ptr [ebp-24h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
        );
        // 0040515f push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00405162 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00405165 call dword ptr ds:[40603Ch]
        call(0x40516b, Cont(kernel32::LCMapStringW_stdcall))
    }
}

pub fn x0040516b() -> Cont {
    unsafe {
        // 0040516b mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040516d mov [ebp-28h],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffd8u32),
            MACHINE.regs.esi,
        );
        // 00405170 cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 00405172 je short 004051A6h
        je(Cont(x00405174), Cont(x004051a6))
    }
}

pub fn x00405174() -> Cont {
    unsafe {
        // 00405174 test byte ptr [ebp+0Dh],4
        and(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xdu32)),
            0x4u8,
        );
        // 00405178 je short 004051BAh
        je(Cont(x0040517a), Cont(x004051ba))
    }
}

pub fn x0040517a() -> Cont {
    unsafe {
        // 0040517a cmp [ebp+1Ch],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
            MACHINE.regs.edi,
        );
        // 0040517d je near ptr 00405235h
        je(Cont(x00405183), Cont(x00405235))
    }
}

pub fn x00405183() -> Cont {
    unsafe {
        // 00405183 cmp esi,[ebp+1Ch]
        sub(
            MACHINE.regs.esi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
        );
        // 00405186 jg short 004051A6h
        jg(Cont(x00405188), Cont(x004051a6))
    }
}

pub fn x00405188() -> Cont {
    unsafe {
        // 00405188 push dword ptr [ebp+1Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
        );
        // 0040518b push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 0040518e push ebx
        push(MACHINE.regs.ebx);
        // 0040518f push dword ptr [ebp-24h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
        );
        // 00405192 push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 00405195 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00405198 call dword ptr ds:[40603Ch]
        call(0x40519e, Cont(kernel32::LCMapStringW_stdcall))
    }
}

pub fn x0040519e() -> Cont {
    unsafe {
        // 0040519e test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004051a0 jne near ptr 00405235h
        jne(Cont(x004051a6), Cont(x00405235))
    }
}

pub fn x004051a6() -> Cont {
    unsafe {
        // 004051a6 xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 004051a8 lea esp,[ebp-38h]
        MACHINE.regs.esp = MACHINE.regs.ebp.wrapping_add(0xffffffc8u32);
        // 004051ab mov ecx,[ebp-10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 004051ae mov fs:[0],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004051b5 pop edi
        MACHINE.regs.edi = pop();
        // 004051b6 pop esi
        MACHINE.regs.esi = pop();
        // 004051b7 pop ebx
        MACHINE.regs.ebx = pop();
        // 004051b8 leave
        leave();
        // 004051b9 ret
        ret(0)
    }
}

pub fn x004051a8() -> Cont {
    unsafe {
        // 004051a8 lea esp,[ebp-38h]
        MACHINE.regs.esp = MACHINE.regs.ebp.wrapping_add(0xffffffc8u32);
        // 004051ab mov ecx,[ebp-10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 004051ae mov fs:[0],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004051b5 pop edi
        MACHINE.regs.edi = pop();
        // 004051b6 pop esi
        MACHINE.regs.esi = pop();
        // 004051b7 pop ebx
        MACHINE.regs.ebx = pop();
        // 004051b8 leave
        leave();
        // 004051b9 ret
        ret(0)
    }
}

pub fn x004051ba() -> Cont {
    unsafe {
        // 004051ba mov dword ptr [ebp-4],1
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32), 0x1u32);
        // 004051c1 lea eax,[esi+esi]
        MACHINE.regs.eax = MACHINE
            .regs
            .esi
            .wrapping_add((MACHINE.regs.esi * 1))
            .wrapping_add(0x0u32);
        // 004051c4 add eax,3
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x3u32);
        // 004051c7 and al,0FCh
        MACHINE.regs.set_al(and(MACHINE.regs.get_al(), 0xfcu8));
        // 004051c9 call 00404AC0h
        call(0x4051ce, Cont(x00404ac0))
    }
}

pub fn x004051ce() -> Cont {
    unsafe {
        // 004051ce mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 004051d1 mov ebx,esp
        MACHINE.regs.ebx = MACHINE.regs.esp;
        // 004051d3 mov [ebp-20h],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe0u32),
            MACHINE.regs.ebx,
        );
        // 004051d6 or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 004051da jmp short 004051EEh
        Cont(x004051ee)
    }
}

pub fn x004051dc() -> Cont {
    unsafe {
        // 004051dc push 1
        push(0x1u32);
        // 004051de pop eax
        MACHINE.regs.eax = pop();
        // 004051df ret
        ret(0)
    }
}

pub fn x004051e0() -> Cont {
    unsafe {
        // 004051e0 mov esp,[ebp-18h]
        MACHINE.regs.esp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe8u32));
        // 004051e3 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 004051e5 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 004051e7 or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 004051eb mov esi,[ebp-28h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffd8u32));
        // 004051ee cmp ebx,edi
        sub(MACHINE.regs.ebx, MACHINE.regs.edi);
        // 004051f0 je short 004051A6h
        je(Cont(x004051f2), Cont(x004051a6))
    }
}

pub fn x004051ee() -> Cont {
    unsafe {
        // 004051ee cmp ebx,edi
        sub(MACHINE.regs.ebx, MACHINE.regs.edi);
        // 004051f0 je short 004051A6h
        je(Cont(x004051f2), Cont(x004051a6))
    }
}

pub fn x004051f2() -> Cont {
    unsafe {
        // 004051f2 push esi
        push(MACHINE.regs.esi);
        // 004051f3 push ebx
        push(MACHINE.regs.ebx);
        // 004051f4 push dword ptr [ebp-1Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe4u32)),
        );
        // 004051f7 push dword ptr [ebp-24h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffdcu32)),
        );
        // 004051fa push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004051fd push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00405200 call dword ptr ds:[40603Ch]
        call(0x405206, Cont(kernel32::LCMapStringW_stdcall))
    }
}

pub fn x00405206() -> Cont {
    unsafe {
        // 00405206 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405208 je short 004051A6h
        je(Cont(x0040520a), Cont(x004051a6))
    }
}

pub fn x0040520a() -> Cont {
    unsafe {
        // 0040520a cmp [ebp+1Ch],edi
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
            MACHINE.regs.edi,
        );
        // 0040520d push edi
        push(MACHINE.regs.edi);
        // 0040520e push edi
        push(MACHINE.regs.edi);
        // 0040520f jne short 00405215h
        jne(Cont(x00405211), Cont(x00405215))
    }
}

pub fn x00405211() -> Cont {
    unsafe {
        // 00405211 push edi
        push(MACHINE.regs.edi);
        // 00405212 push edi
        push(MACHINE.regs.edi);
        // 00405213 jmp short 0040521Bh
        Cont(x0040521b)
    }
}

pub fn x00405215() -> Cont {
    unsafe {
        // 00405215 push dword ptr [ebp+1Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32)),
        );
        // 00405218 push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 0040521b push esi
        push(MACHINE.regs.esi);
        // 0040521c push ebx
        push(MACHINE.regs.ebx);
        // 0040521d push 220h
        push(0x220u32);
        // 00405222 push dword ptr [ebp+20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
        );
        // 00405225 call dword ptr ds:[4060A4h]
        call(0x40522b, Cont(kernel32::WideCharToMultiByte_stdcall))
    }
}

pub fn x0040521b() -> Cont {
    unsafe {
        // 0040521b push esi
        push(MACHINE.regs.esi);
        // 0040521c push ebx
        push(MACHINE.regs.ebx);
        // 0040521d push 220h
        push(0x220u32);
        // 00405222 push dword ptr [ebp+20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32)),
        );
        // 00405225 call dword ptr ds:[4060A4h]
        call(0x40522b, Cont(kernel32::WideCharToMultiByte_stdcall))
    }
}

pub fn x0040522b() -> Cont {
    unsafe {
        // 0040522b mov esi,eax
        MACHINE.regs.esi = MACHINE.regs.eax;
        // 0040522d cmp esi,edi
        sub(MACHINE.regs.esi, MACHINE.regs.edi);
        // 0040522f je near ptr 004051A6h
        je(Cont(x00405235), Cont(x004051a6))
    }
}

pub fn x00405235() -> Cont {
    unsafe {
        // 00405235 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 00405237 jmp near ptr 004051A8h
        Cont(x004051a8)
    }
}

pub fn x0040523c() -> Cont {
    unsafe {
        // 0040523c mov edx,[esp+8]
        MACHINE.regs.edx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x8u32));
        // 00405240 mov eax,[esp+4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32));
        // 00405244 test edx,edx
        and(MACHINE.regs.edx, MACHINE.regs.edx);
        // 00405246 push esi
        push(MACHINE.regs.esi);
        // 00405247 lea ecx,[edx-1]
        MACHINE.regs.ecx = MACHINE.regs.edx.wrapping_add(0xffffffffu32);
        // 0040524a je short 00405259h
        je(Cont(x0040524c), Cont(x00405259))
    }
}

pub fn x0040524c() -> Cont {
    unsafe {
        // 0040524c cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 0040524f je short 00405259h
        je(Cont(x00405251), Cont(x00405259))
    }
}

pub fn x00405251() -> Cont {
    unsafe {
        // 00405251 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00405252 mov esi,ecx
        MACHINE.regs.esi = MACHINE.regs.ecx;
        // 00405254 dec ecx
        MACHINE.regs.ecx = dec(MACHINE.regs.ecx);
        // 00405255 test esi,esi
        and(MACHINE.regs.esi, MACHINE.regs.esi);
        // 00405257 jne short 0040524Ch
        jne(Cont(x00405259), Cont(x0040524c))
    }
}

pub fn x00405259() -> Cont {
    unsafe {
        // 00405259 cmp byte ptr [eax],0
        sub(
            MACHINE
                .memory
                .read::<u8>(MACHINE.regs.eax.wrapping_add(0x0u32)),
            0x0u8,
        );
        // 0040525c pop esi
        MACHINE.regs.esi = pop();
        // 0040525d jne short 00405264h
        jne(Cont(x0040525f), Cont(x00405264))
    }
}

pub fn x0040525f() -> Cont {
    unsafe {
        // 0040525f sub eax,[esp+4]
        MACHINE.regs.eax = sub(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x4u32)),
        );
        // 00405263 ret
        ret(0)
    }
}

pub fn x00405264() -> Cont {
    unsafe {
        // 00405264 mov eax,edx
        MACHINE.regs.eax = MACHINE.regs.edx;
        // 00405266 ret
        ret(0)
    }
}

pub fn x00405267() -> Cont {
    unsafe {
        // 00405267 push ebp
        push(MACHINE.regs.ebp);
        // 00405268 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 0040526a push 0FFFFFFFFh
        push(0xffffffffu32);
        // 0040526c push 4064B0h
        push(0x4064b0u32);
        // 00405271 push 4029A8h
        push(0x4029a8u32);
        // 00405276 mov eax,fs:[0]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32));
        // 0040527c push eax
        push(MACHINE.regs.eax);
        // 0040527d mov fs:[0],esp
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.esp);
        // 00405284 sub esp,18h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x18u32);
        // 00405287 push ebx
        push(MACHINE.regs.ebx);
        // 00405288 push esi
        push(MACHINE.regs.esi);
        // 00405289 push edi
        push(MACHINE.regs.edi);
        // 0040528a mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 0040528d mov eax,ds:[409744h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409744u32);
        // 00405292 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00405294 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00405296 jne short 004052D6h
        jne(Cont(x00405298), Cont(x004052d6))
    }
}

pub fn x00405298() -> Cont {
    unsafe {
        // 00405298 lea eax,[ebp-1Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffffe4u32);
        // 0040529b push eax
        push(MACHINE.regs.eax);
        // 0040529c push 1
        push(0x1u32);
        // 0040529e pop esi
        MACHINE.regs.esi = pop();
        // 0040529f push esi
        push(MACHINE.regs.esi);
        // 004052a0 push 406494h
        push(0x406494u32);
        // 004052a5 push esi
        push(MACHINE.regs.esi);
        // 004052a6 call dword ptr ds:[406034h]
        call(0x4052ac, Cont(kernel32::GetStringTypeW_stdcall))
    }
}

pub fn x004052ac() -> Cont {
    unsafe {
        // 004052ac test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004052ae je short 004052B4h
        je(Cont(x004052b0), Cont(x004052b4))
    }
}

pub fn x004052b0() -> Cont {
    unsafe {
        // 004052b0 mov eax,esi
        MACHINE.regs.eax = MACHINE.regs.esi;
        // 004052b2 jmp short 004052D1h
        Cont(x004052d1)
    }
}

pub fn x004052b4() -> Cont {
    unsafe {
        // 004052b4 lea eax,[ebp-1Ch]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xffffffe4u32);
        // 004052b7 push eax
        push(MACHINE.regs.eax);
        // 004052b8 push esi
        push(MACHINE.regs.esi);
        // 004052b9 push 406490h
        push(0x406490u32);
        // 004052be push esi
        push(MACHINE.regs.esi);
        // 004052bf push ebx
        push(MACHINE.regs.ebx);
        // 004052c0 call dword ptr ds:[406038h]
        call(0x4052c6, Cont(kernel32::GetStringTypeA_stdcall))
    }
}

pub fn x004052c6() -> Cont {
    unsafe {
        // 004052c6 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004052c8 je near ptr 0040539Ch
        je(Cont(x004052ce), Cont(x0040539c))
    }
}

pub fn x004052ce() -> Cont {
    unsafe {
        // 004052ce push 2
        push(0x2u32);
        // 004052d0 pop eax
        MACHINE.regs.eax = pop();
        // 004052d1 mov ds:[409744h],eax
        MACHINE.memory.write::<u32>(0x409744u32, MACHINE.regs.eax);
        // 004052d6 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004052d9 jne short 004052FFh
        jne(Cont(x004052db), Cont(x004052ff))
    }
}

pub fn x004052d1() -> Cont {
    unsafe {
        // 004052d1 mov ds:[409744h],eax
        MACHINE.memory.write::<u32>(0x409744u32, MACHINE.regs.eax);
        // 004052d6 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004052d9 jne short 004052FFh
        jne(Cont(x004052db), Cont(x004052ff))
    }
}

pub fn x004052d6() -> Cont {
    unsafe {
        // 004052d6 cmp eax,2
        sub(MACHINE.regs.eax, 0x2u32);
        // 004052d9 jne short 004052FFh
        jne(Cont(x004052db), Cont(x004052ff))
    }
}

pub fn x004052db() -> Cont {
    unsafe {
        // 004052db mov eax,[ebp+1Ch]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x1cu32));
        // 004052de cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 004052e0 jne short 004052E7h
        jne(Cont(x004052e2), Cont(x004052e7))
    }
}

pub fn x004052e2() -> Cont {
    unsafe {
        // 004052e2 mov eax,ds:[409728h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409728u32);
        // 004052e7 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 004052ea push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 004052ed push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004052f0 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 004052f3 push eax
        push(MACHINE.regs.eax);
        // 004052f4 call dword ptr ds:[406038h]
        call(0x4052fa, Cont(kernel32::GetStringTypeA_stdcall))
    }
}

pub fn x004052e7() -> Cont {
    unsafe {
        // 004052e7 push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 004052ea push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 004052ed push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004052f0 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 004052f3 push eax
        push(MACHINE.regs.eax);
        // 004052f4 call dword ptr ds:[406038h]
        call(0x4052fa, Cont(kernel32::GetStringTypeA_stdcall))
    }
}

pub fn x004052fa() -> Cont {
    unsafe {
        // 004052fa jmp near ptr 0040539Eh
        Cont(x0040539e)
    }
}

pub fn x004052ff() -> Cont {
    unsafe {
        // 004052ff cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00405302 jne near ptr 0040539Ch
        jne(Cont(x00405308), Cont(x0040539c))
    }
}

pub fn x00405308() -> Cont {
    unsafe {
        // 00405308 cmp [ebp+18h],ebx
        sub(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
            MACHINE.regs.ebx,
        );
        // 0040530b jne short 00405315h
        jne(Cont(x0040530d), Cont(x00405315))
    }
}

pub fn x0040530d() -> Cont {
    unsafe {
        // 0040530d mov eax,ds:[409738h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409738u32);
        // 00405312 mov [ebp+18h],eax
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32), MACHINE.regs.eax);
        // 00405315 push ebx
        push(MACHINE.regs.ebx);
        // 00405316 push ebx
        push(MACHINE.regs.ebx);
        // 00405317 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040531a push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 0040531d mov eax,[ebp+20h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32));
        // 00405320 neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 00405322 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405324 and eax,8
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x8u32);
        // 00405327 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00405328 push eax
        push(MACHINE.regs.eax);
        // 00405329 push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 0040532c call dword ptr ds:[406044h]
        call(0x405332, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x00405315() -> Cont {
    unsafe {
        // 00405315 push ebx
        push(MACHINE.regs.ebx);
        // 00405316 push ebx
        push(MACHINE.regs.ebx);
        // 00405317 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040531a push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 0040531d mov eax,[ebp+20h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x20u32));
        // 00405320 neg eax
        MACHINE.regs.eax = neg(MACHINE.regs.eax);
        // 00405322 sbb eax,eax
        MACHINE.regs.eax = sbb(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405324 and eax,8
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x8u32);
        // 00405327 inc eax
        MACHINE.regs.eax = inc(MACHINE.regs.eax);
        // 00405328 push eax
        push(MACHINE.regs.eax);
        // 00405329 push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 0040532c call dword ptr ds:[406044h]
        call(0x405332, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x00405332() -> Cont {
    unsafe {
        // 00405332 mov [ebp-20h],eax
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe0u32),
            MACHINE.regs.eax,
        );
        // 00405335 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 00405337 je short 0040539Ch
        je(Cont(x00405339), Cont(x0040539c))
    }
}

pub fn x00405339() -> Cont {
    unsafe {
        // 00405339 mov [ebp-4],ebx
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.ebx,
        );
        // 0040533c lea edi,[eax+eax]
        MACHINE.regs.edi = MACHINE
            .regs
            .eax
            .wrapping_add((MACHINE.regs.eax * 1))
            .wrapping_add(0x0u32);
        // 0040533f mov eax,edi
        MACHINE.regs.eax = MACHINE.regs.edi;
        // 00405341 add eax,3
        MACHINE.regs.eax = add(MACHINE.regs.eax, 0x3u32);
        // 00405344 and al,0FCh
        MACHINE.regs.set_al(and(MACHINE.regs.get_al(), 0xfcu8));
        // 00405346 call 00404AC0h
        call(0x40534b, Cont(x00404ac0))
    }
}

pub fn x0040534b() -> Cont {
    unsafe {
        // 0040534b mov [ebp-18h],esp
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffe8u32),
            MACHINE.regs.esp,
        );
        // 0040534e mov esi,esp
        MACHINE.regs.esi = MACHINE.regs.esp;
        // 00405350 mov [ebp-24h],esi
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xffffffdcu32),
            MACHINE.regs.esi,
        );
        // 00405353 push edi
        push(MACHINE.regs.edi);
        // 00405354 push ebx
        push(MACHINE.regs.ebx);
        // 00405355 push esi
        push(MACHINE.regs.esi);
        // 00405356 call 00404FC0h
        call(0x40535b, Cont(x00404fc0))
    }
}

pub fn x0040535b() -> Cont {
    unsafe {
        // 0040535b add esp,0Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0xcu32);
        // 0040535e jmp short 0040536Bh
        Cont(x0040536b)
    }
}

pub fn x00405360() -> Cont {
    unsafe {
        // 00405360 push 1
        push(0x1u32);
        // 00405362 pop eax
        MACHINE.regs.eax = pop();
        // 00405363 ret
        ret(0)
    }
}

pub fn x00405364() -> Cont {
    unsafe {
        // 00405364 mov esp,[ebp-18h]
        MACHINE.regs.esp = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe8u32));
        // 00405367 xor ebx,ebx
        MACHINE.regs.ebx ^= MACHINE.regs.ebx;
        // 00405369 xor esi,esi
        MACHINE.regs.esi ^= MACHINE.regs.esi;
        // 0040536b or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 0040536f cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00405371 je short 0040539Ch
        je(Cont(x00405373), Cont(x0040539c))
    }
}

pub fn x0040536b() -> Cont {
    unsafe {
        // 0040536b or dword ptr [ebp-4],0FFFFFFFFh
        MACHINE.memory.write::<u32>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            or(
                MACHINE
                    .memory
                    .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32)),
                0xffffffffu32,
            ),
        );
        // 0040536f cmp esi,ebx
        sub(MACHINE.regs.esi, MACHINE.regs.ebx);
        // 00405371 je short 0040539Ch
        je(Cont(x00405373), Cont(x0040539c))
    }
}

pub fn x00405373() -> Cont {
    unsafe {
        // 00405373 push dword ptr [ebp-20h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xffffffe0u32)),
        );
        // 00405376 push esi
        push(MACHINE.regs.esi);
        // 00405377 push dword ptr [ebp+10h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x10u32)),
        );
        // 0040537a push dword ptr [ebp+0Ch]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 0040537d push 1
        push(0x1u32);
        // 0040537f push dword ptr [ebp+18h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x18u32)),
        );
        // 00405382 call dword ptr ds:[406044h]
        call(0x405388, Cont(kernel32::MultiByteToWideChar_stdcall))
    }
}

pub fn x00405388() -> Cont {
    unsafe {
        // 00405388 cmp eax,ebx
        sub(MACHINE.regs.eax, MACHINE.regs.ebx);
        // 0040538a je short 0040539Ch
        je(Cont(x0040538c), Cont(x0040539c))
    }
}

pub fn x0040538c() -> Cont {
    unsafe {
        // 0040538c push dword ptr [ebp+14h]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x14u32)),
        );
        // 0040538f push eax
        push(MACHINE.regs.eax);
        // 00405390 push esi
        push(MACHINE.regs.esi);
        // 00405391 push dword ptr [ebp+8]
        push(
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32)),
        );
        // 00405394 call dword ptr ds:[406034h]
        call(0x40539a, Cont(kernel32::GetStringTypeW_stdcall))
    }
}

pub fn x0040539a() -> Cont {
    unsafe {
        // 0040539a jmp short 0040539Eh
        Cont(x0040539e)
    }
}

pub fn x0040539c() -> Cont {
    unsafe {
        // 0040539c xor eax,eax
        MACHINE.regs.eax ^= MACHINE.regs.eax;
        // 0040539e lea esp,[ebp-34h]
        MACHINE.regs.esp = MACHINE.regs.ebp.wrapping_add(0xffffffccu32);
        // 004053a1 mov ecx,[ebp-10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 004053a4 mov fs:[0],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004053ab pop edi
        MACHINE.regs.edi = pop();
        // 004053ac pop esi
        MACHINE.regs.esi = pop();
        // 004053ad pop ebx
        MACHINE.regs.ebx = pop();
        // 004053ae leave
        leave();
        // 004053af ret
        ret(0)
    }
}

pub fn x0040539e() -> Cont {
    unsafe {
        // 0040539e lea esp,[ebp-34h]
        MACHINE.regs.esp = MACHINE.regs.ebp.wrapping_add(0xffffffccu32);
        // 004053a1 mov ecx,[ebp-10h]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xfffffff0u32));
        // 004053a4 mov fs:[0],ecx
        MACHINE
            .memory
            .write::<u32>(MACHINE.regs.fs_base.wrapping_add(0x0u32), MACHINE.regs.ecx);
        // 004053ab pop edi
        MACHINE.regs.edi = pop();
        // 004053ac pop esi
        MACHINE.regs.esi = pop();
        // 004053ad pop ebx
        MACHINE.regs.ebx = pop();
        // 004053ae leave
        leave();
        // 004053af ret
        ret(0)
    }
}

pub fn x004053b0() -> Cont {
    unsafe {
        // 004053b0 push ebp
        push(MACHINE.regs.ebp);
        // 004053b1 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 004053b3 push ecx
        push(MACHINE.regs.ecx);
        // 004053b4 cmp dword ptr ds:[409728h],0
        sub(MACHINE.memory.read::<u32>(0x409728u32), 0x0u32);
        // 004053bb push ebx
        push(MACHINE.regs.ebx);
        // 004053bc jne short 004053DBh
        jne(Cont(x004053be), Cont(x004053db))
    }
}

pub fn x004053be() -> Cont {
    unsafe {
        // 004053be mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004053c1 cmp eax,61h
        sub(MACHINE.regs.eax, 0x61u32);
        // 004053c4 jl near ptr 00405479h
        jl(Cont(x004053ca), Cont(x00405479))
    }
}

pub fn x004053ca() -> Cont {
    unsafe {
        // 004053ca cmp eax,7Ah
        sub(MACHINE.regs.eax, 0x7au32);
        // 004053cd jg near ptr 00405479h
        jg(Cont(x004053d3), Cont(x00405479))
    }
}

pub fn x004053d3() -> Cont {
    unsafe {
        // 004053d3 sub eax,20h
        MACHINE.regs.eax = sub(MACHINE.regs.eax, 0x20u32);
        // 004053d6 jmp near ptr 00405479h
        Cont(x00405479)
    }
}

pub fn x004053db() -> Cont {
    unsafe {
        // 004053db mov ebx,[ebp+8]
        MACHINE.regs.ebx = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 004053de cmp ebx,100h
        sub(MACHINE.regs.ebx, 0x100u32);
        // 004053e4 jge short 0040540Eh
        jge(Cont(x004053e6), Cont(x0040540e))
    }
}

pub fn x004053e6() -> Cont {
    unsafe {
        // 004053e6 cmp dword ptr ds:[40953Ch],1
        sub(MACHINE.memory.read::<u32>(0x40953cu32), 0x1u32);
        // 004053ed jle short 004053FBh
        jle(Cont(x004053ef), Cont(x004053fb))
    }
}

pub fn x004053ef() -> Cont {
    unsafe {
        // 004053ef push 2
        push(0x2u32);
        // 004053f1 push ebx
        push(MACHINE.regs.ebx);
        // 004053f2 call 0040547Ch
        call(0x4053f7, Cont(x0040547c))
    }
}

pub fn x004053f7() -> Cont {
    unsafe {
        // 004053f7 pop ecx
        MACHINE.regs.ecx = pop();
        // 004053f8 pop ecx
        MACHINE.regs.ecx = pop();
        // 004053f9 jmp short 00405406h
        Cont(x00405406)
    }
}

pub fn x004053fb() -> Cont {
    unsafe {
        // 004053fb mov eax,ds:[409330h]
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(0x409330u32);
        // 00405400 mov al,[eax+ebx*2]
        MACHINE.regs.set_al(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .eax
                    .wrapping_add((MACHINE.regs.ebx * 2))
                    .wrapping_add(0x0u32),
            ),
        );
        // 00405403 and eax,2
        MACHINE.regs.eax = and(MACHINE.regs.eax, 0x2u32);
        // 00405406 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405408 jne short 0040540Eh
        jne(Cont(x0040540a), Cont(x0040540e))
    }
}

pub fn x00405406() -> Cont {
    unsafe {
        // 00405406 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 00405408 jne short 0040540Eh
        jne(Cont(x0040540a), Cont(x0040540e))
    }
}

pub fn x0040540a() -> Cont {
    unsafe {
        // 0040540a mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 0040540c jmp short 00405479h
        Cont(x00405479)
    }
}

pub fn x0040540e() -> Cont {
    unsafe {
        // 0040540e mov edx,ds:[409330h]
        MACHINE.regs.edx = MACHINE.memory.read::<u32>(0x409330u32);
        // 00405414 mov eax,ebx
        MACHINE.regs.eax = MACHINE.regs.ebx;
        // 00405416 sar eax,8
        sar();
        // 00405419 movzx ecx,al
        MACHINE.regs.ecx = MACHINE.regs.get_al() as _;
        // 0040541c test byte ptr [edx+ecx*2+1],80h
        and(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .edx
                    .wrapping_add((MACHINE.regs.ecx * 2))
                    .wrapping_add(0x1u32),
            ),
            0x80u8,
        );
        // 00405421 je short 00405431h
        je(Cont(x00405423), Cont(x00405431))
    }
}

pub fn x00405423() -> Cont {
    unsafe {
        // 00405423 and byte ptr [ebp+0Ah],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xau32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xau32)),
                0x0u8,
            ),
        );
        // 00405427 mov [ebp+8],al
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.get_al());
        // 0040542a mov [ebp+9],bl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0x9u32), MACHINE.regs.get_bl());
        // 0040542d push 2
        push(0x2u32);
        // 0040542f jmp short 0040543Ah
        Cont(x0040543a)
    }
}

pub fn x00405431() -> Cont {
    unsafe {
        // 00405431 and byte ptr [ebp+9],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0x9u32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.ebp.wrapping_add(0x9u32)),
                0x0u8,
            ),
        );
        // 00405435 mov [ebp+8],bl
        MACHINE
            .memory
            .write::<u8>(MACHINE.regs.ebp.wrapping_add(0x8u32), MACHINE.regs.get_bl());
        // 00405438 push 1
        push(0x1u32);
        // 0040543a pop eax
        MACHINE.regs.eax = pop();
        // 0040543b lea ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 0040543e push 1
        push(0x1u32);
        // 00405440 push 0
        push(0x0u32);
        // 00405442 push 3
        push(0x3u32);
        // 00405444 push ecx
        push(MACHINE.regs.ecx);
        // 00405445 push eax
        push(MACHINE.regs.eax);
        // 00405446 lea eax,[ebp+8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0x8u32);
        // 00405449 push eax
        push(MACHINE.regs.eax);
        // 0040544a push 200h
        push(0x200u32);
        // 0040544f push dword ptr ds:[409728h]
        push(MACHINE.memory.read::<u32>(0x409728u32));
        // 00405455 call 00405018h
        call(0x40545a, Cont(x00405018))
    }
}

pub fn x0040543a() -> Cont {
    unsafe {
        // 0040543a pop eax
        MACHINE.regs.eax = pop();
        // 0040543b lea ecx,[ebp-4]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 0040543e push 1
        push(0x1u32);
        // 00405440 push 0
        push(0x0u32);
        // 00405442 push 3
        push(0x3u32);
        // 00405444 push ecx
        push(MACHINE.regs.ecx);
        // 00405445 push eax
        push(MACHINE.regs.eax);
        // 00405446 lea eax,[ebp+8]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0x8u32);
        // 00405449 push eax
        push(MACHINE.regs.eax);
        // 0040544a push 200h
        push(0x200u32);
        // 0040544f push dword ptr ds:[409728h]
        push(MACHINE.memory.read::<u32>(0x409728u32));
        // 00405455 call 00405018h
        call(0x40545a, Cont(x00405018))
    }
}

pub fn x0040545a() -> Cont {
    unsafe {
        // 0040545a add esp,20h
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x20u32);
        // 0040545d test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040545f je short 0040540Ah
        je(Cont(x00405461), Cont(x0040540a))
    }
}

pub fn x00405461() -> Cont {
    unsafe {
        // 00405461 cmp eax,1
        sub(MACHINE.regs.eax, 0x1u32);
        // 00405464 jne short 0040546Ch
        jne(Cont(x00405466), Cont(x0040546c))
    }
}

pub fn x00405466() -> Cont {
    unsafe {
        // 00405466 movzx eax,byte ptr [ebp-4]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))
            as _;
        // 0040546a jmp short 00405479h
        Cont(x00405479)
    }
}

pub fn x0040546c() -> Cont {
    unsafe {
        // 0040546c movzx eax,byte ptr [ebp-3]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffdu32))
            as _;
        // 00405470 movzx ecx,byte ptr [ebp-4]
        MACHINE.regs.ecx = MACHINE
            .memory
            .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffcu32))
            as _;
        // 00405474 shl eax,8
        MACHINE.regs.eax = shl(MACHINE.regs.eax, 0x8u8);
        // 00405477 or eax,ecx
        MACHINE.regs.eax = or(MACHINE.regs.eax, MACHINE.regs.ecx);
        // 00405479 pop ebx
        MACHINE.regs.ebx = pop();
        // 0040547a leave
        leave();
        // 0040547b ret
        ret(0)
    }
}

pub fn x00405479() -> Cont {
    unsafe {
        // 00405479 pop ebx
        MACHINE.regs.ebx = pop();
        // 0040547a leave
        leave();
        // 0040547b ret
        ret(0)
    }
}

pub fn x0040547c() -> Cont {
    unsafe {
        // 0040547c push ebp
        push(MACHINE.regs.ebp);
        // 0040547d mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 0040547f push ecx
        push(MACHINE.regs.ecx);
        // 00405480 mov eax,[ebp+8]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.ebp.wrapping_add(0x8u32));
        // 00405483 lea ecx,[eax+1]
        MACHINE.regs.ecx = MACHINE.regs.eax.wrapping_add(0x1u32);
        // 00405486 cmp ecx,100h
        sub(MACHINE.regs.ecx, 0x100u32);
        // 0040548c ja short 0040549Ah
        ja(Cont(x0040548e), Cont(x0040549a))
    }
}

pub fn x0040548e() -> Cont {
    unsafe {
        // 0040548e mov ecx,ds:[409330h]
        MACHINE.regs.ecx = MACHINE.memory.read::<u32>(0x409330u32);
        // 00405494 movzx eax,word ptr [ecx+eax*2]
        MACHINE.regs.eax = MACHINE.memory.read::<u16>(
            MACHINE
                .regs
                .ecx
                .wrapping_add((MACHINE.regs.eax * 2))
                .wrapping_add(0x0u32),
        ) as _;
        // 00405498 jmp short 004054ECh
        Cont(x004054ec)
    }
}

pub fn x0040549a() -> Cont {
    unsafe {
        // 0040549a mov ecx,eax
        MACHINE.regs.ecx = MACHINE.regs.eax;
        // 0040549c push esi
        push(MACHINE.regs.esi);
        // 0040549d mov esi,ds:[409330h]
        MACHINE.regs.esi = MACHINE.memory.read::<u32>(0x409330u32);
        // 004054a3 sar ecx,8
        sar();
        // 004054a6 movzx edx,cl
        MACHINE.regs.edx = MACHINE.regs.get_cl() as _;
        // 004054a9 test byte ptr [esi+edx*2+1],80h
        and(
            MACHINE.memory.read::<u8>(
                MACHINE
                    .regs
                    .esi
                    .wrapping_add((MACHINE.regs.edx * 2))
                    .wrapping_add(0x1u32),
            ),
            0x80u8,
        );
        // 004054ae pop esi
        MACHINE.regs.esi = pop();
        // 004054af je short 004054BFh
        je(Cont(x004054b1), Cont(x004054bf))
    }
}

pub fn x004054b1() -> Cont {
    unsafe {
        // 004054b1 and byte ptr [ebp-2],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffeu32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffeu32)),
                0x0u8,
            ),
        );
        // 004054b5 mov [ebp-4],cl
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.get_cl(),
        );
        // 004054b8 mov [ebp-3],al
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffdu32),
            MACHINE.regs.get_al(),
        );
        // 004054bb push 2
        push(0x2u32);
        // 004054bd jmp short 004054C8h
        Cont(x004054c8)
    }
}

pub fn x004054bf() -> Cont {
    unsafe {
        // 004054bf and byte ptr [ebp-3],0
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffdu32),
            and(
                MACHINE
                    .memory
                    .read::<u8>(MACHINE.regs.ebp.wrapping_add(0xfffffffdu32)),
                0x0u8,
            ),
        );
        // 004054c3 mov [ebp-4],al
        MACHINE.memory.write::<u8>(
            MACHINE.regs.ebp.wrapping_add(0xfffffffcu32),
            MACHINE.regs.get_al(),
        );
        // 004054c6 push 1
        push(0x1u32);
        // 004054c8 pop eax
        MACHINE.regs.eax = pop();
        // 004054c9 lea ecx,[ebp+0Ah]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xau32);
        // 004054cc push 1
        push(0x1u32);
        // 004054ce push 0
        push(0x0u32);
        // 004054d0 push 0
        push(0x0u32);
        // 004054d2 push ecx
        push(MACHINE.regs.ecx);
        // 004054d3 push eax
        push(MACHINE.regs.eax);
        // 004054d4 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 004054d7 push eax
        push(MACHINE.regs.eax);
        // 004054d8 push 1
        push(0x1u32);
        // 004054da call 00405267h
        call(0x4054df, Cont(x00405267))
    }
}

pub fn x004054c8() -> Cont {
    unsafe {
        // 004054c8 pop eax
        MACHINE.regs.eax = pop();
        // 004054c9 lea ecx,[ebp+0Ah]
        MACHINE.regs.ecx = MACHINE.regs.ebp.wrapping_add(0xau32);
        // 004054cc push 1
        push(0x1u32);
        // 004054ce push 0
        push(0x0u32);
        // 004054d0 push 0
        push(0x0u32);
        // 004054d2 push ecx
        push(MACHINE.regs.ecx);
        // 004054d3 push eax
        push(MACHINE.regs.eax);
        // 004054d4 lea eax,[ebp-4]
        MACHINE.regs.eax = MACHINE.regs.ebp.wrapping_add(0xfffffffcu32);
        // 004054d7 push eax
        push(MACHINE.regs.eax);
        // 004054d8 push 1
        push(0x1u32);
        // 004054da call 00405267h
        call(0x4054df, Cont(x00405267))
    }
}

pub fn x004054df() -> Cont {
    unsafe {
        // 004054df add esp,1Ch
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x1cu32);
        // 004054e2 test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 004054e4 jne short 004054E8h
        jne(Cont(x004054e6), Cont(x004054e8))
    }
}

pub fn x004054e6() -> Cont {
    unsafe {
        // 004054e6 leave
        leave();
        // 004054e7 ret
        ret(0)
    }
}

pub fn x004054e8() -> Cont {
    unsafe {
        // 004054e8 movzx eax,word ptr [ebp+0Ah]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u16>(MACHINE.regs.ebp.wrapping_add(0xau32)) as _;
        // 004054ec and eax,[ebp+0Ch]
        MACHINE.regs.eax = and(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004054ef leave
        leave();
        // 004054f0 ret
        ret(0)
    }
}

pub fn x004054ec() -> Cont {
    unsafe {
        // 004054ec and eax,[ebp+0Ch]
        MACHINE.regs.eax = and(
            MACHINE.regs.eax,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.ebp.wrapping_add(0xcu32)),
        );
        // 004054ef leave
        leave();
        // 004054f0 ret
        ret(0)
    }
}

pub const BLOCKS: [(u32, fn() -> Cont); 1714] = [
    (0x001001, ddraw::DirectDrawCreateEx_stdcall),
    (0x001002, gdi32::SelectObject_stdcall),
    (0x001003, gdi32::CreateCompatibleDC_stdcall),
    (0x001004, gdi32::GetObjectA_stdcall),
    (0x001005, gdi32::StretchBlt_stdcall),
    (0x001006, gdi32::DeleteDC_stdcall),
    (0x001007, gdi32::GetStockObject_stdcall),
    (0x001008, kernel32::OutputDebugStringA_stdcall),
    (0x001009, kernel32::HeapAlloc_stdcall),
    (0x00100a, kernel32::GetModuleFileNameA_stdcall),
    (0x00100b, kernel32::UnhandledExceptionFilter_stdcall),
    (0x00100c, kernel32::GetStringTypeW_stdcall),
    (0x00100d, kernel32::GetStringTypeA_stdcall),
    (0x00100e, kernel32::LCMapStringW_stdcall),
    (0x00100f, kernel32::LCMapStringA_stdcall),
    (0x001010, kernel32::MultiByteToWideChar_stdcall),
    (0x001011, kernel32::LoadLibraryA_stdcall),
    (0x001012, kernel32::GetProcAddress_stdcall),
    (0x001013, kernel32::GetOEMCP_stdcall),
    (0x001014, kernel32::GetACP_stdcall),
    (0x001015, kernel32::GetCPInfo_stdcall),
    (0x001016, kernel32::VirtualAlloc_stdcall),
    (0x001017, kernel32::WriteFile_stdcall),
    (0x001018, kernel32::RtlUnwind_stdcall),
    (0x001019, kernel32::GetModuleHandleA_stdcall),
    (0x00101a, kernel32::GetStartupInfoA_stdcall),
    (0x00101b, kernel32::GetCommandLineA_stdcall),
    (0x00101c, kernel32::GetVersion_stdcall),
    (0x00101d, kernel32::ExitProcess_stdcall),
    (0x00101e, kernel32::HeapReAlloc_stdcall),
    (0x00101f, kernel32::GetTickCount_stdcall),
    (0x001020, kernel32::TerminateProcess_stdcall),
    (0x001021, kernel32::GetCurrentProcess_stdcall),
    (0x001022, kernel32::HeapSize_stdcall),
    (0x001023, kernel32::HeapFree_stdcall),
    (0x001024, kernel32::HeapCreate_stdcall),
    (0x001025, kernel32::VirtualFree_stdcall),
    (0x001026, kernel32::FreeEnvironmentStringsA_stdcall),
    (0x001027, kernel32::FreeEnvironmentStringsW_stdcall),
    (0x001028, kernel32::WideCharToMultiByte_stdcall),
    (0x001029, kernel32::GetEnvironmentStrings_stdcall),
    (0x00102a, kernel32::GetEnvironmentStringsW_stdcall),
    (0x00102b, kernel32::SetHandleCount_stdcall),
    (0x00102c, kernel32::GetStdHandle_stdcall),
    (0x00102d, kernel32::GetFileType_stdcall),
    (0x00102e, kernel32::GetEnvironmentVariableA_stdcall),
    (0x00102f, kernel32::GetVersionExA_stdcall),
    (0x001030, kernel32::HeapDestroy_stdcall),
    (0x001031, user32::DispatchMessageA_stdcall),
    (0x001032, user32::TranslateMessage_stdcall),
    (0x001033, user32::PeekMessageA_stdcall),
    (0x001034, user32::MessageBoxA_stdcall),
    (0x001035, user32::SetFocus_stdcall),
    (0x001036, user32::UpdateWindow_stdcall),
    (0x001037, user32::ShowWindow_stdcall),
    (0x001038, user32::CreateWindowExA_stdcall),
    (0x001039, user32::GetSystemMetrics_stdcall),
    (0x00103a, user32::LoadImageA_stdcall),
    (0x00103b, user32::DefWindowProcA_stdcall),
    (0x00103c, user32::PostQuitMessage_stdcall),
    (0x00103d, user32::LoadIconA_stdcall),
    (0x00103e, user32::LoadCursorA_stdcall),
    (0x00103f, user32::RegisterClassA_stdcall),
    (0x001040, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0x001041, ddraw::IDirectDraw7::AddRef_stdcall),
    (0x001042, ddraw::IDirectDraw7::Release_stdcall),
    (0x001043, ddraw::IDirectDraw7::Compact_stdcall),
    (0x001044, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0x001045, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0x001046, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0x001047, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0x001048, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0x001049, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0x00104a, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0x00104b, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0x00104c, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0x00104d, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0x00104e, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0x00104f, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0x001050, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (
        0x001051,
        ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall,
    ),
    (0x001052, ddraw::IDirectDraw7::Initialize_stdcall),
    (0x001053, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0x001054, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0x001055, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (0x001056, ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall),
    (0x001057, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0x001058, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0x001059, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (0x00105a, ddraw::IDirectDraw7::TestCooperativeLevel_stdcall),
    (0x00105b, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0x00105c, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0x00105d, ddraw::IDirectDraw7::EvaluateMode_stdcall),
    (0x00105e, ddraw::IDirectDrawSurface7::QueryInterface_stdcall),
    (0x00105f, ddraw::IDirectDrawSurface7::AddRef_stdcall),
    (0x001060, ddraw::IDirectDrawSurface7::Release_stdcall),
    (
        0x001061,
        ddraw::IDirectDrawSurface7::AddAttachedSurface_stdcall,
    ),
    (
        0x001062,
        ddraw::IDirectDrawSurface7::AddOverlayDirtyRect_stdcall,
    ),
    (0x001063, ddraw::IDirectDrawSurface7::Blt_stdcall),
    (0x001064, ddraw::IDirectDrawSurface7::BltBatch_stdcall),
    (0x001065, ddraw::IDirectDrawSurface7::BltFast_stdcall),
    (
        0x001066,
        ddraw::IDirectDrawSurface7::DeleteAttachedSurface_stdcall,
    ),
    (
        0x001067,
        ddraw::IDirectDrawSurface7::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x001068,
        ddraw::IDirectDrawSurface7::EnumOverlayZOrders_stdcall,
    ),
    (0x001069, ddraw::IDirectDrawSurface7::Flip_stdcall),
    (
        0x00106a,
        ddraw::IDirectDrawSurface7::GetAttachedSurface_stdcall,
    ),
    (0x00106b, ddraw::IDirectDrawSurface7::GetBltStatus_stdcall),
    (0x00106c, ddraw::IDirectDrawSurface7::GetCaps_stdcall),
    (0x00106d, ddraw::IDirectDrawSurface7::GetClipper_stdcall),
    (0x00106e, ddraw::IDirectDrawSurface7::GetColorKey_stdcall),
    (0x00106f, ddraw::IDirectDrawSurface7::GetDC_stdcall),
    (0x001070, ddraw::IDirectDrawSurface7::GetFlipStatus_stdcall),
    (
        0x001071,
        ddraw::IDirectDrawSurface7::GetOverlayPosition_stdcall,
    ),
    (0x001072, ddraw::IDirectDrawSurface7::GetPalette_stdcall),
    (0x001073, ddraw::IDirectDrawSurface7::GetPixelFormat_stdcall),
    (0x001074, ddraw::IDirectDrawSurface7::GetSurfaceDesc_stdcall),
    (0x001075, ddraw::IDirectDrawSurface7::Initialize_stdcall),
    (0x001076, ddraw::IDirectDrawSurface7::IsLost_stdcall),
    (0x001077, ddraw::IDirectDrawSurface7::Lock_stdcall),
    (0x001078, ddraw::IDirectDrawSurface7::ReleaseDC_stdcall),
    (0x001079, ddraw::IDirectDrawSurface7::Restore_stdcall),
    (0x00107a, ddraw::IDirectDrawSurface7::SetClipper_stdcall),
    (0x00107b, ddraw::IDirectDrawSurface7::SetColorKey_stdcall),
    (
        0x00107c,
        ddraw::IDirectDrawSurface7::SetOverlayPosition_stdcall,
    ),
    (0x00107d, ddraw::IDirectDrawSurface7::SetPalette_stdcall),
    (0x00107e, ddraw::IDirectDrawSurface7::Unlock_stdcall),
    (0x00107f, ddraw::IDirectDrawSurface7::UpdateOverlay_stdcall),
    (
        0x001080,
        ddraw::IDirectDrawSurface7::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x001081,
        ddraw::IDirectDrawSurface7::UpdateOverlayZOrder_stdcall,
    ),
    (0x001082, ddraw::IDirectDrawSurface7::GetDDInterface_stdcall),
    (0x001083, ddraw::IDirectDrawSurface7::PageLock_stdcall),
    (0x001084, ddraw::IDirectDrawSurface7::PageUnlock_stdcall),
    (0x001085, ddraw::IDirectDrawSurface7::SetSurfaceDesc_stdcall),
    (0x001086, ddraw::IDirectDrawSurface7::SetPrivateData_stdcall),
    (0x001087, ddraw::IDirectDrawSurface7::GetPrivateData_stdcall),
    (
        0x001088,
        ddraw::IDirectDrawSurface7::FreePrivateData_stdcall,
    ),
    (
        0x001089,
        ddraw::IDirectDrawSurface7::GetUniquenessValue_stdcall,
    ),
    (
        0x00108a,
        ddraw::IDirectDrawSurface7::ChangeUniquenessValue_stdcall,
    ),
    (0x00108b, ddraw::IDirectDrawSurface7::SetPriority_stdcall),
    (0x00108c, ddraw::IDirectDrawSurface7::GetPriority_stdcall),
    (0x00108d, ddraw::IDirectDrawSurface7::SetLOD_stdcall),
    (0x00108e, ddraw::IDirectDrawSurface7::GetLOD_stdcall),
    (0x401000, x00401000),
    (0x401005, x00401005),
    (0x401010, x00401010),
    (0x401020, x00401020),
    (0x40102a, x0040102a),
    (0x401040, x00401040),
    (0x401056, x00401056),
    (0x401062, x00401062),
    (0x40106b, x0040106b),
    (0x401070, x00401070),
    (0x401074, x00401074),
    (0x401079, x00401079),
    (0x401092, x00401092),
    (0x40109a, x0040109a),
    (0x4010b9, x004010b9),
    (0x4010da, x004010da),
    (0x4010ec, x004010ec),
    (0x4010fb, x004010fb),
    (0x4010ff, x004010ff),
    (0x401106, x00401106),
    (0x40110d, x0040110d),
    (0x401114, x00401114),
    (0x401116, x00401116),
    (0x40111b, x0040111b),
    (0x40111d, x0040111d),
    (0x401122, x00401122),
    (0x401130, x00401130),
    (0x401139, x00401139),
    (0x401148, x00401148),
    (0x40117a, x0040117a),
    (0x401189, x00401189),
    (0x40118d, x0040118d),
    (0x401194, x00401194),
    (0x40119b, x0040119b),
    (0x40119d, x0040119d),
    (0x4011a8, x004011a8),
    (0x4011be, x004011be),
    (0x4011de, x004011de),
    (0x4011e8, x004011e8),
    (0x4011ea, x004011ea),
    (0x4011ef, x004011ef),
    (0x401200, x00401200),
    (0x401239, x00401239),
    (0x40124a, x0040124a),
    (0x401256, x00401256),
    (0x401275, x00401275),
    (0x40128c, x0040128c),
    (0x401291, x00401291),
    (0x4012ad, x004012ad),
    (0x4012bb, x004012bb),
    (0x4012c2, x004012c2),
    (0x4012c9, x004012c9),
    (0x401310, x00401310),
    (0x401329, x00401329),
    (0x40132d, x0040132d),
    (0x401337, x00401337),
    (0x40134b, x0040134b),
    (0x40134f, x0040134f),
    (0x40135b, x0040135b),
    (0x401376, x00401376),
    (0x40137a, x0040137a),
    (0x401386, x00401386),
    (0x4013ce, x004013ce),
    (0x4013d3, x004013d3),
    (0x4013dd, x004013dd),
    (0x40140c, x0040140c),
    (0x401420, x00401420),
    (0x40142a, x0040142a),
    (0x401433, x00401433),
    (0x401439, x00401439),
    (0x401442, x00401442),
    (0x401448, x00401448),
    (0x401451, x00401451),
    (0x401457, x00401457),
    (0x401460, x00401460),
    (0x401480, x00401480),
    (0x401488, x00401488),
    (0x40148f, x0040148f),
    (0x401495, x00401495),
    (0x401498, x00401498),
    (0x4014a0, x004014a0),
    (0x4014b0, x004014b0),
    (0x4014bb, x004014bb),
    (0x4014c4, x004014c4),
    (0x4014cb, x004014cb),
    (0x4014d0, x004014d0),
    (0x401509, x00401509),
    (0x401513, x00401513),
    (0x40151e, x0040151e),
    (0x401524, x00401524),
    (0x40152c, x0040152c),
    (0x401536, x00401536),
    (0x40153e, x0040153e),
    (0x40154c, x0040154c),
    (0x401550, x00401550),
    (0x401554, x00401554),
    (0x40155f, x0040155f),
    (0x40156a, x0040156a),
    (0x401588, x00401588),
    (0x401596, x00401596),
    (0x4015a1, x004015a1),
    (0x4015d2, x004015d2),
    (0x4015e0, x004015e0),
    (0x4015e7, x004015e7),
    (0x401627, x00401627),
    (0x401640, x00401640),
    (0x4016a2, x004016a2),
    (0x4016a6, x004016a6),
    (0x4016ad, x004016ad),
    (0x4016c6, x004016c6),
    (0x4016ca, x004016ca),
    (0x4016d9, x004016d9),
    (0x4016e5, x004016e5),
    (0x401700, x00401700),
    (0x401730, x00401730),
    (0x401741, x00401741),
    (0x401744, x00401744),
    (0x40174c, x0040174c),
    (0x40174f, x0040174f),
    (0x401777, x00401777),
    (0x401780, x00401780),
    (0x401784, x00401784),
    (0x401786, x00401786),
    (0x401795, x00401795),
    (0x401799, x00401799),
    (0x4017a0, x004017a0),
    (0x4017a7, x004017a7),
    (0x4017a9, x004017a9),
    (0x4017b0, x004017b0),
    (0x4017bc, x004017bc),
    (0x4017d0, x004017d0),
    (0x4017da, x004017da),
    (0x4017e0, x004017e0),
    (0x4017e7, x004017e7),
    (0x4017f0, x004017f0),
    (0x4017f9, x004017f9),
    (0x401800, x00401800),
    (0x401806, x00401806),
    (0x401812, x00401812),
    (0x40182b, x0040182b),
    (0x401831, x00401831),
    (0x401840, x00401840),
    (0x401847, x00401847),
    (0x401848, x00401848),
    (0x401865, x00401865),
    (0x401873, x00401873),
    (0x40187c, x0040187c),
    (0x401885, x00401885),
    (0x40188f, x0040188f),
    (0x401899, x00401899),
    (0x4018a0, x004018a0),
    (0x4018a6, x004018a6),
    (0x4018b4, x004018b4),
    (0x4018bd, x004018bd),
    (0x4018bf, x004018bf),
    (0x4018eb, x004018eb),
    (0x40191e, x0040191e),
    (0x401923, x00401923),
    (0x40192a, x0040192a),
    (0x40192b, x0040192b),
    (0x401933, x00401933),
    (0x401939, x00401939),
    (0x401943, x00401943),
    (0x40194d, x0040194d),
    (0x401952, x00401952),
    (0x401957, x00401957),
    (0x401964, x00401964),
    (0x401969, x00401969),
    (0x401972, x00401972),
    (0x401978, x00401978),
    (0x40197b, x0040197b),
    (0x401987, x00401987),
    (0x40198d, x0040198d),
    (0x401996, x00401996),
    (0x4019a7, x004019a7),
    (0x4019aa, x004019aa),
    (0x4019b5, x004019b5),
    (0x4019be, x004019be),
    (0x4019c3, x004019c3),
    (0x4019cc, x004019cc),
    (0x4019d7, x004019d7),
    (0x4019da, x004019da),
    (0x4019e3, x004019e3),
    (0x4019e8, x004019e8),
    (0x4019f1, x004019f1),
    (0x4019fd, x004019fd),
    (0x4019fe, x004019fe),
    (0x401a0b, x00401a0b),
    (0x401a13, x00401a13),
    (0x401a19, x00401a19),
    (0x401a20, x00401a20),
    (0x401a28, x00401a28),
    (0x401a2e, x00401a2e),
    (0x401a3c, x00401a3c),
    (0x401a47, x00401a47),
    (0x401a4f, x00401a4f),
    (0x401a5a, x00401a5a),
    (0x401a62, x00401a62),
    (0x401a6d, x00401a6d),
    (0x401a74, x00401a74),
    (0x401a7a, x00401a7a),
    (0x401a81, x00401a81),
    (0x401a8c, x00401a8c),
    (0x401a8e, x00401a8e),
    (0x401a96, x00401a96),
    (0x401a9c, x00401a9c),
    (0x401aa7, x00401aa7),
    (0x401aaa, x00401aaa),
    (0x401aae, x00401aae),
    (0x401ab2, x00401ab2),
    (0x401ab5, x00401ab5),
    (0x401aca, x00401aca),
    (0x401ad0, x00401ad0),
    (0x401adb, x00401adb),
    (0x401add, x00401add),
    (0x401ae5, x00401ae5),
    (0x401aee, x00401aee),
    (0x401af1, x00401af1),
    (0x401af5, x00401af5),
    (0x401af9, x00401af9),
    (0x401afc, x00401afc),
    (0x401b14, x00401b14),
    (0x401b16, x00401b16),
    (0x401b1a, x00401b1a),
    (0x401b23, x00401b23),
    (0x401b29, x00401b29),
    (0x401b32, x00401b32),
    (0x401b37, x00401b37),
    (0x401b3e, x00401b3e),
    (0x401b47, x00401b47),
    (0x401b4c, x00401b4c),
    (0x401b50, x00401b50),
    (0x401b58, x00401b58),
    (0x401b5b, x00401b5b),
    (0x401b66, x00401b66),
    (0x401b76, x00401b76),
    (0x401b83, x00401b83),
    (0x401b8b, x00401b8b),
    (0x401b9d, x00401b9d),
    (0x401ba4, x00401ba4),
    (0x401ba9, x00401ba9),
    (0x401baf, x00401baf),
    (0x401bb6, x00401bb6),
    (0x401bc0, x00401bc0),
    (0x401bc2, x00401bc2),
    (0x401bcc, x00401bcc),
    (0x401bd8, x00401bd8),
    (0x401bdb, x00401bdb),
    (0x401be3, x00401be3),
    (0x401bf2, x00401bf2),
    (0x401bf8, x00401bf8),
    (0x401c02, x00401c02),
    (0x401c04, x00401c04),
    (0x401c0e, x00401c0e),
    (0x401c1a, x00401c1a),
    (0x401c1f, x00401c1f),
    (0x401c31, x00401c31),
    (0x401c33, x00401c33),
    (0x401c3b, x00401c3b),
    (0x401c48, x00401c48),
    (0x401c4e, x00401c4e),
    (0x401c57, x00401c57),
    (0x401c59, x00401c59),
    (0x401c60, x00401c60),
    (0x401c64, x00401c64),
    (0x401c67, x00401c67),
    (0x401c7f, x00401c7f),
    (0x401c83, x00401c83),
    (0x401c8c, x00401c8c),
    (0x401c92, x00401c92),
    (0x401c97, x00401c97),
    (0x401c99, x00401c99),
    (0x401c9e, x00401c9e),
    (0x401ca7, x00401ca7),
    (0x401ca9, x00401ca9),
    (0x401cb8, x00401cb8),
    (0x401cc7, x00401cc7),
    (0x401ccb, x00401ccb),
    (0x401cd8, x00401cd8),
    (0x401cdc, x00401cdc),
    (0x401ce9, x00401ce9),
    (0x401ced, x00401ced),
    (0x401cf9, x00401cf9),
    (0x401d03, x00401d03),
    (0x401d0a, x00401d0a),
    (0x401d22, x00401d22),
    (0x401d2b, x00401d2b),
    (0x401d39, x00401d39),
    (0x401d3f, x00401d3f),
    (0x401d41, x00401d41),
    (0x401d4c, x00401d4c),
    (0x401d4d, x00401d4d),
    (0x401d5c, x00401d5c),
    (0x401d5e, x00401d5e),
    (0x401d6d, x00401d6d),
    (0x401d74, x00401d74),
    (0x401d84, x00401d84),
    (0x401d86, x00401d86),
    (0x401d8b, x00401d8b),
    (0x401d91, x00401d91),
    (0x401d97, x00401d97),
    (0x401d99, x00401d99),
    (0x401d9e, x00401d9e),
    (0x401da0, x00401da0),
    (0x401db0, x00401db0),
    (0x401db9, x00401db9),
    (0x401dbe, x00401dbe),
    (0x401dc6, x00401dc6),
    (0x401dc9, x00401dc9),
    (0x401dce, x00401dce),
    (0x401dde, x00401dde),
    (0x401de5, x00401de5),
    (0x401ded, x00401ded),
    (0x401df0, x00401df0),
    (0x401dfe, x00401dfe),
    (0x401e01, x00401e01),
    (0x401e10, x00401e10),
    (0x401e13, x00401e13),
    (0x401e1a, x00401e1a),
    (0x401e23, x00401e23),
    (0x401e28, x00401e28),
    (0x401e2e, x00401e2e),
    (0x401e37, x00401e37),
    (0x401e3c, x00401e3c),
    (0x401e3e, x00401e3e),
    (0x401e3f, x00401e3f),
    (0x401e4e, x00401e4e),
    (0x401e56, x00401e56),
    (0x401e5c, x00401e5c),
    (0x401e61, x00401e61),
    (0x401e63, x00401e63),
    (0x401e68, x00401e68),
    (0x401e70, x00401e70),
    (0x401e78, x00401e78),
    (0x401e7b, x00401e7b),
    (0x401e83, x00401e83),
    (0x401e8e, x00401e8e),
    (0x401e93, x00401e93),
    (0x401e95, x00401e95),
    (0x401e99, x00401e99),
    (0x401e9c, x00401e9c),
    (0x401ea2, x00401ea2),
    (0x401eb1, x00401eb1),
    (0x401eb3, x00401eb3),
    (0x401ebf, x00401ebf),
    (0x401ec9, x00401ec9),
    (0x401ecf, x00401ecf),
    (0x401ed5, x00401ed5),
    (0x401edb, x00401edb),
    (0x401edf, x00401edf),
    (0x401ee4, x00401ee4),
    (0x401ef2, x00401ef2),
    (0x401ef9, x00401ef9),
    (0x401f05, x00401f05),
    (0x401f0a, x00401f0a),
    (0x401f0b, x00401f0b),
    (0x401f19, x00401f19),
    (0x401f1c, x00401f1c),
    (0x401f28, x00401f28),
    (0x401f31, x00401f31),
    (0x401f3c, x00401f3c),
    (0x401f41, x00401f41),
    (0x401f4d, x00401f4d),
    (0x401f56, x00401f56),
    (0x401f74, x00401f74),
    (0x401f87, x00401f87),
    (0x401f93, x00401f93),
    (0x401f9c, x00401f9c),
    (0x401fab, x00401fab),
    (0x401fb7, x00401fb7),
    (0x401fbe, x00401fbe),
    (0x401fca, x00401fca),
    (0x401fd1, x00401fd1),
    (0x401fdd, x00401fdd),
    (0x401fe4, x00401fe4),
    (0x401ff0, x00401ff0),
    (0x401ff7, x00401ff7),
    (0x402003, x00402003),
    (0x40200a, x0040200a),
    (0x402016, x00402016),
    (0x40201d, x0040201d),
    (0x402027, x00402027),
    (0x402031, x00402031),
    (0x40203c, x0040203c),
    (0x402043, x00402043),
    (0x402044, x00402044),
    (0x40204c, x0040204c),
    (0x402051, x00402051),
    (0x40205a, x0040205a),
    (0x40205d, x0040205d),
    (0x402075, x00402075),
    (0x40207f, x0040207f),
    (0x402086, x00402086),
    (0x40208a, x0040208a),
    (0x402099, x00402099),
    (0x40209d, x0040209d),
    (0x40209f, x0040209f),
    (0x4020a0, x004020a0),
    (0x4020a9, x004020a9),
    (0x4020ae, x004020ae),
    (0x4020bb, x004020bb),
    (0x4020c3, x004020c3),
    (0x4020c7, x004020c7),
    (0x4020d0, x004020d0),
    (0x4020d5, x004020d5),
    (0x4020d8, x004020d8),
    (0x4020dd, x004020dd),
    (0x4020e0, x004020e0),
    (0x4020e4, x004020e4),
    (0x4020ea, x004020ea),
    (0x4020f0, x004020f0),
    (0x4020f4, x004020f4),
    (0x4020f8, x004020f8),
    (0x402105, x00402105),
    (0x40210a, x0040210a),
    (0x402112, x00402112),
    (0x402118, x00402118),
    (0x40211c, x0040211c),
    (0x40211d, x0040211d),
    (0x402123, x00402123),
    (0x40212a, x0040212a),
    (0x402137, x00402137),
    (0x402144, x00402144),
    (0x40214b, x0040214b),
    (0x40214c, x0040214c),
    (0x402156, x00402156),
    (0x402157, x00402157),
    (0x40215d, x0040215d),
    (0x402166, x00402166),
    (0x40216c, x0040216c),
    (0x402173, x00402173),
    (0x40217a, x0040217a),
    (0x40217b, x0040217b),
    (0x402183, x00402183),
    (0x402188, x00402188),
    (0x40218e, x0040218e),
    (0x40218f, x0040218f),
    (0x40219a, x0040219a),
    (0x4021b1, x004021b1),
    (0x4021c3, x004021c3),
    (0x4021c8, x004021c8),
    (0x4021da, x004021da),
    (0x4021eb, x004021eb),
    (0x4021ed, x004021ed),
    (0x4021fd, x004021fd),
    (0x40220c, x0040220c),
    (0x402215, x00402215),
    (0x40221c, x0040221c),
    (0x40221d, x0040221d),
    (0x402233, x00402233),
    (0x40224a, x0040224a),
    (0x40226c, x0040226c),
    (0x402274, x00402274),
    (0x402279, x00402279),
    (0x402282, x00402282),
    (0x402286, x00402286),
    (0x402292, x00402292),
    (0x402298, x00402298),
    (0x40229e, x0040229e),
    (0x4022a4, x004022a4),
    (0x4022ab, x004022ab),
    (0x4022b1, x004022b1),
    (0x4022b5, x004022b5),
    (0x4022ba, x004022ba),
    (0x4022bd, x004022bd),
    (0x4022c3, x004022c3),
    (0x4022c8, x004022c8),
    (0x4022d7, x004022d7),
    (0x4022dd, x004022dd),
    (0x4022e2, x004022e2),
    (0x4022e3, x004022e3),
    (0x4022e8, x004022e8),
    (0x4022ec, x004022ec),
    (0x4022f1, x004022f1),
    (0x4022f5, x004022f5),
    (0x4022f8, x004022f8),
    (0x4022fc, x004022fc),
    (0x402300, x00402300),
    (0x402304, x00402304),
    (0x40230d, x0040230d),
    (0x402314, x00402314),
    (0x402319, x00402319),
    (0x40231c, x0040231c),
    (0x402325, x00402325),
    (0x402329, x00402329),
    (0x402331, x00402331),
    (0x402336, x00402336),
    (0x40233f, x0040233f),
    (0x402344, x00402344),
    (0x402348, x00402348),
    (0x40234d, x0040234d),
    (0x402352, x00402352),
    (0x402359, x00402359),
    (0x402362, x00402362),
    (0x402366, x00402366),
    (0x402369, x00402369),
    (0x402377, x00402377),
    (0x402379, x00402379),
    (0x402380, x00402380),
    (0x402381, x00402381),
    (0x402385, x00402385),
    (0x402389, x00402389),
    (0x40238e, x0040238e),
    (0x402394, x00402394),
    (0x40239a, x0040239a),
    (0x40239f, x0040239f),
    (0x4023a4, x004023a4),
    (0x4023aa, x004023aa),
    (0x4023ae, x004023ae),
    (0x4023ba, x004023ba),
    (0x4023c0, x004023c0),
    (0x4023c7, x004023c7),
    (0x4023d3, x004023d3),
    (0x4023d6, x004023d6),
    (0x4023d8, x004023d8),
    (0x4023de, x004023de),
    (0x4023e2, x004023e2),
    (0x4023e6, x004023e6),
    (0x4023ed, x004023ed),
    (0x4023f1, x004023f1),
    (0x4023f4, x004023f4),
    (0x4023fe, x004023fe),
    (0x402419, x00402419),
    (0x40241b, x0040241b),
    (0x402421, x00402421),
    (0x40242d, x0040242d),
    (0x402433, x00402433),
    (0x40243d, x0040243d),
    (0x40244c, x0040244c),
    (0x402455, x00402455),
    (0x402459, x00402459),
    (0x40245b, x0040245b),
    (0x402465, x00402465),
    (0x40246c, x0040246c),
    (0x402473, x00402473),
    (0x40247a, x0040247a),
    (0x402493, x00402493),
    (0x402499, x00402499),
    (0x40249f, x0040249f),
    (0x4024a8, x004024a8),
    (0x4024b5, x004024b5),
    (0x4024b9, x004024b9),
    (0x4024c2, x004024c2),
    (0x4024c7, x004024c7),
    (0x4024cb, x004024cb),
    (0x4024d2, x004024d2),
    (0x4024d6, x004024d6),
    (0x4024db, x004024db),
    (0x4024df, x004024df),
    (0x4024e5, x004024e5),
    (0x4024eb, x004024eb),
    (0x4024f1, x004024f1),
    (0x4024f6, x004024f6),
    (0x4024fb, x004024fb),
    (0x402506, x00402506),
    (0x40250d, x0040250d),
    (0x402511, x00402511),
    (0x402519, x00402519),
    (0x40251c, x0040251c),
    (0x402523, x00402523),
    (0x402527, x00402527),
    (0x402529, x00402529),
    (0x402530, x00402530),
    (0x402541, x00402541),
    (0x402548, x00402548),
    (0x40254f, x0040254f),
    (0x402550, x00402550),
    (0x402566, x00402566),
    (0x40256a, x0040256a),
    (0x402584, x00402584),
    (0x40258f, x0040258f),
    (0x40259b, x0040259b),
    (0x4025a7, x004025a7),
    (0x4025b8, x004025b8),
    (0x4025ba, x004025ba),
    (0x4025c2, x004025c2),
    (0x4025c7, x004025c7),
    (0x4025d1, x004025d1),
    (0x4025d6, x004025d6),
    (0x4025e5, x004025e5),
    (0x4025e9, x004025e9),
    (0x402601, x00402601),
    (0x40260c, x0040260c),
    (0x40260e, x0040260e),
    (0x402614, x00402614),
    (0x40261a, x0040261a),
    (0x402621, x00402621),
    (0x402629, x00402629),
    (0x40262e, x0040262e),
    (0x402635, x00402635),
    (0x402639, x00402639),
    (0x402657, x00402657),
    (0x402660, x00402660),
    (0x402662, x00402662),
    (0x402670, x00402670),
    (0x402678, x00402678),
    (0x40267d, x0040267d),
    (0x402687, x00402687),
    (0x40268e, x0040268e),
    (0x402695, x00402695),
    (0x40269c, x0040269c),
    (0x4026a0, x004026a0),
    (0x4026ac, x004026ac),
    (0x4026b2, x004026b2),
    (0x4026b7, x004026b7),
    (0x4026bd, x004026bd),
    (0x4026c1, x004026c1),
    (0x4026c7, x004026c7),
    (0x4026d3, x004026d3),
    (0x4026db, x004026db),
    (0x4026eb, x004026eb),
    (0x4026f2, x004026f2),
    (0x4026f9, x004026f9),
    (0x402706, x00402706),
    (0x402708, x00402708),
    (0x402715, x00402715),
    (0x40272d, x0040272d),
    (0x402731, x00402731),
    (0x40273a, x0040273a),
    (0x402743, x00402743),
    (0x40274b, x0040274b),
    (0x402762, x00402762),
    (0x40276a, x0040276a),
    (0x40277a, x0040277a),
    (0x402780, x00402780),
    (0x402784, x00402784),
    (0x402788, x00402788),
    (0x40278d, x0040278d),
    (0x4027a0, x004027a0),
    (0x4027a7, x004027a7),
    (0x4027af, x004027af),
    (0x4027c2, x004027c2),
    (0x4027d0, x004027d0),
    (0x4027d6, x004027d6),
    (0x4027da, x004027da),
    (0x4027de, x004027de),
    (0x4027e3, x004027e3),
    (0x4027f6, x004027f6),
    (0x4027f8, x004027f8),
    (0x4027fc, x004027fc),
    (0x402804, x00402804),
    (0x40280a, x0040280a),
    (0x402811, x00402811),
    (0x402816, x00402816),
    (0x40281a, x0040281a),
    (0x40281b, x0040281b),
    (0x40281f, x0040281f),
    (0x402828, x00402828),
    (0x402830, x00402830),
    (0x402835, x00402835),
    (0x40283a, x0040283a),
    (0x402843, x00402843),
    (0x40284d, x0040284d),
    (0x402850, x00402850),
    (0x402867, x00402867),
    (0x402870, x00402870),
    (0x402875, x00402875),
    (0x40287f, x0040287f),
    (0x402889, x00402889),
    (0x40288c, x0040288c),
    (0x402891, x00402891),
    (0x402896, x00402896),
    (0x40289a, x0040289a),
    (0x4028a6, x004028a6),
    (0x4028a9, x004028a9),
    (0x402a80, x00402a80),
    (0x402a8a, x00402a8a),
    (0x402a8e, x00402a8e),
    (0x402a97, x00402a97),
    (0x402aa1, x00402aa1),
    (0x402aab, x00402aab),
    (0x402aad, x00402aad),
    (0x402ab7, x00402ab7),
    (0x402ab8, x00402ab8),
    (0x402ab9, x00402ab9),
    (0x402acc, x00402acc),
    (0x402ad0, x00402ad0),
    (0x402adb, x00402adb),
    (0x402aed, x00402aed),
    (0x402afb, x00402afb),
    (0x402aff, x00402aff),
    (0x402b0c, x00402b0c),
    (0x402b18, x00402b18),
    (0x402b2c, x00402b2c),
    (0x402b30, x00402b30),
    (0x402b41, x00402b41),
    (0x402b43, x00402b43),
    (0x402b56, x00402b56),
    (0x402b5d, x00402b5d),
    (0x402b69, x00402b69),
    (0x402b83, x00402b83),
    (0x402b86, x00402b86),
    (0x402b97, x00402b97),
    (0x402ba4, x00402ba4),
    (0x402bb5, x00402bb5),
    (0x402bc7, x00402bc7),
    (0x402bdd, x00402bdd),
    (0x402be3, x00402be3),
    (0x402bf6, x00402bf6),
    (0x402c02, x00402c02),
    (0x402c09, x00402c09),
    (0x402c0c, x00402c0c),
    (0x402c1f, x00402c1f),
    (0x402c28, x00402c28),
    (0x402c29, x00402c29),
    (0x402c54, x00402c54),
    (0x402c64, x00402c64),
    (0x402c68, x00402c68),
    (0x402c77, x00402c77),
    (0x402c7c, x00402c7c),
    (0x402c7e, x00402c7e),
    (0x402c7f, x00402c7f),
    (0x402cbd, x00402cbd),
    (0x402cd7, x00402cd7),
    (0x402ce0, x00402ce0),
    (0x402ce3, x00402ce3),
    (0x402ceb, x00402ceb),
    (0x402cf0, x00402cf0),
    (0x402d07, x00402d07),
    (0x402d0e, x00402d0e),
    (0x402d29, x00402d29),
    (0x402d2f, x00402d2f),
    (0x402d37, x00402d37),
    (0x402d3a, x00402d3a),
    (0x402d55, x00402d55),
    (0x402d60, x00402d60),
    (0x402d63, x00402d63),
    (0x402d72, x00402d72),
    (0x402d86, x00402d86),
    (0x402d88, x00402d88),
    (0x402d98, x00402d98),
    (0x402d9a, x00402d9a),
    (0x402d9e, x00402d9e),
    (0x402da9, x00402da9),
    (0x402dae, x00402dae),
    (0x402dc3, x00402dc3),
    (0x402dca, x00402dca),
    (0x402de3, x00402de3),
    (0x402de9, x00402de9),
    (0x402e01, x00402e01),
    (0x402e06, x00402e06),
    (0x402e09, x00402e09),
    (0x402e0f, x00402e0f),
    (0x402e17, x00402e17),
    (0x402e38, x00402e38),
    (0x402e4a, x00402e4a),
    (0x402e50, x00402e50),
    (0x402e5e, x00402e5e),
    (0x402e6f, x00402e6f),
    (0x402e75, x00402e75),
    (0x402e85, x00402e85),
    (0x402e98, x00402e98),
    (0x402eac, x00402eac),
    (0x402eb9, x00402eb9),
    (0x402ed9, x00402ed9),
    (0x402f1d, x00402f1d),
    (0x402f26, x00402f26),
    (0x402f2c, x00402f2c),
    (0x402f34, x00402f34),
    (0x402f4a, x00402f4a),
    (0x402f73, x00402f73),
    (0x402f87, x00402f87),
    (0x402f8b, x00402f8b),
    (0x402f95, x00402f95),
    (0x402fa3, x00402fa3),
    (0x402fa8, x00402fa8),
    (0x402fda, x00402fda),
    (0x402fe8, x00402fe8),
    (0x402ff8, x00402ff8),
    (0x403006, x00403006),
    (0x403014, x00403014),
    (0x40301f, x0040301f),
    (0x403024, x00403024),
    (0x403026, x00403026),
    (0x40302d, x0040302d),
    (0x40303b, x0040303b),
    (0x403040, x00403040),
    (0x403042, x00403042),
    (0x403044, x00403044),
    (0x403049, x00403049),
    (0x40304f, x0040304f),
    (0x403057, x00403057),
    (0x40305a, x0040305a),
    (0x40305c, x0040305c),
    (0x40305e, x0040305e),
    (0x403065, x00403065),
    (0x40306b, x0040306b),
    (0x403070, x00403070),
    (0x403072, x00403072),
    (0x403074, x00403074),
    (0x403079, x00403079),
    (0x403082, x00403082),
    (0x403088, x00403088),
    (0x403096, x00403096),
    (0x40309d, x0040309d),
    (0x4030b0, x004030b0),
    (0x4030c4, x004030c4),
    (0x4030e1, x004030e1),
    (0x4030f8, x004030f8),
    (0x4030fb, x004030fb),
    (0x403117, x00403117),
    (0x403124, x00403124),
    (0x403128, x00403128),
    (0x40312d, x0040312d),
    (0x403147, x00403147),
    (0x40314a, x0040314a),
    (0x403152, x00403152),
    (0x40315a, x0040315a),
    (0x40315f, x0040315f),
    (0x403180, x00403180),
    (0x40318a, x0040318a),
    (0x4031ad, x004031ad),
    (0x4031b8, x004031b8),
    (0x4031bb, x004031bb),
    (0x4031d7, x004031d7),
    (0x4031f8, x004031f8),
    (0x403204, x00403204),
    (0x403210, x00403210),
    (0x40321b, x0040321b),
    (0x40322d, x0040322d),
    (0x403239, x00403239),
    (0x403246, x00403246),
    (0x40325c, x0040325c),
    (0x40325f, x0040325f),
    (0x403263, x00403263),
    (0x40326b, x0040326b),
    (0x40326e, x0040326e),
    (0x40328a, x0040328a),
    (0x403292, x00403292),
    (0x40329d, x0040329d),
    (0x4032a4, x004032a4),
    (0x4032ac, x004032ac),
    (0x4032b1, x004032b1),
    (0x4032c4, x004032c4),
    (0x4032df, x004032df),
    (0x4032e3, x004032e3),
    (0x4032f4, x004032f4),
    (0x403313, x00403313),
    (0x40331a, x0040331a),
    (0x40332d, x0040332d),
    (0x403334, x00403334),
    (0x403344, x00403344),
    (0x403348, x00403348),
    (0x40335f, x0040335f),
    (0x403362, x00403362),
    (0x403374, x00403374),
    (0x403378, x00403378),
    (0x40337d, x0040337d),
    (0x403392, x00403392),
    (0x40339e, x0040339e),
    (0x4033b9, x004033b9),
    (0x4033bd, x004033bd),
    (0x4033c5, x004033c5),
    (0x4033cf, x004033cf),
    (0x4033d2, x004033d2),
    (0x40340b, x0040340b),
    (0x403445, x00403445),
    (0x403448, x00403448),
    (0x403458, x00403458),
    (0x40345d, x0040345d),
    (0x4034ac, x004034ac),
    (0x4034b5, x004034b5),
    (0x4034bf, x004034bf),
    (0x4034ce, x004034ce),
    (0x4034d4, x004034d4),
    (0x4034dc, x004034dc),
    (0x4034e1, x004034e1),
    (0x4034f9, x004034f9),
    (0x403500, x00403500),
    (0x40351e, x0040351e),
    (0x403524, x00403524),
    (0x403548, x00403548),
    (0x40355b, x0040355b),
    (0x40355e, x0040355e),
    (0x403584, x00403584),
    (0x403596, x00403596),
    (0x40359c, x0040359c),
    (0x4035aa, x004035aa),
    (0x4035b7, x004035b7),
    (0x4035bd, x004035bd),
    (0x4035cd, x004035cd),
    (0x4035dc, x004035dc),
    (0x4035e0, x004035e0),
    (0x4035f2, x004035f2),
    (0x4035f5, x004035f5),
    (0x403604, x00403604),
    (0x40360b, x0040360b),
    (0x403611, x00403611),
    (0x403633, x00403633),
    (0x403636, x00403636),
    (0x403640, x00403640),
    (0x40364c, x0040364c),
    (0x40364f, x0040364f),
    (0x403657, x00403657),
    (0x40365c, x0040365c),
    (0x403673, x00403673),
    (0x40367a, x0040367a),
    (0x403695, x00403695),
    (0x40369b, x0040369b),
    (0x40369e, x0040369e),
    (0x4036c2, x004036c2),
    (0x4036c5, x004036c5),
    (0x4036e6, x004036e6),
    (0x4036f8, x004036f8),
    (0x4036fe, x004036fe),
    (0x40370c, x0040370c),
    (0x403719, x00403719),
    (0x40371f, x0040371f),
    (0x40372f, x0040372f),
    (0x40373e, x0040373e),
    (0x403742, x00403742),
    (0x40374b, x0040374b),
    (0x40374e, x0040374e),
    (0x403753, x00403753),
    (0x403760, x00403760),
    (0x403767, x00403767),
    (0x40377a, x0040377a),
    (0x403784, x00403784),
    (0x40379a, x0040379a),
    (0x4037a4, x004037a4),
    (0x4037b4, x004037b4),
    (0x4037bc, x004037bc),
    (0x4037c5, x004037c5),
    (0x4037ce, x004037ce),
    (0x4037d3, x004037d3),
    (0x4037dc, x004037dc),
    (0x4037e3, x004037e3),
    (0x4037f8, x004037f8),
    (0x40381a, x0040381a),
    (0x403837, x00403837),
    (0x403840, x00403840),
    (0x403843, x00403843),
    (0x40384c, x0040384c),
    (0x403867, x00403867),
    (0x40386b, x0040386b),
    (0x403879, x00403879),
    (0x403881, x00403881),
    (0x403890, x00403890),
    (0x403892, x00403892),
    (0x403897, x00403897),
    (0x4038ac, x004038ac),
    (0x4038b4, x004038b4),
    (0x4038bc, x004038bc),
    (0x4038c4, x004038c4),
    (0x4038e2, x004038e2),
    (0x4038e4, x004038e4),
    (0x4038ed, x004038ed),
    (0x4038fa, x004038fa),
    (0x403904, x00403904),
    (0x403913, x00403913),
    (0x40391b, x0040391b),
    (0x403931, x00403931),
    (0x403935, x00403935),
    (0x403945, x00403945),
    (0x403949, x00403949),
    (0x40394c, x0040394c),
    (0x403954, x00403954),
    (0x403961, x00403961),
    (0x40396c, x0040396c),
    (0x403972, x00403972),
    (0x403978, x00403978),
    (0x40397d, x0040397d),
    (0x403989, x00403989),
    (0x403991, x00403991),
    (0x403997, x00403997),
    (0x403998, x00403998),
    (0x4039a0, x004039a0),
    (0x4039aa, x004039aa),
    (0x4039af, x004039af),
    (0x4039bb, x004039bb),
    (0x4039c0, x004039c0),
    (0x4039c5, x004039c5),
    (0x4039cb, x004039cb),
    (0x4039cd, x004039cd),
    (0x4039d1, x004039d1),
    (0x4039e2, x004039e2),
    (0x403a02, x00403a02),
    (0x403a06, x00403a06),
    (0x403a33, x00403a33),
    (0x403a42, x00403a42),
    (0x403a49, x00403a49),
    (0x403a4a, x00403a4a),
    (0x403a4b, x00403a4b),
    (0x403a59, x00403a59),
    (0x403a65, x00403a65),
    (0x403a84, x00403a84),
    (0x403a8d, x00403a8d),
    (0x403a92, x00403a92),
    (0x403a9a, x00403a9a),
    (0x403aa1, x00403aa1),
    (0x403aa7, x00403aa7),
    (0x403abc, x00403abc),
    (0x403abe, x00403abe),
    (0x403ac1, x00403ac1),
    (0x403ad4, x00403ad4),
    (0x403ada, x00403ada),
    (0x403adf, x00403adf),
    (0x403ae9, x00403ae9),
    (0x403af0, x00403af0),
    (0x403af3, x00403af3),
    (0x403b02, x00403b02),
    (0x403b04, x00403b04),
    (0x403b07, x00403b07),
    (0x403b11, x00403b11),
    (0x403b16, x00403b16),
    (0x403b26, x00403b26),
    (0x403b2d, x00403b2d),
    (0x403b33, x00403b33),
    (0x403b39, x00403b39),
    (0x403b43, x00403b43),
    (0x403b45, x00403b45),
    (0x403b63, x00403b63),
    (0x403b69, x00403b69),
    (0x403b74, x00403b74),
    (0x403b8c, x00403b8c),
    (0x403b94, x00403b94),
    (0x403b9f, x00403b9f),
    (0x403bab, x00403bab),
    (0x403bb1, x00403bb1),
    (0x403bdb, x00403bdb),
    (0x403be7, x00403be7),
    (0x403beb, x00403beb),
    (0x403bf0, x00403bf0),
    (0x403bf5, x00403bf5),
    (0x403bf7, x00403bf7),
    (0x403c1a, x00403c1a),
    (0x403c1f, x00403c1f),
    (0x403c23, x00403c23),
    (0x403c4c, x00403c4c),
    (0x403c4e, x00403c4e),
    (0x403c53, x00403c53),
    (0x403c77, x00403c77),
    (0x403c80, x00403c80),
    (0x403c87, x00403c87),
    (0x403c90, x00403c90),
    (0x403c98, x00403c98),
    (0x403c9f, x00403c9f),
    (0x403ca1, x00403ca1),
    (0x403ca8, x00403ca8),
    (0x403cae, x00403cae),
    (0x403cb4, x00403cb4),
    (0x403cb9, x00403cb9),
    (0x403cbd, x00403cbd),
    (0x403cc1, x00403cc1),
    (0x403cc6, x00403cc6),
    (0x403ccb, x00403ccb),
    (0x403cd7, x00403cd7),
    (0x403cde, x00403cde),
    (0x403ce3, x00403ce3),
    (0x403ceb, x00403ceb),
    (0x403cee, x00403cee),
    (0x403cf2, x00403cf2),
    (0x403cfa, x00403cfa),
    (0x403d00, x00403d00),
    (0x403d06, x00403d06),
    (0x403d0b, x00403d0b),
    (0x403d0f, x00403d0f),
    (0x403d17, x00403d17),
    (0x403d20, x00403d20),
    (0x403d29, x00403d29),
    (0x403d30, x00403d30),
    (0x403d34, x00403d34),
    (0x403d3c, x00403d3c),
    (0x403d40, x00403d40),
    (0x403d47, x00403d47),
    (0x403d4f, x00403d4f),
    (0x403d58, x00403d58),
    (0x403d61, x00403d61),
    (0x403d66, x00403d66),
    (0x403d70, x00403d70),
    (0x403d72, x00403d72),
    (0x403d77, x00403d77),
    (0x403d9f, x00403d9f),
    (0x403db1, x00403db1),
    (0x403db3, x00403db3),
    (0x403dc3, x00403dc3),
    (0x403dc6, x00403dc6),
    (0x403dca, x00403dca),
    (0x403dcf, x00403dcf),
    (0x403dd2, x00403dd2),
    (0x403dd4, x00403dd4),
    (0x403dd6, x00403dd6),
    (0x403de1, x00403de1),
    (0x403de5, x00403de5),
    (0x403def, x00403def),
    (0x403df7, x00403df7),
    (0x403dfe, x00403dfe),
    (0x403e03, x00403e03),
    (0x403e0c, x00403e0c),
    (0x403e11, x00403e11),
    (0x403e18, x00403e18),
    (0x403e20, x00403e20),
    (0x403e29, x00403e29),
    (0x403e2f, x00403e2f),
    (0x403e34, x00403e34),
    (0x403e38, x00403e38),
    (0x403e40, x00403e40),
    (0x403e58, x00403e58),
    (0x403e60, x00403e60),
    (0x403e68, x00403e68),
    (0x403e73, x00403e73),
    (0x403e7c, x00403e7c),
    (0x403e88, x00403e88),
    (0x403e94, x00403e94),
    (0x403e9c, x00403e9c),
    (0x403fd8, x00403fd8),
    (0x403fe8, x00403fe8),
    (0x403ff3, x00403ff3),
    (0x404000, x00404000),
    (0x40400c, x0040400c),
    (0x404018, x00404018),
    (0x404024, x00404024),
    (0x404175, x00404175),
    (0x404182, x00404182),
    (0x404186, x00404186),
    (0x404197, x00404197),
    (0x40419e, x0040419e),
    (0x4041ac, x004041ac),
    (0x4041ae, x004041ae),
    (0x4041b2, x004041b2),
    (0x4041b3, x004041b3),
    (0x4041b7, x004041b7),
    (0x4041c8, x004041c8),
    (0x4041da, x004041da),
    (0x4041e4, x004041e4),
    (0x4041eb, x004041eb),
    (0x4041ef, x004041ef),
    (0x4041fa, x004041fa),
    (0x404205, x00404205),
    (0x40420e, x0040420e),
    (0x404231, x00404231),
    (0x40423b, x0040423b),
    (0x40423e, x0040423e),
    (0x404248, x00404248),
    (0x40424f, x0040424f),
    (0x404257, x00404257),
    (0x404261, x00404261),
    (0x40427d, x0040427d),
    (0x404284, x00404284),
    (0x40428b, x0040428b),
    (0x404295, x00404295),
    (0x40429e, x0040429e),
    (0x4042a9, x004042a9),
    (0x4042b0, x004042b0),
    (0x4042bc, x004042bc),
    (0x4042d4, x004042d4),
    (0x4042ea, x004042ea),
    (0x4042f6, x004042f6),
    (0x4042f9, x004042f9),
    (0x404308, x00404308),
    (0x40430e, x0040430e),
    (0x404320, x00404320),
    (0x404326, x00404326),
    (0x404332, x00404332),
    (0x40433a, x0040433a),
    (0x40433f, x0040433f),
    (0x404344, x00404344),
    (0x404348, x00404348),
    (0x40434b, x0040434b),
    (0x404350, x00404350),
    (0x404360, x00404360),
    (0x404370, x00404370),
    (0x404375, x00404375),
    (0x404385, x00404385),
    (0x40438a, x0040438a),
    (0x404399, x00404399),
    (0x40439a, x0040439a),
    (0x4043a5, x004043a5),
    (0x4043aa, x004043aa),
    (0x4043af, x004043af),
    (0x4043b2, x004043b2),
    (0x4043b5, x004043b5),
    (0x4043bb, x004043bb),
    (0x4043c1, x004043c1),
    (0x4043c7, x004043c7),
    (0x4043cd, x004043cd),
    (0x4043f6, x004043f6),
    (0x404410, x00404410),
    (0x404419, x00404419),
    (0x404420, x00404420),
    (0x40442c, x0040442c),
    (0x40443a, x0040443a),
    (0x40443f, x0040443f),
    (0x404449, x00404449),
    (0x404466, x00404466),
    (0x40446f, x0040446f),
    (0x404471, x00404471),
    (0x404495, x00404495),
    (0x4044b9, x004044b9),
    (0x4044e1, x004044e1),
    (0x4044ec, x004044ec),
    (0x4044f4, x004044f4),
    (0x404502, x00404502),
    (0x40450a, x0040450a),
    (0x40450f, x0040450f),
    (0x40451f, x0040451f),
    (0x404526, x00404526),
    (0x40452d, x0040452d),
    (0x40452f, x0040452f),
    (0x404536, x00404536),
    (0x40453b, x0040453b),
    (0x404540, x00404540),
    (0x40454c, x0040454c),
    (0x404554, x00404554),
    (0x404559, x00404559),
    (0x40455e, x0040455e),
    (0x40456c, x0040456c),
    (0x404573, x00404573),
    (0x404578, x00404578),
    (0x40457b, x0040457b),
    (0x404584, x00404584),
    (0x40458b, x0040458b),
    (0x404596, x00404596),
    (0x4045a0, x004045a0),
    (0x4045b0, x004045b0),
    (0x4045bd, x004045bd),
    (0x4045c4, x004045c4),
    (0x4045cc, x004045cc),
    (0x4045e4, x004045e4),
    (0x4045eb, x004045eb),
    (0x4045ef, x004045ef),
    (0x4045f6, x004045f6),
    (0x4045fd, x004045fd),
    (0x4045ff, x004045ff),
    (0x404604, x00404604),
    (0x404609, x00404609),
    (0x40460e, x0040460e),
    (0x404611, x00404611),
    (0x40461d, x0040461d),
    (0x404624, x00404624),
    (0x40462f, x0040462f),
    (0x404631, x00404631),
    (0x404636, x00404636),
    (0x404650, x00404650),
    (0x404654, x00404654),
    (0x404658, x00404658),
    (0x404660, x00404660),
    (0x404668, x00404668),
    (0x40466a, x0040466a),
    (0x404672, x00404672),
    (0x40467f, x0040467f),
    (0x404688, x00404688),
    (0x404690, x00404690),
    (0x40469c, x0040469c),
    (0x4046a3, x004046a3),
    (0x4046ab, x004046ab),
    (0x4046b0, x004046b0),
    (0x4046c8, x004046c8),
    (0x4046cf, x004046cf),
    (0x4046d3, x004046d3),
    (0x4046da, x004046da),
    (0x4046e1, x004046e1),
    (0x4046e3, x004046e3),
    (0x4046ed, x004046ed),
    (0x4046f7, x004046f7),
    (0x404701, x00404701),
    (0x40470b, x0040470b),
    (0x40471e, x0040471e),
    (0x404722, x00404722),
    (0x40473a, x0040473a),
    (0x404743, x00404743),
    (0x40474e, x0040474e),
    (0x404752, x00404752),
    (0x404761, x00404761),
    (0x404765, x00404765),
    (0x40476a, x0040476a),
    (0x404772, x00404772),
    (0x404778, x00404778),
    (0x40477d, x0040477d),
    (0x404783, x00404783),
    (0x40478e, x0040478e),
    (0x404797, x00404797),
    (0x4047a0, x004047a0),
    (0x4047a7, x004047a7),
    (0x4047ac, x004047ac),
    (0x4047b5, x004047b5),
    (0x4047bb, x004047bb),
    (0x4047bf, x004047bf),
    (0x4047c8, x004047c8),
    (0x4047cb, x004047cb),
    (0x4047d0, x004047d0),
    (0x4047d5, x004047d5),
    (0x4047db, x004047db),
    (0x4047df, x004047df),
    (0x4047e7, x004047e7),
    (0x4047f7, x004047f7),
    (0x404803, x00404803),
    (0x40480b, x0040480b),
    (0x40480f, x0040480f),
    (0x40481a, x0040481a),
    (0x40481e, x0040481e),
    (0x404826, x00404826),
    (0x40482f, x0040482f),
    (0x404836, x00404836),
    (0x40483a, x0040483a),
    (0x404845, x00404845),
    (0x404849, x00404849),
    (0x404852, x00404852),
    (0x404858, x00404858),
    (0x40485d, x0040485d),
    (0x404869, x00404869),
    (0x40486b, x0040486b),
    (0x404877, x00404877),
    (0x40487d, x0040487d),
    (0x404886, x00404886),
    (0x404893, x00404893),
    (0x4048a1, x004048a1),
    (0x4048a5, x004048a5),
    (0x4048ab, x004048ab),
    (0x4048b1, x004048b1),
    (0x4048bb, x004048bb),
    (0x4048c0, x004048c0),
    (0x4048c5, x004048c5),
    (0x4048ce, x004048ce),
    (0x4048d2, x004048d2),
    (0x4048d7, x004048d7),
    (0x4048e7, x004048e7),
    (0x4048ed, x004048ed),
    (0x4048fe, x004048fe),
    (0x404902, x00404902),
    (0x404907, x00404907),
    (0x40490d, x0040490d),
    (0x404915, x00404915),
    (0x40491a, x0040491a),
    (0x404921, x00404921),
    (0x404923, x00404923),
    (0x404925, x00404925),
    (0x404930, x00404930),
    (0x404940, x00404940),
    (0x404946, x00404946),
    (0x404958, x00404958),
    (0x40495f, x0040495f),
    (0x404963, x00404963),
    (0x40496b, x0040496b),
    (0x404976, x00404976),
    (0x40499c, x0040499c),
    (0x4049a3, x004049a3),
    (0x4049aa, x004049aa),
    (0x4049b2, x004049b2),
    (0x4049b4, x004049b4),
    (0x4049b8, x004049b8),
    (0x4049bf, x004049bf),
    (0x4049c3, x004049c3),
    (0x4049c7, x004049c7),
    (0x4049cb, x004049cb),
    (0x4049d2, x004049d2),
    (0x4049d6, x004049d6),
    (0x4049da, x004049da),
    (0x4049de, x004049de),
    (0x4049e0, x004049e0),
    (0x4049e7, x004049e7),
    (0x4049ee, x004049ee),
    (0x4049f5, x004049f5),
    (0x404a00, x00404a00),
    (0x404a11, x00404a11),
    (0x404a18, x00404a18),
    (0x404a25, x00404a25),
    (0x404a29, x00404a29),
    (0x404a2c, x00404a2c),
    (0x404a30, x00404a30),
    (0x404a34, x00404a34),
    (0x404a3a, x00404a3a),
    (0x404a41, x00404a41),
    (0x404a44, x00404a44),
    (0x404a4b, x00404a4b),
    (0x404a54, x00404a54),
    (0x404a5b, x00404a5b),
    (0x404a65, x00404a65),
    (0x404a67, x00404a67),
    (0x404a73, x00404a73),
    (0x404a7a, x00404a7a),
    (0x404a80, x00404a80),
    (0x404a8b, x00404a8b),
    (0x404aab, x00404aab),
    (0x404aad, x00404aad),
    (0x404aaf, x00404aaf),
    (0x404ab1, x00404ab1),
    (0x404ac0, x00404ac0),
    (0x404acc, x00404acc),
    (0x404ae0, x00404ae0),
    (0x404aef, x00404aef),
    (0x404afc, x00404afc),
    (0x404b07, x00404b07),
    (0x404b0d, x00404b0d),
    (0x404b1b, x00404b1b),
    (0x404b24, x00404b24),
    (0x404b2c, x00404b2c),
    (0x404b39, x00404b39),
    (0x404b3e, x00404b3e),
    (0x404b47, x00404b47),
    (0x404b49, x00404b49),
    (0x404b4f, x00404b4f),
    (0x404b58, x00404b58),
    (0x404b5b, x00404b5b),
    (0x404b5d, x00404b5d),
    (0x404b70, x00404b70),
    (0x404b74, x00404b74),
    (0x404b80, x00404b80),
    (0x404b89, x00404b89),
    (0x404b9d, x00404b9d),
    (0x404ba2, x00404ba2),
    (0x404ba4, x00404ba4),
    (0x404bad, x00404bad),
    (0x404bb1, x00404bb1),
    (0x404bb9, x00404bb9),
    (0x404bc0, x00404bc0),
    (0x404bc5, x00404bc5),
    (0x404bcf, x00404bcf),
    (0x404bd2, x00404bd2),
    (0x404bda, x00404bda),
    (0x404be2, x00404be2),
    (0x404bec, x00404bec),
    (0x404bf4, x00404bf4),
    (0x404bfb, x00404bfb),
    (0x404bfe, x00404bfe),
    (0x404c01, x00404c01),
    (0x404c03, x00404c03),
    (0x404c09, x00404c09),
    (0x404c11, x00404c11),
    (0x404c2b, x00404c2b),
    (0x404c2f, x00404c2f),
    (0x404c33, x00404c33),
    (0x404c3b, x00404c3b),
    (0x404c43, x00404c43),
    (0x404c47, x00404c47),
    (0x404c51, x00404c51),
    (0x404c5b, x00404c5b),
    (0x404c5f, x00404c5f),
    (0x404c67, x00404c67),
    (0x404c69, x00404c69),
    (0x404c71, x00404c71),
    (0x404c76, x00404c76),
    (0x404c80, x00404c80),
    (0x404c98, x00404c98),
    (0x404ca0, x00404ca0),
    (0x404ca8, x00404ca8),
    (0x404cb3, x00404cb3),
    (0x404cbc, x00404cbc),
    (0x404cc8, x00404cc8),
    (0x404cd4, x00404cd4),
    (0x404cdc, x00404cdc),
    (0x404e18, x00404e18),
    (0x404e28, x00404e28),
    (0x404e33, x00404e33),
    (0x404e40, x00404e40),
    (0x404e4c, x00404e4c),
    (0x404e58, x00404e58),
    (0x404e64, x00404e64),
    (0x404fc0, x00404fc0),
    (0x404fcc, x00404fcc),
    (0x404fda, x00404fda),
    (0x404fe1, x00404fe1),
    (0x404fe3, x00404fe3),
    (0x404fe9, x00404fe9),
    (0x405001, x00405001),
    (0x405007, x00405007),
    (0x40500d, x0040500d),
    (0x405013, x00405013),
    (0x405018, x00405018),
    (0x405048, x00405048),
    (0x405060, x00405060),
    (0x405064, x00405064),
    (0x40506c, x0040506c),
    (0x40507c, x0040507c),
    (0x405084, x00405084),
    (0x40508e, x0040508e),
    (0x405093, x00405093),
    (0x40509e, x0040509e),
    (0x4050a3, x004050a3),
    (0x4050ad, x004050ad),
    (0x4050c5, x004050c5),
    (0x4050ca, x004050ca),
    (0x4050d3, x004050d3),
    (0x4050d8, x004050d8),
    (0x4050e0, x004050e0),
    (0x4050fd, x004050fd),
    (0x40510a, x0040510a),
    (0x40511a, x0040511a),
    (0x405128, x00405128),
    (0x40512c, x0040512c),
    (0x40513b, x0040513b),
    (0x405140, x00405140),
    (0x405155, x00405155),
    (0x405159, x00405159),
    (0x40516b, x0040516b),
    (0x405174, x00405174),
    (0x40517a, x0040517a),
    (0x405183, x00405183),
    (0x405188, x00405188),
    (0x40519e, x0040519e),
    (0x4051a6, x004051a6),
    (0x4051a8, x004051a8),
    (0x4051ba, x004051ba),
    (0x4051ce, x004051ce),
    (0x4051dc, x004051dc),
    (0x4051e0, x004051e0),
    (0x4051ee, x004051ee),
    (0x4051f2, x004051f2),
    (0x405206, x00405206),
    (0x40520a, x0040520a),
    (0x405211, x00405211),
    (0x405215, x00405215),
    (0x40521b, x0040521b),
    (0x40522b, x0040522b),
    (0x405235, x00405235),
    (0x40523c, x0040523c),
    (0x40524c, x0040524c),
    (0x405251, x00405251),
    (0x405259, x00405259),
    (0x40525f, x0040525f),
    (0x405264, x00405264),
    (0x405267, x00405267),
    (0x405298, x00405298),
    (0x4052ac, x004052ac),
    (0x4052b0, x004052b0),
    (0x4052b4, x004052b4),
    (0x4052c6, x004052c6),
    (0x4052ce, x004052ce),
    (0x4052d1, x004052d1),
    (0x4052d6, x004052d6),
    (0x4052db, x004052db),
    (0x4052e2, x004052e2),
    (0x4052e7, x004052e7),
    (0x4052fa, x004052fa),
    (0x4052ff, x004052ff),
    (0x405308, x00405308),
    (0x40530d, x0040530d),
    (0x405315, x00405315),
    (0x405332, x00405332),
    (0x405339, x00405339),
    (0x40534b, x0040534b),
    (0x40535b, x0040535b),
    (0x405360, x00405360),
    (0x405364, x00405364),
    (0x40536b, x0040536b),
    (0x405373, x00405373),
    (0x405388, x00405388),
    (0x40538c, x0040538c),
    (0x40539a, x0040539a),
    (0x40539c, x0040539c),
    (0x40539e, x0040539e),
    (0x4053b0, x004053b0),
    (0x4053be, x004053be),
    (0x4053ca, x004053ca),
    (0x4053d3, x004053d3),
    (0x4053db, x004053db),
    (0x4053e6, x004053e6),
    (0x4053ef, x004053ef),
    (0x4053f7, x004053f7),
    (0x4053fb, x004053fb),
    (0x405406, x00405406),
    (0x40540a, x0040540a),
    (0x40540e, x0040540e),
    (0x405423, x00405423),
    (0x405431, x00405431),
    (0x40543a, x0040543a),
    (0x40545a, x0040545a),
    (0x405461, x00405461),
    (0x405466, x00405466),
    (0x40546c, x0040546c),
    (0x405479, x00405479),
    (0x40547c, x0040547c),
    (0x40548e, x0040548e),
    (0x40549a, x0040549a),
    (0x4054b1, x004054b1),
    (0x4054bf, x004054bf),
    (0x4054c8, x004054c8),
    (0x4054df, x004054df),
    (0x4054e6, x004054e6),
    (0x4054e8, x004054e8),
    (0x4054ec, x004054ec),
    (0xf000_0000, runtime::return_from_main),
];
