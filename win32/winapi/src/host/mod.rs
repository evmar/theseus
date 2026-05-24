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

// This isn't really a win32 ABI enum, but we use ABIEnum derive to get the number=>value reverse mapping.
#[derive(win32_derive::ABIEnum)]
pub enum MouseButton {
    Left = 1,
    Middle = 2,
    Right = 3,
}

pub struct MouseMessage {
    pub x: u32,
    pub y: u32,
    /// In a click, which button triggered the click.
    pub button: MouseButton,
    /// Bitfield, which buttons are pressed.
    pub buttons: u32,
}

pub enum Message {
    #[cfg(not(target_family = "wasm"))] // no "quit" menu on web
    Quit,
    #[cfg(not(target_family = "wasm"))] // no paint on web, browser does painting
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
    MouseMove(MouseMessage),
}

pub fn init() {
    logger::init();
    LazyLock::force(&HOST);
}

pub fn host() -> &'static Host {
    &*HOST
}
