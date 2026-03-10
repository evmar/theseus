mod generated;

use runtime::*;
use winapi::*;

pub fn entry_point() {
    runtime::HOST.init(&generated::BLOCKS);
    kernel32::init_process();
    generated::init_memory();

    run_loop(Cont(generated::x004018bf));
}

fn main() {
    entry_point();
}
