mod generated;

use runtime::*;
use winapi::*;

pub fn entry_point() {
    runtime::HOST.init(generated::indirect);
    kernel32::init_process();
    generated::init_memory();

    run_loop(Cont(generated::x00401000));
}
