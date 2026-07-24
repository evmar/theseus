use runtime::{Cont, Context, segofs};
use zerocopy::FromBytes;

#[repr(C)]
#[derive(zerocopy::FromBytes, Debug)]
struct Header {
    sig: [u8; 2],
    version: u16,
    entry_ofs: u16,
    entry_seg: u16,
}

pub fn x11(ctx: &mut Context) -> Cont {
    ctx.cpu.regs.set_ax(0xfbfb);
    ctx.cpu.regs.set_cs(0);
    dos::int(ctx, 0x12, 0x2f)
}

pub fn x12(ctx: &mut Context) -> Cont {
    ctx.dump();
    let addr = segofs(ctx.cpu.regs.es, ctx.cpu.regs.get_bx());
    let header = <Header>::read_from_prefix(&ctx.memory[addr..]).unwrap().0;
    println!("got header {header:x?}");
    todo!()
}
