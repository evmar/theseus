#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

pub mod ddraw;
pub mod gdi32;
pub mod kernel32;
pub mod user32;

pub struct ABIReturn(u32);

impl From<u32> for ABIReturn {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for ABIReturn {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl From<u16> for ABIReturn {
    fn from(value: u16) -> Self {
        Self(value as u32)
    }
}

impl From<bool> for ABIReturn {
    fn from(value: bool) -> Self {
        Self(if value { 1 } else { 0 })
    }
}

impl From<()> for ABIReturn {
    fn from(_: ()) -> Self {
        Self(0)
    }
}
