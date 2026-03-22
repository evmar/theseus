#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(static_mut_refs)]
#![allow(unused_parens)]

use runtime::*;
use winapi::*;

fn init_mappings() {
    unsafe {
        let mut mappings = kernel32::state().mappings.borrow_mut();
        mappings.alloc("null page".to_string(), Some(0x0), 0x1000);
        mappings.alloc("imported functions".to_string(), Some(0x1000), 0x1000);
        mappings.alloc("exe header".to_string(), Some(0x400000), 0x1000);
        let bytes = include_bytes!("../data/00400000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x400000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".text".to_string(), Some(0x401000), 0x1000);
        let bytes = include_bytes!("../data/00401000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x401000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".rdata".to_string(), Some(0x402000), 0x1000);
        let bytes = include_bytes!("../data/00402000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x402000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".data".to_string(), Some(0x403000), 0x1000);
        mappings.alloc(".reloc".to_string(), Some(0x404000), 0x1000);
        let bytes = include_bytes!("../data/00404000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x404000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
    }
}
#[allow(unused_variables)]
pub fn x00401000(m: &mut Machine) -> Cont {
    // 00401000 push ebp
    push(m, m.regs.ebp);
    // 00401001 mov ebp,esp
    m.regs.ebp = m.regs.esp;
    // 00401003 and esp,0FFFFFFF0h
    m.regs.esp = and(m.regs.esp, 0xfffffff0u32, &mut m.flags);
    // 00401006 sub esp,10h
    m.regs.esp = sub(m.regs.esp, 0x10u32, &mut m.flags);
    // 00401009 call 00401015h
    call(m, 0x40100e, Cont(x00401015))
}

#[allow(unused_variables)]
pub fn x0040100e(m: &mut Machine) -> Cont {
    // 0040100e push 0
    push(m, 0x0u32);
    // 00401010 call 00401077h
    call(m, 0x401015, Cont(x00401077))
}

#[allow(unused_variables)]
pub fn x00401015(m: &mut Machine) -> Cont {
    // 00401015 push ebp
    push(m, m.regs.ebp);
    // 00401016 push ebx
    push(m, m.regs.ebx);
    // 00401017 push edi
    push(m, m.regs.edi);
    // 00401018 push esi
    push(m, m.regs.esi);
    // 00401019 push eax
    push(m, m.regs.eax);
    // 0040101a cmp byte ptr ds:[403000h],0
    sub(m.memory.read::<u8>(0x403000u32), 0x0u8, &mut m.flags);
    // 00401021 jne short 0040102Ah
    jne(m, Cont(x00401023), Cont(x0040102a))
}

#[allow(unused_variables)]
pub fn x00401023(m: &mut Machine) -> Cont {
    // 00401023 mov byte ptr ds:[403000h],1
    m.memory.write::<u8>(0x403000u32, 0x1u8);
    // 0040102a mov eax,fs:[18h]
    m.regs.eax = m.memory.read::<u32>(m.regs.fs_base.wrapping_add(0x18u32));
    // 00401030 mov eax,[eax+30h]
    m.regs.eax = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x30u32));
    // 00401033 xor edi,edi
    m.regs.edi = xor(m.regs.edi, m.regs.edi, &mut m.flags);
    // 00401035 mov eax,[eax+10h]
    m.regs.eax = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x10u32));
    // 00401038 mov esi,[eax+20h]
    m.regs.esi = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x20u32));
    // 0040103b push 0Eh
    push(m, 0xeu32);
    // 0040103d pop ebx
    m.regs.ebx = pop(m);
    // 0040103e mov ebp,esp
    m.regs.ebp = m.regs.esp;
    // 00401040 cmp edi,0Eh
    sub(m.regs.edi, 0xeu32, &mut m.flags);
    // 00401043 je short 00401068h
    je(m, Cont(x00401045), Cont(x00401068))
}

