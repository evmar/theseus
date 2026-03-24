use super::math::sub;
use crate::{Context, Flags};

/// Width of an operation, e.g. movsb/w/d.
#[derive(Clone, Copy)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Dword = 4,
}

#[derive(Debug)]
pub enum Rep {
    REP,
    REPNE,
    REPE,
}

pub fn rep(ctx: &mut Context, rep: Rep, func: impl Fn(&mut Context)) {
    while ctx.cpu.regs.ecx > 0 {
        func(ctx);
        ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);
        match rep {
            Rep::REPE if !ctx.cpu.flags.contains(Flags::ZF) => break,
            Rep::REPNE if ctx.cpu.flags.contains(Flags::ZF) => break,
            _ => {}
        }
    }
}

pub fn lodsb(ctx: &mut Context) {
    let addr = ctx.cpu.regs.esi;
    ctx.cpu.regs.set_al(ctx.memory.read::<u8>(addr));
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.esi = addr.wrapping_sub(1);
    } else {
        ctx.cpu.regs.esi = addr.wrapping_add(1);
    }
}

pub fn lodsd(ctx: &mut Context) {
    let addr = ctx.cpu.regs.esi;
    ctx.cpu.regs.eax = ctx.memory.read::<u32>(addr);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.esi = addr.wrapping_sub(4);
    } else {
        ctx.cpu.regs.esi = addr.wrapping_add(4);
    }
}

pub fn stosb(ctx: &mut Context) {
    let addr = ctx.cpu.regs.edi;
    ctx.memory.write::<u8>(addr, ctx.cpu.regs.eax as u8);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.edi = addr.wrapping_sub(1);
    } else {
        ctx.cpu.regs.edi = addr.wrapping_add(1);
    }
}

pub fn stosd(ctx: &mut Context) {
    let addr = ctx.cpu.regs.edi;
    ctx.memory.write::<u32>(addr, ctx.cpu.regs.eax);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.edi = addr.wrapping_sub(4);
    } else {
        ctx.cpu.regs.edi = addr.wrapping_add(4);
    }
}

pub fn scasb(ctx: &mut Context) {
    let addr = ctx.cpu.regs.edi;
    let mem = ctx.memory.read::<u8>(addr);
    let _ = sub(ctx.cpu.regs.get_al(), mem, &mut ctx.cpu.flags);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.edi = addr.wrapping_sub(1);
    } else {
        ctx.cpu.regs.edi = addr.wrapping_add(1);
    }
}

pub fn cmpsb(ctx: &mut Context) {
    let src = ctx.memory.read::<u8>(ctx.cpu.regs.esi);
    let dst = ctx.memory.read::<u8>(ctx.cpu.regs.edi);
    let _ = sub(src, dst, &mut ctx.cpu.flags);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_sub(1);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_sub(1);
    } else {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_add(1);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add(1);
    }
}

pub fn movsd(ctx: &mut Context) {
    let val = ctx.memory.read::<u32>(ctx.cpu.regs.esi);
    ctx.memory.write::<u32>(ctx.cpu.regs.edi, val);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_sub(4);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_sub(4);
    } else {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_add(4);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add(4);
    }
}

pub fn movsb(ctx: &mut Context) {
    let val = ctx.memory.read::<u8>(ctx.cpu.regs.esi);
    ctx.memory.write::<u8>(ctx.cpu.regs.edi, val);
    if ctx.cpu.flags.contains(Flags::DF) {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_sub(1);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_sub(1);
    } else {
        ctx.cpu.regs.esi = ctx.cpu.regs.esi.wrapping_add(1);
        ctx.cpu.regs.edi = ctx.cpu.regs.edi.wrapping_add(1);
    }
}
