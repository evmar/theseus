use runtime::segofs;

use crate::{DOSModule, memory::Memory};

pub fn load_com(mem: &mut Memory, buf: Vec<u8>) -> DOSModule {
    let addr = segofs(dos::DOSBOX_SEG, 0x100);
    mem.reserve("com".into(), addr, buf.len() as u32);
    mem.bytes[addr as usize..].copy_from_slice(&buf);
    DOSModule {
        is_com: true,
        psp_segment: dos::DOSBOX_SEG,
        load_segment: dos::DOSBOX_SEG,
        stack_segment: dos::DOSBOX_SEG,
        stack_pointer: 0xfffe,
        entry_point: 0x100,
        code_memory: (addr..buf.len() as u32),
    }
}
