mod generated;

pub fn entry_point() {
    winapi::run(&generated::EXEDATA);
}

#[cfg(feature = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn entry_point_wasm() {
    entry_point();
}
