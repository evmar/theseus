pub(crate) use super::blaster;
use runtime::{Cont, Context};

pub fn noop_load_driver(ctx: &mut Context) -> Cont {
    //ctx.cpu.flags.remove(runtime::Flags::CF); // success
    let ds = ctx.pop16();
    ctx.cpu.regs.set_ds(ds);
    ctx.ret16(0)
}
