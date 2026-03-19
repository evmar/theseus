use crate::{MACHINE, machine::Flags};
use super::math::sub;

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

pub fn movsx() {
    todo!("movsx");
}

pub fn movsd() {
    todo!("movsd");
}

pub fn movsb() {
    todo!("movsb");
}

pub fn movq() {
    todo!("movq");
}

pub fn movdqa() {
    todo!("movdqa");
}

pub fn xchg() {
    todo!("xchg");
}

pub fn cmpxchg() {
    todo!("cmpxchg");
}
