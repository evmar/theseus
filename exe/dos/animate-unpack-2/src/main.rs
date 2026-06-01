mod externs;
mod generated;

fn main() {
    let exe = &generated::EXEDATA;
    winapi::run(exe);
}
