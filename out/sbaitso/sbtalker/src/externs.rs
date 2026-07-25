use runtime::{Cont, Context, segofs};
use zerocopy::FromBytes;

// sbtalker API from https://github.com/systoolz/dosbtalk/blob/main/speech.c

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
}

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::KnownLayout, zerocopy::IntoBytes, Debug)]
struct DriverData {
    sig: [u8; 2],
    version: u16,
    entry_ofs: u16,
    entry_seg: u16,
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

impl DriverData {
    /// Call int 2f to ask the TSR for the driver entry point.
    #[allow(unused)]
    fn get_addr(ctx: &mut Context) {
        ctx.cpu.regs.set_ax(0xfbfb);
        ctx.cpu.regs.set_cs(0);
        dos::int(ctx, 0x11, 0x2f);
        let seg = ctx.cpu.regs.es;
        let ofs = ctx.cpu.regs.get_bx();
        println!("driver at {seg:x}:{ofs:x}");
    }

    fn from_context(ctx: &mut Context) -> &mut DriverData {
        let driver_addr = segofs(0x823, 0x1ce);
        <DriverData>::mut_from_prefix(&mut ctx.memory[driver_addr..])
            .unwrap()
            .0
    }
}

fn call(ctx: &mut Context, seg: u16, ofs: u16) {
    let orig_sp = ctx.cpu.regs.get_sp();
    ctx.push16(0); // cs
    ctx.push16(0x11); // ip

    let mut f = ctx.jmpf16(seg, ofs);
    while ctx.cpu.regs.get_sp() != orig_sp {
        eprintln!("loop esp={:x}", ctx.cpu.regs.get_sp());
        // TODO: interrupts are disabled when calling interrupts, but also it appears
        // the dosbox interrupt handlers immediately reenable interrupts when they are invoked.
        // if i % 0x2000 == 0 {
        //     state().check_interrupts(ctx);
        // }
        f = f.0(ctx);
    }
}

pub fn x11(ctx: &mut Context) -> Cont {
    let driver = DriverData::from_context(ctx);
    println!("got header {driver:x?}");
    driver.buf1.set(b"hello there\0");

    let (seg, ofs) = (driver.entry_seg, driver.entry_ofs);
    println!("call {seg:x}:{ofs:x}");

    // TODO: fails due to not supporting far jmp  0823:0b37 jmp dword ptr cs:[7Ah]
    ctx.cpu.regs.set_al(5);
    call(ctx, seg, ofs);

    todo!()
}
