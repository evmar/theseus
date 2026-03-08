use crate::Cont;
use crate::machine::{Flags, MACHINE};

pub fn call(ret: u32, addr: Cont) -> Cont {
    super::push(ret);
    addr
}

pub fn je(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jne(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jb(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::CF) {
            return x;
        }
        from
    }
}

pub fn js(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) {
            return x;
        }
        from
    }
}

pub fn jns(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::SF) {
            return x;
        }
        from
    }
}

pub fn ja(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::CF) && !MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jae(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::CF) {
            return x;
        }
        from
    }
}

pub fn jl(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) != MACHINE.regs.flags.contains(Flags::OF) {
            return x;
        }
        from
    }
}

pub fn jge(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) == MACHINE.regs.flags.contains(Flags::OF) {
            return x;
        }
        from
    }
}

pub fn jecxz(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.ecx == 0 {
            return x;
        }
        from
    }
}

pub fn jg(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::ZF)
            && MACHINE.regs.flags.contains(Flags::SF) == MACHINE.regs.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }
}

pub fn jle(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::ZF)
            || MACHINE.regs.flags.contains(Flags::SF) != MACHINE.regs.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }
}

pub fn jbe(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::CF) || MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn ret(n: u16) -> Cont {
    unsafe {
        let ret = super::pop();
        MACHINE.regs.esp += n as u32;
        (MACHINE.indirect)(ret)
    }
}
