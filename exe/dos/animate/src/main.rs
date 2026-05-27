mod generated;

fn main() {
    let exe = &generated::EXEDATA;
    let mut ctx = winapi::load(exe);
    ctx.cpu.regs.esi = 0x0000f03c;
    ctx.cpu.regs.edi = 0x00000100;
    ctx.cpu.regs.esp = 0x0000fff0;
    ctx.cpu.regs.ebp = 0x00000100;
    winapi::start(&mut ctx, exe);
}
