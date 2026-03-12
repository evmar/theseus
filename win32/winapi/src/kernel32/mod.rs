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
    pub mappings: RefCell<Mappings>,
    pub heaps: RefCell<HashMap<u32, Rc<Heap>>>,
    pub process_heap: Rc<Heap>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn init_state() {
    let mut mappings = Default::default();
    let mut heaps = Default::default();
    let process_heap = heap_create(&mut mappings, &mut heaps, "process heap".into(), 1 << 20);
    let state = State {
        mappings: Default::default(),
        heaps: RefCell::new(heaps),
        process_heap,
    };
    STATE.0.set(state).unwrap_or_else(|_| panic!());
}

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| panic!())
}
