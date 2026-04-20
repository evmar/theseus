use crate::dllexport::win32flags;
use crate::heap::Heap;
use crate::kernel32;
use crate::stub;
use runtime::Context;
use zerocopy::FromBytes;
use zerocopy::IntoBytes;

const fn make_dhsresult(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

#[allow(dead_code)]
const DSERR_NODRIVER: u32 = make_dhsresult(120);

#[win32_derive::dllexport]
pub fn DirectSoundCreate(ctx: &mut Context, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    assert_eq!(lpGuid, 0);
    assert_eq!(pUnkOuter, 0);

    let mut kernel32 = kernel32::lock();
    let addr = IDirectSound::new(ctx, &mut kernel32.process_heap);
    ctx.memory.write(ppDS, addr);
    0
}

#[win32_derive::dllexport]
pub fn ordinal1(ctx: &mut Context, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    DirectSoundCreate(ctx, lpGuid, ppDS, pUnkOuter)
}

pub mod IDirectSound {
    use super::*;

    #[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
    #[repr(C)]
    pub struct VTable {
        QueryInterface: u32,
        AddRef: u32,
        Release: u32,
        CreateSoundBuffer: u32,
        GetCaps: u32,
        DuplicateSoundBuffer: u32,
        SetCooperativeLevel: u32,
        Compact: u32,
        GetSpeakerConfig: u32,
        SetSpeakerConfig: u32,
        Initialize: u32,
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn CreateSoundBuffer(
        ctx: &mut Context,
        _this: u32,
        lpcDSBufferDesc: u32,
        _lplpDirectSoundBuffer: u32,
        pUnkOuter: u32,
    ) -> u32 {
        assert_eq!(pUnkOuter, 0);
        let desc = <DSBUFFERDESC>::read_from_prefix(&ctx.memory[lpcDSBufferDesc..])
            .unwrap()
            .0;
        log::warn!("desc {:#x?}", desc);

        if !desc.dwFlags.contains(DSBCAPS::PRIMARYBUFFER) {
            let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[desc.lpwfxFormat..])
                .unwrap()
                .0;
            log::warn!("fmt {:#x?}", fmt);
        }

        todo!();
        stub!(0)
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSoundBuffer(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_ctx: &mut Context, _this: u32, _dwLevel: u32) -> u32 {
        stub!(0)
    }

    #[win32_derive::dllexport]
    pub fn Compact(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetSpeakerConfig(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetSpeakerConfig(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    fn vtable(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let vtable_addr = heap.alloc(&mut ctx.memory, std::mem::size_of::<VTable>() as u32);
        let func_addr = runtime::proc_addr(ctx, QueryInterface_stdcall);
        let vtable = VTable {
            QueryInterface: func_addr + 0,
            AddRef: func_addr + 1,
            Release: func_addr + 2,
            CreateSoundBuffer: func_addr + 3,
            GetCaps: func_addr + 4,
            DuplicateSoundBuffer: func_addr + 5,
            SetCooperativeLevel: func_addr + 6,
            Compact: func_addr + 7,
            GetSpeakerConfig: func_addr + 8,
            SetSpeakerConfig: func_addr + 9,
            Initialize: func_addr + 10,
        };
        vtable
            .write_to_prefix(&mut ctx.memory[vtable_addr..])
            .unwrap();
        vtable_addr
    }

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        let vtable = vtable(ctx, heap);
        ctx.memory.write(addr, vtable);
        addr
    }
}

win32flags! {
    pub struct DSBCAPS {
        const PRIMARYBUFFER       = 0x00000001;
        const STATIC              = 0x00000002;
        const LOCHARDWARE         = 0x00000004;
        const LOCSOFTWARE         = 0x00000008;
        const CTRL3D              = 0x00000010;
        const CTRLFREQUENCY       = 0x00000020;
        const CTRLPAN             = 0x00000040;
        const CTRLVOLUME          = 0x00000080;
        const CTRLPOSITIONNOTIFY  = 0x00000100;
        const CTRLFX              = 0x00000200;
        const STICKYFOCUS         = 0x00004000;
        const GLOBALFOCUS         = 0x00008000;
        const GETCURRENTPOSITION2 = 0x00010000;
        const MUTE3DATMAXDISTANCE = 0x00020000;
        const LOCDEFER            = 0x00040000;
    }
}

win32flags! {
    pub struct DSBLOCK {
        const FROMWRITECURSOR = 0x00000001;
        const ENTIREBUFFER    = 0x00000002;
    }
}

#[derive(
    Debug, zerocopy::FromBytes, zerocopy::Immutable, zerocopy::KnownLayout, zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: DSBCAPS,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: u32,
    // pub guid3DAlgorithm: GUID,
}

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}

pub const EXPORTS: [&'static str; 11] = [
    // IDirectSound
    "IDirectSound::QueryInterface",
    "IDirectSound::AddRef",
    "IDirectSound::Release",
    "IDirectSound::CreateSoundBuffer",
    "IDirectSound::GetCaps",
    "IDirectSound::DuplicateSoundBuffer",
    "IDirectSound::SetCooperativeLevel",
    "IDirectSound::Compact",
    "IDirectSound::GetSpeakerConfig",
    "IDirectSound::SetSpeakerConfig",
    "IDirectSound::Initialize",
];
