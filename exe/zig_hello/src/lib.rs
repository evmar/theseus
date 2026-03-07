#![no_std]

mod generated;

#[unsafe(no_mangle)]
pub extern "C" fn entry_point() {
    //init_memory();
    let mut ip = 0x00401000u32;
    loop {
        let index = generated::BLOCKS
            .binary_search_by_key(&ip, |(ip, _)| *ip)
            .unwrap();
        let f = generated::BLOCKS[index].1;
        ip = f();
    }
}
