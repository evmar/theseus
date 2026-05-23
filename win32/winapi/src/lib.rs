#![cfg_attr(target_family = "wasm", feature(stdarch_wasm_atomic_wait))]
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
mod host;
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
use runtime::{CPU, Context, Memory};
pub(crate) use stub;

pub struct EXEData {
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub blocks: &'static [(u32, fn(&mut Context) -> runtime::Cont)],
    pub init_memory: fn(&mut Context, &mut kernel32::Mappings),
    pub entry_point: runtime::Cont,
}

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

    let memory_size = 32 << 20;
    // safety: safe to assume_init on zeroed u8
    let memory: Box<[u8]> = unsafe { Box::<[u8]>::new_zeroed_slice(memory_size).assume_init() };
    let static_memory: &'static mut [u8] = Box::leak(memory);
    let memory = Memory::new(static_memory);

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

    (exe.init_memory)(&mut ctx, &mut lock.mappings);
    lock.init_process(&mut ctx);
    ctx
}

pub fn start(ctx: &mut Context, exe: &EXEData) {
    ctx.call_x86(exe.entry_point, vec![]);
    // TODO: per Windows, we need to join any spawned threads here.
}

pub fn run(exe: &EXEData) {
    let mut ctx = load(exe);
    start(&mut ctx, exe);
}
