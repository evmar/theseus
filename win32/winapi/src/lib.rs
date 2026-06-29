#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod advapi32;
pub mod bitmap_format;
pub mod ddraw;
mod dllexport;
pub mod dsound;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod locked_state;
pub mod msvcrt;
mod point;
mod ptr;
mod rect;
pub mod shell32;
pub mod trace;
pub mod user32;
pub mod winmm;

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
use runtime::{CPU, Context, EXEData, Memory};
pub(crate) use stub;

#[cfg(target_family = "wasm")]
fn thesesus_trace() -> String {
    "+".into()
}

#[cfg(not(target_family = "wasm"))]
fn thesesus_trace() -> String {
    std::env::var("THESEUS_TRACE").unwrap_or_default()
}

pub fn load(exe: &EXEData) -> Context {
    host::init();
    crate::trace::init(&thesesus_trace());
    assert_eq!(exe.bitness, 32);

    let memory_size = 32 << 20;
    let memory = Memory::leak_new(memory_size);

    kernel32::init_state(exe.image_base, exe.resources.clone());
    let mut lock = kernel32::lock();

    let mut ctx = Context {
        cpu: CPU::default(),
        thread_handle: lock.objects.add(kernel32::Object::Thread).to_raw(),
        thread_id: 1,
        memory,
        blocks: exe.blocks,
        recent: [Context::return_from_x86; 4],
    };

    (exe.init)(&mut ctx, &mut lock.mappings);
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
