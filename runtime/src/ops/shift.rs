use super::int::Int;
use crate::Flags;

pub fn shl<I: Int + num_traits::WrappingShl>(x: I, y: u8, flags: &mut Flags) -> I {
    assert!(I::bits() < 64);
    let y = y % 32;
    if y == 0 {
        return x;
    }

    // Carry is the highest bit that will be shifted out.
    let cf = (x >> (I::bits() - y as usize) & I::one()).is_one();
    let val = x << y as usize;
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

pub fn shld(x: u32, y: u32, count: u8, flags: &mut Flags) -> u32 {
    let count = count % 32;
    if count == 0 {
        return x;
    }
    // "CF flag is filled with the last bit shifted out of the destination operand"
    flags.set(Flags::CF, ((x >> (32 - count)) & 1) != 0);
    if count == 1 {
        // "OF flag is set if a sign change occurred"
        flags.set(Flags::OF, (x >> 31) != ((x >> 30) & 1));
    }
    let result = (x << count) | (y >> (32 - count));
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

pub fn shr<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    assert!(I::bits() < 64);
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
    assert!(I::bits() < 64);
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
