mod generated;

use runtime::*;
use winapi::*;

pub fn entry_point() {
    runtime::HOST.init(&generated::BLOCKS);
    kernel32::init_state();
    generated::init_mappings();
    kernel32::init_process();

    run_loop(Cont(generated::x00401000));
}

#[cfg(feature = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn entry_point_wasm() {
    entry_point();
}
