mod dialog;
mod message;
mod misc;
mod rect;
mod resource;
mod window;

use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

pub use dialog::*;
pub use message::*;
pub use misc::*;
pub use rect::*;
pub use resource::*;
pub use window::*;

use crate::HANDLE;

pub type HWND = HANDLE;
pub type HMENU = u32;
pub type HINSTANCE = u32;
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HACCEL = u32;

pub struct State {
    pub wndclass: RefCell<Option<WndClass>>,
    pub window: RefCell<Option<Rc<RefCell<Window>>>>,
    message_queue: RefCell<MessageQueue>,
}

// TODO: reuse locking pattern from kernel32
// XXX sdl is not thread-safe so we cannot put it in a Mutex anyway, argh
struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| State {
        window: Default::default(),
        wndclass: Default::default(),
        message_queue: Default::default(),
    })
}
