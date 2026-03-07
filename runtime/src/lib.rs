#![allow(static_mut_refs)]

#[cfg(feature = "wasm")]
mod wasm;

use std::io::Write;

use bitflags::bitflags;

pub trait Host {
    fn panic(&self, msg: &str);
    fn print(&self, text: &[u8]);
}

pub struct NativeHost {}
impl Host for NativeHost {
    fn panic(&self, msg: &str) {
        panic!("{}", msg);
    }

    fn print(&self, text: &[u8]) {
        std::io::stdout().write_all(text).unwrap();
    }
}

pub static HOST: NativeHost = NativeHost {};

#[repr(C)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,

    pub flags: Flags,
}

bitflags! {
    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Flags: u32 {
        /// carry
        const CF = 1 << 0;
        /// parity
        const PF = 1 << 2;
        /// zero
        const ZF = 1 << 6;
        /// sign
        const SF = 1 << 7;
        /// direction
        const DF = 1 << 10;
        /// overflow
        const OF = 1 << 11;
        /// cpuid
        const ID = 1 << 21;

        // any flag may be set by operations like SAHF
        const ALL = !0;
    }
}

pub static mut REGS: Regs = Regs {
    eax: 0,
    ecx: 0,
    edx: 0,
    ebx: 0,

    esi: 0,
    edi: 0,
    esp: 0x2000,
    ebp: 0x2000,

    flags: Flags::empty(),
};
//const REGS: &mut Regs = unsafe { &mut *(0x1000 as *mut Regs) };

pub fn push(x: u32) {
    unsafe {
        REGS.esp -= 4;
        *(REGS.esp as *mut u32) = x;
    }
}

pub fn pop() -> u32 {
    unsafe {
        let x = *(REGS.esp as *mut u32);
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

// pub(crate) for use in the cmp opcode impl.
pub fn sub<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
) -> I {
    sbb(x, y, false)
}

pub fn jne(from: u32, x: u32) -> u32 {
    unsafe {
        if !REGS.flags.contains(Flags::ZF) {
            return x;
        }
        from
    }
}
