use crate::{Cont, ContFn, Context, Flags, RETURN_FROM_X86_ADDR, segofs};

impl Context {
    pub fn call32(&mut self, ret: u32, addr: Cont) -> Cont {
        self.push32(ret);
        addr
    }

    pub fn call16(&mut self, ret: u16, addr: Cont) -> Cont {
        self.push16(ret);
        addr
    }

    /// Call a ContFn (builtin implementation) synchronously, without returning a continuation.
    pub fn call_builtin(&mut self, from: u32, func: ContFn) {
        // Because ContFn is stdcall it expects to pop a return address off the stack.
        // Ensure it is valid, though we ignore it.
        self.push32(RETURN_FROM_X86_ADDR);
        self.cpu.regs.eip_context = from;
        func(self); // pops the above return address
    }

    pub fn je(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }

    pub fn jne(&mut self, from: Cont, x: Cont) -> Cont {
        if !self.cpu.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }

    pub fn jb(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::CF) {
            return x;
        }
        from
    }

    pub fn js(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::SF) {
            return x;
        }
        from
    }

    pub fn jns(&mut self, from: Cont, x: Cont) -> Cont {
        if !self.cpu.flags.contains(Flags::SF) {
            return x;
        }
        from
    }

    pub fn ja(&mut self, from: Cont, x: Cont) -> Cont {
        if !self.cpu.flags.contains(Flags::CF) && !self.cpu.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }

    pub fn jae(&mut self, from: Cont, x: Cont) -> Cont {
        if !self.cpu.flags.contains(Flags::CF) {
            return x;
        }
        from
    }

    pub fn jl(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::SF) != self.cpu.flags.contains(Flags::OF) {
            return x;
        }
        from
    }

    pub fn jge(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::SF) == self.cpu.flags.contains(Flags::OF) {
            return x;
        }
        from
    }

    pub fn jecxz(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.regs.ecx == 0 {
            return x;
        }
        from
    }

    pub fn jg(&mut self, from: Cont, x: Cont) -> Cont {
        if !self.cpu.flags.contains(Flags::ZF)
            && self.cpu.flags.contains(Flags::SF) == self.cpu.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }

    pub fn jle(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::ZF)
            || self.cpu.flags.contains(Flags::SF) != self.cpu.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }

    pub fn jbe(&mut self, from: Cont, x: Cont) -> Cont {
        if self.cpu.flags.contains(Flags::CF) || self.cpu.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }

    pub fn ret32(&mut self, n: u16) -> Cont {
        let ret = self.pop32();
        self.cpu.regs.esp += n as u32;
        self.indirect(ret)
    }

    pub fn ret16(&mut self, n: u16) -> Cont {
        let ret = self.pop16();
        self.cpu.regs.esp += n as u32;
        self.indirect16(ret)
    }

    pub fn iret16(&mut self) -> Cont {
        let ip = self.pop16();
        let cs = self.pop16();
        self.cpu.regs.set_cs(cs);
        let flags = self.pop16();
        self.cpu.flags = Flags::from_bits(flags as u32).unwrap();
        self.indirect(segofs(cs, ip))
    }

    pub fn loop_(&mut self, from: Cont, x: Cont) -> Cont {
        self.cpu.regs.ecx = self.cpu.regs.ecx.wrapping_sub(1);
        if self.cpu.regs.ecx != 0 { x } else { from }
    }

    pub fn loopne(&mut self, from: Cont, x: Cont) -> Cont {
        self.cpu.regs.ecx = self.cpu.regs.ecx.wrapping_sub(1);
        if self.cpu.regs.ecx != 0 && !self.cpu.flags.contains(Flags::ZF) {
            x
        } else {
            from
        }
    }
}
