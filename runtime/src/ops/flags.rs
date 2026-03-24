use crate::{Context, Flags};

pub fn stc(ctx: &mut Context) {
    ctx.cpu.flags.insert(Flags::CF);
}

pub fn clc(ctx: &mut Context) {
    ctx.cpu.flags.remove(Flags::CF);
}

pub fn std(ctx: &mut Context) {
    ctx.cpu.flags.insert(Flags::DF);
}

pub fn cld(ctx: &mut Context) {
    ctx.cpu.flags.remove(Flags::DF);
}

pub fn sahf(ctx: &mut Context) {
    // This constructs flags from the AH register, but only specific flags.
    let flags = Flags::from_bits(ctx.cpu.regs.get_ah() as u32).unwrap();
    ctx.cpu.flags.set(Flags::CF, flags.contains(Flags::CF));
    ctx.cpu.flags.set(Flags::PF, flags.contains(Flags::PF));
    // ctx.cpu.flags.set(Flags::AF, flags.contains(Flags::AF));
    ctx.cpu.flags.set(Flags::ZF, flags.contains(Flags::ZF));
    ctx.cpu.flags.set(Flags::OF, flags.contains(Flags::OF));
}
