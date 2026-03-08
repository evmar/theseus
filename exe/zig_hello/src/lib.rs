mod generated;

use runtime::*;
use winapi::*;

pub fn entry_point() {
    runtime::HOST.init(generated::indirect);
    kernel32::init_process();
    generated::init_memory();

    run_loop(Cont(generated::x00401000));
}

#[cfg(feature = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn entry_point_wasm() {
    entry_point();
}
