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
    flags.set(Flags::SF, (result >> 31) != 0);
    flags.set(Flags::ZF, result == 0);
    result
}

pub fn shrd(x: u32, y: u32, count: u8, flags: &mut Flags) -> u32 {
    let count = count % 32;
    if count == 0 {
        return x;
    }
    flags.set(Flags::CF, ((x >> (count - 1)) & 1) != 0);
    let result = (x >> count) | (y << (32 - count));
    if count == 1 {
        // For a 1-bit shrd, OF is set if the sign bit changed.
        flags.set(Flags::OF, ((x >> 31) & 1) != ((result >> 31) & 1));
    }
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    flags.set(Flags::SF, (result >> 31) != 0);
    flags.set(Flags::ZF, result == 0);
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

pub fn rol<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }
    let result = x.rotate_left(y as u32);
    let carry = (result & I::one()).is_one();
    flags.set(Flags::CF, carry);
    // Note: OF only defined for 1-bit rotates.
    flags.set(Flags::OF, carry ^ (result.high_bit()).is_one());
    result
}

pub fn ror<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }
    let result = x.rotate_right(y as u32);
    flags.set(Flags::CF, result.high_bit().is_one());
    // Note: OF only defined for 1-bit rotates.
    flags.set(
        Flags::OF,
        result.high_bit().is_one() ^ ((result >> (I::bits() - 2)) & I::one()).is_one(),
    );
    result
}

pub fn rcl<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    assert!(I::bits() < 64);
    let y = y % 32;
    let count = y as usize % (I::bits() + 1);
    if count == 0 {
        return x;
    }

    let width = I::bits() + 1;
    let mask = (1u64 << width) - 1;
    let x = (x.to_u64().unwrap() << 1) | u64::from(flags.contains(Flags::CF));
    let x = ((x << count) | (x >> (width - count))) & mask;
    let result = I::from(x >> 1).unwrap();

    flags.set(Flags::CF, (x & 1) != 0);
    // Note: OF only defined for 1-bit rotates.
    flags.set(
        Flags::OF,
        flags.contains(Flags::CF) ^ result.high_bit().is_one(),
    );
    result
}

pub fn rcr<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    assert!(I::bits() < 64);
    let y = y % 32;
    let count = y as usize % (I::bits() + 1);
    if count == 0 {
        return x;
    }

    let bits = I::bits();
    let width = bits + 1;
    let mask = (1u64 << width) - 1;
    let result_mask = (1u64 << bits) - 1;
    let x = x.to_u64().unwrap() | (u64::from(flags.contains(Flags::CF)) << bits);
    let x = ((x >> count) | (x << (width - count))) & mask;
    let result = I::from(x & result_mask).unwrap();

    flags.set(Flags::CF, ((x >> bits) & 1) != 0);
    // Note: OF only defined for 1-bit rotates.
    flags.set(
        Flags::OF,
        result.high_bit().is_one() ^ ((result >> (bits - 2)) & I::one()).is_one(),
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrd() {
        let mut flags = Flags::default();
        assert_eq!(super::shrd(0x8000_0001, 0, 1, &mut flags), 0x4000_0000);
        assert_eq!("CF PF OF", flags.to_string());

        let mut flags = Flags::default();
        assert_eq!(
            super::shrd(0x1234_5678, 0xfedc_ba98, 4, &mut flags),
            0x8123_4567
        );
        assert_eq!("CF SF", flags.to_string());
    }

    #[test]
    fn rcl() {
        let mut flags = Flags::CF;
        assert_eq!(super::rcl(0b1000_0000u8, 1, &mut flags), 0b0000_0001);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::default();
        assert_eq!(super::rcl(0b1010_0001u8, 3, &mut flags), 0b0000_1010);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::rcl(0x1234_5678u32, 32, &mut flags), 0x1234_5678);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::rcl(0x1234u16, 17, &mut flags), 0x1234);
        assert_eq!("CF OF", flags.to_string());
    }

    #[test]
    fn ror() {
        let mut flags = Flags::default();
        assert_eq!(super::ror(0b0000_0001u8, 1, &mut flags), 0b1000_0000);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::ror(0b0000_0010u8, 1, &mut flags), 0b0000_0001);
        assert_eq!("", flags.to_string());

        let mut flags = Flags::default();
        assert_eq!(super::ror(0x1234_5678u32, 4, &mut flags), 0x8123_4567);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::ror(0x1234_5678u32, 32, &mut flags), 0x1234_5678);
        assert_eq!("CF OF", flags.to_string());
    }

    #[test]
    fn rcr() {
        let mut flags = Flags::CF;
        assert_eq!(super::rcr(0b0000_0001u8, 1, &mut flags), 0b1000_0000);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::default();
        assert_eq!(super::rcr(0b1000_0101u8, 3, &mut flags), 0b0101_0000);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::rcr(0x1234_5678u32, 32, &mut flags), 0x1234_5678);
        assert_eq!("CF OF", flags.to_string());

        let mut flags = Flags::CF | Flags::OF;
        assert_eq!(super::rcr(0x1234u16, 17, &mut flags), 0x1234);
        assert_eq!("CF OF", flags.to_string());
    }
}
