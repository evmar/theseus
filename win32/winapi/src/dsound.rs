//! DirectSound, backed by a software mixer.
//!
//! Apps expect many independent sound buffers playing at once, each with its
//! own format, rate, volume and pan, while the host gives us a single output
//! stream. So buffers here are just PCM in guest memory plus a playback
//! cursor, and [`pump`] mixes whatever is playing into the host stream.
//!
//! Nothing drives the mixer on its own: `pump` is called from the places the
//! app passes through anyway — the message pump, ddraw's presentation, and the
//! DirectSound calls that report playback progress. That last one matters:
//! a game that waits for a sound to finish by polling GetStatus never pumps
//! messages meanwhile, and would wait forever on playback that only advanced
//! when it moved on.

use std::{collections::HashMap, sync::Mutex};

use runtime::{Context, Memory};
use zerocopy::FromBytes;

use crate::{dllexport::win32flags, heap::Heap, kernel32, locked_state::LockedState};

/// Set THESEUS_DSOUND_WAV to a path to dump the mixed output there, for
/// checking what the mixer actually produced.
fn wav_debug_path() -> Option<String> {
    std::env::var("THESEUS_DSOUND_WAV").ok()
}

/// Host mix rate, in stereo i16.
const HOST_RATE: u32 = 44100;
/// Stereo frames produced per mixer chunk (~23ms).
const CHUNK_FRAMES: usize = 1024;
/// How far ahead of playback we keep the host queue, in stereo frames (~140ms).
/// Enough slack that playback never runs dry between pumps.
const TARGET_QUEUE_FRAMES: u32 = 6144;
/// Safety cap on chunks produced in a single pump, in case a buffer is
/// misconfigured and the queue never fills.
const MAX_CHUNKS_PER_PUMP: usize = 12;
/// Bytes per stereo i16 frame.
const HOST_FRAME_BYTES: u32 = 4;

