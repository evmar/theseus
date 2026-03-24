pub trait Int: num_traits::PrimInt {
    fn bits() -> usize;
    fn low_byte(&self) -> u8;

    fn high_bit(&self) -> Self {
        *self >> (Self::bits() - 1)
    }
}

impl Int for u64 {
    fn bits() -> usize {
        64
    }
    fn low_byte(&self) -> u8 {
        *self as u8
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

impl Int for u16 {
    fn bits() -> usize {
        16
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
