use runtime::segofs;

use crate::{Module, memory::Memory};

pub fn load_com(mem: &mut Memory, buf: Vec<u8>) -> Module {
    let addr = segofs(dos::DOSBOX_SEG, 0x100);
    mem.reserve("com".into(), addr, buf.len() as u32);
    mem.bytes[addr as usize..].copy_from_slice(&buf);
    Module {
        bitness: 16,
        code_segment: Some(dos::DOSBOX_SEG),
        image_base: 0x100,
        entry_point: 0x100,
        code_memory: 0..0,
        resources: None,
        imports: Vec::new(),
        vtables: Vec::new(),
    }
}
