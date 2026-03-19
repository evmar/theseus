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
        mappings.alloc(".reloc".to_string(), Some(0x403000), 0x1000);
        let bytes = include_bytes!("../data/00403000.raw").as_slice();
        let out = &mut MACHINE.memory.bytes[0x403000 as usize..][..bytes.len()];
        out.copy_from_slice(bytes);
    }
}
pub fn x00401000() -> Cont {
    unsafe {
        // 00401000 push 0FFFFFFF5h
        push(0xfffffff5u32);
        // 00401002 call dword ptr ds:[402058h]
        call(0x401008, Cont(kernel32::GetStdHandle_stdcall))
    }
}

pub fn x00401008() -> Cont {
    unsafe {
        // 00401008 xor ecx,ecx
        MACHINE.regs.ecx = xor(MACHINE.regs.ecx, MACHINE.regs.ecx);
        // 0040100a push ecx
        push(MACHINE.regs.ecx);
        // 0040100b push ecx
        push(MACHINE.regs.ecx);
        // 0040100c push 6
        push(0x6u32);
        // 0040100e push 402000h
        push(0x402000u32);
        // 00401013 push eax
        push(MACHINE.regs.eax);
        // 00401014 call dword ptr ds:[40205Ch]
        call(0x40101a, Cont(kernel32::WriteFile_stdcall))
    }
}

pub fn x0040101a() -> Cont {
    unsafe {
        // 0040101a ret
        ret(0)
    }
}

