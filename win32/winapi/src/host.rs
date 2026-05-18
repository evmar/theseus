use std::sync::LazyLock;

pub use crate::sdl::*;

static HOST: LazyLock<Host> = LazyLock::new(Host::new);

pub struct MouseMessage {
    pub down: bool,
    pub x: u32,
    pub y: u32,
    pub button: u32,
}

pub enum Message {
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
}

pub fn init() {
    logger::init();
    LazyLock::force(&HOST);
}

pub fn host() -> &'static Host {
    &*HOST
}
