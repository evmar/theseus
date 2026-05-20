use std::marker::PhantomData;

use crate::{FromABIParam, Memory};

/// Ptr represents a (possibly-unaligned) pointer into win32 memory for a given type `T`,
/// wrapping the zerocopy read/write operations.
pub struct Ptr<T> {
    pub addr: u32,
    _phantom: PhantomData<T>,
}

impl<T> Ptr<T> {
    pub fn new(addr: u32) -> Self {
        Self {
            addr,
            _phantom: PhantomData,
        }
    }

    pub fn advance(&mut self) {
        self.addr += std::mem::size_of::<T>() as u32;
    }
}

impl<T: zerocopy::FromBytes> Ptr<T> {
    pub fn read(&self, memory: &Memory) -> Option<T> {
        if self.addr < 0x1000 {
            memory.null_ptr();
            return None;
        }
        let bytes = &memory[self.addr..][..std::mem::size_of::<T>()];
        Some(<T>::read_from_bytes(bytes).unwrap())
    }
}

impl<T: zerocopy::FromBytes + zerocopy::Immutable + zerocopy::KnownLayout> Ptr<T> {
    pub fn aligned_ref<'a>(&self, memory: &'a Memory) -> &'a T {
        <T>::ref_from_bytes(&memory[self.addr..][..std::mem::size_of::<T>()]).unwrap()
    }
}

impl<T: zerocopy::IntoBytes + zerocopy::Immutable> Ptr<T> {
    pub fn write(&self, memory: &mut Memory, value: T) -> Option<()> {
        if self.addr < 0x1000 {
            memory.null_ptr();
            return None;
        }
        let bytes = &mut memory[self.addr..][..std::mem::size_of::<T>()];
        value.write_to(bytes).unwrap();
        Some(())
    }
}

impl<T: zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::Immutable + zerocopy::KnownLayout>
    Ptr<T>
{
    pub fn aligned_mut<'a>(&self, memory: &'a mut Memory) -> &'a mut T {
        <T>::mut_from_bytes(&mut memory[self.addr..][..std::mem::size_of::<T>()]).unwrap()
    }
}

impl<T> std::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.addr)
    }
}

impl<T> FromABIParam for Ptr<T> {
    fn from_abi(val: u32) -> Self {
        Self::new(val)
    }
}