const fn make_dserror(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

const DS_OK: u32 = 0;
#[allow(dead_code)]
const DSERR_NODRIVER: u32 = make_dserror(120);
const DSERR_INVALIDPARAM: u32 = 0x80070057;

/// DSBVOLUME/DSBPAN are in hundredths of a decibel of attenuation, with 0 as
/// full volume and -10000 as silence.
const DSBVOLUME_MIN: i32 = -10000;

/// Convert an attenuation in hundredths of a decibel to an amplitude factor.
fn gain(hundredths_db: i32) -> f32 {
    if hundredths_db <= DSBVOLUME_MIN {
        return 0.0;
    }
    10f32.powf(hundredths_db as f32 / 2000.0)
}

#[derive(Clone, Copy)]
struct WaveFormat {
    channels: u32,
    bits: u32,
    rate: u32,
}

impl Default for WaveFormat {
    fn default() -> Self {
        WaveFormat {
            channels: 2,
            bits: 16,
            rate: 44100,
        }
    }
}

impl WaveFormat {
    fn from_wave(fmt: &WAVEFORMATEX) -> Self {
        WaveFormat {
            channels: fmt.nChannels.max(1) as u32,
            bits: if fmt.wBitsPerSample == 0 {
                8
            } else {
                fmt.wBitsPerSample as u32
            },
            rate: fmt.nSamplesPerSec.max(1),
        }
    }

    fn frame_bytes(&self) -> u32 {
        self.channels * (self.bits / 8).max(1)
    }
}

struct Buffer {
    /// COM reference count, so a balanced AddRef/Release doesn't free a buffer
    /// the app is still playing.
    refs: u32,
    /// PCM data in guest memory. Zero for the primary buffer, which holds no
    /// samples of its own.
    addr: u32,
    size: u32,
    format: WaveFormat,
    primary: bool,
    caps_flags: DSBCAPS,
    playing: bool,
    looping: bool,
    /// Playback position in source frames; fractional so resampling works.
    cursor: f64,
    volume: i32,
    pan: i32,
}

impl Buffer {
    fn frame_count(&self) -> f64 {
        (self.size / self.format.frame_bytes()) as f64
    }

    /// Per-channel amplitude, combining volume and pan.
    fn gains(&self) -> (f32, f32) {
        let volume = gain(self.volume);
        // Panning attenuates the channel you're panning away from.
        let left = if self.pan > 0 { gain(-self.pan) } else { 1.0 };
        let right = if self.pan < 0 { gain(self.pan) } else { 1.0 };
        (volume * left, volume * right)
    }

    /// The sample pair of one whole frame, in i16 scale.
    fn frame_at(&self, mem: &Memory, index: u32) -> (i32, i32) {
        let base = self.addr + index * self.format.frame_bytes();
        let stereo = self.format.channels >= 2;
        if self.format.bits >= 16 {
            let left = mem.read::<i16>(base) as i32;
            let right = if stereo {
                mem.read::<i16>(base + 2) as i32
            } else {
                left
            };
            (left, right)
        } else {
            // 8-bit PCM is unsigned, centered on 128.
            let left = (mem[base] as i32 - 128) << 8;
            let right = if stereo {
                (mem[base + 1] as i32 - 128) << 8
            } else {
                left
            };
            (left, right)
        }
    }

    /// The sample pair at the current cursor, in i16 scale, interpolated
    /// between the frames it falls between.
    ///
    /// These sounds are recorded at 11 or 22 kHz and play out at 44.1, so most
    /// output samples land between two source frames. Repeating the nearer one
    /// instead — the obvious thing — turns each source sample into a little
    /// staircase, and those steps are audible as a gritty edge on every sound.
    fn sample(&self, mem: &Memory) -> (f32, f32) {
        let index = self.cursor.floor();
        let frac = (self.cursor - index) as f32;
        let current = self.frame_at(mem, index as u32);
        let next_index = index + 1.0;
        let next = if next_index < self.frame_count() {
            self.frame_at(mem, next_index as u32)
        } else if self.looping {
            // The frame after the last is the first one again.
            self.frame_at(mem, 0)
        } else {
            // A one-shot ends here; hold rather than interpolate into
            // whatever memory follows the buffer.
            current
        };
        let blend = |a: i32, b: i32| a as f32 + (b - a) as f32 * frac;
        (blend(current.0, next.0), blend(current.1, next.1))
    }
}

struct State {
    buffers: HashMap<u32, Buffer>,
    stream: Option<host::AudioStream>,
    /// Heap holding the buffers' PCM data.
    heap: Option<Heap>,
    write: Option<WavWrite>,
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
            stream: None,
            heap: None,
            write: wav_debug_path().map(|path| WavWrite::new(&path)),
        });
    }
}

impl State {
    fn heap(&mut self) -> &Heap {
        self.heap.get_or_insert_with(|| {
            // Games allocate a buffer per sound instance and are lax about
            // releasing them, so leave plenty of room.
            const HEAP_SIZE: u32 = 64 << 20;
            let addr = kernel32::lock()
                .mappings
                .alloc("dsound buffers".into(), HEAP_SIZE);
            Heap::new(addr, HEAP_SIZE)
        })
    }

    fn stream(&mut self) -> &host::AudioStream {
        self.stream.get_or_insert_with(|| {
            let stream = host::host().create_audio_stream(host::AudioSpec {
                sample_rate: HOST_RATE,
                channels: 2,
            });
            stream.resume();
            stream
        })
    }

    /// Mix one chunk of every playing buffer and hand it to the host.
    fn mix_chunk(&mut self, mem: &Memory) {
        let mut mixed = [(0f32, 0f32); CHUNK_FRAMES];
        for buffer in self.buffers.values_mut() {
            if !buffer.playing || buffer.primary || buffer.size == 0 {
                continue;
            }
            let frames = buffer.frame_count();
            let step = buffer.format.rate as f64 / HOST_RATE as f64;
            let (left_gain, right_gain) = buffer.gains();
            for slot in mixed.iter_mut() {
                if buffer.cursor >= frames {
                    if !buffer.looping {
                        // Reaching the end stops playback and rewinds, so
                        // playing the same sound again starts it over rather
                        // than replaying the silence past its end.
                        buffer.playing = false;
                        buffer.cursor = 0.0;
                        break;
                    }
                    buffer.cursor %= frames;
                }
                let (left, right) = buffer.sample(mem);
                slot.0 += left * left_gain;
                slot.1 += right * right_gain;
                buffer.cursor += step;
            }
        }

        let mut bytes = Vec::with_capacity(CHUNK_FRAMES * HOST_FRAME_BYTES as usize);
        for (left, right) in mixed {
            for sample in [left, right] {
                let sample = sample.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                bytes.extend_from_slice(&sample.to_le_bytes());
            }
        }
        if let Some(write) = self.write.as_mut() {
            write.write(&bytes);
        }
        self.stream().put_data(&bytes);
    }
}

