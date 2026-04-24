use crate::Context;

pub fn push(ctx: &mut Context, x: u32) {
    ctx.cpu.regs.esp -= 4;
    ctx.memory.write::<u32>(ctx.cpu.regs.esp, x);
}

pub fn push16(_ctx: &mut Context, _x: u16) {
    todo!();
}

pub fn pop(ctx: &mut Context) -> u32 {
    let x = ctx.memory.read::<u32>(ctx.cpu.regs.esp);
    ctx.cpu.regs.esp += 4;
    x
}

pub fn pop16(_ctx: &mut Context) -> u16 {
    todo!();
}

pub fn pushad(ctx: &mut Context) {
    let esp = ctx.cpu.regs.esp;
    push(ctx, ctx.cpu.regs.eax);
    push(ctx, ctx.cpu.regs.ecx);
    push(ctx, ctx.cpu.regs.edx);
    push(ctx, ctx.cpu.regs.ebx);
    push(ctx, esp);
    push(ctx, ctx.cpu.regs.ebp);
    push(ctx, ctx.cpu.regs.esi);
    push(ctx, ctx.cpu.regs.edi);
}

pub fn popad(ctx: &mut Context) {
    ctx.cpu.regs.edi = pop(ctx);
    ctx.cpu.regs.esi = pop(ctx);
    ctx.cpu.regs.ebp = pop(ctx);
    pop(ctx);
    ctx.cpu.regs.ebx = pop(ctx);
    ctx.cpu.regs.edx = pop(ctx);
    ctx.cpu.regs.ecx = pop(ctx);
    ctx.cpu.regs.eax = pop(ctx);
}
