#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(static_mut_refs)]

use runtime::*;

pub fn x00401000() -> Cont {
    unsafe {
        // 00401000 push ebp
        push(MACHINE.regs.ebp);
        // 00401001 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401003 and esp,0FFFFFFF0h
        MACHINE.regs.esp &= 0xfffffff0u32;
        // 00401006 sub esp,10h
        MACHINE.regs.esp = sub(MACHINE.regs.esp, 0x10u32);
        // 00401009 call 00401015h
        call(0x40100e, Cont(x00401015))
    }
}

pub fn x0040100e() -> Cont {
    unsafe {
        // 0040100e push 0
        push(0x0u32);
        // 00401010 call 00401077h
        call(0x401015, Cont(x00401077))
    }
}

pub fn x00401015() -> Cont {
    unsafe {
        // 00401015 push ebp
        push(MACHINE.regs.ebp);
        // 00401016 push ebx
        push(MACHINE.regs.ebx);
        // 00401017 push edi
        push(MACHINE.regs.edi);
        // 00401018 push esi
        push(MACHINE.regs.esi);
        // 00401019 push eax
        push(MACHINE.regs.eax);
        // 0040101a cmp byte ptr ds:[403000h],0
        sub(
            *(MACHINE.memory.add((0x403000u32) as usize) as *mut u8),
            0x0u8,
        );
        // 00401021 jne short 0040102Ah
        jne(Cont(x00401023), Cont(x0040102a))
    }
}

