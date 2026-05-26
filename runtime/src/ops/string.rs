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

impl Context {
    pub fn rep(&mut self, rep: Rep, func: impl Fn(&mut Context)) {
        while self.cpu.regs.ecx > 0 {
            func(self);
            self.cpu.regs.ecx = self.cpu.regs.ecx.wrapping_sub(1);
            match rep {
                Rep::REPE if !self.cpu.flags.contains(Flags::ZF) => break,
                Rep::REPNE if self.cpu.flags.contains(Flags::ZF) => break,
                _ => {}
            }
        }
    }

    pub fn lodsb(&mut self) {
        let addr = self.cpu.regs.esi;
        self.cpu.regs.set_al(self.memory.read::<u8>(addr));
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = addr.wrapping_sub(1);
        } else {
            self.cpu.regs.esi = addr.wrapping_add(1);
        }
    }

    pub fn lodsd(&mut self) {
        let addr = self.cpu.regs.esi;
        self.cpu.regs.eax = self.memory.read::<u32>(addr);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = addr.wrapping_sub(4);
        } else {
            self.cpu.regs.esi = addr.wrapping_add(4);
        }
    }

    pub fn stosb(&mut self) {
        let addr = self.cpu.regs.edi;
        self.memory.write::<u8>(addr, self.cpu.regs.eax as u8);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = addr.wrapping_sub(1);
        } else {
            self.cpu.regs.edi = addr.wrapping_add(1);
        }
    }

    pub fn stosw(&mut self) {
        let addr = self.cpu.regs.edi;
        self.memory.write::<u16>(addr, self.cpu.regs.eax as u16);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = addr.wrapping_sub(2);
        } else {
            self.cpu.regs.edi = addr.wrapping_add(2);
        }
    }

    pub fn stosd(&mut self) {
        let addr = self.cpu.regs.edi;
        self.memory.write::<u32>(addr, self.cpu.regs.eax);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = addr.wrapping_sub(4);
        } else {
            self.cpu.regs.edi = addr.wrapping_add(4);
        }
    }

    pub fn scasb(&mut self) {
        let addr = self.cpu.regs.edi;
        let mem = self.memory.read::<u8>(addr);
        let _ = sub(self.cpu.regs.get_al(), mem, &mut self.cpu.flags);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = addr.wrapping_sub(1);
        } else {
            self.cpu.regs.edi = addr.wrapping_add(1);
        }
    }

    pub fn cmpsb(&mut self) {
        let src = self.memory.read::<u8>(self.cpu.regs.esi);
        let dst = self.memory.read::<u8>(self.cpu.regs.edi);
        let _ = sub(src, dst, &mut self.cpu.flags);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(1);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(1);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(1);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(1);
        }
    }

    pub fn movsd(&mut self) {
        let val = self.memory.read::<u32>(self.cpu.regs.esi);
        self.memory.write::<u32>(self.cpu.regs.edi, val);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(4);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(4);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(4);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(4);
        }
    }

    pub fn movsw(&mut self) {
        let val = self.memory.read::<u16>(self.cpu.regs.esi);
        self.memory.write::<u16>(self.cpu.regs.edi, val);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(2);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(2);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(2);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(2);
        }
    }

    pub fn movsb(&mut self) {
        let val = self.memory.read::<u8>(self.cpu.regs.esi);
        self.memory.write::<u8>(self.cpu.regs.edi, val);
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(1);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(1);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(1);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(1);
        }
    }
}