/// Top the host's output queue up to [`TARGET_QUEUE_FRAMES`].
///
/// Cheap and safe to call as often as you like: with a full queue it does
/// nothing, and with no sound buffers it returns immediately.
pub fn pump(ctx: &mut Context) {
    let Ok(state) = STATE.lock() else { return };
    let idle = match state.as_ref() {
        Some(state) => state.buffers.is_empty(),
        None => true,
    };
    drop(state);
    if idle {
        return;
    }

    let mut state = lock();
    if !state.stream().is_open() {
        // No audio device: mixing would just burn cpu, since the queue below
        // never fills.
        return;
    }
    for _ in 0..MAX_CHUNKS_PER_PUMP {
        if state.stream().queued_bytes() >= TARGET_QUEUE_FRAMES * HOST_FRAME_BYTES {
            break;
        }
        state.mix_chunk(&ctx.memory);
    }
}

#[win32_derive::dllexport]
pub fn DirectSoundCreate(ctx: &mut Context, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    assert_eq!(lpGuid, 0);
    assert_eq!(pUnkOuter, 0);

    init();

    let mut kernel32 = kernel32::lock();
    let addr = IDirectSound::new(ctx, &mut kernel32.process_heap);
    drop(kernel32);
    ctx.memory.write(ppDS, addr);
    DS_OK
}

