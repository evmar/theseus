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

    pub fn setge(self: &Context) -> u8 {
        (self.cpu.flags.contains(Flags::ZF) == self.cpu.flags.contains(Flags::OF)) as u8
    }

    pub fn setne(self: &Context) -> u8 {
        !self.cpu.flags.contains(Flags::ZF) as u8
    }

    pub fn sti(&mut self) {
        // TODO: self.cpu.flags.insert(Flags::IF);
    }

    pub fn cli(&mut self) {
        // TODO: self.cpu.flags.remove(Flags::IF);
    }

    pub fn out(&mut self, port: u16, data: u8) {
        log::warn!("TODO: out({:#x}, {:#x})", port, data);
    }
}
