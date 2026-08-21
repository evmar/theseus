use widestring::U16String;

/// Memory represents the inner machine's memory, as a flat byte array (no paging etc.).
///
/// It is unsafely mutably shared across multiple threads.  In principle any mangling
/// that multi-threaded access can do could just as well be done by single-threaded code,
/// since it is fully under the control of the target executable.
pub struct Memory<'a> {
    pub bytes: &'a mut [u8],
    /// When true, panic on access to low memory.
    /// TODO: full memory mapping access controls etc.
    pub null_page: bool,
}

/// A trait for types that can be read from memory with .read().
pub trait MemRead: zerocopy::FromBytes {}
impl<T: zerocopy::FromBytes> MemRead for T {}

/// A trait for types that can be written to memory with .write().
pub trait MemWrite: zerocopy::IntoBytes + zerocopy::Immutable {}
impl<T: zerocopy::IntoBytes + zerocopy::Immutable> MemWrite for T {}

impl<'a> Memory<'a> {
    pub fn new(bytes: &'static mut [u8]) -> Self {
        Memory {
            bytes,
            null_page: true,
        }
    }

    pub fn unsafe_clone(&mut self) -> Memory<'a> {
        Memory {
            bytes: unsafe {
                std::slice::from_raw_parts_mut(self.bytes.as_mut_ptr(), self.bytes.len())
            },
            null_page: self.null_page,
        }
    }

    #[inline(never)]
    pub fn null_ptr(&self) {
        log::error!("null page read/write");
    }

    #[inline]
    fn check_access(&self, addr: u32) {
        if addr < 0x1000 && self.null_page {
            self.null_ptr();
        }
    }

    pub fn read<T: MemRead>(&self, addr: u32) -> T {
        self.check_access(addr);
        let addr = addr as usize;
        T::read_from_bytes(&self.bytes[addr..addr + std::mem::size_of::<T>()]).unwrap()
    }

    pub fn write<T: MemWrite>(&mut self, addr: u32, val: T) {
        self.check_access(addr);
        let addr = addr as usize;
        val.write_to(&mut self.bytes[addr..addr + std::mem::size_of::<T>()])
            .unwrap();
    }

    pub fn read_str(&self, addr: u32) -> &str {
        self.check_access(addr);
        let buf = &self.bytes[addr as usize..];
        let nul = buf.iter().position(|&c| c == 0).unwrap();
        let buf = &buf[..nul];
        std::str::from_utf8(buf).unwrap()
    }

    /// This returns an allocated string rather than a reference due to alignment.
    pub fn read_wstr(&self, addr: u32) -> U16String {
        self.check_access(addr);
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

impl Memory<'static> {
    pub fn leak_new(size: usize) -> Self {
        // safety: safe to assume_init on zeroed u8
        let memory: Box<[u8]> = unsafe { Box::<[u8]>::new_zeroed_slice(size).assume_init() };
        let static_memory: &'static mut [u8] = Box::leak(memory);
        Memory::new(static_memory)
    }
}

impl<'a> std::ops::Index<u32> for Memory<'a> {
    type Output = u8;

    fn index(&self, addr: u32) -> &Self::Output {
        self.check_access(addr);
        &self.bytes[addr as usize]
    }
}

impl<'a> std::ops::IndexMut<u32> for Memory<'a> {
    fn index_mut(&mut self, addr: u32) -> &mut Self::Output {
        self.check_access(addr);
        &mut self.bytes[addr as usize]
    }
}

impl<'a> std::ops::Index<std::ops::RangeFrom<u32>> for Memory<'a> {
    type Output = [u8];

    fn index(&self, index: std::ops::RangeFrom<u32>) -> &Self::Output {
        self.check_access(index.start);
        &self.bytes[index.start as usize..]
    }
}

impl<'a> std::ops::IndexMut<std::ops::RangeFrom<u32>> for Memory<'a> {
    fn index_mut(&mut self, index: std::ops::RangeFrom<u32>) -> &mut Self::Output {
        self.check_access(index.start);
        &mut self.bytes[index.start as usize..]
    }
}

impl<'a> std::ops::Index<std::ops::Range<u32>> for Memory<'a> {
    type Output = [u8];

    fn index(&self, index: std::ops::Range<u32>) -> &Self::Output {
        self.check_access(index.start);
        &self.bytes[index.start as usize..index.end as usize]
    }
}

impl<'a> std::ops::IndexMut<std::ops::Range<u32>> for Memory<'a> {
    fn index_mut(&mut self, index: std::ops::Range<u32>) -> &mut Self::Output {
        self.check_access(index.start);
        &mut self.bytes[index.start as usize..index.end as usize]
    }
}
