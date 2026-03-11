#![allow(unused)]

pub use winapi::kernel32::Mapping;

#[derive(Default)]
pub struct Memory {
    pub mappings: winapi::kernel32::Mappings,
    pub data: Vec<u8>,
}

impl Memory {
    pub fn alloc(&mut self, name: String, addr: AddrAbs, size: u32) {
        let addr = self.mappings.alloc(name, addr.0, size);
        let len = (addr + size) as usize;
        println!("alloc at {:x}", addr);
        if len > self.data.len() {
            self.data.resize(len, 0);
        }
    }

    pub fn put(&mut self, addr: AddrAbs, data: &[u8]) {
        self.slice_mut(addr, data.len() as u32)
            .copy_from_slice(data);
    }

    pub fn slice(&self, addr: AddrAbs, len: u32) -> &[u8] {
        let addr = addr.0 as usize;
        &self.data[addr..addr + len as usize]
    }

    pub fn slice_all(&self, addr: AddrAbs) -> &[u8] {
        let addr = addr.0 as usize;
        &self.data[addr..]
    }

    pub fn slice_mut(&mut self, addr: AddrAbs, len: u32) -> &mut [u8] {
        let addr = addr.0 as usize;
        &mut self.data[addr..addr + len as usize]
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.mappings);
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct AddrAbs(pub u32);
impl std::fmt::LowerHex for AddrAbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl std::fmt::Debug for AddrAbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#08x}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct AddrImage(pub u32);
impl AddrImage {
    pub fn to_abs(self, image_base: AddrAbs) -> AddrAbs {
        AddrAbs(image_base.0 + self.0)
    }
}
