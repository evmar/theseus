mod generated;

use runtime::*;

pub fn entry_point() {
    runtime::HOST.init(generated::indirect);
    kernel32::init_process();
    generated::init_memory();

    run_loop(Cont(generated::x004018bf));
}

fn main() {
    entry_point();
}
