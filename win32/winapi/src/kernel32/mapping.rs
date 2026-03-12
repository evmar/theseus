#[derive(Default, Clone)]
pub struct Mapping {
    #[allow(unused)]
    pub desc: String,
    pub addr: u32,
    pub size: u32,
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
    pub fn alloc(&mut self, desc: String, addr: u32, size: u32) -> u32 {
        let size = round_to_page(size);
        let mut new_mapping = Mapping { desc, addr, size };

        let mut prev_end = 0;
        for (i, mapping) in self.mappings.iter().enumerate() {
            let space = mapping.addr - prev_end;
            if new_mapping.addr != 0 {
                if mapping.addr >= new_mapping.addr + new_mapping.size {
                    if space < new_mapping.size {
                        panic!("no space for {new_mapping:#x?}");
                    }
                    self.mappings.insert(i, new_mapping);
                    return prev_end;
                }
            } else {
                if space >= size {
                    new_mapping.addr = prev_end;
                    self.mappings.insert(i, new_mapping);
                    return prev_end;
                }
            }
            prev_end = mapping.addr + mapping.size;
        }
        if new_mapping.addr != 0 {
            assert!(new_mapping.addr >= prev_end);
        } else {
            new_mapping.addr = prev_end;
        }
        let addr = new_mapping.addr;
        self.mappings.push(new_mapping);
        return addr;
    }

    pub fn dump(&self) {
        println!("{:#x?}", self.mappings);
    }

    pub fn vec(&self) -> &Vec<Mapping> {
        &self.mappings
    }
}
