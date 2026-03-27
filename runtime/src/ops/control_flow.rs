use crate::{Cont, Context, Flags, indirect};

pub fn call(ctx: &mut Context, ret: u32, addr: Cont) -> Cont {
    super::push(ctx, ret);
    addr
}

pub fn je(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jne(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if !ctx.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jb(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::CF) {
        return x;
    }
    from
}

pub fn js(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::SF) {
        return x;
    }
    from
}

pub fn jns(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if !ctx.cpu.flags.contains(Flags::SF) {
        return x;
    }
    from
}

pub fn ja(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if !ctx.cpu.flags.contains(Flags::CF) && !ctx.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn jae(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if !ctx.cpu.flags.contains(Flags::CF) {
        return x;
    }
    from
}

pub fn jl(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::SF) != ctx.cpu.flags.contains(Flags::OF) {
        return x;
    }
    from
}

pub fn jge(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::SF) == ctx.cpu.flags.contains(Flags::OF) {
        return x;
    }
    from
}

pub fn jecxz(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.regs.ecx == 0 {
        return x;
    }
    from
}

pub fn jg(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if !ctx.cpu.flags.contains(Flags::ZF)
        && ctx.cpu.flags.contains(Flags::SF) == ctx.cpu.flags.contains(Flags::OF)
    {
        return x;
    }
    from
}

pub fn jle(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::ZF)
        || ctx.cpu.flags.contains(Flags::SF) != ctx.cpu.flags.contains(Flags::OF)
    {
        return x;
    }
    from
}

pub fn jbe(ctx: &mut Context, from: Cont, x: Cont) -> Cont {
    if ctx.cpu.flags.contains(Flags::CF) || ctx.cpu.flags.contains(Flags::ZF) {
        return x;
    }
    from
}

pub fn ret(ctx: &mut Context, n: u16) -> Cont {
    let ret = super::pop(ctx);
    ctx.cpu.regs.esp += n as u32;
    indirect(ctx, ret)
}

pub fn enter(ctx: &mut Context, bytes: u16, nesting: u8) {
    assert_eq!(nesting, 0);
    super::push(ctx, ctx.cpu.regs.ebp);
    ctx.cpu.regs.ebp = ctx.cpu.regs.esp;
    ctx.cpu.regs.esp -= bytes as u32;
}

pub fn leave(ctx: &mut Context) {
    ctx.cpu.regs.esp = ctx.cpu.regs.ebp;
    ctx.cpu.regs.ebp = super::pop(ctx);
}

pub fn sete(ctx: &Context) -> u8 {
    ctx.cpu.flags.contains(Flags::ZF) as u8
}
