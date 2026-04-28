use runtime::Context;

/// GetTickCount wants the ticks since the process started.
/// To make this a simple read, we unsafely store the current instant during initialization.
/// Note that this is never mutated after process startup, so it is correctly Sync
/// after that point.
pub struct UnsafeTickCount(std::time::Instant);
unsafe impl Sync for UnsafeTickCount {}

impl UnsafeTickCount {
    pub const fn new_uninitialized() -> Self {
        unsafe { UnsafeTickCount(std::mem::MaybeUninit::zeroed().assume_init()) }
    }

    pub fn init() {
        unsafe {
            START_TICK_COUNT.0 = std::time::Instant::now();
        }
    }

    pub fn get() -> std::time::Instant {
        unsafe { START_TICK_COUNT.0 }
    }
}
static mut START_TICK_COUNT: UnsafeTickCount = UnsafeTickCount::new_uninitialized();

pub fn get_tick_count() -> u32 {
    UnsafeTickCount::get().elapsed().as_millis() as u32
}

#[win32_derive::dllexport]
pub fn GetTickCount(_ctx: &mut Context) -> u32 {
    get_tick_count()
}

#[win32_derive::dllexport]
pub fn Sleep(_ctx: &mut Context, dwMilliseconds: u32) {
    std::thread::sleep(std::time::Duration::from_millis(dwMilliseconds as u64));
}
