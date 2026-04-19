mod message;
mod misc;
mod resource;
mod window;

use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

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
    #[allow(unused)]
    sdl: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    event_pump: RefCell<sdl3::EventPump>,
    pub window: RefCell<Option<Rc<RefCell<Window>>>>,
    message_queue: RefCell<MessageQueue>,
}

// TODO: reuse locking pattern from kernel32
struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| {
        assert!(sdl3::hint::set(sdl3::hint::names::NO_SIGNAL_HANDLERS, "1"));
        let sdl = sdl3::init().unwrap();
        let video = sdl.video().unwrap();
        let event_pump = RefCell::new(sdl.event_pump().unwrap());
        State {
            sdl,
            video,
            event_pump,
            window: Default::default(),
            message_queue: Default::default(),
        }
    })
}
