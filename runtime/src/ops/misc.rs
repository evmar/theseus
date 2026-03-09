use crate::machine::MACHINE;

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

pub fn std() {
    todo!("std");
}

pub fn cld() {
    todo!("cld");
}

pub fn pushfd() {
    todo!("pushfd");
}
