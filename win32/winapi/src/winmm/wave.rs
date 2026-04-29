use std::collections::VecDeque;

use runtime::Context;
use zerocopy::FromBytes;

use crate::{FromABIParam, dllexport::win32flags, kernel32, stub, user32, winmm::state};

const MMSYSERR_NOERROR: u32 = 0;

pub struct State {
    sender: std::sync::mpsc::Sender<u32>,
}

// TODO: sdl has wacky safety requirements; the audio objects are Send
// but the subsystem itself is not because it must be freed on main thread.
struct StreamHack(sdl3::audio::AudioStreamOwner);
unsafe impl Send for StreamHack {}

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_ctx: &mut Context) -> u32 {
    1
}

#[repr(C)]
#[derive(Debug, zerocopy::Immutable, zerocopy::IntoBytes)]
pub struct WAVEOUTCAPS {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    // TODO: TCHAR, could this be unicode based on cbwoc param?
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}

enum WAVE_FORMAT {
    _4M16 = 0x0000_0400,
}

#[win32_derive::dllexport]
pub fn waveOutGetDevCapsA(ctx: &mut Context, _uDeviceID: u32, pwoc: u32, cbwoc: u32) -> u32 {
    assert_eq!(cbwoc, std::mem::size_of::<WAVEOUTCAPS>() as u32);

    ctx.memory.write(
        pwoc,
        WAVEOUTCAPS {
            wMid: 0,
            wPid: 0,
            vDriverVersion: 1,
            szPname: [0; 32],
            dwFormats: WAVE_FORMAT::_4M16 as u32,
            wChannels: 1, // mono
            wReserved1: 0,
            dwSupport: 0, // no features
        },
    );
    MMSYSERR_NOERROR
}

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}

/// The types of callbacks that can be used with waveOutOpen.
#[derive(Debug, PartialEq, Eq, win32_derive::ABIEnum)]
pub enum CALLBACK {
    NULL = 0x00000000,
    WINDOW = 0x00010000,
    TASK = 0x00020000,
    FUNCTION = 0x00030000,
    EVENT = 0x00050000,
}

enum MM_WOM {
    // OPEN = 0x3BB,
    // CLOSE = 0x3BC,
    DONE = 0x3BD,
}

