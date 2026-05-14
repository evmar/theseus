use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use crate::{HANDLE, handle::Handles, locked_state::LockedState};

mod bitmap;
pub use bitmap::*;
mod dc;
pub use dc::*;
mod misc;
pub use misc::*;
mod object;
pub use object::*;

pub type HGDIOBJ = HANDLE;
pub type COLORREF = u32;

#[derive(Default)]
pub struct State {
    pub dcs: RefCell<Handles<DC>>,
    pub objects: RefCell<Handles<Arc<Bitmap>>>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub type Lock = LockedState<State>;
pub fn lock() -> Lock {
    LockedState::from_or_init(&STATE, Default::default)
}
