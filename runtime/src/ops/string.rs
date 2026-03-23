use super::math::sub;
use crate::{Flags, Machine};

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

pub fn rep(m: &mut Machine, rep: Rep, func: impl Fn(&mut Machine)) {
    while m.cpu.regs.ecx > 0 {
        func(m);
        m.cpu.regs.ecx = m.cpu.regs.ecx.wrapping_sub(1);
        match rep {
            Rep::REPE if !m.cpu.flags.contains(Flags::ZF) => break,
            Rep::REPNE if m.cpu.flags.contains(Flags::ZF) => break,
            _ => {}
        }
    }
}

pub fn lodsb(m: &mut Machine) {
    let addr = m.cpu.regs.esi;
    m.cpu.regs.set_al(m.memory.read::<u8>(addr));
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.esi = addr.wrapping_sub(1);
    } else {
        m.cpu.regs.esi = addr.wrapping_add(1);
    }
}

pub fn lodsd(m: &mut Machine) {
    let addr = m.cpu.regs.esi;
    m.cpu.regs.eax = m.memory.read::<u32>(addr);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.esi = addr.wrapping_sub(4);
    } else {
        m.cpu.regs.esi = addr.wrapping_add(4);
    }
}

pub fn stosb(m: &mut Machine) {
    let addr = m.cpu.regs.edi;
    m.memory.write::<u8>(addr, m.cpu.regs.eax as u8);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.edi = addr.wrapping_sub(1);
    } else {
        m.cpu.regs.edi = addr.wrapping_add(1);
    }
}

pub fn stosd(m: &mut Machine) {
    let addr = m.cpu.regs.edi;
    m.memory.write::<u32>(addr, m.cpu.regs.eax);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.edi = addr.wrapping_sub(4);
    } else {
        m.cpu.regs.edi = addr.wrapping_add(4);
    }
}

pub fn scasb(m: &mut Machine) {
    let addr = m.cpu.regs.edi;
    let mem = m.memory.read::<u8>(addr);
    let _ = sub(m.cpu.regs.get_al(), mem, &mut m.cpu.flags);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.edi = addr.wrapping_sub(1);
    } else {
        m.cpu.regs.edi = addr.wrapping_add(1);
    }
}

pub fn cmpsb(m: &mut Machine) {
    let src = m.memory.read::<u8>(m.cpu.regs.esi);
    let dst = m.memory.read::<u8>(m.cpu.regs.edi);
    let _ = sub(src, dst, &mut m.cpu.flags);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_sub(1);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_sub(1);
    } else {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_add(1);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_add(1);
    }
}

pub fn movsd(m: &mut Machine) {
    let val = m.memory.read::<u32>(m.cpu.regs.esi);
    m.memory.write::<u32>(m.cpu.regs.edi, val);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_sub(4);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_sub(4);
    } else {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_add(4);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_add(4);
    }
}

pub fn movsb(m: &mut Machine) {
    let val = m.memory.read::<u8>(m.cpu.regs.esi);
    m.memory.write::<u8>(m.cpu.regs.edi, val);
    if m.cpu.flags.contains(Flags::DF) {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_sub(1);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_sub(1);
    } else {
        m.cpu.regs.esi = m.cpu.regs.esi.wrapping_add(1);
        m.cpu.regs.edi = m.cpu.regs.edi.wrapping_add(1);
    }
}
