use crate::kernel32::state;

#[derive(Debug, Default)]
pub struct Mapping {
    #[allow(unused)]
    pub desc: String,
    pub addr: u32,
    pub size: u32,
}

pub fn alloc_mapping(desc: String, size: u32) -> u32 {
    let mut new_mapping = Mapping {
        desc,
        addr: 0,
        size,
    };

    let mut mappings = state().mappings.borrow_mut();
    let mut prev_end = 0;
    for (i, mapping) in mappings.iter().enumerate() {
        let space = mapping.addr - prev_end;
        if space >= size {
            new_mapping.addr = prev_end;
            mappings.insert(i, new_mapping);
            return prev_end;
        }
        prev_end = mapping.addr + mapping.size;
    }
    new_mapping.addr = prev_end;
    mappings.push(new_mapping);
    return prev_end;
}

pub fn dump_mappings() {
    let mappings = state().mappings.borrow();
    println!("{:#x?}", mappings);
}
