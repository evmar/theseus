use runtime::{Cont, Context};

pub fn xff82(ctx: &mut Context) -> Cont {
    do_unpack(ctx);
    std::process::exit(0);
}

fn encode_state(ctx: &Context) -> Result<Vec<u8>, iced_x86::IcedError> {
    use iced_x86::code_asm::*;
    let mut a = CodeAssembler::new(16)?;
    a.mov(cx, ctx.cpu.regs.get_cx() as u32)?;
    a.mov(si, ctx.cpu.regs.get_si() as u32)?;
    a.mov(di, ctx.cpu.regs.get_di() as u32)?;
    a.mov(sp, ctx.cpu.regs.get_sp() as u32)?;
    a.mov(bp, ctx.cpu.regs.get_bp() as u32)?;
    a.stc()?;
    a.jmp(0xff82u16 as u64)?;
    a.assemble(0x100)
}

fn do_unpack(ctx: &mut Context) {
    ctx.dump();

    let init = encode_state(ctx).unwrap();
    let data = &mut ctx.memory.bytes[0x100..];
    data[0..init.len()].copy_from_slice(&init);

    std::fs::write("animate.com", data).unwrap();
}
