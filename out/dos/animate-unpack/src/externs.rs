use runtime::{Cont, Context};

pub fn xff82(ctx: &mut Context) -> Cont {
    do_unpack(ctx);
    std::process::exit(0);
}

fn do_unpack(ctx: &mut Context) {
    println!("unpacking at ip=0xff82, final state:");
    ctx.dump();
    let data = dos::dump_com(ctx);
    std::fs::write("animate.com", data).unwrap();
    println!("wrote animate.com");
}
