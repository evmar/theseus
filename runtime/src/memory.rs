use zerocopy::{FromBytes, IntoBytes};

pub struct Memory {
    // TODO: this could be a slice owned by Memory, except that
    // we want to store a Machine in a static.
    pub bytes: &'static mut [u8],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { bytes: &mut [] }
    }
}

impl Memory {
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
