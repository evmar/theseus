use zerocopy::{FromBytes, IntoBytes};

/// Memory represents the inner machine's memory, as a flat byte array (no paging etc.).
///
/// It is unsafely mutably shared across multiple threads.  In principle any mangling
/// that multi-threaded access can do could just as well be done by single-threaded code,
/// since it is fully under the control of the target executable.
///
/// To support this unsafe sharing we also leak the array.
pub struct Memory {
    pub bytes: &'static mut [u8],
}

impl Memory {
    pub fn alloc(size: usize) -> Self {
        let bytes = unsafe {
            // Allocate the memory using manual allocation so we can align it to a page boundary,
            // just to make pointers easier to read.
            let align = 0x1000;
            let mem = std::alloc::alloc(std::alloc::Layout::from_size_align(size, align).unwrap());
            std::slice::from_raw_parts_mut(mem, size)
        };
        Memory { bytes }
    }

    pub fn unsafe_clone(&mut self) -> Memory {
        Memory {
            bytes: unsafe {
                std::slice::from_raw_parts_mut(self.bytes.as_mut_ptr(), self.bytes.len())
            },
        }
    }

    pub fn read<T: FromBytes>(&self, addr: u32) -> T {
        if addr < 0x1000 {
            log::error!("null read");
        }
        T::read_from_prefix(&self.bytes[addr as usize..]).unwrap().0
    }

    pub fn write<T: IntoBytes + zerocopy::Immutable>(&mut self, addr: u32, val: T) {
        if addr < 0x1000 {
            log::error!("null write");
        }
        val.write_to_prefix(&mut self.bytes[addr as usize..])
            .unwrap();
    }

    pub fn read_str(&self, addr: u32) -> &str {
        if addr < 0x1000 {
            log::error!("null read");
        }
        let buf = &self.bytes[addr as usize..];
        let nul = buf.iter().position(|&c| c == 0).unwrap();
        let buf = &buf[..nul];
        std::str::from_utf8(buf).unwrap()
    }

    pub fn slice(&self, r: std::ops::Range<u32>) -> &[u8] {
        if r.start < 0x1000 {
            log::error!("null slice");
        }
        &self.bytes[r.start as usize..r.end as usize]
    }

    pub fn slice_mut(&mut self, r: std::ops::Range<u32>) -> &mut [u8] {
        if r.start < 0x1000 {
            log::error!("null slice");
        }
        &mut self.bytes[r.start as usize..r.end as usize]
    }

    pub fn slice_from(&self, start: u32) -> &[u8] {
        if start < 0x1000 {
            log::error!("null slice");
        }
        &self.bytes[start as usize..]
    }

    pub fn slice_mut_from(&mut self, start: u32) -> &mut [u8] {
        if start < 0x1000 {
            log::error!("null slice");
        }
        &mut self.bytes[start as usize..]
    }
}
