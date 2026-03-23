use crate::{Cont, Flags, Machine, indirect};

pub fn call(m: &mut Machine, ret: u32, addr: Cont) -> Cont {
    super::push(m, ret);
    addr
}

pub fn je(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jne(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if !m.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jb(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::CF) {
        return x;
    }
    from
}

pub fn js(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::SF) {
        return x;
    }
    from
}

pub fn jns(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if !m.cpu.flags.contains(Flags::SF) {
        return x;
    }
    from
}

pub fn ja(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if !m.cpu.flags.contains(Flags::CF) && !m.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jae(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if !m.cpu.flags.contains(Flags::CF) {
        return x;
    }
    from
}

pub fn jl(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::SF) != m.cpu.flags.contains(Flags::OF) {
        return x;
    }
    from
}

pub fn jge(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::SF) == m.cpu.flags.contains(Flags::OF) {
        return x;
    }
    from
}

pub fn jecxz(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.regs.ecx == 0 {
        return x;
    }
    from
}

pub fn jg(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if !m.cpu.flags.contains(Flags::ZF)
        && m.cpu.flags.contains(Flags::SF) == m.cpu.flags.contains(Flags::OF)
    {
        return x;
    }
    from
}

pub fn jle(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::ZF)
        || m.cpu.flags.contains(Flags::SF) != m.cpu.flags.contains(Flags::OF)
    {
        return x;
    }
    from
}

pub fn jbe(m: &mut Machine, from: Cont, x: Cont) -> Cont {
    if m.cpu.flags.contains(Flags::CF) || m.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn ret(m: &mut Machine, n: u16) -> Cont {
    let ret = super::pop(m);
    m.cpu.regs.esp += n as u32;
    indirect(m, ret)
}

pub fn enter(m: &mut Machine, bytes: u16, nesting: u8) {
    assert_eq!(nesting, 0);
    super::push(m, m.cpu.regs.ebp);
    m.cpu.regs.ebp = m.cpu.regs.esp;
    m.cpu.regs.esp -= bytes as u32;
}

pub fn leave(m: &mut Machine) {
    m.cpu.regs.esp = m.cpu.regs.ebp;
    m.cpu.regs.ebp = super::pop(m);
}

pub fn sete(m: &Machine) -> u8 {
    m.cpu.flags.contains(Flags::ZF) as u8
}

pub fn setge(_m: &mut Machine) {
    todo!("setge");
}

pub fn setne(_m: &mut Machine) {
    todo!("setne");
}

pub fn setg(_m: &mut Machine) {
    todo!("setg");
}
