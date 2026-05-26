use crate::{Module, memory::Memory};

pub fn load_com(mem: &mut Memory, buf: Vec<u8>) -> Module {
    mem.reserve("com".into(), 0x100, buf.len() as u32);
    mem.bytes[0x100..].copy_from_slice(&buf);
    Module {
        bitness: 16,
        image_base: 0x100,
        entry_point: 0x100,
        code_memory: 0..0,
        resources: None,
        imports: Vec::new(),
        vtables: Vec::new(),
    }
}
