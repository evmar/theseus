use crate::Machine;

pub fn push(m: &mut Machine, x: u32) {
    m.cpu.regs.esp -= 4;
    m.memory.write::<u32>(m.cpu.regs.esp, x);
}

pub fn pop(m: &mut Machine) -> u32 {
    let x = m.memory.read::<u32>(m.cpu.regs.esp);
    m.cpu.regs.esp += 4;
    x
}

pub fn pushfd() {
    todo!("pushfd");
}

pub fn cwde(_m: &mut Machine) {
    todo!("cwde");
}

pub fn pushad(m: &mut Machine) {
    let esp = m.cpu.regs.esp;
    push(m, m.cpu.regs.eax);
    push(m, m.cpu.regs.ecx);
    push(m, m.cpu.regs.edx);
    push(m, m.cpu.regs.ebx);
    push(m, esp);
    push(m, m.cpu.regs.ebp);
    push(m, m.cpu.regs.esi);
    push(m, m.cpu.regs.edi);
}

pub fn popad(m: &mut Machine) {
    m.cpu.regs.edi = pop(m);
    m.cpu.regs.esi = pop(m);
    m.cpu.regs.ebp = pop(m);
    pop(m);
    m.cpu.regs.ebx = pop(m);
    m.cpu.regs.edx = pop(m);
    m.cpu.regs.ecx = pop(m);
    m.cpu.regs.eax = pop(m);
}
