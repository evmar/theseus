use std::{collections::HashMap, sync::Mutex};

use runtime::Context;
use zerocopy::FromBytes;

use crate::{dllexport::win32flags, heap::Heap, kernel32, locked_state::LockedState, stub, user32};

const fn make_dhsresult(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

#[allow(dead_code)]
const DSERR_NODRIVER: u32 = make_dhsresult(120);

const DS_OK: u32 = 0;

struct AudioStream(sdl3::audio::AudioStreamOwner);
unsafe impl Send for AudioStream {}

struct WavWrite {
    f: std::fs::File,
}

impl WavWrite {
    fn new(path: &str) -> Self {
        let f = std::fs::File::create(path).unwrap();
        let mut w = Self { f };
        w.write_header().unwrap();
        w
    }

    fn write_header(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        use zerocopy::IntoBytes;

        #[repr(C)]
        #[derive(zerocopy::IntoBytes, zerocopy::Immutable)]
        struct Chunk {
            id: [u8; 4],
            chunk_size: u32,
        }

        #[repr(C)]
        #[derive(zerocopy::IntoBytes, zerocopy::Immutable, Default)]
        struct Fmt {
            format: u16,
            channels: u16,
            sample_rate: u32,
            byte_per_sec: u32,
            byte_per_block: u16,
            bits_per_sample: u16,
        }

        #[repr(C)]
        #[derive(zerocopy::IntoBytes, zerocopy::Immutable)]
        struct Header {
            file_header: Chunk,
            format: [u8; 4],
            fmt_header: Chunk,
            fmt: Fmt,
            data_header: Chunk,
        }

        let mut header = Header {
            file_header: Chunk {
                id: *b"RIFF",
                chunk_size: 0xffff_ffff,
            },
            format: *b"WAVE",
            fmt_header: Chunk {
                id: *b"fmt ",
                chunk_size: std::mem::size_of::<Fmt>() as u32,
            },
            fmt: Fmt {
                format: 1,
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 16,
                ..Default::default()
            },
            data_header: Chunk {
                id: *b"data",
                chunk_size: 0xffff_ffff,
            },
        };

        let fmt = &mut header.fmt;
        fmt.byte_per_block = fmt.channels * fmt.bits_per_sample / 8;
        fmt.byte_per_sec = fmt.sample_rate * fmt.byte_per_block as u32;

        self.f.write_all(header.as_bytes())?;

        Ok(())
    }

    fn write(&mut self, data: &[u8]) {
        use std::io::Write;
        self.f.write_all(data).unwrap();
    }
}

struct Buffer {
    addr: u32,
    size: u32,
    total_written: u32,
    stream: AudioStream,
    write: WavWrite,
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

    pub const VTABLE_ENTRIES: [&'static str; 11] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "CreateSoundBuffer",
        "GetCaps",
        "DuplicateSoundBuffer",
        "SetCooperativeLevel",
        "Compact",
        "GetSpeakerConfig",
        "SetSpeakerConfig",
        "Initialize",
    ];

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
        // log::info!("new buffer flags {:#x?}", desc.dwFlags);

        let mut kernel32 = kernel32::lock();
        let addr = IDirectSoundBuffer::new(ctx, &mut kernel32.process_heap);

        if !desc.dwFlags.contains(DSBCAPS::PRIMARYBUFFER) {
            let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[desc.lpwfxFormat..])
                .unwrap()
                .0;
            const WAVE_FORMAT_PCM: u16 = 1;
            assert_eq!(fmt.wFormatTag, WAVE_FORMAT_PCM);

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

            let write = WavWrite::new("out.wav");

            let buffer = Buffer {
                addr: kernel32
                    .process_heap
                    .alloc(&mut ctx.memory, desc.dwBufferBytes),
                size: desc.dwBufferBytes,
                total_written: 0,
                stream: AudioStream(stream),
                write,
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

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }
}

pub mod IDirectSoundBuffer {
    use super::*;

    pub const VTABLE_ENTRIES: [&'static str; 21] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "GetCaps",
        "GetCurrentPosition",
        "GetFormat",
        "GetVolume",
        "GetPan",
        "GetFrequency",
        "GetStatus",
        "Initialize",
        "Lock",
        "Play",
        "SetCurrentPosition",
        "SetFormat",
        "SetVolume",
        "SetPan",
        "SetFrequency",
        "Stop",
        "Unlock",
        "Restore",
    ];

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
        this: u32,
        pdwCurrentPlayCursor: u32,
        pdwCurrentWriteCursor: u32,
    ) -> u32 {
        let mut lock = lock();
        let buffer = lock.buffers.get_mut(&this).unwrap();
        if pdwCurrentPlayCursor != 0 {
            let unplayed = buffer.stream.0.queued_bytes().unwrap() as u32;
            let play_cursor = (buffer.total_written - unplayed) % buffer.size;
            ctx.memory.write(pdwCurrentPlayCursor, play_cursor);
        }
        if pdwCurrentWriteCursor != 0 {
            let write_cursor = buffer.total_written % buffer.size;
            ctx.memory.write(pdwCurrentWriteCursor, write_cursor);
        }
        DS_OK
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

        let len = if dwFlags.contains(DSBLOCK::ENTIREBUFFER) {
            assert_eq!(dwBytes, 0);
            buffer.size
        } else {
            dwBytes.min(buffer.size - dwOffset)
        };

        let addr = if len == 0 {
            // it appears chillin relies on getting null back in this case
            0
        } else {
            buffer.addr + dwOffset
        };
        ctx.memory.write(ppvAudioPtr1, addr);
        ctx.memory.write(pdwAudioBytes1, len);
        if ppvAudioPtr2 != 0 {
            ctx.memory.write(ppvAudioPtr2, 0u32);
            ctx.memory.write(pdwAudioBytes2, 0u32);
        }

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
        let _fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[pcfxFormat..])
            .unwrap()
            .0;
        // TODO: verify format is as expected, possibly change format?
        // log::warn!("fmt {:#x?}", fmt);
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
        dwAudioBytes1: u32,
        pvAudioPtr2: u32,
        dwAudioBytes2: u32,
    ) -> u32 {
        let mut state = lock();
        let buffer = state.buffers.get_mut(&this).unwrap();
        assert!(pvAudioPtr2 == 0);
        assert!(dwAudioBytes2 == 0);

        if dwAudioBytes1 == 0 {
            return DS_OK;
        }

        let data = &ctx.memory[buffer.addr..][..dwAudioBytes1 as usize];
        buffer.stream.0.put_data(data).unwrap();
        buffer.write.write(data);
        buffer.total_written += data.len() as u32;

        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Restore(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    pub static mut VTABLE: u32 = 0;
    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
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

pub const VTABLES: [(&'static str, &[&str]); 2] = [
    ("IDirectSound", IDirectSound::VTABLE_ENTRIES.as_slice()),
    (
        "IDirectSoundBuffer",
        IDirectSoundBuffer::VTABLE_ENTRIES.as_slice(),
    ),
];
