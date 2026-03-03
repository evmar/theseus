pub struct Mapping {
    pub name: String,
    pub addr: AddrAbs,
    pub data: Box<[u8]>,
}

impl Mapping {
    pub fn contains(&self, addr: AddrAbs) -> bool {
        let addr = addr.0;
        self.addr.0 <= addr && addr < self.addr.0 + self.data.len() as u32
    }
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {:#x} ({} bytes)",
            self.name,
            self.addr,
            self.data.len()
        )
    }
}

#[derive(Default, Debug)]
pub struct Memory {
    regions: Vec<Mapping>,
}

impl Memory {
    pub fn alloc(&mut self, name: String, addr: AddrAbs, size: u32) {
        self.regions.push(Mapping {
            name,
            addr,
            data: vec![0; size as usize].into_boxed_slice(),
        });
    }

    pub fn find(&mut self, addr: AddrAbs) -> &mut Mapping {
        self.regions.iter_mut().find(|m| m.contains(addr)).unwrap()
    }

    pub fn put(&mut self, addr: AddrAbs, data: &[u8]) {
        let m = self.find(addr);
        let ofs = addr.to_mapping(m).0 as usize;
        m.data[ofs..ofs + data.len()].copy_from_slice(data);
    }
}

#[derive(Clone, Copy)]
pub struct AddrAbs(pub u32);
impl AddrAbs {
    pub fn to_mapping(self, mapping: &Mapping) -> AddrMapping {
        AddrMapping(self.0 - mapping.addr.0)
    }
}
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

#[derive(Clone, Copy)]
pub struct AddrMapping(pub u32);
impl AddrMapping {
    pub fn to_abs(self, mapping: &Mapping) -> AddrAbs {
        AddrAbs(mapping.addr.0 + self.0)
    }
}
