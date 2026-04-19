use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

use crate::{HANDLE, handle::Handles};

mod bitmap;
mod dc;
mod object;

pub use bitmap::*;
pub use dc::*;
pub use object::*;

pub type HDC = HANDLE;
pub type HGDIOBJ = HANDLE;

pub struct State {
    pub dcs: RefCell<Handles<DC>>,
    pub objects: RefCell<Handles<Rc<Bitmap>>>,
}

// TODO: reuse locking pattern from kernel32
struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| State {
        dcs: Default::default(),
        objects: Default::default(),
    })
}
