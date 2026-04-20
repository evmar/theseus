use std::collections::HashMap;
use std::sync::Mutex;

use crate::dllexport::win32flags;
use crate::heap::Heap;
use crate::kernel32;
use crate::locked_state::LockedState;
use crate::stub;
use crate::user32;
use runtime::Context;
use zerocopy::FromBytes;
use zerocopy::IntoBytes;

const fn make_dhsresult(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

#[allow(dead_code)]
const DSERR_NODRIVER: u32 = make_dhsresult(120);

const DS_OK: u32 = 0;

struct AudioStream(sdl3::audio::AudioStreamOwner);
unsafe impl Send for AudioStream {}

struct Buffer {
    addr: u32,
    size: u32,
    stream: AudioStream,
    lock: Option<BufferLock>,
}
struct BufferLock {
    // TODO: track locked portion, match in unlock
}

// We need a sdl3::AudioSubsystem to make audio calls.
// The underlying SDL3 audio APIs are thread-safe and don't depend on any audio system pointer.
// But sdl3::AudioSubsystem is not send because it must be shut down on the main thread.
// So we hackily init it on the main thread (currently in user32), then init it a second time here
// (getting refcount=2) so this ref will never be shut down.
// TODO: we should move all of this kind of management to a Host abstraction.
struct SDLHack(sdl3::AudioSubsystem);
unsafe impl Send for SDLHack {}

struct State {
    buffers: HashMap<u32, Buffer>,
    audio: SDLHack,
}
static STATE: Mutex<Option<State>> = Mutex::new(None);
type Lock = LockedState<State>;
fn lock() -> Lock {
    LockedState::from(&STATE)
}

fn init() {
    let mut state = STATE.lock().unwrap();
    if state.is_none() {
        *state = Some(State {
            buffers: HashMap::default(),
            audio: SDLHack(user32::state().sdl.audio().unwrap()),
        });
    }
}

#[win32_derive::dllexport]
pub fn DirectSoundCreate(ctx: &mut Context, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    assert_eq!(lpGuid, 0);
    assert_eq!(pUnkOuter, 0);

    init();

    let mut kernel32 = kernel32::lock();
    let addr = IDirectSound::new(ctx, &mut kernel32.process_heap);
    ctx.memory.write(ppDS, addr);
    DS_OK
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
        lplpDirectSoundBuffer: u32,
        pUnkOuter: u32,
    ) -> u32 {
        assert_eq!(pUnkOuter, 0);
        let desc = <DSBUFFERDESC>::read_from_prefix(&ctx.memory[lpcDSBufferDesc..])
            .unwrap()
            .0;
        assert_eq!(desc.dwSize, std::mem::size_of::<DSBUFFERDESC>() as u32);
        log::info!("new buffer flags {:#x?}", desc.dwFlags);

        let mut kernel32 = kernel32::lock();
        let addr = IDirectSoundBuffer::new(ctx, &mut kernel32.process_heap);

        if !desc.dwFlags.contains(DSBCAPS::PRIMARYBUFFER) {
            let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[desc.lpwfxFormat..])
                .unwrap()
                .0;
            log::info!("new buffer fmt {:#x?}", fmt);

            let mut lock = lock();
            let stream = lock
                .audio
                .0
                .default_playback_device()
                .open_device_stream(Some(&sdl3::audio::AudioSpec {
                    freq: Some(fmt.nSamplesPerSec as i32),
                    channels: Some(fmt.nChannels as i32),
                    format: Some(match fmt.wBitsPerSample {
                        16 => sdl3::audio::AudioFormat::S16LE,
                        _ => todo!(),
                    }),
                }))
                .unwrap();

            let buffer = Buffer {
                addr: kernel32
                    .process_heap
                    .alloc(&mut ctx.memory, desc.dwBufferBytes),
                size: desc.dwBufferBytes,
                stream: AudioStream(stream),
                lock: None,
            };

            lock.buffers.insert(addr, buffer);
        }

        ctx.memory.write(lplpDirectSoundBuffer, addr);
        DS_OK
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
    pub fn SetCooperativeLevel(_ctx: &mut Context, _this: u32, _hwnd: u32, _dwLevel: u32) -> u32 {
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

pub mod IDirectSoundBuffer {
    use super::*;

    #[derive(Default, zerocopy::IntoBytes, zerocopy::Immutable)]
    #[repr(C)]
    pub struct VTable {
        QueryInterface: u32,
        AddRef: u32,
        Release: u32,
        GetCaps: u32,
        GetCurrentPosition: u32,
        GetFormat: u32,
        GetVolume: u32,
        GetPan: u32,
        GetFrequency: u32,
        GetStatus: u32,
        Initialize: u32,
        Lock: u32,
        Play: u32,
        SetCurrentPosition: u32,
        SetFormat: u32,
        SetVolume: u32,
        SetPan: u32,
        SetFrequency: u32,
        Stop: u32,
        Unlock: u32,
        Restore: u32,
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
    pub fn GetCaps(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetCurrentPosition(
        ctx: &mut Context,
        _this: u32,
        pdwCurrentPlayCursor: u32,
        pdwCurrentWriteCursor: u32,
    ) -> u32 {
        if pdwCurrentPlayCursor != 0 {
            ctx.memory.write(pdwCurrentPlayCursor, 0);
        }
        if pdwCurrentWriteCursor != 0 {
            ctx.memory.write(pdwCurrentWriteCursor, 0);
        }
        stub!(DS_OK)
    }

    #[win32_derive::dllexport]
    pub fn GetFormat(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetVolume(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetPan(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetFrequency(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetStatus(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        ctx: &mut Context,
        this: u32,
        dwOffset: u32,
        dwBytes: u32,
        ppvAudioPtr1: u32,
        pdwAudioBytes1: u32,
        ppvAudioPtr2: u32,
        pdwAudioBytes2: u32,
        dwFlags: DSBLOCK,
    ) -> u32 {
        let mut lock = lock();
        let buffer = lock.buffers.get_mut(&this).unwrap();
        assert!(buffer.lock.is_none());

        let len = if dwFlags.contains(DSBLOCK::ENTIREBUFFER) {
            assert_eq!(dwOffset, 0);
            assert_eq!(dwBytes, 0);
            buffer.size
        } else {
            dwBytes
        };
        ctx.memory.write(ppvAudioPtr1, buffer.addr);
        ctx.memory.write(pdwAudioBytes1, len);
        if ppvAudioPtr2 != 0 {
            ctx.memory.write(ppvAudioPtr2, 0);
            ctx.memory.write(pdwAudioBytes2, 0);
        }

        buffer.lock = Some(BufferLock {});
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Play(
        _ctx: &mut Context,
        this: u32,
        _dwReserved1: u32,
        _dwPriority: u32,
        _dwFlags: u32,
    ) -> u32 {
        let mut lock = lock();
        let buffer = lock.buffers.get_mut(&this).unwrap();
        buffer.stream.0.resume().unwrap();
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCurrentPosition(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetFormat(ctx: &mut Context, _this: u32, pcfxFormat: u32) -> u32 {
        let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[pcfxFormat..])
            .unwrap()
            .0;
        log::warn!("fmt {:#x?}", fmt);
        stub!(DS_OK)
    }

    #[win32_derive::dllexport]
    pub fn SetVolume(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetPan(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetFrequency(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Stop(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Unlock(
        ctx: &mut Context,
        this: u32,
        _pvAudioPtr1: u32,
        _dwAudioBytes1: u32,
        _pvAudioPtr2: u32,
        _dwAudioBytes2: u32,
    ) -> u32 {
        let mut lock = lock();
        let buffer = lock.buffers.get_mut(&this).unwrap();
        assert!(buffer.lock.is_some());
        buffer.lock = None;

        let data = &ctx.memory[buffer.addr..][..buffer.size as usize];
        log::info!("writing data {:x?}", &data[..100]);
        buffer.stream.0.put_data(data).unwrap();

        stub!(DS_OK)
    }

    #[win32_derive::dllexport]
    pub fn Restore(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    fn vtable(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let vtable_addr = heap.alloc(&mut ctx.memory, std::mem::size_of::<VTable>() as u32);
        let func_addr = runtime::proc_addr(ctx, QueryInterface_stdcall);
        let vtable = VTable {
            QueryInterface: func_addr + 0,
            AddRef: func_addr + 1,
            Release: func_addr + 2,
            GetCaps: func_addr + 3,
            GetCurrentPosition: func_addr + 4,
            GetFormat: func_addr + 5,
            GetVolume: func_addr + 6,
            GetPan: func_addr + 7,
            GetFrequency: func_addr + 8,
            GetStatus: func_addr + 9,
            Initialize: func_addr + 10,
            Lock: func_addr + 11,
            Play: func_addr + 12,
            SetCurrentPosition: func_addr + 13,
            SetFormat: func_addr + 14,
            SetVolume: func_addr + 15,
            SetPan: func_addr + 16,
            SetFrequency: func_addr + 17,
            Stop: func_addr + 18,
            Unlock: func_addr + 19,
            Restore: func_addr + 20,
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

pub const EXPORTS: [&'static str; 32] = [
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
    // IDirectSoundBuffer
    "IDirectSoundBuffer::QueryInterface",
    "IDirectSoundBuffer::AddRef",
    "IDirectSoundBuffer::Release",
    "IDirectSoundBuffer::GetCaps",
    "IDirectSoundBuffer::GetCurrentPosition",
    "IDirectSoundBuffer::GetFormat",
    "IDirectSoundBuffer::GetVolume",
    "IDirectSoundBuffer::GetPan",
    "IDirectSoundBuffer::GetFrequency",
    "IDirectSoundBuffer::GetStatus",
    "IDirectSoundBuffer::Initialize",
    "IDirectSoundBuffer::Lock",
    "IDirectSoundBuffer::Play",
    "IDirectSoundBuffer::SetCurrentPosition",
    "IDirectSoundBuffer::SetFormat",
    "IDirectSoundBuffer::SetVolume",
    "IDirectSoundBuffer::SetPan",
    "IDirectSoundBuffer::SetFrequency",
    "IDirectSoundBuffer::Stop",
    "IDirectSoundBuffer::Unlock",
    "IDirectSoundBuffer::Restore",
];
