use runtime::{Cont, Context};

pub fn x11(ctx: &mut Context) -> Cont {
    ctx.cpu.regs.set_ax(0xfbfb);
    ctx.cpu.regs.set_cs(0);
    dos::int(ctx, 0, 0x2f);
    todo!()
}
