mod generated;

pub fn entry_point() {
    winapi::run(&generated::EXEDATA);
}
