use runtime::{Cont, Context, SegOfs};
use zerocopy::FromBytes;

// sbtalker API from https://github.com/systoolz/dosbtalk/blob/main/speech.c

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
struct DriverData {
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

impl DriverData {
    /// Call int 2f to ask the TSR for the driver entry point.
    #[allow(unused)]
    fn get_addr(ctx: &mut Context) -> SegOfs {
        ctx.cpu.regs.set_ax(0xfbfb);
        ctx.cpu.regs.set_cs(0);
        dos::int(ctx, 0x11, 0x2f);
        let addr = SegOfs::new(ctx.cpu.regs.es, ctx.cpu.regs.get_bx());
        addr
    }

    fn from_context(ctx: &mut Context) -> &mut DriverData {
        // let driver_addr = Self::get_addr(ctx);
        // println!("driver at {addr}");
        let driver_addr = SegOfs::new(0x823, 0x1ce);
        println!("driver at {}", driver_addr);
        <DriverData>::mut_from_prefix(&mut ctx.memory[driver_addr.abs()..])
            .unwrap()
            .0
    }
}

fn call(ctx: &mut Context, addr: SegOfs) {
    let orig_sp = ctx.cpu.regs.get_sp();
    ctx.push16(0); // cs
    ctx.push16(0x11); // ip

    println!("call driver {addr}");
    let mut f = ctx.jmpf16(addr.seg, addr.ofs);
    while ctx.cpu.regs.get_sp() != orig_sp {
        f = f.0(ctx);
    }
}

pub fn x11(ctx: &mut Context) -> Cont {
    let driver = DriverData::from_context(ctx);
    driver.buf1.set(b"hello there, i am dr. sbaitso");
    driver.buf2.set(b"test text");

    let entry_point = driver.entry_point;
    ctx.cpu.regs.set_al(/* parse */ 0);
    call(ctx, entry_point);

    let driver = DriverData::from_context(ctx);
    println!("buf1: {:?}", driver.buf1.to_str());
    println!("buf2: {:?}", driver.buf2.to_str());

    ctx.cpu.regs.set_al(/* say */ 7);
    call(ctx, entry_point);

    todo!()
}
