//! API exposed by SBTALKER.EXE TSR.

// sbtalker API from https://github.com/systoolz/dosbtalk/blob/main/speech.c

use runtime::{Context, SegOfs};
use zerocopy::FromBytes;

use crate::blaster;

/// Length-prefixed strings used in sbtalker API.
#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, Debug)]
struct StrBuf {
    len: u8,
    chars: [u8; 255],
}

impl StrBuf {
    fn set(&mut self, text: &[u8]) {
        assert!(text.len() <= self.chars.len());
        self.len = text.len() as u8;
        self.chars[..text.len()].copy_from_slice(text);
    }

    fn to_str(&self) -> &str {
        std::str::from_utf8(&self.chars[..self.len as usize]).unwrap()
    }
}

/// sbtalker header, as returned by the int 2f entry point.
#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::KnownLayout, zerocopy::IntoBytes, Debug)]
struct Header {
    sig: [u8; 2],
    version: u16,
    entry_point: SegOfs,
    pad: [u8; 24],
    buf1: StrBuf,
    buf2: StrBuf,
    gender: u16,
    tone: u16,
    volume: u16,
    pitch: u16,
    speed: u16,
    count: u16,
    action: u16,
}

impl Header {
    /// Call int 2f to ask the TSR for the driver entry point.
    #[allow(unused)]
    fn get_addr(ctx: &mut Context) -> SegOfs {
        ctx.cpu.regs.set_ax(0xfbfb);
        ctx.cpu.regs.set_cs(0);
        dos::int(ctx, 0x11, 0x2f);
        let addr = SegOfs::new(ctx.cpu.regs.es, ctx.cpu.regs.get_bx());
        addr
    }

    fn from_context(ctx: &mut Context) -> &mut Header {
        // let driver_addr = Self::get_addr(ctx);
        // println!("driver at {addr}");
        let driver_addr = SegOfs::new(0x823, 0x1ce);
        <Header>::mut_from_prefix(&mut ctx.memory[driver_addr.abs()..])
            .unwrap()
            .0
    }
}

pub struct SBTalker {
    entry_point: SegOfs,
}

impl SBTalker {
    pub fn new(ctx: &mut Context) -> Self {
        let entry_point = Header::from_context(ctx).entry_point;
        Self { entry_point }
    }

    fn call(&self, ctx: &mut Context, func: u8) {
        ctx.cpu.regs.set_al(func);
        let orig_sp = ctx.cpu.regs.get_sp();

        ctx.push16(0); // cs
        ctx.push16(0x11); // ip

        let f = ctx.jmpf16(self.entry_point.seg, self.entry_point.ofs);
        dos::run_loop(ctx, f, true, |ctx| ctx.cpu.regs.get_sp() != orig_sp);
    }

    pub fn parse(&self, ctx: &mut Context, text: &str) -> String {
        assert!(text.is_ascii());
        Header::from_context(ctx).buf1.set(text.as_bytes());
        self.call(ctx, /* parse */ 0);
        Header::from_context(ctx).buf2.to_str().to_owned()
    }

    pub fn speak(&self, ctx: &mut Context, text: &str) -> Vec<u8> {
        assert!(text.is_ascii());
        Header::from_context(ctx).buf1.set(text.as_bytes());
        self.call(ctx, /* say */ 7);
        std::mem::take(&mut blaster::AUDIO.lock().unwrap().buf)
    }
}
