mod message;
mod misc;
mod resource;
mod window;

use std::cell::{OnceCell, RefCell};

pub use message::*;
pub use misc::*;
pub use resource::*;
pub use window::*;

pub type HWND = u32;
pub type HMENU = u32;
pub type HINSTANCE = u32;
pub type HCURSOR = u32;
pub type HICON = u32;

pub struct State {
    event_loop: RefCell<winit::event_loop::EventLoop<()>>,
    window: RefCell<Option<Window>>,
    message_queue: RefCell<MessageQueue>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| State {
        event_loop: RefCell::new(winit::event_loop::EventLoop::new().unwrap()),
        window: Default::default(),
        message_queue: Default::default(),
    })
}
