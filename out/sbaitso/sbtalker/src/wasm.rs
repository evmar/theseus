use crate::sbtalker;
use runtime::Context;
use wasm_bindgen::prelude::*;

/// SBTalker wrapper that stores ctx for wasm use.
#[wasm_bindgen]
pub struct SBTalker {
    ctx: Context,
    sbtalker: sbtalker::SBTalker,
}

#[wasm_bindgen]
impl SBTalker {
    pub(crate) fn new(mut ctx: Context) -> Self {
        let sbtalker = sbtalker::SBTalker::new(&mut ctx);
        SBTalker { ctx, sbtalker }
    }

    pub fn start(&mut self) {
        super::run(&mut self.ctx);
    }

    pub fn speak(&mut self, text: &str) -> Box<[u8]> {
        self.sbtalker.speak(&mut self.ctx, text).into_boxed_slice()
    }
}
