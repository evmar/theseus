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
use runtime::{CPU, Context, Machine, Memory};
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

    let mut machine = Machine {
        memory: Memory::alloc(32 << 20),
        blocks: exe.blocks,
    };
    let m = &mut machine;
    kernel32::init_state(exe.image_base, exe.resources.clone());

    let mut ctx = Context {
        cpu: CPU::default(),
        thread_id: 1,
        memory: m.memory.unsafe_clone(),
        blocks: m.blocks,
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
