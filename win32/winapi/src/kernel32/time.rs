use runtime::Context;

#[win32_derive::dllexport]
pub fn GetTickCount(_ctx: &mut Context) -> u32 {
    host::host().time()
}

#[win32_derive::dllexport]
pub fn Sleep(_ctx: &mut Context, dwMilliseconds: u32) {
    std::thread::sleep(std::time::Duration::from_millis(dwMilliseconds as u64));
}

#[win32_derive::dllexport]
pub fn GetTimeZoneInformation(ctx: &mut Context, lpTimeZoneInformation: crate::Ptr<u8>) -> u32 /* TIME_ZONE_ID */
{
    // TIME_ZONE_INFORMATION is 172 bytes; report UTC by zeroing it.
    ctx.memory[lpTimeZoneInformation.addr..][..172].fill(0);
    0 // TIME_ZONE_ID_UNKNOWN
}

#[win32_derive::dllexport]
pub fn FileTimeToLocalFileTime(
    ctx: &mut Context,
    lpFileTime: crate::Ptr<u64>,
    lpLocalFileTime: crate::Ptr<u64>,
) -> bool {
    let time = lpFileTime.read(&ctx.memory).unwrap_or(0);
    lpLocalFileTime.write(&mut ctx.memory, time).is_some()
}

#[win32_derive::dllexport]
pub fn FileTimeToSystemTime(
    ctx: &mut Context,
    _lpFileTime: crate::Ptr<u64>,
    lpSystemTime: crate::Ptr<u8>,
) -> bool {
    // SYSTEMTIME is 16 bytes; the game only shows these values incidentally.
    ctx.memory[lpSystemTime.addr..][..16].fill(0);
    true
}
