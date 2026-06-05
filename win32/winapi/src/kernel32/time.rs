use runtime::Context;

#[win32_derive::dllexport]
pub fn GetTickCount(_ctx: &mut Context) -> u32 {
    host::host().time()
}

#[win32_derive::dllexport]
pub fn Sleep(_ctx: &mut Context, dwMilliseconds: u32) {
    std::thread::sleep(std::time::Duration::from_millis(dwMilliseconds as u64));
}