pub fn x00401023() -> Cont {
    unsafe {
        // 00401023 mov byte ptr ds:[403000h],1
        *(MACHINE.memory.add((0x403000u32) as usize) as *mut u8) = 0x1u8;
        // 0040102a mov eax,fs:[18h]
        MACHINE.regs.eax = *(MACHINE
            .memory
            .add((MACHINE.regs.fs_base + 0x18u32) as usize)
            as *mut u32);
        // 00401030 mov eax,[eax+30h]
        MACHINE.regs.eax = *(MACHINE.memory.add((MACHINE.regs.eax + 0x30u32) as usize) as *mut u32);
        // 00401033 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401035 mov eax,[eax+10h]
        MACHINE.regs.eax = *(MACHINE.memory.add((MACHINE.regs.eax + 0x10u32) as usize) as *mut u32);
        // 00401038 mov esi,[eax+20h]
        MACHINE.regs.esi = *(MACHINE.memory.add((MACHINE.regs.eax + 0x20u32) as usize) as *mut u32);
        // 0040103b push 0Eh
        push(0xeu32);
        // 0040103d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040103e mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401040 cmp edi,0Eh
        sub(MACHINE.regs.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(Cont(x00401045), Cont(x00401068))
    }
}

pub fn x0040102a() -> Cont {
    unsafe {
        // 0040102a mov eax,fs:[18h]
        MACHINE.regs.eax = *(MACHINE
            .memory
            .add((MACHINE.regs.fs_base + 0x18u32) as usize)
            as *mut u32);
        // 00401030 mov eax,[eax+30h]
        MACHINE.regs.eax = *(MACHINE.memory.add((MACHINE.regs.eax + 0x30u32) as usize) as *mut u32);
        // 00401033 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401035 mov eax,[eax+10h]
        MACHINE.regs.eax = *(MACHINE.memory.add((MACHINE.regs.eax + 0x10u32) as usize) as *mut u32);
        // 00401038 mov esi,[eax+20h]
        MACHINE.regs.esi = *(MACHINE.memory.add((MACHINE.regs.eax + 0x20u32) as usize) as *mut u32);
        // 0040103b push 0Eh
        push(0xeu32);
        // 0040103d pop ebx
        MACHINE.regs.ebx = pop();
        // 0040103e mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401040 cmp edi,0Eh
        sub(MACHINE.regs.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(Cont(x00401045), Cont(x00401068))
    }
}

pub fn x00401040() -> Cont {
    unsafe {
        // 00401040 cmp edi,0Eh
        sub(MACHINE.regs.edi, 0xeu32);
        // 00401043 je short 00401068h
        je(Cont(x00401045), Cont(x00401068))
    }
}

pub fn x00401045() -> Cont {
    unsafe {
        // 00401045 mov ecx,ebx
        MACHINE.regs.ecx = MACHINE.regs.ebx;
        // 00401047 lea eax,[edi+402000h]
        MACHINE.regs.eax = MACHINE.regs.edi + 0x402000u32;
        // 0040104d sub ecx,edi
        MACHINE.regs.ecx = sub(MACHINE.regs.ecx, MACHINE.regs.edi);
        // 0040104f push 0
        push(0x0u32);
        // 00401051 push ebp
        push(MACHINE.regs.ebp);
        // 00401052 push ecx
        push(MACHINE.regs.ecx);
        // 00401053 push eax
        push(MACHINE.regs.eax);
        // 00401054 push esi
        push(MACHINE.regs.esi);
        // 00401055 call 00401083h
        call(0x40105a, Cont(x00401083))
    }
}

pub fn x0040105a() -> Cont {
    unsafe {
        // 0040105a test eax,eax
        and(MACHINE.regs.eax, MACHINE.regs.eax);
        // 0040105c je short 00401063h
        je(Cont(x0040105e), Cont(x00401063))
    }
}

pub fn x0040105e() -> Cont {
    unsafe {
        // 0040105e add edi,[esp]
        MACHINE.regs.edi += *(MACHINE.memory.add((MACHINE.regs.esp + 0x0u32) as usize) as *mut u32);
        // 00401061 jmp short 00401040h
        Cont(x00401040)
    }
}

pub fn x00401063() -> Cont {
    unsafe {
        // 00401063 call 0040107Dh
        call(0x401068, Cont(x0040107d))
    }
}

pub fn x00401068() -> Cont {
    unsafe {
        // 00401068 mov byte ptr ds:[403000h],0
        *(MACHINE.memory.add((0x403000u32) as usize) as *mut u8) = 0x0u8;
        // 0040106f add esp,4
        MACHINE.regs.esp += 0x4u32;
        // 00401072 pop esi
        MACHINE.regs.esi = pop();
        // 00401073 pop edi
        MACHINE.regs.edi = pop();
        // 00401074 pop ebx
        MACHINE.regs.ebx = pop();
        // 00401075 pop ebp
        MACHINE.regs.ebp = pop();
        // 00401076 ret
        indirect(pop())
    }
}

pub fn x00401077() -> Cont {
    unsafe {
        // 00401077 jmp dword ptr ds:[402048h]
        Cont(kernel32::stdcall_ExitProcess)
    }
}

pub fn x0040107d() -> Cont {
    unsafe {
        // 0040107d jmp dword ptr ds:[40204Ch]
        Cont(kernel32::stdcall_GetLastError)
    }
}

pub fn x00401083() -> Cont {
    unsafe {
        // 00401083 jmp dword ptr ds:[402050h]
        Cont(kernel32::stdcall_WriteFile)
    }
}

pub fn init_memory() {
    unsafe {
        let sections = [
            (0x400000, include_bytes!("../data/00400000.raw").as_slice()),
            (0x401000, include_bytes!("../data/00401000.raw").as_slice()),
            (0x402000, include_bytes!("../data/00402000.raw").as_slice()),
            (0x403000, include_bytes!("../data/00403000.raw").as_slice()),
            (0x404000, include_bytes!("../data/00404000.raw").as_slice()),
        ];

        for (addr, data) in sections {
            let out = core::slice::from_raw_parts_mut(MACHINE.memory.add(addr), data.len());
            out.copy_from_slice(data);
        }
    }
}

const BLOCKS: [(u32, fn() -> Cont); 14] = [
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

pub fn indirect(addr: u32) -> Cont {
    let index = BLOCKS
        .binary_search_by_key(&addr, |(addr, _)| *addr)
        .unwrap();
    Cont(BLOCKS[index].1)
}
