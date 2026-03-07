mod generated;

use runtime::*;

pub fn entry_point() {
    runtime::HOST.init();
    kernel32::init_process();
    generated::init_memory();

    //init_memory();
    let mut f: fn() -> Cont = generated::x00401000;
    loop {
        f = f().0;
    }
}

#[cfg(feature = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn entry_point_wasm() {
    entry_point();
}
