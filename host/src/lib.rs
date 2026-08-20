//! Interface for the host environment, for APIs like "create window" or "play sound".
//! Implemented using SDL or web technologies.

#![cfg_attr(target_family = "wasm", feature(stdarch_wasm_atomic_wait))]

use std::sync::LazyLock;

pub mod fs;
#[cfg(not(target_family = "wasm"))]
mod sdl;
mod single_thread;
#[cfg(not(target_family = "wasm"))]
pub use sdl::*;
pub use single_thread::SingleThreader;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::*;

static HOST: LazyLock<Host> = LazyLock::new(Host::new);

#[derive(Clone, Copy)]
pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
    pub struct MouseButton: u16 {
        const Left = 1 << 0;
        const Middle = 1 << 1;
        const Right = 1 << 2;
    }
}

pub struct MouseMessage {
    pub x: u32,
    pub y: u32,
    /// In a click, which button triggered the click.
    pub button: MouseButton,
    /// Bitfield, which buttons are pressed.
    pub buttons: MouseButton,
}

/// A key press/release, described the way DOS-era Windows apps expect: by
/// PC/AT scan code (what DirectInput calls DIK_*) plus virtual key code.
pub struct KeyMessage {
    /// PC "set 1" scan code, without the 0xe0 prefix of extended keys.
    pub scancode: u8,
    /// Windows VK_* code. Uses the side-specific code (VK_RSHIFT, not VK_SHIFT)
    /// where one exists; user32 widens it for window messages.
    pub vkey: u8,
    /// Key from the extended (0xe0-prefixed) part of the keyboard: arrows,
    /// right ctrl/alt, keypad enter. Distinguishes e.g. arrow up from keypad 8,
    /// which share scan code 0x48.
    pub extended: bool,
    /// Event produced by auto-repeat rather than a fresh press.
    pub repeat: bool,
}

pub enum Message {
    #[cfg(not(target_family = "wasm"))] // no "quit" menu on web
    Quit,
    #[cfg(not(target_family = "wasm"))] // no paint on web, browser does painting
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
    MouseMove(MouseMessage),
    KeyDown(KeyMessage),
    KeyUp(KeyMessage),
}

/// Which winapi calls to trace, in the syntax `trace::init` parses.
#[cfg(not(target_family = "wasm"))]
pub fn trace_spec() -> String {
    std::env::var("THESEUS_TRACE").unwrap_or_default()
}

/// The web build has no environment to read a spec from, so the page sets one
/// with `set_trace` before starting the program. Tracing every call is slow
/// enough to change how a program behaves, so it is off unless asked for.
#[cfg(target_family = "wasm")]
static TRACE_SPEC: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

#[cfg(target_family = "wasm")]
pub fn trace_spec() -> String {
    TRACE_SPEC.lock().unwrap().clone()
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn set_trace(spec: &str) {
    *TRACE_SPEC.lock().unwrap() = spec.to_string();
}

pub fn init() {
    logger::init();
    LazyLock::force(&HOST);
}

pub fn host() -> &'static Host {
    &*HOST
}
