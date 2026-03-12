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
        mappings.alloc(".text".to_string(), 0x401000, 0x1000);
        let bytes = include_bytes!("../data/00401000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x401000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".rdata".to_string(), 0x402000, 0x1000);
        let bytes = include_bytes!("../data/00402000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x402000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
        mappings.alloc(".data".to_string(), 0x403000, 0x1000);
        mappings.alloc(".reloc".to_string(), 0x404000, 0x1000);
        let bytes = include_bytes!("../data/00404000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x404000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
    }
}
pub fn x00401000() -> Cont {
    unsafe {
        // 00401000 push ebp
        push(MACHINE.regs.ebp);
        // 00401001 mov ebp,esp
        MACHINE.regs.ebp = MACHINE.regs.esp;
        // 00401003 and esp,0FFFFFFF0h
        MACHINE.regs.esp = and(MACHINE.regs.esp, 0xfffffff0u32);
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
        sub(MACHINE.memory.read::<u8>(0x403000u32), 0x0u8);
        // 00401021 jne short 0040102Ah
        jne(Cont(x00401023), Cont(x0040102a))
    }
}

pub fn x00401023() -> Cont {
    unsafe {
        // 00401023 mov byte ptr ds:[403000h],1
        MACHINE.memory.write::<u8>(0x403000u32, 0x1u8);
        // 0040102a mov eax,fs:[18h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.fs_base.wrapping_add(0x18u32));
        // 00401030 mov eax,[eax+30h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x30u32));
        // 00401033 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401035 mov eax,[eax+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00401038 mov esi,[eax+20h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x20u32));
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
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.fs_base.wrapping_add(0x18u32));
        // 00401030 mov eax,[eax+30h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x30u32));
        // 00401033 xor edi,edi
        MACHINE.regs.edi ^= MACHINE.regs.edi;
        // 00401035 mov eax,[eax+10h]
        MACHINE.regs.eax = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x10u32));
        // 00401038 mov esi,[eax+20h]
        MACHINE.regs.esi = MACHINE
            .memory
            .read::<u32>(MACHINE.regs.eax.wrapping_add(0x20u32));
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
        MACHINE.regs.eax = MACHINE.regs.edi.wrapping_add(0x402000u32);
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
        MACHINE.regs.edi = add(
            MACHINE.regs.edi,
            MACHINE
                .memory
                .read::<u32>(MACHINE.regs.esp.wrapping_add(0x0u32)),
        );
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
        MACHINE.memory.write::<u8>(0x403000u32, 0x0u8);
        // 0040106f add esp,4
        MACHINE.regs.esp = add(MACHINE.regs.esp, 0x4u32);
        // 00401072 pop esi
        MACHINE.regs.esi = pop();
        // 00401073 pop edi
        MACHINE.regs.edi = pop();
        // 00401074 pop ebx
        MACHINE.regs.ebx = pop();
        // 00401075 pop ebp
        MACHINE.regs.ebp = pop();
        // 00401076 ret
        ret(0)
    }
}

pub fn x00401077() -> Cont {
    unsafe {
        // 00401077 jmp dword ptr ds:[402048h]
        Cont(kernel32::ExitProcess_stdcall)
    }
}

pub fn x0040107d() -> Cont {
    unsafe {
        // 0040107d jmp dword ptr ds:[40204Ch]
        Cont(kernel32::GetLastError_stdcall)
    }
}

pub fn x00401083() -> Cont {
    unsafe {
        // 00401083 jmp dword ptr ds:[402050h]
        Cont(kernel32::WriteFile_stdcall)
    }
}

pub const BLOCKS: [(u32, fn() -> Cont); 48] = [
    (0x001001, kernel32::ExitProcess_stdcall),
    (0x001002, kernel32::GetLastError_stdcall),
    (0x001003, kernel32::WriteFile_stdcall),
    (0x001004, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0x001005, ddraw::IDirectDraw7::AddRef_stdcall),
    (0x001006, ddraw::IDirectDraw7::Release_stdcall),
    (0x001007, ddraw::IDirectDraw7::Compact_stdcall),
    (0x001008, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0x001009, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0x00100a, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0x00100b, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0x00100c, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0x00100d, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0x00100e, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0x00100f, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0x001010, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0x001011, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0x001012, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0x001013, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0x001014, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (
        0x001015,
        ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall,
    ),
    (0x001016, ddraw::IDirectDraw7::Initialize_stdcall),
    (0x001017, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0x001018, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0x001019, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (0x00101a, ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall),
    (0x00101b, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0x00101c, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0x00101d, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (0x00101e, ddraw::IDirectDraw7::TestCooperativeLevel_stdcall),
    (0x00101f, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0x001020, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0x001021, ddraw::IDirectDraw7::EvaluateMode_stdcall),
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
