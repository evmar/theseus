use wasm_bindgen::prelude::*;

mod generated;

#[wasm_bindgen]
pub fn main() {
    winapi::run(&generated::EXEDATA);
}
