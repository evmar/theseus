//! mmio*: the multimedia file I/O API, used to walk RIFF files (.wav), plus MCI.
//!
//! Files are read into memory whole on open and served from there, which keeps
//! seeking and the chunk walk trivial. Callers that read through MMIOINFO's
//! direct buffer instead of calling mmioRead get a pointer into a guest-side
//! copy of the same data.

use std::collections::HashMap;

use runtime::Context;

use crate::{heap::Heap, kernel32};

const MMSYSERR_NOERROR: u32 = 0;
const MMIOERR_CANNOTOPEN: u32 = 258;
const MMIOERR_CHUNKNOTFOUND: u32 = 261;

/// mmioRead/mmioSeek report failure as -1.
const MMIO_FAILURE: i32 = -1;

const MMIO_FINDCHUNK: u32 = 0x0010;
const MMIO_FINDRIFF: u32 = 0x0020;
const MMIO_FINDLIST: u32 = 0x0040;

const FOURCC_RIFF: u32 = u32::from_le_bytes(*b"RIFF");
const FOURCC_LIST: u32 = u32::from_le_bytes(*b"LIST");

/// MMCKINFO, the chunk descriptor mmioDescend fills in and mmioAscend reads.
#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
    zerocopy::Immutable,
    zerocopy::KnownLayout,
)]
struct MMCKINFO {
    ckid: u32,
    cksize: u32,
    fccType: u32,
    dwDataOffset: u32,
    dwFlags: u32,
}

/// Field offsets within MMIOINFO. Only the buffer-related fields matter to us,
/// so rather than model the whole 0x48-byte struct we poke the fields we own
/// and leave the rest of the caller's memory untouched.
mod mmioinfo {
    pub const DWFLAGS: u32 = 0x00;
    pub const CCHBUFFER: u32 = 0x14;
    pub const PCHBUFFER: u32 = 0x18;
    pub const PCHNEXT: u32 = 0x1c;
    pub const PCHENDREAD: u32 = 0x20;
    pub const PCHENDWRITE: u32 = 0x24;
    pub const LBUFOFFSET: u32 = 0x28;
    pub const LDISKOFFSET: u32 = 0x2c;
    pub const HMMIO: u32 = 0x44;
}

struct File {
    data: Vec<u8>,
    pos: usize,
    /// Guest-side copy of `data`, allocated on first direct-buffer access.
    buffer: u32,
}

