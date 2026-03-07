use crate::machine::{Flags, MEMORY, REGS};

pub fn push(x: u32) {
    unsafe {
        REGS.esp -= 4;
        *(MEMORY.add(REGS.esp as usize) as *mut u32) = x;
    }
}

pub fn pop() -> u32 {
    unsafe {
        let x = *(MEMORY.add(REGS.esp as usize) as *mut u32);
        REGS.esp += 4;
        x
    }
}

pub fn call(ret: u32, addr: u32) -> u32 {
    push(ret);
    addr
}

pub trait Int: num_traits::PrimInt {
    fn bits() -> usize;
    fn low_byte(&self) -> u8;

    fn high_bit(&self) -> Self {
        *self >> (Self::bits() - 1)
    }
}

impl Int for u32 {
    fn bits() -> usize {
        32
    }
    fn low_byte(&self) -> u8 {
        *self as u8
    }
}

impl Int for u8 {
    fn bits() -> usize {
        8
    }
    fn low_byte(&self) -> u8 {
        *self
    }
}

fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    b: bool,
) -> I {
    let z = if b { y.wrapping_add(&I::one()) } else { y };
    let (result, borrow) = x.overflowing_sub(&z);
    unsafe {
        REGS.flags.set(Flags::CF, borrow || (b && z == I::zero()));
        REGS.flags.set(Flags::ZF, result.is_zero());
        REGS.flags.set(Flags::SF, result.high_bit().is_one());
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  1  1
        //   1  0  0
        let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
        REGS.flags.set(Flags::OF, of);
        REGS.flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn sub<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
) -> I {
    sbb(x, y, false)
}

pub fn je(from: u32, x: u32) -> u32 {
    unsafe {
        if REGS.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jne(from: u32, x: u32) -> u32 {
    unsafe {
        if !REGS.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jmp(x: u32) -> u32 {
    x
}
