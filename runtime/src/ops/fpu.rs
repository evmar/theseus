use crate::MACHINE;

/// fild: Load Integer
pub fn fild(f: f64) {
    unsafe {
        MACHINE.fpu.push(f);
    }
}

pub fn fmul(x: f64, y: f64) -> f64 {
    x * y
}
