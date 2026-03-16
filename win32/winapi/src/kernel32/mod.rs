mod dll;
mod env;
mod file;
mod heap;
mod mapping;
mod misc;
mod nls;
mod process;

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

pub use dll::*;
pub use env::*;
pub use file::*;
pub use heap::*;
pub use mapping::*;
pub use misc::*;
pub use nls::*;
pub use process::*;

use crate::heap::Heap;

pub struct State {
    pub start: std::time::Instant,
    pub mappings: RefCell<Mappings>,
    pub heaps: RefCell<HashMap<u32, Rc<Heap>>>,
    pub process_heap: RefCell<Rc<Heap>>,
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub command_line: RefCell<CommandLine>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn init_state(image_base: u32, resources: std::ops::Range<u32>) {
    let state = State {
        start: std::time::Instant::now(),
        mappings: Default::default(),
        heaps: Default::default(),
        process_heap: Default::default(),
        image_base,
        resources,
        command_line: Default::default(),
    };
    STATE.0.set(state).unwrap_or_else(|_| panic!());
}

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| panic!())
}
