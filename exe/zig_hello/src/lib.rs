mod generated;

use runtime::*;

pub fn entry_point() {
    runtime::HOST.init();
    kernel32::init_process();
    generated::init_memory();

    //init_memory();
    let mut ip = 0x00401000u32;
    loop {
        //println!("ip: {ip:#08x}");
        let index = generated::BLOCKS
            .binary_search_by_key(&ip, |(ip, _)| *ip)
            .unwrap();
        let f = generated::BLOCKS[index].1;
        ip = f();
    }
}

#[cfg(feature = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn entry_point_wasm() {
    entry_point();
}
