//! Interface for the host environment, for APIs like "create window" or "play sound".
//! Implemented using SDL or web technologies.

use std::sync::LazyLock;

#[cfg(not(target_family = "wasm"))]
mod sdl;
#[cfg(not(target_family = "wasm"))]
pub use sdl::*;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::*;

static HOST: LazyLock<Host> = LazyLock::new(Host::new);

pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u32,
}

pub struct MouseMessage {
    pub x: u32,
    pub y: u32,
    pub button: u32,
    pub buttons: u32,
}

pub enum Message {
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
    MouseMove(MouseMessage),
    Quit,
}

pub fn init() {
    logger::init();
    LazyLock::force(&HOST);
}

pub fn host() -> &'static Host {
    &*HOST
}