const BLOCKS: [(u32, fn() -> Cont); 85] = [
    (0x001001, kernel32::GetStdHandle_stdcall),
    (0x001002, kernel32::WriteFile_stdcall),
    (0x001003, ddraw::IDirectDraw7::QueryInterface_stdcall),
    (0x001004, ddraw::IDirectDraw7::AddRef_stdcall),
    (0x001005, ddraw::IDirectDraw7::Release_stdcall),
    (0x001006, ddraw::IDirectDraw7::Compact_stdcall),
    (0x001007, ddraw::IDirectDraw7::CreateClipper_stdcall),
    (0x001008, ddraw::IDirectDraw7::CreatePalette_stdcall),
    (0x001009, ddraw::IDirectDraw7::CreateSurface_stdcall),
    (0x00100a, ddraw::IDirectDraw7::DuplicateSurface_stdcall),
    (0x00100b, ddraw::IDirectDraw7::EnumDisplayModes_stdcall),
    (0x00100c, ddraw::IDirectDraw7::EnumSurfaces_stdcall),
    (0x00100d, ddraw::IDirectDraw7::FlipToGDISurface_stdcall),
    (0x00100e, ddraw::IDirectDraw7::GetCaps_stdcall),
    (0x00100f, ddraw::IDirectDraw7::GetDisplayMode_stdcall),
    (0x001010, ddraw::IDirectDraw7::GetFourCCCodes_stdcall),
    (0x001011, ddraw::IDirectDraw7::GetGDISurface_stdcall),
    (0x001012, ddraw::IDirectDraw7::GetMonitorFrequency_stdcall),
    (0x001013, ddraw::IDirectDraw7::GetScanLine_stdcall),
    (
        0x001014,
        ddraw::IDirectDraw7::GetVerticalBlankStatus_stdcall,
    ),
    (0x001015, ddraw::IDirectDraw7::Initialize_stdcall),
    (0x001016, ddraw::IDirectDraw7::RestoreDisplayMode_stdcall),
    (0x001017, ddraw::IDirectDraw7::SetCooperativeLevel_stdcall),
    (0x001018, ddraw::IDirectDraw7::SetDisplayMode_stdcall),
    (0x001019, ddraw::IDirectDraw7::WaitForVerticalBlank_stdcall),
    (0x00101a, ddraw::IDirectDraw7::GetAvailableVidMem_stdcall),
    (0x00101b, ddraw::IDirectDraw7::GetSurfaceFromDC_stdcall),
    (0x00101c, ddraw::IDirectDraw7::RestoreAllSurfaces_stdcall),
    (0x00101d, ddraw::IDirectDraw7::TestCooperativeLevel_stdcall),
    (0x00101e, ddraw::IDirectDraw7::GetDeviceIdentifier_stdcall),
    (0x00101f, ddraw::IDirectDraw7::StartModeTest_stdcall),
    (0x001020, ddraw::IDirectDraw7::EvaluateMode_stdcall),
    (0x001021, ddraw::IDirectDrawSurface7::QueryInterface_stdcall),
    (0x001022, ddraw::IDirectDrawSurface7::AddRef_stdcall),
    (0x001023, ddraw::IDirectDrawSurface7::Release_stdcall),
    (
        0x001024,
        ddraw::IDirectDrawSurface7::AddAttachedSurface_stdcall,
    ),
    (
        0x001025,
        ddraw::IDirectDrawSurface7::AddOverlayDirtyRect_stdcall,
    ),
    (0x001026, ddraw::IDirectDrawSurface7::Blt_stdcall),
    (0x001027, ddraw::IDirectDrawSurface7::BltBatch_stdcall),
    (0x001028, ddraw::IDirectDrawSurface7::BltFast_stdcall),
    (
        0x001029,
        ddraw::IDirectDrawSurface7::DeleteAttachedSurface_stdcall,
    ),
    (
        0x00102a,
        ddraw::IDirectDrawSurface7::EnumAttachedSurfaces_stdcall,
    ),
    (
        0x00102b,
        ddraw::IDirectDrawSurface7::EnumOverlayZOrders_stdcall,
    ),
    (0x00102c, ddraw::IDirectDrawSurface7::Flip_stdcall),
    (
        0x00102d,
        ddraw::IDirectDrawSurface7::GetAttachedSurface_stdcall,
    ),
    (0x00102e, ddraw::IDirectDrawSurface7::GetBltStatus_stdcall),
    (0x00102f, ddraw::IDirectDrawSurface7::GetCaps_stdcall),
    (0x001030, ddraw::IDirectDrawSurface7::GetClipper_stdcall),
    (0x001031, ddraw::IDirectDrawSurface7::GetColorKey_stdcall),
    (0x001032, ddraw::IDirectDrawSurface7::GetDC_stdcall),
    (0x001033, ddraw::IDirectDrawSurface7::GetFlipStatus_stdcall),
    (
        0x001034,
        ddraw::IDirectDrawSurface7::GetOverlayPosition_stdcall,
    ),
    (0x001035, ddraw::IDirectDrawSurface7::GetPalette_stdcall),
    (0x001036, ddraw::IDirectDrawSurface7::GetPixelFormat_stdcall),
    (0x001037, ddraw::IDirectDrawSurface7::GetSurfaceDesc_stdcall),
    (0x001038, ddraw::IDirectDrawSurface7::Initialize_stdcall),
    (0x001039, ddraw::IDirectDrawSurface7::IsLost_stdcall),
    (0x00103a, ddraw::IDirectDrawSurface7::Lock_stdcall),
    (0x00103b, ddraw::IDirectDrawSurface7::ReleaseDC_stdcall),
    (0x00103c, ddraw::IDirectDrawSurface7::Restore_stdcall),
    (0x00103d, ddraw::IDirectDrawSurface7::SetClipper_stdcall),
    (0x00103e, ddraw::IDirectDrawSurface7::SetColorKey_stdcall),
    (
        0x00103f,
        ddraw::IDirectDrawSurface7::SetOverlayPosition_stdcall,
    ),
    (0x001040, ddraw::IDirectDrawSurface7::SetPalette_stdcall),
    (0x001041, ddraw::IDirectDrawSurface7::Unlock_stdcall),
    (0x001042, ddraw::IDirectDrawSurface7::UpdateOverlay_stdcall),
    (
        0x001043,
        ddraw::IDirectDrawSurface7::UpdateOverlayDisplay_stdcall,
    ),
    (
        0x001044,
        ddraw::IDirectDrawSurface7::UpdateOverlayZOrder_stdcall,
    ),
    (0x001045, ddraw::IDirectDrawSurface7::GetDDInterface_stdcall),
    (0x001046, ddraw::IDirectDrawSurface7::PageLock_stdcall),
    (0x001047, ddraw::IDirectDrawSurface7::PageUnlock_stdcall),
    (0x001048, ddraw::IDirectDrawSurface7::SetSurfaceDesc_stdcall),
    (0x001049, ddraw::IDirectDrawSurface7::SetPrivateData_stdcall),
    (0x00104a, ddraw::IDirectDrawSurface7::GetPrivateData_stdcall),
    (
        0x00104b,
        ddraw::IDirectDrawSurface7::FreePrivateData_stdcall,
    ),
    (
        0x00104c,
        ddraw::IDirectDrawSurface7::GetUniquenessValue_stdcall,
    ),
    (
        0x00104d,
        ddraw::IDirectDrawSurface7::ChangeUniquenessValue_stdcall,
    ),
    (0x00104e, ddraw::IDirectDrawSurface7::SetPriority_stdcall),
    (0x00104f, ddraw::IDirectDrawSurface7::GetPriority_stdcall),
    (0x001050, ddraw::IDirectDrawSurface7::SetLOD_stdcall),
    (0x001051, ddraw::IDirectDrawSurface7::GetLOD_stdcall),
    (0x401000, x00401000),
    (0x401008, x00401008),
    (0x40101a, x0040101a),
    (0xf000_0000, runtime::return_from_main),
];

pub const EXEDATA: EXEData = EXEData {
    image_base: 0x400000,
    resources: 0..0,
    blocks: &BLOCKS,
    init_mappings,
    entry_point: Cont(x00401000),
};
