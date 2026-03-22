use crate::Machine;

pub fn push(m: &mut Machine, x: u32) {
    m.regs.esp -= 4;
    m.memory.write::<u32>(m.regs.esp, x);
}

pub fn pop(m: &mut Machine) -> u32 {
    let x = m.memory.read::<u32>(m.regs.esp);
    m.regs.esp += 4;
    x
}

pub fn pushfd() {
    todo!("pushfd");
}

pub fn cwde(_m: &mut Machine) {
    todo!("cwde");
}

pub fn pushad(m: &mut Machine) {
    let esp = m.regs.esp;
    push(m, m.regs.eax);
    push(m, m.regs.ecx);
    push(m, m.regs.edx);
    push(m, m.regs.ebx);
    push(m, esp);
    push(m, m.regs.ebp);
    push(m, m.regs.esi);
    push(m, m.regs.edi);
}

pub fn popad(m: &mut Machine) {
    m.regs.edi = pop(m);
    m.regs.esi = pop(m);
    m.regs.ebp = pop(m);
    pop(m);
    m.regs.ebx = pop(m);
    m.regs.edx = pop(m);
    m.regs.ecx = pop(m);
    m.regs.eax = pop(m);
}
