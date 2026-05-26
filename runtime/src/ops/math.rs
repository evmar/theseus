use super::int::Int;
use crate::Flags;

fn sbb_impl<I: Int>(x: I, y: I, b: bool, flags: &mut Flags) -> I {
    let z = if b { y.wrapping_add(&I::one()) } else { y };
    let (result, borrow) = x.overflowing_sub(&z);
    flags.set(Flags::CF, borrow || (b && z == I::zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1  (pos + pos => negative)
    //   1  0  0  (neg - pos => positive)
    let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn sbb<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    sbb_impl(x, y, flags.contains(Flags::CF), flags)
}

pub fn sub<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    sbb_impl(x, y, false, flags)
}

pub fn add<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    addc(x, y, I::zero(), flags)
}

pub fn addc<I: Int>(x: I, y: I, z: I, flags: &mut Flags) -> I {
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
pub fn neg<I: Int>(x: I, flags: &mut Flags) -> I {
    let (result, of) = I::zero().overflowing_sub(&x);
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::CF, !result.is_zero());
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn dec<I: Int>(x: I, flags: &mut Flags) -> I {
    let old_cf = flags.contains(Flags::CF);
    let result = sub(x, I::one(), flags);
    flags.set(Flags::CF, old_cf);
    result
}

pub fn inc<I: Int>(x: I, flags: &mut Flags) -> I {
    let old_cf = flags.contains(Flags::CF);
    let result = add(x, I::one(), flags);
    flags.set(Flags::CF, old_cf);
    result
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

/// Shared impl of div_rmXX. The caller passes a doubled-width dividend and
/// a widened divisor and receives quotient and remainder separately.
pub fn div<I: Int>(x: I, y: I) -> (I, I) {
    (x / y, x % y)
}

/// One-argument imul, 32 bit inputs.
pub fn imul1_32(x: u32, y: u32, flags: &mut Flags) -> u64 {
    let x = x as i32;
    let y = y as i32;
    let res = (x as i64 * y as i64) as u64;
    let flag = res != (res as u32 as i32 as i64 as u64);
    flags.set(Flags::CF, flag);
    flags.set(Flags::OF, flag);
    res
}

/// One-argument imul, 16 bit inputs.
pub fn imul1_16(x: u16, y: u16, flags: &mut Flags) -> u32 {
    let x = x as i16;
    let y = y as i16;
    let res = (x as i32 * y as i32) as u32;
    let flag = res != (res as u16 as i16 as i32 as u32);
    flags.set(Flags::CF, flag);
    flags.set(Flags::OF, flag);
    res
}

/// One-argument imul, 8 bit inputs.
pub fn imul1_8(x: u8, y: u8, flags: &mut Flags) -> u16 {
    let x = x as i8;
    let y = y as i8;
    let res = (x as i16 * y as i16) as u16;
    let flag = res != (res as u8 as i8 as i16 as u16);
    flags.set(Flags::CF, flag);
    flags.set(Flags::OF, flag);
    res
}

/// Two or three-argument imul, 32 bit inputs.
pub fn imul2_32(x: u32, y: u32, flags: &mut Flags) -> u32 {
    let x = x as i32;
    let y = y as i32;
    let (res, overflow) = x.overflowing_mul(y);
    flags.set(Flags::CF, overflow);
    flags.set(Flags::OF, overflow);
    res as u32
}

/// Two or three-argument imul, 16 bit inputs.
pub fn imul2_16(x: u16, y: u16, flags: &mut Flags) -> u16 {
    let x = x as i16;
    let y = y as i16;
    let (res, overflow) = x.overflowing_mul(y);
    flags.set(Flags::CF, overflow);
    flags.set(Flags::OF, overflow);
    res as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sbb8(x: u8, y: u8, carry: bool) -> (u8, Flags) {
        let mut flags = Flags::default();
        flags.set(Flags::CF, carry);
        let result = sbb(x, y, &mut flags);
        (result, flags)
    }

    #[test]
    fn sbb_edge_cases() {
        let (result, flags) = sbb8(0x00, 0x7f, true);
        assert_eq!(result, 0x80);
        assert_eq!(flags.to_string(), "CF SF");

        let (result, flags) = sbb8(0x80, 0x00, true);
        assert_eq!(result, 0x7f);
        assert_eq!(flags.to_string(), "OF");

        let (result, flags) = sbb8(0x00, 0xff, true);
        assert_eq!(result, 0x00);
        assert_eq!(flags.to_string(), "CF PF ZF");

        let (result, flags) = sbb8(0x01, 0x00, true);
        assert_eq!(result, 0x00);
        assert_eq!(flags.to_string(), "PF ZF");
    }
}
