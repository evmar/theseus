use crate::{Context, Flags, segofs};

impl Context {
    pub fn push32(&mut self, x: u32) {
        self.cpu.regs.esp -= 4;
        self.memory.write::<u32>(self.cpu.regs.esp, x);
    }

    pub fn push16(&mut self, x: u16) {
        self.cpu.regs.esp -= 2;
        self.memory
            .write::<u16>(segofs(self.cpu.regs.ss, self.cpu.regs.get_sp()), x);
    }

    /// cpuid, describing a plain Pentium: an FPU and nothing else (no TSC, no
    /// MMX), so programs take the paths this runtime implements.
    pub fn cpuid(&mut self) {
        let (eax, ebx, ecx, edx) = match self.cpu.regs.eax {
            // Highest leaf, and "GenuineIntel" in ebx:edx:ecx.
            0 => (1, 0x756e_6547, 0x6c65_746e, 0x4965_6e69),
            // Family 5 model 4 stepping 3; feature flags: FPU.
            1 => (0x543, 0, 0, 0x1),
            _ => (0, 0, 0, 0),
        };
        self.cpu.regs.eax = eax;
        self.cpu.regs.ebx = ebx;
        self.cpu.regs.ecx = ecx;
        self.cpu.regs.edx = edx;
    }

    pub fn pop32(&mut self) -> u32 {
        let x = self.memory.read::<u32>(self.cpu.regs.esp);
        self.cpu.regs.esp += 4;
        x
    }

    pub fn pop16(&mut self) -> u16 {
        let x = self
            .memory
            .read::<u16>(segofs(self.cpu.regs.ss, self.cpu.regs.get_sp()));
        self.cpu.regs.esp += 2;
        x
    }

    pub fn pushad(&mut self) {
        let esp = self.cpu.regs.esp;
        self.push32(self.cpu.regs.eax);
        self.push32(self.cpu.regs.ecx);
        self.push32(self.cpu.regs.edx);
        self.push32(self.cpu.regs.ebx);
        self.push32(esp);
        self.push32(self.cpu.regs.ebp);
        self.push32(self.cpu.regs.esi);
        self.push32(self.cpu.regs.edi);
    }

    pub fn popad(&mut self) {
        self.cpu.regs.edi = self.pop32();
        self.cpu.regs.esi = self.pop32();
        self.cpu.regs.ebp = self.pop32();
        self.pop32();
        self.cpu.regs.ebx = self.pop32();
        self.cpu.regs.edx = self.pop32();
        self.cpu.regs.ecx = self.pop32();
        self.cpu.regs.eax = self.pop32();
    }

    pub fn enter(&mut self, bytes: u16, nesting: u8) {
        assert_eq!(nesting, 0);
        self.push32(self.cpu.regs.ebp);
        self.cpu.regs.ebp = self.cpu.regs.esp;
        self.cpu.regs.esp -= bytes as u32;
    }

    pub fn leave(self: &mut Context) {
        self.cpu.regs.esp = self.cpu.regs.ebp;
        self.cpu.regs.ebp = self.pop32();
    }

    pub fn sete(self: &Context) -> u8 {
        self.cpu.flags.contains(Flags::ZF) as u8
    }

    pub fn setne(self: &Context) -> u8 {
        !self.cpu.flags.contains(Flags::ZF) as u8
    }

    pub fn setg(self: &Context) -> u8 {
        (!self.cpu.flags.contains(Flags::ZF)
            && self.cpu.flags.contains(Flags::SF) == self.cpu.flags.contains(Flags::OF))
            as u8
    }

    pub fn setge(self: &Context) -> u8 {
        (self.cpu.flags.contains(Flags::SF) == self.cpu.flags.contains(Flags::OF)) as u8
    }

    pub fn setl(self: &Context) -> u8 {
        (self.cpu.flags.contains(Flags::SF) != self.cpu.flags.contains(Flags::OF)) as u8
    }

    pub fn setle(self: &Context) -> u8 {
        (self.cpu.flags.contains(Flags::ZF)
            || self.cpu.flags.contains(Flags::SF) != self.cpu.flags.contains(Flags::OF))
            as u8
    }

    pub fn seta(self: &Context) -> u8 {
        (!self.cpu.flags.contains(Flags::CF) && !self.cpu.flags.contains(Flags::ZF)) as u8
    }

    pub fn setae(self: &Context) -> u8 {
        !self.cpu.flags.contains(Flags::CF) as u8
    }

    pub fn setb(self: &Context) -> u8 {
        self.cpu.flags.contains(Flags::CF) as u8
    }

    pub fn setbe(self: &Context) -> u8 {
        (self.cpu.flags.contains(Flags::CF) || self.cpu.flags.contains(Flags::ZF)) as u8
    }

    pub fn sti(&mut self) {
        // TODO: self.cpu.flags.insert(Flags::IF);
    }

    pub fn cli(&mut self) {
        // TODO: self.cpu.flags.remove(Flags::IF);
    }

    pub fn xlat(&mut self, segment: u16) {
        let offset = if self.cpu.real_mode {
            self.cpu.regs.get_bx() as u32
        } else {
            self.cpu.regs.ebx
        };
        let offset = offset.wrapping_add(self.cpu.regs.get_al() as u32);
        let value = self.memory.read::<u8>(self.addr(segment, offset));
        self.cpu.regs.set_al(value);
    }
}
