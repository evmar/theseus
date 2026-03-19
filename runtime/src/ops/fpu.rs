use crate::MACHINE;

/// fild: Load Integer
pub fn fild(f: f64) {
    unsafe {
        MACHINE.fpu.push(f);
    }
}
