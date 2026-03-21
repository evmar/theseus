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
