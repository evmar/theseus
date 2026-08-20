use crate::{Cont, Context, Memory, mapping::Mappings};

pub struct EXEData {
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
    pub init: fn(&mut Memory, &mut Mappings),
    pub entry_point: Cont,
}
