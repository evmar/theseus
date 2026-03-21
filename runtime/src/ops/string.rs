use super::math::sub;
use crate::{Flags, MACHINE};

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

pub fn rep(rep: Rep, func: impl Fn()) {
    unsafe {
        while MACHINE.regs.ecx > 0 {
            func();
            MACHINE.regs.ecx = MACHINE.regs.ecx.wrapping_sub(1);
            match rep {
                Rep::REPE if !MACHINE.regs.flags.contains(Flags::ZF) => break,
                Rep::REPNE if MACHINE.regs.flags.contains(Flags::ZF) => break,
                _ => {}
            }
        }
    }
}

pub fn lodsb() {
    unsafe {
        let addr = MACHINE.regs.esi;
        MACHINE.regs.set_al(MACHINE.memory.read::<u8>(addr));
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.esi = addr.wrapping_sub(1);
        } else {
            MACHINE.regs.esi = addr.wrapping_add(1);
        }
    }
}

pub fn lodsd() {
    unsafe {
        let addr = MACHINE.regs.esi;
        MACHINE.regs.eax = MACHINE.memory.read::<u32>(addr);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.esi = addr.wrapping_sub(4);
        } else {
            MACHINE.regs.esi = addr.wrapping_add(4);
        }
    }
}

pub fn stosb() {
    unsafe {
        let addr = MACHINE.regs.edi;
        MACHINE.memory.write::<u8>(addr, MACHINE.regs.eax as u8);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.edi = addr.wrapping_sub(1);
        } else {
            MACHINE.regs.edi = addr.wrapping_add(1);
        }
    }
}

pub fn stosd() {
    unsafe {
        let addr = MACHINE.regs.edi;
        MACHINE.memory.write::<u32>(addr, MACHINE.regs.eax);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.edi = addr.wrapping_sub(4);
        } else {
            MACHINE.regs.edi = addr.wrapping_add(4);
        }
    }
}

pub fn scasb() {
    unsafe {
        let addr = MACHINE.regs.edi;
        let mem = MACHINE.memory.read::<u8>(addr);
        let _ = sub(MACHINE.regs.get_al(), mem);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.edi = addr.wrapping_sub(1);
        } else {
            MACHINE.regs.edi = addr.wrapping_add(1);
        }
    }
}

pub fn cmpsb() {
    unsafe {
        let src = MACHINE.memory.read::<u8>(MACHINE.regs.esi);
        let dst = MACHINE.memory.read::<u8>(MACHINE.regs.edi);
        let _ = sub(src, dst);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_sub(1);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_sub(1);
        } else {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_add(1);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_add(1);
        }
    }
}

pub fn movsd() {
    unsafe {
        let val = MACHINE.memory.read::<u32>(MACHINE.regs.esi);
        MACHINE.memory.write::<u32>(MACHINE.regs.edi, val);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_sub(4);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_sub(4);
        } else {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_add(4);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_add(4);
        }
    }
}

pub fn movsb() {
    unsafe {
        let val = MACHINE.memory.read::<u8>(MACHINE.regs.esi);
        MACHINE.memory.write::<u8>(MACHINE.regs.edi, val);
        if MACHINE.regs.flags.contains(Flags::DF) {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_sub(1);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_sub(1);
        } else {
            MACHINE.regs.esi = MACHINE.regs.esi.wrapping_add(1);
            MACHINE.regs.edi = MACHINE.regs.edi.wrapping_add(1);
        }
    }
}
