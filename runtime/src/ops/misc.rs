use crate::{Machine, machine::MACHINE};

pub fn push(x: u32) {
    unsafe {
        MACHINE.regs.esp -= 4;
        MACHINE.memory.write::<u32>(MACHINE.regs.esp, x);
    }
}

pub fn pop() -> u32 {
    unsafe {
        let x = MACHINE.memory.read::<u32>(MACHINE.regs.esp);
        MACHINE.regs.esp += 4;
        x
    }
}

pub fn pushfd() {
    todo!("pushfd");
}

pub fn cwde(_m: &mut Machine) {
    todo!("cwde");
}

pub fn pushad(m: &mut Machine) {
    let esp = m.regs.esp;
    push(m.regs.eax);
    push(m.regs.ecx);
    push(m.regs.edx);
    push(m.regs.ebx);
    push(esp);
    push(m.regs.ebp);
    push(m.regs.esi);
    push(m.regs.edi);
}

pub fn popad(m: &mut Machine) {
    m.regs.edi = pop();
    m.regs.esi = pop();
    m.regs.ebp = pop();
    pop();
    m.regs.ebx = pop();
    m.regs.edx = pop();
    m.regs.ecx = pop();
    m.regs.eax = pop();
}
