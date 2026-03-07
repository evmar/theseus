#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(static_mut_refs)]

use runtime::*;

pub fn x00401000() -> u32 {
    unsafe {
        // 00401000 push ebp
        push(REGS.ebp);
        // 00401001 mov ebp,esp
        REGS.ebp = REGS.esp;
        // 00401003 and esp,0FFFFFFF0h
        REGS.esp &= 0xfffffff0u32;
        // 00401006 sub esp,10h
        REGS.esp = sub(REGS.esp, 0x10u32);
        // 00401009 call 00401015h
        call(0x40100e, 0x401015)
    }
}

pub fn x0040100e() -> u32 {
    unsafe {
        // 0040100e push 0
        push(0x0u32);
        // 00401010 call 00401077h
        call(0x401015, 0x401077)
    }
}

pub fn x00401015() -> u32 {
    unsafe {
        // 00401015 push ebp
        push(REGS.ebp);
        // 00401016 push ebx
        push(REGS.ebx);
        // 00401017 push edi
        push(REGS.edi);
        // 00401018 push esi
        push(REGS.esi);
        // 00401019 push eax
        push(REGS.eax);
        // 0040101a cmp byte ptr ds:[403000h],0
        sub(*(MEMORY.add((0x403000u32) as usize) as *mut u8), 0x0u8);
        // 00401021 jne short 0040102Ah
        jne(0x401023, 0x40102au32)
    }
}

pub fn x00401023() -> u32 {
    unsafe {
        // 00401023 mov byte ptr ds:[403000h],1
        *(MEMORY.add((0x403000u32) as usize) as *mut u8) = 0x1u8;
        // 0040102a mov eax,fs:[18h]
        REGS.eax = *(MEMORY.add((REGS.fs_base + 0x18u32) as usize) as *mut u32);
        // 00401030 mov eax,[eax+30h]
        REGS.eax = *(MEMORY.add((REGS.eax + 0x30u32) as usize) as *mut u32);
        // 00401033 xor edi,edi
        REGS.edi ^= REGS.edi;
        // 00401035 mov eax,[eax+10h]
        REGS.eax = *(MEMORY.add((REGS.eax + 0x10u32) as usize) as *mut u32);
        // 00401038 mov esi,[eax+20h]
        REGS.esi = *(MEMORY.add((REGS.eax + 0x20u32) as usize) as *mut u32);
        // 0040103b push 0Eh
        push(0xeu32);
        // 0040103d pop ebx
        REGS.ebx = pop();
        // 0040103e mov ebp,esp
        REGS.ebp = REGS.esp;
        // 00401040 cmp edi,0Eh
        sub(REGS.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(0x401045, 0x401068u32)
    }
}

pub fn x0040102a() -> u32 {
    unsafe {
        // 0040102a mov eax,fs:[18h]
        REGS.eax = *(MEMORY.add((REGS.fs_base + 0x18u32) as usize) as *mut u32);
        // 00401030 mov eax,[eax+30h]
        REGS.eax = *(MEMORY.add((REGS.eax + 0x30u32) as usize) as *mut u32);
        // 00401033 xor edi,edi
        REGS.edi ^= REGS.edi;
        // 00401035 mov eax,[eax+10h]
        REGS.eax = *(MEMORY.add((REGS.eax + 0x10u32) as usize) as *mut u32);
        // 00401038 mov esi,[eax+20h]
        REGS.esi = *(MEMORY.add((REGS.eax + 0x20u32) as usize) as *mut u32);
        // 0040103b push 0Eh
        push(0xeu32);
        // 0040103d pop ebx
        REGS.ebx = pop();
        // 0040103e mov ebp,esp
        REGS.ebp = REGS.esp;
        // 00401040 cmp edi,0Eh
        sub(REGS.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(0x401045, 0x401068u32)
    }
}

pub fn x00401040() -> u32 {
    unsafe {
        // 00401040 cmp edi,0Eh
        sub(REGS.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(0x401045, 0x401068u32)
    }
}

pub fn x00401045() -> u32 {
    unsafe {
        // 00401045 mov ecx,ebx
        REGS.ecx = REGS.ebx;
        // 00401047 lea eax,[edi+402000h]
        REGS.eax = REGS.edi + 0x402000u32;
        // 0040104d sub ecx,edi
        REGS.ecx = sub(REGS.ecx, REGS.edi);
        // 0040104f push 0
        push(0x0u32);
        // 00401051 push ebp
        push(REGS.ebp);
        // 00401052 push ecx
        push(REGS.ecx);
        // 00401053 push eax
        push(REGS.eax);
        // 00401054 push esi
        push(REGS.esi);
        // 00401055 call 00401083h
        call(0x40105a, 0x401083)
    }
}

pub fn x0040105a() -> u32 {
    unsafe {
        // 0040105a test eax,eax
        and(REGS.eax, REGS.eax);
        // 0040105c je short 00401063h
        je(0x40105e, 0x401063u32)
    }
}

pub fn x0040105e() -> u32 {
    unsafe {
        // 0040105e add edi,[esp]
        REGS.edi += *(MEMORY.add((REGS.esp + 0x0u32) as usize) as *mut u32);
        // 00401061 jmp short 00401040h
        jmp(0x401040u32)
    }
}

pub fn x00401063() -> u32 {
    unsafe {
        // 00401063 call 0040107Dh
        call(0x401068, 0x40107d)
    }
}

pub fn x00401068() -> u32 {
    unsafe {
        // 00401068 mov byte ptr ds:[403000h],0
        *(MEMORY.add((0x403000u32) as usize) as *mut u8) = 0x0u8;
        // 0040106f add esp,4
        REGS.esp += 0x4u32;
        // 00401072 pop esi
        REGS.esi = pop();
        // 00401073 pop edi
        REGS.edi = pop();
        // 00401074 pop ebx
        REGS.ebx = pop();
        // 00401075 pop ebp
        REGS.ebp = pop();
        // 00401076 ret
        todo!("ret");
    }
}

pub fn x00401077() -> u32 {
    unsafe {
        // 00401077 jmp dword ptr ds:[402048h]
        jmp(kernel32::stdcall_ExitProcess())
    }
}

pub fn x0040107d() -> u32 {
    unsafe {
        // 0040107d jmp dword ptr ds:[40204Ch]
        jmp(kernel32::stdcall_GetLastError())
    }
}

pub fn x00401083() -> u32 {
    unsafe {
        // 00401083 jmp dword ptr ds:[402050h]
        jmp(kernel32::stdcall_WriteFile())
    }
}

pub const BLOCKS: [(u32, fn() -> u32); 14] = [
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
];