impl File {
    fn read_u32(&self, at: usize) -> Option<u32> {
        let bytes = self.data.get(at..at + 4)?;
        Some(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

#[derive(Default)]
pub struct State {
    files: HashMap<u32, File>,
    next_handle: u32,
    /// Guest heap backing the direct-access buffers handed out by mmioGetInfo.
    heap: Option<Heap>,
}

impl State {
    /// The heap backing direct-access buffers, created on first use.
    fn heap(&mut self) -> &Heap {
        self.heap.get_or_insert_with(|| {
            const HEAP_SIZE: u32 = 16 << 20;
            let addr = kernel32::lock()
                .mappings
                .alloc("winmm mmio buffers".into(), HEAP_SIZE);
            Heap::new(addr, HEAP_SIZE)
        })
    }
}

/// Ensure a guest-side copy of the file exists; returns (buffer, pos, len).
fn ensure_buffer(ctx: &mut Context, hmmio: u32) -> Option<(u32, u32, u32)> {
    let mut winmm = super::state();
    let mmio = winmm.mmio();
    let file = mmio.files.get(&hmmio)?;
    let (buffer, pos, len) = (file.buffer, file.pos as u32, file.data.len() as u32);
    if buffer != 0 {
        return Some((buffer, pos, len));
    }

    let addr = mmio.heap().alloc(&mut ctx.memory, len.max(1));
    let file = mmio.files.get_mut(&hmmio)?;
    ctx.memory[addr..][..file.data.len()].copy_from_slice(&file.data);
    file.buffer = addr;
    Some((addr, pos, len))
}

#[win32_derive::dllexport]
pub fn mciSendCommandA(
    _ctx: &mut Context,
    _mciId: u32,
    _uMsg: u32,
    _dwParam1: u32,
    _dwParam2: u32,
) -> u32 {
    // CD audio etc.; pretend success and play nothing.
    crate::stub!(0)
}

#[win32_derive::dllexport]
pub fn mmioOpenA(ctx: &mut Context, szFilename: u32, _lpmmioinfo: u32, _dwOpenFlags: u32) -> u32 {
    if szFilename == 0 {
        // Opening a memory buffer rather than a file; no caller needs it.
        log::warn!("mmioOpenA: no filename");
        return 0;
    }
    let name = ctx.memory.read_str(szFilename).to_owned();
    let path = kernel32::resolve_path(&name);
    let data = match host::fs::read(&path) {
        Ok(data) => data,
        Err(err) => {
            log::warn!("mmioOpenA({name:?}): {err}");
            return 0; // a null HMMIO means the open failed
        }
    };

    let mut winmm = super::state();
    let mmio = winmm.mmio();
    // Handle 0 is reserved for failure.
    mmio.next_handle += 1;
    let handle = mmio.next_handle;
    mmio.files.insert(
        handle,
        File {
            data,
            pos: 0,
            buffer: 0,
        },
    );
    handle
}

#[win32_derive::dllexport]
pub fn mmioClose(ctx: &mut Context, hmmio: u32, _wFlags: u32) -> u32 {
    let mut winmm = super::state();
    let mmio = winmm.mmio();
    let Some(file) = mmio.files.remove(&hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };
    if file.buffer != 0 {
        mmio.heap().free(&mut ctx.memory, file.buffer);
    }
    MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn mmioRead(ctx: &mut Context, hmmio: u32, pch: u32, cch: u32) -> i32 {
    let mut winmm = super::state();
    let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
        return MMIO_FAILURE;
    };
    let end = (file.pos + cch as usize).min(file.data.len());
    let read = end - file.pos;
    ctx.memory[pch..][..read].copy_from_slice(&file.data[file.pos..end]);
    file.pos = end;
    read as i32
}

#[win32_derive::dllexport]
pub fn mmioSeek(_ctx: &mut Context, hmmio: u32, lOffset: i32, iOrigin: i32) -> i32 {
    let mut winmm = super::state();
    let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
        return MMIO_FAILURE;
    };
    let base = match iOrigin {
        0 => 0,                      // SEEK_SET
        1 => file.pos as i64,        // SEEK_CUR
        2 => file.data.len() as i64, // SEEK_END
        _ => return MMIO_FAILURE,
    };
    let pos = base + lOffset as i64;
    if pos < 0 || pos as usize > file.data.len() {
        return MMIO_FAILURE;
    }
    file.pos = pos as usize;
    file.pos as i32
}

#[win32_derive::dllexport]
pub fn mmioDescend(ctx: &mut Context, hmmio: u32, lpck: u32, lpckParent: u32, wFlags: u32) -> u32 {
    let mut want = ctx.memory.read::<MMCKINFO>(lpck);
    // A parent chunk bounds the search to its contents.
    let parent_end = if lpckParent != 0 {
        let parent = ctx.memory.read::<MMCKINFO>(lpckParent);
        parent.dwDataOffset.saturating_add(parent.cksize) as usize
    } else {
        usize::MAX
    };

    let mut winmm = super::state();
    let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };

    loop {
        let header = file.pos;
        if header + 8 > file.data.len() || header + 8 > parent_end {
            return MMIOERR_CHUNKNOTFOUND;
        }
        let ckid = file.read_u32(header).unwrap();
        let cksize = file.read_u32(header + 4).unwrap();
        // RIFF and LIST chunks start with a form type, and the API reports
        // dwDataOffset pointing at it.
        let body = header + 8;
        let container = ckid == FOURCC_RIFF || ckid == FOURCC_LIST;
        let fcc_type = if container {
            file.read_u32(body).unwrap_or(0)
        } else {
            0
        };

        let matched = if wFlags & MMIO_FINDRIFF != 0 {
            ckid == FOURCC_RIFF && fcc_type == want.fccType
        } else if wFlags & MMIO_FINDLIST != 0 {
            ckid == FOURCC_LIST && fcc_type == want.fccType
        } else if wFlags & MMIO_FINDCHUNK != 0 {
            ckid == want.ckid
        } else {
            true // just describe whatever chunk is here
        };

        if matched {
            want.ckid = ckid;
            want.cksize = cksize;
            want.fccType = fcc_type;
            want.dwDataOffset = body as u32;
            // Descending leaves the file at the chunk's readable contents,
            // which for a container is past the form type.
            file.pos = if container { body + 4 } else { body };
            ctx.memory.write(lpck, want);
            return MMSYSERR_NOERROR;
        }

        // On to the next sibling; chunks are word-aligned.
        let mut next = body + cksize as usize;
        next += next % 2;
        if next <= header || next >= file.data.len() || next >= parent_end {
            return MMIOERR_CHUNKNOTFOUND;
        }
        file.pos = next;
    }
}

#[win32_derive::dllexport]
pub fn mmioAscend(ctx: &mut Context, hmmio: u32, lpck: u32, _wFlags: u32) -> u32 {
    let chunk = ctx.memory.read::<MMCKINFO>(lpck);
    let mut end = chunk.dwDataOffset.saturating_add(chunk.cksize) as usize;
    end += end % 2;
    let mut winmm = super::state();
    let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };
    file.pos = end.min(file.data.len());
    MMSYSERR_NOERROR
}

