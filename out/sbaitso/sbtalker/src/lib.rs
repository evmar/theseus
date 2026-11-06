use runtime::Context;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod blaster;
mod externs;
mod generated;
mod sbtalker;
#[cfg(target_family = "wasm")]
mod wasm;

pub fn init() -> Context {
    // let drv = include_bytes!("../../../../scratch/dos/sbaitso/BLASTER.DRV").as_slice();
    let mut ctx = dos::load(&generated::EXEDATA, Some(" /dBLASTER"));
    dos::state().read_file = Some(Box::new(|path| {
        if path == "BLASTER.DRV" {
            Some(vec![])
        } else {
            None
        }
    }));

    // set return address for once TSR is loaded
    dos::ivt(&mut ctx.memory)[0x22] = (1, 1).into();
    ctx
}

pub fn run(ctx: &mut Context) {
    dos::start(ctx, &generated::EXEDATA);
}

pub fn main() {
    let mut ctx = init();
    run(&mut ctx);
}

#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn new() -> wasm::SBTalker {
    wasm::SBTalker::new(init())
}
