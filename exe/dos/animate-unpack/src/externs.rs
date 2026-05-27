use runtime::{Cont, Context};

pub fn xff82(ctx: &mut Context) -> Cont {
    do_unpack(ctx);
    std::process::exit(0);
}

fn do_unpack(ctx: &mut Context) {
    ctx.dump();

    let data = &mut ctx.memory.bytes[0x100..];
    let entry_point = 0xff82u16;

    let mut ofs = 0;
    // TODO: this is a hacky copy of register/cpu state
    let movs = b"\xbe\x3c\xf0\xbf\x00\x01\xbc\xf0\xff\xbd\x00\x01\xf9";
    data[ofs..ofs + movs.len()].copy_from_slice(movs);
    ofs += movs.len();

    let end_ofs = 0x100u16 + ofs as u16 + 3;
    let rel_jmp = entry_point - end_ofs;
    data[ofs] = 0xe9; // jmp
    ofs += 1;
    data[ofs..=ofs + 1].copy_from_slice(&rel_jmp.to_le_bytes());

    std::fs::write("animate.com", data).unwrap();
}
