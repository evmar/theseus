use crate::kernel32::state;

#[derive(Debug, Default, Clone)]
pub struct Mapping {
    #[allow(unused)]
    pub desc: String,
    pub addr: u32,
    pub size: u32,
}

pub fn alloc_mapping(desc: String, addr: u32, size: u32) -> u32 {
    dump_mappings();
    let mut new_mapping = Mapping { desc, addr, size };

    let mut mappings = state().mappings.borrow_mut();
    let mut prev_end = 0;
    for (i, mapping) in mappings.iter().enumerate() {
        let space = mapping.addr - prev_end;
        if new_mapping.addr != 0 {
            if mapping.addr >= new_mapping.addr + new_mapping.size {
                if space < new_mapping.size {
                    panic!("no space for {new_mapping:#x?}");
                }
                mappings.insert(i, new_mapping);
                return prev_end;
            }
        } else {
            if space >= size {
                new_mapping.addr = prev_end;
                mappings.insert(i, new_mapping);
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
    mappings.push(new_mapping);
    return addr;
}

pub fn dump_mappings() {
    let mappings = state().mappings.borrow();
    println!("{:#x?}", mappings);
}
