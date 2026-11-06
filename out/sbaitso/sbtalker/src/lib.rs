use runtime::Context;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod blaster;
mod externs;
mod generated;
mod sbtalker;
#[cfg(target_family = "wasm")]
mod wasm;

pub use sbtalker::SBTalker;

pub fn main() -> Context {
    // let drv = include_bytes!("../../../../scratch/dos/sbaitso/BLASTER.DRV").as_slice();
    let mut ctx = dos::load(&generated::EXEDATA, Some(" /dBLASTER"));
    dos::state().read_file = Some(Box::new(|path| {
        if path == "BLASTER.DRV" {
            Some(vec![])
        } else {
            None
        }
    }));
    dos::start(&mut ctx, &generated::EXEDATA);
    ctx
}

#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn init() -> wasm::SBTalker {
    wasm::SBTalker::init(main())
}
