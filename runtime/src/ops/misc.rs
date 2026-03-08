use crate::machine::MACHINE;

pub fn push(x: u32) {
    unsafe {
        MACHINE.regs.esp -= 4;
        *(MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32) = x;
    }
}

pub fn pop() -> u32 {
    unsafe {
        let x = *(MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32);
        MACHINE.regs.esp += 4;
        x
    }
}
