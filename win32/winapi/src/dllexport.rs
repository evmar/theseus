pub struct ABIReturn(u32);

impl ABIReturn {
    pub fn to_abi_return(&self) -> u32 {
        self.0
    }
}

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

pub trait FromABIParam {
    fn from_abi(val: u32) -> Self;
}

impl FromABIParam for u32 {
    fn from_abi(val: u32) -> Self {
        val
    }
}

impl FromABIParam for i32 {
    fn from_abi(val: u32) -> Self {
        val as i32
    }
}

impl<T: TryFrom<u32>> FromABIParam for Result<T, u32> {
    fn from_abi(val: u32) -> Self {
        T::try_from(val).map_err(|_| val)
    }
}
