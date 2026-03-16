use std::cell::{OnceCell, RefCell};

use crate::handle::Handles;

mod bitmap;
mod dc;
mod object;

pub use bitmap::*;
pub use dc::*;
pub use object::*;

pub type HDC = u32;
pub type HGDIOBJ = u32;

pub struct State {
    pub objects: RefCell<Handles<Bitmap>>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| State {
        objects: Default::default(),
    })
}
