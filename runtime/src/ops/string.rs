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
    while m.regs.ecx > 0 {
        func(m);
        m.regs.ecx = m.regs.ecx.wrapping_sub(1);
        match rep {
            Rep::REPE if !m.flags.contains(Flags::ZF) => break,
            Rep::REPNE if m.flags.contains(Flags::ZF) => break,
            _ => {}
        }
    }
}

pub fn lodsb(m: &mut Machine) {
    let addr = m.regs.esi;
    m.regs.set_al(m.memory.read::<u8>(addr));
    if m.flags.contains(Flags::DF) {
        m.regs.esi = addr.wrapping_sub(1);
    } else {
        m.regs.esi = addr.wrapping_add(1);
    }
}

pub fn lodsd(m: &mut Machine) {
    let addr = m.regs.esi;
    m.regs.eax = m.memory.read::<u32>(addr);
    if m.flags.contains(Flags::DF) {
        m.regs.esi = addr.wrapping_sub(4);
    } else {
        m.regs.esi = addr.wrapping_add(4);
    }
}

pub fn stosb(m: &mut Machine) {
    let addr = m.regs.edi;
    m.memory.write::<u8>(addr, m.regs.eax as u8);
    if m.flags.contains(Flags::DF) {
        m.regs.edi = addr.wrapping_sub(1);
    } else {
        m.regs.edi = addr.wrapping_add(1);
    }
}

pub fn stosd(m: &mut Machine) {
    let addr = m.regs.edi;
    m.memory.write::<u32>(addr, m.regs.eax);
    if m.flags.contains(Flags::DF) {
        m.regs.edi = addr.wrapping_sub(4);
    } else {
        m.regs.edi = addr.wrapping_add(4);
    }
}

pub fn scasb(m: &mut Machine) {
    let addr = m.regs.edi;
    let mem = m.memory.read::<u8>(addr);
    let _ = sub(m.regs.get_al(), mem);
    if m.flags.contains(Flags::DF) {
        m.regs.edi = addr.wrapping_sub(1);
    } else {
        m.regs.edi = addr.wrapping_add(1);
    }
}

pub fn cmpsb(m: &mut Machine) {
    let src = m.memory.read::<u8>(m.regs.esi);
    let dst = m.memory.read::<u8>(m.regs.edi);
    let _ = sub(src, dst);
    if m.flags.contains(Flags::DF) {
        m.regs.esi = m.regs.esi.wrapping_sub(1);
        m.regs.edi = m.regs.edi.wrapping_sub(1);
    } else {
        m.regs.esi = m.regs.esi.wrapping_add(1);
        m.regs.edi = m.regs.edi.wrapping_add(1);
    }
}

pub fn movsd(m: &mut Machine) {
    let val = m.memory.read::<u32>(m.regs.esi);
    m.memory.write::<u32>(m.regs.edi, val);
    if m.flags.contains(Flags::DF) {
        m.regs.esi = m.regs.esi.wrapping_sub(4);
        m.regs.edi = m.regs.edi.wrapping_sub(4);
    } else {
        m.regs.esi = m.regs.esi.wrapping_add(4);
        m.regs.edi = m.regs.edi.wrapping_add(4);
    }
}

pub fn movsb(m: &mut Machine) {
    let val = m.memory.read::<u8>(m.regs.esi);
    m.memory.write::<u8>(m.regs.edi, val);
    if m.flags.contains(Flags::DF) {
        m.regs.esi = m.regs.esi.wrapping_sub(1);
        m.regs.edi = m.regs.edi.wrapping_sub(1);
    } else {
        m.regs.esi = m.regs.esi.wrapping_add(1);
        m.regs.edi = m.regs.edi.wrapping_add(1);
    }
}
