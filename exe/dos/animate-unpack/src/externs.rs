use runtime::{Cont, Context};

pub fn xff82(ctx: &mut Context) -> Cont {
    do_unpack(ctx);
    std::process::exit(0);
}

fn do_unpack(ctx: &mut Context) {
    let data = &mut ctx.memory.bytes[0x100..];

    let entry_point = 0xff82u16;
    let rel_jmp = entry_point - 0x100u16 - 3;
    data[0] = 0xe9; // jmp
    data[1..=2].copy_from_slice(&rel_jmp.to_le_bytes());

    std::fs::write("animate.com", data).unwrap();
    ctx.dump();
}
