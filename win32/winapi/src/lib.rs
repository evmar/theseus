#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]

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
pub mod user32;
pub mod winmm;

pub use dllexport::{ABIReturn, FromABIParam};
pub use handle::{HANDLE, Handles};
pub use point::POINT;
pub use rect::RECT;

macro_rules! stub {
    ($arg:expr) => {{
        log::warn!("{}:{}: stub: returning {:?}", file!(), line!(), $arg);
        $arg
    }};
}
use runtime::{MACHINE, Machine};
pub(crate) use stub;

pub struct EXEData {
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub blocks: &'static [(u32, fn(&mut Machine) -> runtime::Cont)],
    pub init_mappings: fn(&mut Machine),
    pub entry_point: runtime::Cont,
}

pub fn run(exe: &EXEData) {
    let m = unsafe { &mut MACHINE };
    use runtime::Host;
    runtime::HOST.init(exe.blocks);
    kernel32::init_state(exe.image_base, exe.resources.clone());
    (exe.init_mappings)(m);
    kernel32::init_process(m);

    runtime::push(m, 0xf000_0000); // return_from_main
    runtime::run_loop(m, exe.entry_point);
}
