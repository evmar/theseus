#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod advapi32;
pub mod bitmap_format;
pub mod ddraw;
pub mod dinput;
mod dllexport;
pub mod dsound;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod locked_state;
pub mod msacm32;
pub mod msvcrt;
pub mod ole32;
mod point;
mod ptr;
mod rect;
pub mod shell32;
pub mod trace;
pub mod user32;
pub mod winmm;

/// Functions a program may resolve at runtime with LoadLibrary/GetProcAddress
/// instead of importing statically. The translator reserves a callable address
/// for each of these, so a call through the returned pointer lands somewhere.
///
/// Statically imported functions need no entry here; only add a name when a
/// program is seen looking it up by hand.
pub const DYNAMIC_EXPORTS: &[(&str, &[&str])] = &[
    // The Microsoft C runtime loads user32 on demand to report fatal errors.
    (
        "user32",
        &["MessageBoxA", "GetActiveWindow", "GetLastActivePopup"],
    ),
];

pub use dllexport::{ABIReturn, FromABIParam};
pub use handle::{HANDLE, Handles};
pub use point::POINT;
pub use ptr::Ptr;
pub use rect::RECT;

macro_rules! stub {
    ($arg:expr) => {{
        log::warn!("stub: using {:?}", $arg);
        $arg
    }};
}
use runtime::{CPU, Context, EXEData, Mappings, Memory};
pub(crate) use stub;

pub fn load(exe: &EXEData) -> Context {
    host::init();
    crate::trace::init(&host::trace_spec());

    // Room for the program's image, its heaps and the flat pool games of this
    // era carve out for themselves.
    let memory_size = 256 << 20;
    let mut memory = Memory::leak_new(memory_size);

    kernel32::init_state(exe.image_base, exe.resources.clone());

    let mut mappings = Mappings::default();
    (exe.init)(&mut memory, &mut mappings);

    let mut lock = kernel32::lock();
    let mut ctx = Context {
        cpu: CPU::default(),
        thread_handle: lock.objects.add(kernel32::Object::Thread).to_raw(),
        thread_id: 1,
        memory,
        blocks: exe.blocks,
        cache: Default::default(),
        recent: [Context::return_from_x86; 4],
    };
    lock.init_process(&mut ctx);
    ctx
}

pub fn start(ctx: &mut Context, exe: &EXEData) {
    assert!(!ctx.cpu.real_mode);
    ctx.call32_x86(exe.entry_point, vec![]);
    // TODO: per Windows, we need to join any spawned threads here.
}

pub fn run(exe: &EXEData) {
    let mut ctx = load(exe);
    start(&mut ctx, exe);
}