/// Expose the file's contents as a buffer the caller can read from directly.
#[win32_derive::dllexport]
pub fn mmioGetInfo(ctx: &mut Context, hmmio: u32, lpmmioinfo: u32, _wFlags: u32) -> u32 {
    let Some((buffer, pos, len)) = ensure_buffer(ctx, hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };
    // MMIO_DIRTY is for writing, which we don't support; report a plain
    // readable buffer covering the whole file.
    ctx.memory.write::<u32>(lpmmioinfo + mmioinfo::DWFLAGS, 0);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::CCHBUFFER, len);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHBUFFER, buffer);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHNEXT, buffer + pos);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHENDREAD, buffer + len);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHENDWRITE, buffer + len);
    // The buffer covers the file from its start, so buffer offsets and file
    // offsets coincide.
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::LBUFOFFSET, 0);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::LDISKOFFSET, len);
    ctx.memory.write::<u32>(lpmmioinfo + mmioinfo::HMMIO, hmmio);
    MMSYSERR_NOERROR
}

/// Take back the file position the caller advanced through pchNext.
#[win32_derive::dllexport]
pub fn mmioSetInfo(ctx: &mut Context, hmmio: u32, lpmmioinfo: u32, _wFlags: u32) -> u32 {
    let next = ctx.memory.read::<u32>(lpmmioinfo + mmioinfo::PCHNEXT);
    let buffer = ctx.memory.read::<u32>(lpmmioinfo + mmioinfo::PCHBUFFER);
    let mut winmm = super::state();
    let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };
    file.pos = (next.saturating_sub(buffer) as usize).min(file.data.len());
    MMSYSERR_NOERROR
}

/// Refill the caller's buffer window. The whole file is already resident, so
/// this only syncs the position; pchNext == pchEndRead then signals EOF.
#[win32_derive::dllexport]
pub fn mmioAdvance(ctx: &mut Context, hmmio: u32, lpmmioinfo: u32, _wFlags: u32) -> u32 {
    let Some((buffer, _, len)) = ensure_buffer(ctx, hmmio) else {
        return MMIOERR_CANNOTOPEN;
    };
    let next = ctx.memory.read::<u32>(lpmmioinfo + mmioinfo::PCHNEXT);
    let pos = next.saturating_sub(buffer).min(len);
    {
        let mut winmm = super::state();
        let Some(file) = winmm.mmio().files.get_mut(&hmmio) else {
            return MMIOERR_CANNOTOPEN;
        };
        file.pos = pos as usize;
    }

    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHNEXT, buffer + pos);
    ctx.memory
        .write::<u32>(lpmmioinfo + mmioinfo::PCHENDREAD, buffer + len);
    MMSYSERR_NOERROR
}
