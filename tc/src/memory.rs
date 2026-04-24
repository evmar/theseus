#![allow(unused)]

pub use winapi::kernel32::Mapping;

#[derive(Default)]
/// Memory represents the process memory after loading an executable.
// TODO: this is maybe redundant with runtime's memory, should they be merged?
pub struct Memory {
    pub mappings: winapi::kernel32::Mappings,
    pub bytes: Vec<u8>,
}

impl Memory {
    pub fn alloc(&mut self, name: String, addr: u32, size: u32) {
        let addr = self.mappings.alloc(name, Some(addr), size);
        let len = (addr + size) as usize;
        if len > self.bytes.len() {
            self.bytes.resize(len, 0);
        }
    }

    pub fn put(&mut self, addr: u32, data: &[u8]) {
        self.slice_mut(addr, data.len() as u32)
            .copy_from_slice(data);
    }

    pub fn write<T: zerocopy::IntoBytes + zerocopy::Immutable>(&mut self, addr: u32, val: T) {
        val.write_to_prefix(&mut self.bytes[addr as usize..])
            .unwrap();
    }

    pub fn slice(&self, addr: u32, len: u32) -> &[u8] {
        &self.bytes[addr as usize..][..len as usize]
    }

    pub fn slice_all(&self, addr: u32) -> &[u8] {
        &self.bytes[addr as usize..]
    }

    pub fn slice_mut(&mut self, addr: u32, len: u32) -> &mut [u8] {
        &mut self.bytes[addr as usize..][..len as usize]
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.mappings);
        Ok(())
    }
}
