use widestring::U16String;
use zerocopy::{FromBytes, IntoBytes};

/// Memory represents the inner machine's memory, as a flat byte array (no paging etc.).
///
/// It is unsafely mutably shared across multiple threads.  In principle any mangling
/// that multi-threaded access can do could just as well be done by single-threaded code,
/// since it is fully under the control of the target executable.
///
/// TODO: to support this unsafe sharing we also use a static lifetime, because otherwise
/// we need to figure out how to guarantee the memory outlives the threads.
/// Maybe by sticking a std::thread::scope in some outer structure?
pub struct Memory {
    pub bytes: &'static mut [u8],
}

impl Memory {
    pub fn new(bytes: &'static mut [u8]) -> Self {
        Memory { bytes }
    }

    pub fn unsafe_clone(&mut self) -> Memory {
        Memory {
            bytes: unsafe {
                std::slice::from_raw_parts_mut(self.bytes.as_mut_ptr(), self.bytes.len())
            },
        }
    }

    #[inline(never)]
    fn null_ptr(&self) {
        log::error!("null page read/write");
    }

    pub fn read<T: FromBytes>(&self, addr: u32) -> T {
        if addr < 0x1000 {
            self.null_ptr();
        }
        let addr = addr as usize;
        T::read_from_bytes(&self.bytes[addr..addr + std::mem::size_of::<T>()]).unwrap()
    }

    pub fn write<T: IntoBytes + zerocopy::Immutable>(&mut self, addr: u32, val: T) {
        if addr < 0x1000 {
            self.null_ptr();
        }
        let addr = addr as usize;
        val.write_to(&mut self.bytes[addr..addr + std::mem::size_of::<T>()])
            .unwrap();
    }

    pub fn read_str(&self, addr: u32) -> &str {
        if addr < 0x1000 {
            self.null_ptr();
        }
        let buf = &self.bytes[addr as usize..];
        let nul = buf.iter().position(|&c| c == 0).unwrap();
        let buf = &buf[..nul];
        std::str::from_utf8(buf).unwrap()
    }

    /// This returns an allocated string rather than a reference due to alignment.
    pub fn read_wstr(&self, addr: u32) -> U16String {
        if addr < 0x1000 {
            self.null_ptr();
        }
        let buf = &self.bytes[addr as usize..];
        let mut str: Vec<u16> = vec![];
        for chunk in buf.chunks_exact(2) {
            if chunk == &[0, 0] {
                break;
            }
            str.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
        U16String::from_vec(str)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr()
    }
}

impl std::ops::Index<u32> for Memory {
    type Output = u8;

    fn index(&self, index: u32) -> &Self::Output {
        if index < 0x1000 {
            log::error!("null index");
        }
        &self.bytes[index as usize]
    }
}

impl std::ops::IndexMut<u32> for Memory {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        if index < 0x1000 {
            log::error!("null index");
        }
        &mut self.bytes[index as usize]
    }
}

impl std::ops::Index<std::ops::RangeFrom<u32>> for Memory {
    type Output = [u8];

    fn index(&self, index: std::ops::RangeFrom<u32>) -> &Self::Output {
        if index.start < 0x1000 {
            log::error!("null index");
        }
        &self.bytes[index.start as usize..]
    }
}

impl std::ops::IndexMut<std::ops::RangeFrom<u32>> for Memory {
    fn index_mut(&mut self, index: std::ops::RangeFrom<u32>) -> &mut Self::Output {
        if index.start < 0x1000 {
            log::error!("null index");
        }
        &mut self.bytes[index.start as usize..]
    }
}

impl std::ops::Index<std::ops::Range<u32>> for Memory {
    type Output = [u8];

    fn index(&self, index: std::ops::Range<u32>) -> &Self::Output {
        if index.start < 0x1000 {
            log::error!("null index");
        }
        &self.bytes[index.start as usize..index.end as usize]
    }
}

impl std::ops::IndexMut<std::ops::Range<u32>> for Memory {
    fn index_mut(&mut self, index: std::ops::Range<u32>) -> &mut Self::Output {
        if index.start < 0x1000 {
            log::error!("null index");
        }
        &mut self.bytes[index.start as usize..index.end as usize]
    }
}
