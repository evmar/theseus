mod generated;

use runtime::*;
use winapi::*;

pub fn entry_point() {
    runtime::HOST.init(&generated::BLOCKS);
    kernel32::init_state(generated::RESOURCES);
    generated::init_mappings();
    kernel32::init_process();

    run_loop(Cont(generated::x004018bf));
}

fn main() {
    entry_point();
}