#[win32_derive::dllexport]
pub fn ordinal1(ctx: &mut Context, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    DirectSoundCreate(ctx, lpGuid, ppDS, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectSoundEnumerateA(_ctx: &mut Context, _lpCallback: u32, _lpContext: u32) -> u32 {
    // Report no devices to enumerate; apps that care use the default device,
    // which DirectSoundCreate always provides.
    DS_OK
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
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> u32 {
        DSERR_INVALIDPARAM
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        0
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

        let mut kernel32 = kernel32::lock();
        let addr = IDirectSoundBuffer::new(ctx, &mut kernel32.process_heap);
        drop(kernel32);

        let primary = desc.dwFlags.contains(DSBCAPS::PRIMARYBUFFER);
        let format = if desc.lpwfxFormat != 0 {
            let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[desc.lpwfxFormat..])
                .unwrap()
                .0;
            const WAVE_FORMAT_PCM: u16 = 1;
            if fmt.wFormatTag != WAVE_FORMAT_PCM {
                log::warn!("dsound: non-PCM format {}", fmt.wFormatTag);
            }
            WaveFormat::from_wave(&fmt)
        } else {
            WaveFormat::default()
        };

        let mut state = lock();
        // The primary buffer describes the output mix rather than holding
        // samples, so it gets no memory of its own.
        let (data, size) = if primary {
            (0, 0)
        } else {
            let size = desc.dwBufferBytes;
            (state.heap().alloc(&mut ctx.memory, size), size)
        };
        state.buffers.insert(
            addr,
            Buffer {
                refs: 1,
                addr: data,
                size,
                format,
                primary,
                caps_flags: desc.dwFlags,
                playing: false,
                looping: false,
                cursor: 0.0,
                volume: 0,
                pan: 0,
            },
        );
        drop(state);

        ctx.memory.write(lplpDirectSoundBuffer, addr);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(ctx: &mut Context, _this: u32, lpDSCaps: u32) -> u32 {
        if lpDSCaps == 0 {
            return DSERR_INVALIDPARAM;
        }
        // DSCAPS: dwSize, dwFlags, then a long run of counters. Report a
        // software-only device with generous limits and zero everything else.
        const DSCAPS_PRIMARYSTEREO: u32 = 0x0000_0002;
        const DSCAPS_PRIMARY16BIT: u32 = 0x0000_0008;
        const DSCAPS_CONTINUOUSRATE: u32 = 0x0000_0010;
        let size = ctx.memory.read::<u32>(lpDSCaps);
        ctx.memory[lpDSCaps + 4..][..(size as usize).saturating_sub(4)].fill(0);
        ctx.memory.write::<u32>(
            lpDSCaps + 4,
            DSCAPS_PRIMARYSTEREO | DSCAPS_PRIMARY16BIT | DSCAPS_CONTINUOUSRATE,
        );
        ctx.memory.write::<u32>(lpDSCaps + 8, 100); // dwMinSecondarySampleRate
        ctx.memory.write::<u32>(lpDSCaps + 12, 100000); // dwMaxSecondarySampleRate
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn DuplicateSoundBuffer(
        ctx: &mut Context,
        _this: u32,
        pDSBufferOriginal: u32,
        ppDSBufferDuplicate: u32,
    ) -> u32 {
        let mut kernel32 = kernel32::lock();
        let addr = IDirectSoundBuffer::new(ctx, &mut kernel32.process_heap);
        drop(kernel32);

        let mut state = lock();
        let Some(original) = state.buffers.get(&pDSBufferOriginal) else {
            return DSERR_INVALIDPARAM;
        };
        // A duplicate shares the original's samples but plays independently.
        let duplicate = Buffer {
            refs: 1,
            addr: original.addr,
            size: original.size,
            format: original.format,
            primary: original.primary,
            caps_flags: original.caps_flags,
            playing: false,
            looping: false,
            cursor: 0.0,
            volume: original.volume,
            pan: original.pan,
        };
        state.buffers.insert(addr, duplicate);
        drop(state);

        ctx.memory.write(ppDSBufferDuplicate, addr);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_ctx: &mut Context, _this: u32, _hwnd: u32, _dwLevel: u32) -> u32 {
        // We always have exclusive use of the host's audio output.
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Compact(_ctx: &mut Context, _this: u32) -> u32 {
        DS_OK // nothing to defragment
    }

    #[win32_derive::dllexport]
    pub fn GetSpeakerConfig(ctx: &mut Context, _this: u32, lpdwSpeakerConfig: u32) -> u32 {
        const DSSPEAKER_STEREO: u32 = 2;
        if lpdwSpeakerConfig != 0 {
            ctx.memory.write::<u32>(lpdwSpeakerConfig, DSSPEAKER_STEREO);
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetSpeakerConfig(_ctx: &mut Context, _this: u32, _dwSpeakerConfig: u32) -> u32 {
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32, _lpGuid: u32) -> u32 {
        DS_OK
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
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> u32 {
        DSERR_INVALIDPARAM
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, this: u32) -> u32 {
        let mut state = lock();
        match state.buffers.get_mut(&this) {
            Some(buffer) => {
                buffer.refs += 1;
                buffer.refs
            }
            None => 0,
        }
    }

    #[win32_derive::dllexport]
    pub fn Release(ctx: &mut Context, this: u32) -> u32 {
        let mut state = lock();
        if let Some(buffer) = state.buffers.get_mut(&this) {
            buffer.refs = buffer.refs.saturating_sub(1);
            if buffer.refs > 0 {
                return buffer.refs;
            }
        }
        if let Some(buffer) = state.buffers.remove(&this) {
            // Duplicates share their original's samples, so only free memory
            // that no surviving buffer still points at.
            let shared = state
                .buffers
                .values()
                .any(|other| other.addr == buffer.addr);
            if buffer.addr != 0 && !shared {
                state.heap().free(&mut ctx.memory, buffer.addr);
            }
        }
        0
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(ctx: &mut Context, this: u32, lpDSBufferCaps: u32) -> u32 {
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        // DSBCAPS: dwSize, dwFlags, dwBufferBytes, dwUnlockTransferRate,
        // dwPlayCpuOverhead.
        let (flags, size) = (buffer.caps_flags.bits(), buffer.size);
        drop(state);
        ctx.memory.write::<u32>(lpDSBufferCaps + 4, flags);
        ctx.memory.write::<u32>(lpDSBufferCaps + 8, size);
        ctx.memory.write::<u32>(lpDSBufferCaps + 12, 0);
        ctx.memory.write::<u32>(lpDSBufferCaps + 16, 0);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetCurrentPosition(
        ctx: &mut Context,
        this: u32,
        pdwCurrentPlayCursor: u32,
        pdwCurrentWriteCursor: u32,
    ) -> u32 {
        // Asking where playback is has to move it along first, or an app that
        // polls in a tight loop would never see it advance.
        pump(ctx);
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let frame_bytes = buffer.format.frame_bytes();
        let size = buffer.size;
        let play = ((buffer.cursor as u32).saturating_mul(frame_bytes)).min(size.saturating_sub(1));
        drop(state);

        if pdwCurrentPlayCursor != 0 {
            ctx.memory.write(pdwCurrentPlayCursor, play);
        }
        if pdwCurrentWriteCursor != 0 {
            // Real hardware keeps the write cursor a little ahead of playback,
            // wrapping like the play cursor does.
            let write = if size > 0 {
                (play + frame_bytes) % size
            } else {
                0
            };
            ctx.memory.write(pdwCurrentWriteCursor, write);
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetFormat(
        ctx: &mut Context,
        this: u32,
        lpwfxFormat: u32,
        dwSizeAllocated: u32,
        lpdwSizeWritten: u32,
    ) -> u32 {
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let format = buffer.format;
        drop(state);

        let size = std::mem::size_of::<WAVEFORMATEX>() as u32;
        if lpwfxFormat != 0 {
            if dwSizeAllocated < size {
                return DSERR_INVALIDPARAM;
            }
            let block_align = format.frame_bytes() as u16;
            ctx.memory.write::<u16>(lpwfxFormat, 1); // WAVE_FORMAT_PCM
            ctx.memory
                .write::<u16>(lpwfxFormat + 2, format.channels as u16);
            ctx.memory.write::<u32>(lpwfxFormat + 4, format.rate);
            ctx.memory
                .write::<u32>(lpwfxFormat + 8, format.rate * block_align as u32);
            ctx.memory.write::<u16>(lpwfxFormat + 12, block_align);
            ctx.memory
                .write::<u16>(lpwfxFormat + 14, format.bits as u16);
            ctx.memory.write::<u16>(lpwfxFormat + 16, 0); // cbSize
        }
        if lpdwSizeWritten != 0 {
            ctx.memory.write::<u32>(lpdwSizeWritten, size);
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetVolume(ctx: &mut Context, this: u32, lplVolume: u32) -> u32 {
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let volume = buffer.volume;
        drop(state);
        ctx.memory.write::<i32>(lplVolume, volume);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetPan(ctx: &mut Context, this: u32, lplPan: u32) -> u32 {
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let pan = buffer.pan;
        drop(state);
        ctx.memory.write::<i32>(lplPan, pan);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetFrequency(ctx: &mut Context, this: u32, lpdwFrequency: u32) -> u32 {
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let rate = buffer.format.rate;
        drop(state);
        ctx.memory.write::<u32>(lpdwFrequency, rate);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetStatus(ctx: &mut Context, this: u32, lpdwStatus: u32) -> u32 {
        const DSBSTATUS_PLAYING: u32 = 0x0001;
        const DSBSTATUS_LOOPING: u32 = 0x0004;
        // Games wait for a sound to finish by polling this, without pumping
        // messages meanwhile. Mixing here is what lets that wait end: otherwise
        // the app waits for playback that only advances when the app moves on.
        pump(ctx);
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let mut status = 0;
        if buffer.playing {
            status |= DSBSTATUS_PLAYING;
            if buffer.looping {
                status |= DSBSTATUS_LOOPING;
            }
        }
        drop(state);
        ctx.memory.write::<u32>(lpdwStatus, status);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Initialize(
        _ctx: &mut Context,
        _this: u32,
        _lpDirectSound: u32,
        _lpcDSBufferDesc: u32,
    ) -> u32 {
        DS_OK
    }

    /// Hand out a pointer straight into the buffer's memory. The app writes
    /// samples there and the mixer reads them from the same place, so Unlock
    /// has nothing to copy back.
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
        let state = lock();
        let Some(buffer) = state.buffers.get(&this) else {
            return DSERR_INVALIDPARAM;
        };

        let (offset, len) = if dwFlags.contains(DSBLOCK::ENTIREBUFFER) {
            (0, buffer.size)
        } else {
            let offset = dwOffset.min(buffer.size);
            (offset, dwBytes.min(buffer.size - offset))
        };
        // Some callers rely on getting null back for an empty region.
        let addr = if len == 0 { 0 } else { buffer.addr + offset };
        drop(state);

        ctx.memory.write(ppvAudioPtr1, addr);
        ctx.memory.write(pdwAudioBytes1, len);
        // We never split a locked region, so the second one is always empty.
        if ppvAudioPtr2 != 0 {
            ctx.memory.write(ppvAudioPtr2, 0u32);
        }
        if pdwAudioBytes2 != 0 {
            ctx.memory.write(pdwAudioBytes2, 0u32);
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(
        _ctx: &mut Context,
        _this: u32,
        _pvAudioPtr1: u32,
        _dwAudioBytes1: u32,
        _pvAudioPtr2: u32,
        _dwAudioBytes2: u32,
    ) -> u32 {
        // Lock exposed the buffer's real memory, so the app's writes are
        // already visible to the mixer.
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Play(
        _ctx: &mut Context,
        this: u32,
        _dwReserved1: u32,
        _dwPriority: u32,
        dwFlags: u32,
    ) -> u32 {
        const DSBPLAY_LOOPING: u32 = 0x0001;
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        // Play resumes from wherever the cursor was left, which is the start
        // for a fresh buffer and where Stop left off otherwise. A buffer
        // stopped exactly at its end has nothing left to resume, so start it
        // over instead of playing nothing.
        buffer.playing = true;
        buffer.looping = dwFlags & DSBPLAY_LOOPING != 0;
        if buffer.cursor >= buffer.frame_count() {
            buffer.cursor = 0.0;
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Stop(_ctx: &mut Context, this: u32) -> u32 {
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        // Stop leaves the cursor alone; a later Play resumes from here.
        buffer.playing = false;
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCurrentPosition(_ctx: &mut Context, this: u32, dwNewPosition: u32) -> u32 {
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        let frames = buffer.frame_count();
        let cursor = (dwNewPosition / buffer.format.frame_bytes()) as f64;
        // Clamped: a position past the end would sample outside the buffer.
        buffer.cursor = if frames > 0.0 { cursor % frames } else { 0.0 };
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetFormat(ctx: &mut Context, this: u32, pcfxFormat: u32) -> u32 {
        let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[pcfxFormat..])
            .unwrap()
            .0;
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        // Only the primary buffer's format is settable, and since we always mix
        // to the host's own format, recording it is all we need to do.
        buffer.format = WaveFormat::from_wave(&fmt);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetVolume(_ctx: &mut Context, this: u32, lVolume: i32) -> u32 {
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        buffer.volume = lVolume.clamp(DSBVOLUME_MIN, 0);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetPan(_ctx: &mut Context, this: u32, lPan: i32) -> u32 {
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        buffer.pan = lPan.clamp(DSBVOLUME_MIN, -DSBVOLUME_MIN);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetFrequency(_ctx: &mut Context, this: u32, dwFrequency: u32) -> u32 {
        let mut state = lock();
        let Some(buffer) = state.buffers.get_mut(&this) else {
            return DSERR_INVALIDPARAM;
        };
        // Zero means "back to the buffer's original rate", which for us is the
        // rate it was created with; we only ever change it from here.
        if dwFrequency != 0 {
            buffer.format.rate = dwFrequency;
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Restore(_ctx: &mut Context, _this: u32) -> u32 {
        // Buffers live in our own memory and are never lost.
        DS_OK
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

/// Debug helper: dumps the mixed output to a .wav.
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
                sample_rate: HOST_RATE,
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
