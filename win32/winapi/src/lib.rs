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
    pub bitness: u32,
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

fn alloc_leak_memory(size: usize) -> Memory {
    // safety: safe to assume_init on zeroed u8
    let memory: Box<[u8]> = unsafe { Box::<[u8]>::new_zeroed_slice(size).assume_init() };
    let static_memory: &'static mut [u8] = Box::leak(memory);
    Memory::new(static_memory)
}

pub fn load(exe: &EXEData) -> Context {
    host::init();
    crate::trace::init(&thesesus_trace());
    match exe.bitness {
        16 => load16(exe),
        32 => load32(exe),
        _ => unreachable!(),
    }
}

fn load16(exe: &EXEData) -> Context {
    let memory_size = 64 << 10;
    let memory = alloc_leak_memory(memory_size);

    let mut ctx = Context {
        cpu: CPU::default(),
        thread_handle: 0,
        thread_id: 1,
        memory,
        blocks: exe.blocks,
        recent: [Context::return_from_x86; 4],
    };
    ctx.cpu.real_mode = true;

    let mut mappings = kernel32::Mappings::default();
    (exe.init_memory)(&mut ctx, &mut mappings);

    // initial cx: https://stackoverflow.com/questions/79440940/why-cx-register-already-has-a-non-zero-value-on-startup-of-a-dos-program-unlike
    // this value copied from dosbox
    ctx.cpu.regs.ecx = 0xff;
    ctx.cpu.regs.esp = 0xfffe;
    ctx
}

pub fn start16(ctx: &mut Context, exe: &EXEData) {
    ctx.cpu_loop(exe.entry_point, 0);
    panic!();
}

fn load32(exe: &EXEData) -> Context {
    let memory_size = 32 << 20;
    let memory = alloc_leak_memory(memory_size);

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

pub fn start32(ctx: &mut Context, exe: &EXEData) {
    ctx.call32_x86(exe.entry_point, vec![]);
    // TODO: per Windows, we need to join any spawned threads here.
}

pub fn run(exe: &EXEData) {
    let mut ctx = load(exe);
    if ctx.cpu.real_mode {
        start16(&mut ctx, exe);
    } else {
        start32(&mut ctx, exe);
    }
}