#[allow(unused_variables)]
pub fn x0040102a(m: &mut Machine) -> Cont {
    // 0040102a mov eax,fs:[18h]
    m.regs.eax = m.memory.read::<u32>(m.regs.fs_base.wrapping_add(0x18u32));
    // 00401030 mov eax,[eax+30h]
    m.regs.eax = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x30u32));
    // 00401033 xor edi,edi
    m.regs.edi = xor(m.regs.edi, m.regs.edi, &mut m.flags);
    // 00401035 mov eax,[eax+10h]
    m.regs.eax = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x10u32));
    // 00401038 mov esi,[eax+20h]
    m.regs.esi = m.memory.read::<u32>(m.regs.eax.wrapping_add(0x20u32));
    // 0040103b push 0Eh
    push(m, 0xeu32);
    // 0040103d pop ebx
    m.regs.ebx = pop(m);
    // 0040103e mov ebp,esp
    m.regs.ebp = m.regs.esp;
    // 00401040 cmp edi,0Eh
    sub(m.regs.edi, 0xeu32, &mut m.flags);
    // 00401043 je short 00401068h
    je(m, Cont(x00401045), Cont(x00401068))
}

#[allow(unused_variables)]
pub fn x00401040(m: &mut Machine) -> Cont {
    // 00401040 cmp edi,0Eh
    sub(m.regs.edi, 0xeu32, &mut m.flags);
    // 00401043 je short 00401068h
    je(m, Cont(x00401045), Cont(x00401068))
}

#[allow(unused_variables)]
pub fn x00401045(m: &mut Machine) -> Cont {
    // 00401045 mov ecx,ebx
    m.regs.ecx = m.regs.ebx;
    // 00401047 lea eax,[edi+402000h]
    m.regs.eax = m.regs.edi.wrapping_add(0x402000u32);
    // 0040104d sub ecx,edi
    m.regs.ecx = sub(m.regs.ecx, m.regs.edi, &mut m.flags);
    // 0040104f push 0
    push(m, 0x0u32);
    // 00401051 push ebp
    push(m, m.regs.ebp);
    // 00401052 push ecx
    push(m, m.regs.ecx);
    // 00401053 push eax
    push(m, m.regs.eax);
    // 00401054 push esi
    push(m, m.regs.esi);
    // 00401055 call 00401083h
    call(m, 0x40105a, Cont(x00401083))
}

#[allow(unused_variables)]
pub fn x0040105a(m: &mut Machine) -> Cont {
    // 0040105a test eax,eax
    and(m.regs.eax, m.regs.eax, &mut m.flags);
    // 0040105c je short 00401063h
    je(m, Cont(x0040105e), Cont(x00401063))
}

#[allow(unused_variables)]
pub fn x0040105e(m: &mut Machine) -> Cont {
    // 0040105e add edi,[esp]
    m.regs.edi = add(m.regs.edi, m.memory.read::<u32>(m.regs.esp), &mut m.flags);
    // 00401061 jmp short 00401040h
    Cont(x00401040)
}

#[allow(unused_variables)]
pub fn x00401063(m: &mut Machine) -> Cont {
    // 00401063 call 0040107Dh
    call(m, 0x401068, Cont(x0040107d))
}

#[allow(unused_variables)]
pub fn x00401068(m: &mut Machine) -> Cont {
    // 00401068 mov byte ptr ds:[403000h],0
    m.memory.write::<u8>(0x403000u32, 0x0u8);
    // 0040106f add esp,4
    m.regs.esp = add(m.regs.esp, 0x4u32, &mut m.flags);
    // 00401072 pop esi
    m.regs.esi = pop(m);
    // 00401073 pop edi
    m.regs.edi = pop(m);
    // 00401074 pop ebx
    m.regs.ebx = pop(m);
    // 00401075 pop ebp
    m.regs.ebp = pop(m);
    // 00401076 ret
    ret(m, 0)
}

#[allow(unused_variables)]
pub fn x00401077(m: &mut Machine) -> Cont {
    // 00401077 jmp dword ptr ds:[402048h]
    Cont(kernel32::ExitProcess_stdcall)
}

#[allow(unused_variables)]
pub fn x0040107d(m: &mut Machine) -> Cont {
    // 0040107d jmp dword ptr ds:[40204Ch]
    Cont(kernel32::GetLastError_stdcall)
}

#[allow(unused_variables)]
pub fn x00401083(m: &mut Machine) -> Cont {
    // 00401083 jmp dword ptr ds:[402050h]
    Cont(kernel32::WriteFile_stdcall)
}

const BLOCKS: [(u32, fn(&mut Machine) -> Cont); 18] = [
    (0x001001, kernel32::ExitProcess_stdcall),
    (0x001002, kernel32::GetLastError_stdcall),
    (0x001003, kernel32::WriteFile_stdcall),
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
    (0xf000_0000, runtime::return_from_main),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x00401000),
};
