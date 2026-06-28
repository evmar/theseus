mod externs;
mod generated;

fn main() {
    winapi::run(&generated::EXEDATA);
}
