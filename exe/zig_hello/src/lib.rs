#![no_std]

mod generated;

#[unsafe(no_mangle)]
pub extern "C" fn entry_point() {
    //init_memory();
    generated::x00401000();
}
