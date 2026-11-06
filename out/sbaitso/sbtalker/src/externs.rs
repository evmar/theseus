use crate::sbtalker::SBTalker;

pub(crate) use super::blaster;
use runtime::{Cont, Context};

fn demo(ctx: &mut Context) {
    let sbtalker = SBTalker::new(ctx);

    let parsed = sbtalker.parse(ctx, "hello mastodon, i am dr. sbaitso.");
    println!("parsed: {}", parsed);

    let audio = sbtalker.speak(ctx, "hello mastodon, i am dr. sbaitso.");
    log::info!("audio buffer: {:x} bytes", audio.len());
}

pub fn tsr_loaded(ctx: &mut Context) -> Cont {
    dos::exit(ctx);
    unreachable!();
}

pub fn noop_load_driver(ctx: &mut Context) -> Cont {
    //ctx.cpu.flags.remove(runtime::Flags::CF); // success
    let ds = ctx.pop16();
    ctx.cpu.regs.set_ds(ds);
    ctx.ret16(0)
}
