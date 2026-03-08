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

fn sbb_impl<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
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

pub fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
) -> I {
    unsafe { sbb_impl(x, y, MACHINE.regs.flags.contains(Flags::CF)) }
}

pub fn sub<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
) -> I {
    sbb_impl(x, y, false)
}

pub fn add<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I) -> I {
    addc(x, y, I::zero())
}

pub fn addc<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, z: I) -> I {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let result = x.wrapping_add(&y.wrapping_add(&z));
    unsafe {
        MACHINE
            .regs
            .flags
            .set(Flags::CF, result < x || (result == x && !z.is_zero()));
        MACHINE.regs.flags.set(Flags::ZF, result.is_zero());
        MACHINE
            .regs
            .flags
            .set(Flags::SF, result.high_bit().is_one());
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  0  1
        //   1  1  0
        let of = ((x ^ !y) & (x ^ result)).high_bit().is_one();
        MACHINE.regs.flags.set(Flags::OF, of);
        MACHINE
            .regs
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
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

pub fn js(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::SF) {
            return x;
        }
        from
    }
}

pub fn jns(from: Cont, x: Cont) -> Cont {
    unsafe {
        if !MACHINE.regs.flags.contains(Flags::SF) {
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

pub fn jbe(from: Cont, x: Cont) -> Cont {
    unsafe {
        if MACHINE.regs.flags.contains(Flags::CF) || MACHINE.regs.flags.contains(Flags::ZF) {
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

/// or: Logical Inclusive OR
pub fn or<I: Int>(x: I, y: I) -> I {
    let result = x | y;
    unsafe {
        MACHINE.regs.flags.remove(Flags::OF | Flags::CF);
        MACHINE
            .regs
            .flags
            .set(Flags::SF, result.high_bit().is_one());
        MACHINE.regs.flags.set(Flags::ZF, result.is_zero());
        MACHINE
            .regs
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn ret(n: u16) -> Cont {
    unsafe {
        let ret = pop();
        MACHINE.regs.esp += n as u32;
        (MACHINE.indirect)(ret)
    }
}

/// neg: Two's Complement Negation
pub fn neg<I: Int + num_traits::ops::overflowing::OverflowingSub>(x: I) -> I {
    let (result, of) = I::zero().overflowing_sub(&x);
    unsafe {
        MACHINE.regs.flags.set(Flags::ZF, result.is_zero());
        MACHINE.regs.flags.set(Flags::CF, !result.is_zero());
        MACHINE.regs.flags.set(Flags::OF, of);
        MACHINE
            .regs
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn shl<I: Int + num_traits::WrappingShl>(x: I, y: u8) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }

    // Carry is the highest bit that will be shifted out.
    let cf = (x >> (I::bits() - y as usize) & I::one()).is_one();
    let val = x.wrapping_shl(y as u32);
    unsafe {
        MACHINE.regs.flags.set(Flags::CF, cf);
        let msb = val.high_bit().is_one();
        MACHINE.regs.flags.set(Flags::SF, msb);
        // Note: OF only defined for 1-bit rotates.
        // "For left shifts, the OF flag is set to 0 if the mostsignificant bit of the result is the
        // same as the CF flag (that is, the top two bits of the original operand were the same) [...]"
        MACHINE.regs.flags.set(
            Flags::OF,
            x.shr(I::bits() - 1).is_one() ^ (x.shr(I::bits() - 2) & I::one()).is_one(),
        );
        MACHINE.regs.flags.set(Flags::ZF, val.is_zero());
        MACHINE
            .regs
            .flags
            .set(Flags::PF, val.low_byte().count_ones() % 2 == 0);
    }
    val
}

/// shr: Shift
pub fn shr<I: Int>(x: I, y: u8) -> I {
    // In all modes but 64 it is correct to mask to 32 bits.
    assert!(I::bits() < 64); // 64 not implemented
    let y = y % 32;

    if y == 0 {
        return x; // Don't affect flags.
    }

    let val = x >> y as usize;
    unsafe {
        MACHINE
            .regs
            .flags
            .set(Flags::CF, ((x >> (y - 1) as usize) & I::one()).is_one());
        MACHINE.regs.flags.set(Flags::SF, false); // ?
        MACHINE.regs.flags.set(Flags::ZF, val.is_zero());

        // Note: OF state undefined for shifts > 1 bit.
        MACHINE.regs.flags.set(Flags::OF, x.high_bit().is_one());
        MACHINE
            .regs
            .flags
            .set(Flags::PF, val.low_byte().count_ones() % 2 == 0);
    }
    val
}
