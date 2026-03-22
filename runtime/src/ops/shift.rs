use super::int::Int;
use crate::Flags;

pub fn shl<I: Int + num_traits::WrappingShl>(x: I, y: u8, flags: &mut Flags) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }

    // Carry is the highest bit that will be shifted out.
    let cf = (x >> (I::bits() - y as usize) & I::one()).is_one();
    let val = x.wrapping_shl(y as u32);
    flags.set(Flags::CF, cf);
    let msb = val.high_bit().is_one();
    flags.set(Flags::SF, msb);
    // Note: OF only defined for 1-bit rotates.
    // "For left shifts, the OF flag is set to 0 if the mostsignificant bit of the result is the
    // same as the CF flag (that is, the top two bits of the original operand were the same) [...]"
    flags.set(
        Flags::OF,
        x.shr(I::bits() - 1).is_one() ^ (x.shr(I::bits() - 2) & I::one()).is_one(),
    );
    flags.set(Flags::ZF, val.is_zero());
    flags.set(Flags::PF, val.low_byte().count_ones() % 2 == 0);

    val
}

pub fn shr<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    // In all modes but 64 it is correct to mask to 32 bits.
    assert!(I::bits() < 64); // 64 not implemented
    let y = y % 32;
    if y == 0 {
        return x; // Don't affect flags.
    }

    let val = x >> y as usize;
    flags.set(Flags::CF, ((x >> (y - 1) as usize) & I::one()).is_one());
    flags.set(Flags::SF, false); // ?
    flags.set(Flags::ZF, val.is_zero());

    // Note: OF state undefined for shifts > 1 bit.
    flags.set(Flags::OF, x.high_bit().is_one());
    flags.set(Flags::PF, val.low_byte().count_ones() % 2 == 0);
    val
}

pub fn sar<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }

    flags.set(Flags::CF, x.shr(y as usize - 1).bitand(I::one()).is_one());
    // Note: OF only defined for 1-bit rotates.
    flags.set(Flags::OF, false);
    // There's a random "u32" type in the num-traits signed_shr signature, so cast here.
    let result = x.signed_shr(y as u32);

    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}
