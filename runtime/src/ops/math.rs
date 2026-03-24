use super::int::Int;
use crate::Flags;

fn sbb_impl<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    b: bool,
    flags: &mut Flags,
) -> I {
    let z = if b { y.wrapping_add(&I::one()) } else { y };
    let (result, borrow) = x.overflowing_sub(&z);
    flags.set(Flags::CF, borrow || (b && z == I::zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1
    //   1  0  0
    let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    flags: &mut Flags,
) -> I {
    sbb_impl(x, y, flags.contains(Flags::CF), flags)
}

pub fn sub<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    flags: &mut Flags,
) -> I {
    sbb_impl(x, y, false, flags)
}

pub fn add<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, flags: &mut Flags) -> I {
    addc(x, y, I::zero(), flags)
}

pub fn addc<I: Int + num_traits::ops::wrapping::WrappingAdd>(
    x: I,
    y: I,
    z: I,
    flags: &mut Flags,
) -> I {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let result = x.wrapping_add(&y.wrapping_add(&z));
    flags.set(Flags::CF, result < x || (result == x && !z.is_zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = ((x ^ !y) & (x ^ result)).high_bit().is_one();
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn and<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x & y;
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::OF, false);
    flags.set(Flags::CF, false);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

/// or: Logical Inclusive OR
pub fn or<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x | y;
    flags.remove(Flags::OF | Flags::CF);
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

/// neg: Two's Complement Negation
pub fn neg<I: Int + num_traits::ops::overflowing::OverflowingSub>(x: I, flags: &mut Flags) -> I {
    let (result, of) = I::zero().overflowing_sub(&x);
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::CF, !result.is_zero());
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn div() {
    todo!("div");
}

pub fn dec<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    flags: &mut Flags,
) -> I {
    let old_cf = flags.contains(Flags::CF);
    let result = sub(x, I::one(), flags);
    flags.set(Flags::CF, old_cf);
    result
}

pub fn inc<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, flags: &mut Flags) -> I {
    let old_cf = flags.contains(Flags::CF);
    let result = add(x, I::one(), flags);
    flags.set(Flags::CF, old_cf);
    result
}

pub fn imul(x: i32, y: i32, flags: &mut Flags) -> i32 {
    let (res, overflow) = x.overflowing_mul(y);
    flags.set(Flags::CF, overflow);
    flags.set(Flags::OF, overflow);
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
pub fn xor<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x ^ y;
    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is undefined.
    flags.remove(Flags::OF);
    flags.remove(Flags::CF);
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

/// Shared impl of mul_rmXX.  The trick is to pass in a higher width int,
/// e.g. x as u32 for the 16-bit mul, so there is enough space in the result.
pub fn mul<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let res = x.mul(y);
    let tophalf = res.shr(I::bits() / 2);
    flags.set(Flags::OF, !tophalf.is_zero());
    flags.set(Flags::CF, !tophalf.is_zero());
    res
}
