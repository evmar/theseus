mod externs;
mod generated;

pub fn entry_point() {
    winapi::run(&generated::EXEDATA);
}

fn main() {
    entry_point();
}
