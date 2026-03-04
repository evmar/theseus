#![allow(unused)]

pub struct Mapping {
    pub name: String,
    pub addr: AddrAbs,
    pub len: u32,
}

impl Mapping {
    pub fn contains(&self, addr: AddrAbs) -> bool {
        let addr = addr.0;
        (self.addr.0..self.addr.0 + self.len).contains(&addr)
    }
}

impl std::fmt::LowerHex for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {:#x} ({} bytes)", self.name, self.addr, self.len)
    }
}

#[derive(Default)]
pub struct Memory {
    pub mappings: Vec<Mapping>,
    pub data: Vec<u8>,
}

impl Memory {
    pub fn alloc(&mut self, name: String, addr: AddrAbs, size: u32) {
        self.mappings.push(Mapping {
            name,
            addr,
            len: size,
        });
        let len = (addr.0 + size) as usize;
        if len > self.data.len() {
            self.data.resize(len, 0);
        }
    }

    pub fn find(&self, addr: AddrAbs) -> &Mapping {
        self.mappings.iter().find(|m| m.contains(addr)).unwrap()
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
        for m in &self.mappings {
            writeln!(f, "{:#x}", m)?;
        }
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

#[derive(Clone, Copy)]
pub struct AddrImage(pub u32);
impl AddrImage {
    pub fn to_abs(self, image_base: AddrAbs) -> AddrAbs {
        AddrAbs(image_base.0 + self.0)
    }
}
