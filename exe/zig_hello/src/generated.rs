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
        sub(*((0x403000u32) as *mut u8), 0x0u8);
        // 00401021 jne short 0040102Ah
        jne(4198435, 4198442)
    }
}

pub fn x0040102a() -> u32 {
    unsafe {
        // 0040102a mov eax,fs:[18h]
        REGS.eax = todo!();
        // 00401030 mov eax,[eax+30h]
        REGS.eax = *((REGS.eax + 0x30u32) as *mut u32);
        // 00401033 xor edi,edi
        REGS.edi ^= REGS.edi;
        // 00401035 mov eax,[eax+10h]
        REGS.eax = *((REGS.eax + 0x10u32) as *mut u32);
        // 00401038 mov esi,[eax+20h]
        REGS.esi = *((REGS.eax + 0x20u32) as *mut u32);
        // 0040103b push 0Eh
        push(0xeu32);
        // 0040103d pop ebx
        REGS.ebx = pop();
        // 0040103e mov ebp,esp
        REGS.ebp = REGS.esp;
        // 00401040 cmp edi,0Eh
        sub(REGS.edi, 0xeu32);
        // 00401043 je short 00401068h
        todo!("je short 00401068h");
    }
}

pub fn x00401068() -> u32 {
    unsafe {
        // 00401068 mov byte ptr ds:[403000h],0
        *((0x403000u32) as *mut u8) = 0x0u8;
        // 0040106f add esp,4
        REGS.esp -= 0x4u32;
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

pub const BLOCKS: [(u32, fn() -> u32); 4] = [
    (0x401000, x00401000),
    (0x401015, x00401015),
    (0x40102a, x0040102a),
    (0x401068, x00401068),
];
