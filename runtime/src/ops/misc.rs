use crate::{Context, Flags};

impl Context {
    pub fn push(&mut self, x: u32) {
        self.cpu.regs.esp -= 4;
        self.memory.write::<u32>(self.cpu.regs.esp, x);
    }

    pub fn push16(&mut self, _x: u16) {
        todo!();
    }

    pub fn pop(&mut self) -> u32 {
        let x = self.memory.read::<u32>(self.cpu.regs.esp);
        self.cpu.regs.esp += 4;
        x
    }

    pub fn pop16(&mut self) -> u16 {
        todo!();
    }

    pub fn pushad(&mut self) {
        let esp = self.cpu.regs.esp;
        self.push(self.cpu.regs.eax);
        self.push(self.cpu.regs.ecx);
        self.push(self.cpu.regs.edx);
        self.push(self.cpu.regs.ebx);
        self.push(esp);
        self.push(self.cpu.regs.ebp);
        self.push(self.cpu.regs.esi);
        self.push(self.cpu.regs.edi);
    }

    pub fn popad(&mut self) {
        self.cpu.regs.edi = self.pop();
        self.cpu.regs.esi = self.pop();
        self.cpu.regs.ebp = self.pop();
        self.pop();
        self.cpu.regs.ebx = self.pop();
        self.cpu.regs.edx = self.pop();
        self.cpu.regs.ecx = self.pop();
        self.cpu.regs.eax = self.pop();
    }

    pub fn enter(&mut self, bytes: u16, nesting: u8) {
        assert_eq!(nesting, 0);
        self.push(self.cpu.regs.ebp);
        self.cpu.regs.ebp = self.cpu.regs.esp;
        self.cpu.regs.esp -= bytes as u32;
    }

    pub fn leave(self: &mut Context) {
        self.cpu.regs.esp = self.cpu.regs.ebp;
        self.cpu.regs.ebp = self.pop();
    }

    pub fn sete(self: &Context) -> u8 {
        self.cpu.flags.contains(Flags::ZF) as u8
    }

    pub fn setne(self: &Context) -> u8 {
        !self.cpu.flags.contains(Flags::ZF) as u8
    }
}
