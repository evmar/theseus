use crate::MACHINE;

pub fn fpu_set(index: usize, value: f64) {
    unsafe {
        *MACHINE.fpu.get(index) = value;
    }
}

/// fild: Load Integer
pub fn fild(f: f64) {
    unsafe {
        MACHINE.fpu.push(f);
    }
}
