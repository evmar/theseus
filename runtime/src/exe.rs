use crate::{Cont, Context, Memory, Regs, mapping::Mappings};

pub struct EXEData {
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
    pub init: fn(&mut Regs, &mut Memory, &mut Mappings),
    pub entry_point: Cont,
}
