mod dll;
mod env;
mod file;
mod heap;
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
pub use misc::*;
pub use nls::*;
pub use process::*;

pub struct State {
    heaps: RefCell<HashMap<u32, Rc<()>>>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn init_state() {
    STATE
        .0
        .set(State {
            heaps: Default::default(),
        })
        .unwrap_or_else(|_| panic!());
}

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| panic!())
}
