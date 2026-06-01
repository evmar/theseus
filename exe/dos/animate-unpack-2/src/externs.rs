use crate::generated::xff82;
use runtime::{Cont, Context};

static mut UNPACK: bool = false;

pub fn x100(ctx: &mut Context) -> Cont {
    let unpack = unsafe {
        if UNPACK {
            true
        } else {
            UNPACK = true;
            false
        }
    };

    if !unpack {
        ctx.cpu.flags.insert(runtime::Flags::CF);
        ctx.cpu.regs.ecx = 0x00000000;
        ctx.cpu.regs.edx = 0x00000813;
        ctx.cpu.regs.esi = 0x0000f03c;
        ctx.cpu.regs.edi = 0x00000100;
        ctx.cpu.regs.esp = 0x0000fff4;
        ctx.cpu.regs.ebp = 0x00000100;
        Cont(xff82)
    } else {
        do_unpack(ctx);
        std::process::exit(0);
    }
}

fn do_unpack(ctx: &mut Context) {
    ctx.dump();
    let data = &ctx.memory.bytes[0x100..];
    std::fs::write("animate2.com", data).unwrap();
    println!("wrote animate.com");
}
