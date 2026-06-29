use crate::{Cont, Context, mapping::Mappings};

pub struct EXEData {
    pub bitness: u32,
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub blocks: &'static [(u32, fn(&mut Context) -> Cont)],
    pub init: fn(&mut Context, &mut Mappings),
    pub entry_point: Cont,
}
