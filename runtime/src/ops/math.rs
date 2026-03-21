use crate::{Flags, MACHINE};

use super::int::Int;

fn sbb_impl<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    b: bool,
) -> I {
    let z = if b { y.wrapping_add(&I::one()) } else { y };
    let (result, borrow) = x.overflowing_sub(&z);
    unsafe {
        MACHINE
            .flags
            .set(Flags::CF, borrow || (b && z == I::zero()));
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE.flags.set(Flags::SF, result.high_bit().is_one());
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  1  1
        //   1  0  0
        let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
        MACHINE.flags.set(Flags::OF, of);
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
) -> I {
    unsafe { sbb_impl(x, y, MACHINE.flags.contains(Flags::CF)) }
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
            .flags
            .set(Flags::CF, result < x || (result == x && !z.is_zero()));
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE.flags.set(Flags::SF, result.high_bit().is_one());
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  0  1
        //   1  1  0
        let of = ((x ^ !y) & (x ^ result)).high_bit().is_one();
        MACHINE.flags.set(Flags::OF, of);
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn and<I: Int>(x: I, y: I) -> I {
    let result = x & y;
    unsafe {
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE.flags.set(Flags::SF, result.high_bit().is_one());
        MACHINE.flags.set(Flags::OF, false);
        MACHINE.flags.set(Flags::CF, false);
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

/// or: Logical Inclusive OR
pub fn or<I: Int>(x: I, y: I) -> I {
    let result = x | y;
    unsafe {
        MACHINE.flags.remove(Flags::OF | Flags::CF);
        MACHINE.flags.set(Flags::SF, result.high_bit().is_one());
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

/// neg: Two's Complement Negation
pub fn neg<I: Int + num_traits::ops::overflowing::OverflowingSub>(x: I) -> I {
    let (result, of) = I::zero().overflowing_sub(&x);
    unsafe {
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE.flags.set(Flags::CF, !result.is_zero());
        MACHINE.flags.set(Flags::OF, of);
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}

pub fn div() {
    todo!("div");
}

pub fn dec<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
) -> I {
    let old_cf = unsafe { MACHINE.flags.contains(Flags::CF) };
    let result = sub(x, I::one());
    unsafe {
        MACHINE.flags.set(Flags::CF, old_cf);
    }
    result
}

pub fn inc<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I) -> I {
    let old_cf = unsafe { MACHINE.flags.contains(Flags::CF) };
    let result = add(x, I::one());
    unsafe {
        MACHINE.flags.set(Flags::CF, old_cf);
    }
    result
}

pub fn imul(x: i32, y: i32) -> i32 {
    let (res, overflow) = x.overflowing_mul(y);
    unsafe {
        MACHINE.flags.set(Flags::CF, overflow);
        MACHINE.flags.set(Flags::OF, overflow);
    }
    res
}

pub fn not() {
    todo!("not");
}

pub fn cdq() {
    todo!("cdq");
}

pub fn bt() {
    todo!("bt");
}

/// xor: Logical Exclusive OR
pub fn xor<I: Int>(x: I, y: I) -> I {
    let result = x ^ y;
    unsafe {
        // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is undefined.
        MACHINE.flags.remove(Flags::OF);
        MACHINE.flags.remove(Flags::CF);
        MACHINE.flags.set(Flags::ZF, result.is_zero());
        MACHINE.flags.set(Flags::SF, result.high_bit().is_one());
        MACHINE
            .flags
            .set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    }
    result
}
