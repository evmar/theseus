use super::math::sub;
use crate::{
    Context, Flags, Regs,
    memory::{MemRead, MemWrite},
    ops::int::Int,
    segofs,
};

#[derive(Debug)]
pub enum Rep {
    REP,
    REPNE,
    REPE,
}

/// A trait for the types of ints that we can run string operations for; concretely, u8/u16/u32.
/// We need Int for sub() (used in scas), MemRead/Write for reading/writing memory.
trait StringInt: Int + MemRead + MemWrite {
    fn from_eax(u: u32) -> Self;
    fn set_eax(&self, regs: &mut Regs);
}

impl StringInt for u8 {
    fn from_eax(u: u32) -> Self {
        u as u8
    }
    fn set_eax(&self, regs: &mut Regs) {
        // Note: cannot use `eax = Self as u8` because that clears high bits of eax.
        regs.set_al(*self);
    }
}
impl StringInt for u16 {
    fn from_eax(u: u32) -> Self {
        u as u16
    }
    fn set_eax(&self, regs: &mut Regs) {
        // Note: cannot use `eax = Self as u16` because that clears high bits of eax.
        regs.set_ax(*self);
    }
}
impl StringInt for u32 {
    fn from_eax(u: u32) -> Self {
        u as u32
    }
    fn set_eax(&self, regs: &mut Regs) {
        regs.eax = *self;
    }
}

impl Context {
    pub fn rep16(&mut self, rep: Rep, func: impl Fn(&mut Context)) {
        while self.cpu.regs.get_cx() != 0 {
            func(self);
            self.cpu.regs.set_cx(self.cpu.regs.get_cx().wrapping_sub(1));
            match rep {
                Rep::REPE if !self.cpu.flags.contains(Flags::ZF) => break,
                Rep::REPNE if self.cpu.flags.contains(Flags::ZF) => break,
                _ => {}
            }
        }
    }

    pub fn rep32(&mut self, rep: Rep, func: impl Fn(&mut Context)) {
        while self.cpu.regs.ecx != 0 {
            func(self);
            self.cpu.regs.ecx = self.cpu.regs.ecx.wrapping_sub(1);
            match rep {
                Rep::REPE if !self.cpu.flags.contains(Flags::ZF) => break,
                Rep::REPNE if self.cpu.flags.contains(Flags::ZF) => break,
                _ => {}
            }
        }
    }

    pub fn addr(&self, seg: u16, offset: u32) -> u32 {
        if self.cpu.real_mode {
            segofs(seg, offset as u16)
        } else {
            offset
        }
    }

    fn lods<S: StringInt>(&mut self, segment: u16) {
        self.memory
            .read::<S>(self.addr(segment, self.cpu.regs.esi))
            .set_eax(&mut self.cpu.regs);
        let step = std::mem::size_of::<S>() as u32;
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(step);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(step);
        }
    }

    pub fn lodsb(&mut self, segment: u16) {
        self.lods::<u8>(segment)
    }
    pub fn lodsw(&mut self, segment: u16) {
        self.lods::<u16>(segment)
    }
    pub fn lodsd(&mut self, segment: u16) {
        self.lods::<u32>(segment)
    }

    fn stos<S: StringInt>(&mut self) {
        // Note: no segment override allowed.
        self.memory.write::<S>(
            self.addr(self.cpu.regs.es, self.cpu.regs.edi),
            S::from_eax(self.cpu.regs.eax),
        );
        let step = std::mem::size_of::<S>() as u32;
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(step);
        } else {
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(step);
        }
    }

    pub fn stosb(&mut self) {
        self.stos::<u8>()
    }
    pub fn stosw(&mut self) {
        self.stos::<u16>()
    }
    pub fn stosd(&mut self) {
        self.stos::<u32>()
    }

    fn scas<S: StringInt>(&mut self) {
        // Note: no segment override allowed.
        let mem = self
            .memory
            .read::<S>(self.addr(self.cpu.regs.es, self.cpu.regs.edi));
        let _ = sub::<S>(S::from_eax(self.cpu.regs.eax), mem, &mut self.cpu.flags);
        let step = std::mem::size_of::<S>() as u32;
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(step);
        } else {
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(step);
        }
    }

    pub fn scasb(&mut self) {
        self.scas::<u8>()
    }
    pub fn scasw(&mut self) {
        self.scas::<u16>()
    }
    pub fn scasd(&mut self) {
        self.scas::<u32>()
    }

    fn cmps<S: StringInt>(&mut self, segment: u16) {
        let src = self.memory.read::<S>(self.addr(segment, self.cpu.regs.esi));
        let dst = self
            .memory
            .read::<S>(self.addr(self.cpu.regs.es, self.cpu.regs.edi));
        let _ = sub::<S>(src, dst, &mut self.cpu.flags);
        let step = std::mem::size_of::<S>() as u32;
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(step);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(step);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(step);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(step);
        }
    }

    pub fn cmpsb(&mut self, segment: u16) {
        self.cmps::<u8>(segment)
    }
    pub fn cmpsw(&mut self, segment: u16) {
        self.cmps::<u16>(segment)
    }
    pub fn cmpsd(&mut self, segment: u16) {
        self.cmps::<u32>(segment)
    }

    fn movs<S: StringInt>(&mut self, segment: u16) {
        let src_addr = self.addr(segment, self.cpu.regs.esi);
        let val = self.memory.read::<S>(src_addr);
        let dst_addr = self.addr(self.cpu.regs.es, self.cpu.regs.edi);
        self.memory.write::<S>(dst_addr, val);
        let step = std::mem::size_of::<S>() as u32;
        if self.cpu.flags.contains(Flags::DF) {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_sub(step);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_sub(step);
        } else {
            self.cpu.regs.esi = self.cpu.regs.esi.wrapping_add(step);
            self.cpu.regs.edi = self.cpu.regs.edi.wrapping_add(step);
        }
    }

    pub fn movsb(&mut self, segment: u16) {
        self.movs::<u8>(segment)
    }
    pub fn movsw(&mut self, segment: u16) {
        self.movs::<u16>(segment)
    }
    pub fn movsd(&mut self, segment: u16) {
        self.movs::<u32>(segment)
    }
}
