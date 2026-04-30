#[derive(Default, Clone)]
pub struct Mapping {
    pub desc: String,
    pub addr: u32,
    pub size: u32,
    // If true, created from a file section (not dynamically created)
    pub section: bool,
}

impl Mapping {
    pub fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }
    pub fn contains(&self, addr: u32) -> bool {
        self.range().contains(&addr)
    }
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x} {} ({:#x} bytes)",
            self.addr, self.desc, self.size
        )
    }
}

#[derive(Debug, Default)]
pub struct Mappings {
    mappings: Vec<Mapping>,
}

pub enum MappingData<'a> {
    Bytes(&'a [u8]),
    U32s(&'a [(u32, u32)]),
}

pub fn round_to_page(size: u32) -> u32 {
    (size + 0x1000 - 1) & !(0x1000 - 1)
}

impl Mappings {
    pub fn reserve(&mut self, mut mapping: Mapping) -> u32 {
        let index = self.insert_index(&mut mapping);
        let addr = mapping.addr;
        self.mappings.insert(index, mapping);
        return addr;
    }

    pub fn alloc(&mut self, desc: String, size: u32) -> u32 {
        let size = round_to_page(size);
        let mut new_mapping = Mapping {
            desc,
            addr: 0,
            section: false,
            size,
        };

        let index = self.insert_index(&mut new_mapping);
        let addr = new_mapping.addr;
        self.mappings.insert(index, new_mapping);
        return addr;
    }

    /// Choose the index into self.mappings to add this mapping, potentially assigning it an address.
    fn insert_index(&self, new_mapping: &mut Mapping) -> usize {
        let mut prev_end = 0;
        for (i, mapping) in self.mappings.iter().enumerate() {
            let space = mapping.addr - prev_end;
            if new_mapping.addr != 0 {
                if mapping.addr >= new_mapping.addr + new_mapping.size {
                    if space < new_mapping.size {
                        panic!("no space for {new_mapping:#x?}");
                    }
                    return i;
                }
            } else {
                if space >= new_mapping.size {
                    new_mapping.addr = prev_end;
                    return i;
                }
            }
            prev_end = mapping.addr + mapping.size;
        }
        if new_mapping.addr != 0 {
            assert!(new_mapping.addr >= prev_end);
        } else {
            new_mapping.addr = prev_end;
        }
        self.mappings.len()
    }

    pub fn dump(&self) {
        println!("{:#x?}", self.mappings);
    }

    pub fn vec(&self) -> &Vec<Mapping> {
        &self.mappings
    }

    pub fn from(mappings: Vec<Mapping>) -> Self {
        Self { mappings }
    }
}
