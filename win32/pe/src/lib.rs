mod exports;
mod file;
mod imports;
pub mod parse;
mod relocations;
mod resources;

pub use exports::*;
pub use file::*;
pub use imports::*;
pub use relocations::*;
pub use resources::*;

/// Read a C-style nul terminated string from a buffer.
/// Various PE structures use these, sometimes with an optional nul.
pub fn c_str(buf: &[u8]) -> &[u8] {
    let len = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
    &buf[..len]
}

pub struct PodIterator<'m, T: zerocopy::FromBytes> {
    buf: &'m [u8],
    _marker: std::marker::PhantomData<&'m T>,
}

impl<'m, T: zerocopy::FromBytes> std::iter::Iterator for PodIterator<'m, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < size_of::<T>() {
            return None;
        }
        let (obj, buf) = <T>::read_from_prefix(self.buf).unwrap();
        self.buf = buf;
        Some(obj)
    }
}

pub fn iter_pod<'a, T: zerocopy::FromBytes>(memory: &'a [u8]) -> PodIterator<'a, T> {
    PodIterator {
        buf: &memory,
        _marker: std::marker::PhantomData,
    }
}

pub fn iter_pod_n<'a, T: zerocopy::FromBytes>(
    memory: &'a [u8],
    addr: u32,
    count: u32,
) -> PodIterator<'a, T> {
    iter_pod(&memory[addr as usize..][..(count as usize * size_of::<T>())])
}