/// Thread procedure to pass data from wave APIs to SDL.
/// We use a win32 thread here because the callbacks to the executable
/// that indicate more data is needed come from a thread.
///
/// Strategy: waveOutWrite pushes the pointer to the buffer to the queue,
/// and we pull it out here and pass it to SDL.
fn thread_proc(
    ctx: &mut Context,
    stream: StreamHack,
    receiver: std::sync::mpsc::Receiver<u32>,
    callback: u32,
    callback_data: u32,
) {
    let stream = stream.0;

    // We need to notify when a queued block is done, but SDL doesn't make that easy.
    // We keep track of how much data we have passed to SDL, then compare it against
    // how much data SDL says is yet to be processed, and use the difference to decide
    // whether a given block has been fully processed.
    // total_pending is the total number of bytes that have been submitted to SDL,
    // and is the sum of the lengths of all queued blocks.
    let mut total_pending = 0u32;
    struct QueuedBlock {
        addr: u32,
        len: u32,
    }
    let mut queued_blocks: VecDeque<QueuedBlock> = VecDeque::new();

    let f = runtime::indirect(ctx, callback);
    loop {
        while stream.queued_bytes().unwrap() < 8 << 10 {
            let addr = receiver.recv().unwrap();
            let header = <WAVEHDR>::ref_from_prefix(&ctx.memory[addr..]).unwrap().0;
            let buf = &ctx.memory[header.lpData..][..header.dwBufferLength as usize];
            stream.put_data(buf).unwrap();
            total_pending += header.dwBufferLength;
            queued_blocks.push_back(QueuedBlock {
                addr,
                len: header.dwBufferLength,
            });
        }

        if let Some(block) = queued_blocks.front() {
            let queued_bytes = stream.queued_bytes().unwrap() as u32;
            let consumed = total_pending - queued_bytes;
            if consumed >= block.len {
                let QueuedBlock { addr, len } = *block;
                total_pending -= len;
                queued_blocks.pop_front();

                let header = <WAVEHDR>::mut_from_prefix(&mut ctx.memory[addr..])
                    .unwrap()
                    .0;
                header.dwFlags.insert(WHDR::DONE);

                let hwo = 1u32; // XXX
                let uMsg = MM_WOM::DONE as u32;
                // waveOutProc, WOM_DONE message
                runtime::call_x86(ctx, f, vec![hwo, uMsg, callback_data, addr, 0]);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

#[win32_derive::dllexport]
pub fn waveOutOpen(
    ctx: &mut Context,
    phwo: u32,
    _uDeviceID: u32,
    pwfx: u32,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: u32,
) -> u32 {
    if fdwOpen & !0x000F_0000 != 0 {
        todo!("{fdwOpen:x?}");
    }

    let fmt = <WAVEFORMATEX>::read_from_prefix(&ctx.memory[pwfx..])
        .unwrap()
        .0;

    let audio = user32::state().sdl.audio().unwrap();
    let device = audio.default_playback_device();
    let stream = device
        .open_device_stream(Some(&sdl3::audio::AudioSpec {
            freq: Some(fmt.nSamplesPerSec as i32),
            channels: Some(fmt.nChannels as i32),
            format: Some(if fmt.wBitsPerSample == 16 {
                sdl3::audio::AudioFormat::S16LE
            } else {
                todo!()
            }),
        }))
        .unwrap();
    stream.resume().unwrap();
    let stream_hack = StreamHack(stream);

    let (sender, receiver) = std::sync::mpsc::channel::<u32>();
    state().wave = Some(State { sender });

    let callback = CALLBACK::from_abi(fdwOpen);
    match callback {
        CALLBACK::NULL => {}
        CALLBACK::FUNCTION => {
            kernel32::lock().create_thread(ctx, format!("winmm thread"), move |ctx| {
                thread_proc(ctx, stream_hack, receiver, dwCallback, dwInstance)
            });
        }
        _ => todo!("{callback:?}"),
    }

    ctx.memory.write::<u32>(phwo, 1);

    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutReset(_ctx: &mut Context, _hwo: u32) -> u32 {
    stub!(MMSYSERR_NOERROR)
}

#[win32_derive::dllexport]
pub fn waveOutClose(_ctx: &mut Context, _hwo: u32) -> u32 {
    stub!(MMSYSERR_NOERROR)
}

#[repr(C)]
#[derive(
    Debug, zerocopy::FromBytes, zerocopy::Immutable, zerocopy::KnownLayout, zerocopy::IntoBytes,
)]
pub struct WAVEHDR {
    lpData: u32,
    dwBufferLength: u32,
    dwBytesRecorded: u32,
    dwUser: u32,
    dwFlags: WHDR,
    dwLoops: u32,
    lpNext: u32,
    reserved: u32,
}

win32flags! {
    pub struct WHDR {
        const DONE      = 0x00000001;
        const PREPARED  = 0x00000002;
        const BEGINLOOP = 0x00000004;
        const ENDLOOP   = 0x00000008;
        const INQUEUE   = 0x00000010;
    }
}

#[win32_derive::dllexport]
pub fn waveOutPrepareHeader(ctx: &mut Context, _hwo: u32, pwh: u32, cbwh: u32) -> u32 {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    let header = <WAVEHDR>::mut_from_prefix(&mut ctx.memory[pwh..])
        .unwrap()
        .0;
    header.dwFlags.remove(WHDR::DONE);
    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutUnprepareHeader(_ctx: &mut Context, _hwo: u32, _pwh: u32, _cbwh: u32) -> u32 {
    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutWrite(_ctx: &mut Context, _hwo: u32, pwh: u32, cbwh: u32) -> u32 {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    let mut state = state();
    let state = state.wave.as_mut().unwrap();
    state.sender.send(pwh).unwrap();
    MMSYSERR_NOERROR
}
