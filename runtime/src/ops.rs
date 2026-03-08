use crate::Cont;
use crate::machine::{Flags, MACHINE};

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

pub fn call(ret: u32, addr: Cont) -> Cont {
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
        MACHINE
            .regs
            .flags
            .set(Flags::CF, borrow || (b && z == I::zero()));
        MACHINE.regs.flags.set(Flags::ZF, result.is_zero());
        MACHINE
            .regs
            .flags
            .set(Flags::SF, result.high_bit().is_one());
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  1  1
        //   1  0  0
        let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
        MACHINE.regs.flags.set(Flags::OF, of);
        MACHINE
            .regs
            .flags
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

pub fn je(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jne(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jb(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::CF) {
            return x;
        }
        from
    }
}

pub fn ja(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::CF) && !MACHINE.regs.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}

pub fn jae(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::CF) {
            return x;
        }
        from
    }
}

pub fn jl(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) != MACHINE.regs.flags.contains(Flags::OF) {
            return x;
        }
        from
    }
}

pub fn jge(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) == MACHINE.regs.flags.contains(Flags::OF) {
            return x;
        }
        from
    }
}

pub fn jecxz(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.ecx == 0 {
            return x;
        }
        from
    }
}

pub fn jg(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::ZF)
            && MACHINE.regs.flags.contains(Flags::SF) == MACHINE.regs.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }
}

pub fn jle(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::ZF)
            || MACHINE.regs.flags.contains(Flags::SF) != MACHINE.regs.flags.contains(Flags::OF)
        {
            return x;
        }
        from
    }
}

pub fn and<I: Int>(x: I, y: I) -> I {
    let result = x & y;
    unsafe {
        MACHINE.regs.flags.set(Flags::ZF, result.is_zero());
        MACHINE
            .regs
            .flags
            .set(Flags::SF, result.high_bit().is_one());
        MACHINE.regs.flags.set(Flags::OF, false);
        MACHINE.regs.flags.set(Flags::CF, false);
        MACHINE
            .regs
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}
