//! Interface for the host environment, for APIs like "create window" or "play sound".
//! Implemented using SDL or web technologies.

#![cfg_attr(target_family = "wasm", feature(stdarch_wasm_atomic_wait))]

use std::sync::LazyLock;

#[cfg(not(target_family = "wasm"))]
mod sdl;
pub mod fs;
mod single_thread;
#[cfg(not(target_family = "wasm"))]
pub use sdl::*;
pub use single_thread::SingleThreader;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::*;

static HOST: LazyLock<Host> = LazyLock::new(Host::new);

pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

pub enum Message {
    #[cfg(not(target_family = "wasm"))] // no "quit" menu on web
    Quit,
    #[cfg(not(target_family = "wasm"))] // no paint on web, browser does painting
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
    MouseMove(MouseMessage),
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
