#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod bitmap;
pub mod ddraw;
mod dllexport;
pub mod dsound;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod locked_state;
mod point;
mod rect;
pub mod trace;
pub mod user32;
pub mod winmm;

pub use dllexport::{ABIReturn, FromABIParam};
pub use handle::{HANDLE, Handles};
pub use point::POINT;
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
    pub init_mappings: fn(&mut Context, &mut kernel32::Mappings),
    pub entry_point: runtime::Cont,
}

pub fn run(exe: &EXEData) {
    use runtime::Host;
    runtime::HOST.init();

    crate::trace::init(&std::env::var("THESEUS_TRACE").unwrap_or_default());

    let memory_size = 32 << 10;
    // safety: safe to assume_init on zeroed u8
    let mut memory: Box<[u8]> = unsafe { Box::<[u8]>::new_zeroed_slice(memory_size).assume_init() };
    // safety: see discussion of lifetime in Memory docstring
    let static_memory: &'static mut [u8] = unsafe { std::mem::transmute(memory.as_mut()) };
    let memory = Memory::new(static_memory);

    kernel32::init_state(exe.image_base, exe.resources.clone());

    let mut ctx = Context {
        cpu: CPU::default(),
        thread_id: 1,
        memory,
        blocks: exe.blocks,
        recent: [runtime::return_from_x86; 4],
    };
    let ctx = &mut ctx;
    {
        let mut lock = kernel32::lock();
        (exe.init_mappings)(ctx, &mut lock.mappings);
        kernel32::init_process(ctx, &mut lock);
    }

    runtime::call_x86(ctx, exe.entry_point, vec![]);
    // TODO: per Windows, we need to join any spawned threads here.
}
