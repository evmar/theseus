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

#[derive(Default)]
pub struct State {
    mappings: RefCell<Mappings>,
    heaps: RefCell<HashMap<u32, Rc<Heap>>>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn init_state() {
    let state = State::default();

    {
        let mut mappings = state.mappings.borrow_mut();
        mappings.alloc("null page".into(), 0, 0x1000);
    }

    STATE.0.set(state).unwrap_or_else(|_| panic!());
}

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| panic!())
}
