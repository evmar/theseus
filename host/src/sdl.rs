//! Implementation of host interfaces using SDL.

use std::{ffi::CString, mem::MaybeUninit};

use sdl3_sys as sdl;

use crate::{self as host, SingleThreader};

fn check(res: bool) {
    if !res {
        let err = sdl::error::SDL_GetError();
        panic!(
            "SDL error: {}",
            unsafe { std::ffi::CStr::from_ptr(err) }.to_string_lossy()
        );
    }
}

fn check_ptr<T>(t: *mut T) -> *mut T {
    check(!t.is_null());
    t
}

pub struct MainThread {
    headless: bool,
    /// Mouse buttons currently held. Button events carry no mask of their own,
    /// so it is tracked as they arrive.
    buttons: std::cell::Cell<host::MouseButton>,
}

pub struct Host {
    pub main_thread: SingleThreader<MainThread>,
}

impl Host {
    pub fn new() -> Self {
        let headless = std::env::var("THESEUS_HEADLESS").unwrap_or_default() != "";
        Self {
            main_thread: SingleThreader::new(MainThread::new(headless)),
        }
    }
}

fn mouse_buttons_from_sdl(state: sdl::mouse::SDL_MouseButtonFlags) -> host::MouseButton {
    let mut buttons = host::MouseButton::empty();
    if state.0 & sdl::mouse::SDL_BUTTON_LMASK.0 != 0 {
        buttons.insert(host::MouseButton::Left);
    }
    if state.0 & sdl::mouse::SDL_BUTTON_MMASK.0 != 0 {
        buttons.insert(host::MouseButton::Middle);
    }
    if state.0 & sdl::mouse::SDL_BUTTON_RMASK.0 != 0 {
        buttons.insert(host::MouseButton::Right);
    }
    buttons
}

/// SDL scan code -> (PC set-1 scan code, Windows VK_*, extended key).
///
/// SDL scan codes are USB HID usages, while Windows apps (and DirectInput's
/// DIK_* constants) speak PC/AT set 1, so the two need an explicit table.
/// Keys with no PC/AT equivalent are simply absent.
#[rustfmt::skip]
const KEY_MAP: &[(sdl::scancode::SDL_Scancode, u8, u8, bool)] = {
    use sdl::scancode::SDL_Scancode as SC;
    &[
        (SC::ESCAPE, 0x01, 0x1b, false),
        (SC::_1, 0x02, b'1', false),
        (SC::_2, 0x03, b'2', false),
        (SC::_3, 0x04, b'3', false),
        (SC::_4, 0x05, b'4', false),
        (SC::_5, 0x06, b'5', false),
        (SC::_6, 0x07, b'6', false),
        (SC::_7, 0x08, b'7', false),
        (SC::_8, 0x09, b'8', false),
        (SC::_9, 0x0a, b'9', false),
        (SC::_0, 0x0b, b'0', false),
        (SC::MINUS, 0x0c, 0xbd, false),        // VK_OEM_MINUS
        (SC::EQUALS, 0x0d, 0xbb, false),       // VK_OEM_PLUS
        (SC::BACKSPACE, 0x0e, 0x08, false),
        (SC::TAB, 0x0f, 0x09, false),
        (SC::Q, 0x10, b'Q', false),
        (SC::W, 0x11, b'W', false),
        (SC::E, 0x12, b'E', false),
        (SC::R, 0x13, b'R', false),
        (SC::T, 0x14, b'T', false),
        (SC::Y, 0x15, b'Y', false),
        (SC::U, 0x16, b'U', false),
        (SC::I, 0x17, b'I', false),
        (SC::O, 0x18, b'O', false),
        (SC::P, 0x19, b'P', false),
        (SC::LEFTBRACKET, 0x1a, 0xdb, false),  // VK_OEM_4
        (SC::RIGHTBRACKET, 0x1b, 0xdd, false), // VK_OEM_6
        (SC::RETURN, 0x1c, 0x0d, false),
        (SC::LCTRL, 0x1d, 0xa2, false),        // VK_LCONTROL
        (SC::A, 0x1e, b'A', false),
        (SC::S, 0x1f, b'S', false),
        (SC::D, 0x20, b'D', false),
        (SC::F, 0x21, b'F', false),
        (SC::G, 0x22, b'G', false),
        (SC::H, 0x23, b'H', false),
        (SC::J, 0x24, b'J', false),
        (SC::K, 0x25, b'K', false),
        (SC::L, 0x26, b'L', false),
        (SC::SEMICOLON, 0x27, 0xba, false),    // VK_OEM_1
        (SC::APOSTROPHE, 0x28, 0xde, false),   // VK_OEM_7
        (SC::GRAVE, 0x29, 0xc0, false),        // VK_OEM_3
        (SC::LSHIFT, 0x2a, 0xa0, false),       // VK_LSHIFT
        (SC::BACKSLASH, 0x2b, 0xdc, false),    // VK_OEM_5
        (SC::Z, 0x2c, b'Z', false),
        (SC::X, 0x2d, b'X', false),
        (SC::C, 0x2e, b'C', false),
        (SC::V, 0x2f, b'V', false),
        (SC::B, 0x30, b'B', false),
        (SC::N, 0x31, b'N', false),
        (SC::M, 0x32, b'M', false),
        (SC::COMMA, 0x33, 0xbc, false),        // VK_OEM_COMMA
        (SC::PERIOD, 0x34, 0xbe, false),       // VK_OEM_PERIOD
        (SC::SLASH, 0x35, 0xbf, false),        // VK_OEM_2
        (SC::RSHIFT, 0x36, 0xa1, false),       // VK_RSHIFT
        (SC::KP_MULTIPLY, 0x37, 0x6a, false),  // VK_MULTIPLY
        (SC::LALT, 0x38, 0xa4, false),         // VK_LMENU
        (SC::SPACE, 0x39, 0x20, false),
        (SC::CAPSLOCK, 0x3a, 0x14, false),
        (SC::F1, 0x3b, 0x70, false),
        (SC::F2, 0x3c, 0x71, false),
        (SC::F3, 0x3d, 0x72, false),
        (SC::F4, 0x3e, 0x73, false),
        (SC::F5, 0x3f, 0x74, false),
        (SC::F6, 0x40, 0x75, false),
        (SC::F7, 0x41, 0x76, false),
        (SC::F8, 0x42, 0x77, false),
        (SC::F9, 0x43, 0x78, false),
        (SC::F10, 0x44, 0x79, false),
        (SC::NUMLOCKCLEAR, 0x45, 0x90, false),
        (SC::SCROLLLOCK, 0x46, 0x91, false),
        (SC::KP_7, 0x47, 0x67, false),
        (SC::KP_8, 0x48, 0x68, false),
        (SC::KP_9, 0x49, 0x69, false),
        (SC::KP_MINUS, 0x4a, 0x6d, false),
        (SC::KP_4, 0x4b, 0x64, false),
        (SC::KP_5, 0x4c, 0x65, false),
        (SC::KP_6, 0x4d, 0x66, false),
        (SC::KP_PLUS, 0x4e, 0x6b, false),
        (SC::KP_1, 0x4f, 0x61, false),
        (SC::KP_2, 0x50, 0x62, false),
        (SC::KP_3, 0x51, 0x63, false),
        (SC::KP_0, 0x52, 0x60, false),
        (SC::KP_PERIOD, 0x53, 0x6e, false),
        (SC::F11, 0x57, 0x7a, false),
        (SC::F12, 0x58, 0x7b, false),
        // Extended keys: same scan code as their non-extended twin, but
        // prefixed with 0xe0 on the wire.
        (SC::KP_ENTER, 0x1c, 0x0d, true),
        (SC::RCTRL, 0x1d, 0xa3, true),         // VK_RCONTROL
        (SC::KP_DIVIDE, 0x35, 0x6f, true),     // VK_DIVIDE
        (SC::RALT, 0x38, 0xa5, true),          // VK_RMENU
        (SC::HOME, 0x47, 0x24, true),
        (SC::UP, 0x48, 0x26, true),
        (SC::PAGEUP, 0x49, 0x21, true),
        (SC::LEFT, 0x4b, 0x25, true),
        (SC::RIGHT, 0x4d, 0x27, true),
        (SC::END, 0x4f, 0x23, true),
        (SC::DOWN, 0x50, 0x28, true),
        (SC::PAGEDOWN, 0x51, 0x22, true),
        (SC::INSERT, 0x52, 0x2d, true),
        (SC::DELETE, 0x53, 0x2e, true),
        (SC::LGUI, 0x5b, 0x5b, true),          // VK_LWIN
        (SC::RGUI, 0x5c, 0x5c, true),          // VK_RWIN
        (SC::APPLICATION, 0x5d, 0x5d, true),   // VK_APPS
    ]
};

fn key_from_sdl(event: &sdl::events::SDL_KeyboardEvent) -> Option<host::KeyMessage> {
    let &(_, scancode, vkey, extended) = KEY_MAP.iter().find(|key| key.0 == event.scancode)?;
    Some(host::KeyMessage {
        scancode,
        vkey,
        extended,
        repeat: event.repeat,
    })
}

impl MainThread {
    fn msg_from_event(&self, event: &sdl::events::SDL_Event) -> Option<host::Message> {
        unsafe {
            use sdl::events::SDL_EventType;
            let typ: sdl::events::SDL_EventType = std::mem::transmute(event.r#type);
            match typ {
                SDL_EventType::WINDOW_EXPOSED => return Some(host::Message::Paint),
                SDL_EventType::MOUSE_MOTION => {
                    let event = &event.motion;
                    // Motion events do carry the mask, so resync from them.
                    self.buttons.set(mouse_buttons_from_sdl(event.state));
                    return Some(host::Message::MouseMove(host::MouseMessage {
                        x: event.x as u32,
                        y: event.y as u32,
                        button: host::MouseButton::empty(),
                        buttons: mouse_buttons_from_sdl(event.state),
                    }));
                }
                SDL_EventType::MOUSE_BUTTON_DOWN | SDL_EventType::MOUSE_BUTTON_UP => {
                    let event = &event.button;
                    let button = match event.button as _ {
                        sdl::mouse::SDL_BUTTON_LEFT => host::MouseButton::Left,
                        sdl::mouse::SDL_BUTTON_MIDDLE => host::MouseButton::Middle,
                        sdl::mouse::SDL_BUTTON_RIGHT => host::MouseButton::Right,
                        _ => return None,
                    };
                    // `buttons` has to be the state right after this event: a
                    // release that reported its button as still held would leave
                    // it stuck down. Button events carry no mask, and the live
                    // state is the state now rather than when the event happened,
                    // so track it as events arrive.
                    let mut buttons = self.buttons.get();
                    if typ == SDL_EventType::MOUSE_BUTTON_DOWN {
                        buttons.insert(button);
                    } else {
                        buttons.remove(button);
                    }
                    self.buttons.set(buttons);
                    let message = host::MouseMessage {
                        x: event.x as u32,
                        y: event.y as u32,
                        button,
                        buttons,
                    };
                    if typ == SDL_EventType::MOUSE_BUTTON_DOWN {
                        return Some(host::Message::MouseDown(message));
                    } else {
                        return Some(host::Message::MouseUp(message));
                    }
                }
                SDL_EventType::KEY_DOWN | SDL_EventType::KEY_UP => {
                    let key = key_from_sdl(&event.key)?;
                    if typ == SDL_EventType::KEY_DOWN {
                        return Some(host::Message::KeyDown(key));
                    } else {
                        return Some(host::Message::KeyUp(key));
                    }
                }
                SDL_EventType::QUIT => {
                    return Some(host::Message::Quit);
                }
                _ => {}
            }
            //log::warn!("todo: handle sdl event: {:#x?}", typ);
        }
        None
    }
}

impl MainThread {
    fn new(headless: bool) -> Self {
        unsafe {
            check(sdl::hints::SDL_SetHint(
                sdl::hints::SDL_HINT_NO_SIGNAL_HANDLERS,
                c"1".as_ptr(),
            ));
            check(sdl::hints::SDL_SetHint(
                sdl::hints::SDL_HINT_RENDER_VSYNC,
                c"1".as_ptr(),
            ));
            check(sdl::init::SDL_Init(if headless {
                sdl::init::SDL_INIT_EVENTS
            } else {
                sdl::init::SDL_INIT_VIDEO | sdl::init::SDL_INIT_AUDIO
            }));
        }
        Self {
            headless,
            buttons: Default::default(),
        }
    }

    pub fn poll(&self) -> Option<host::Message> {
        let event = unsafe {
            let mut event = MaybeUninit::uninit();
            if !sdl::events::SDL_PollEvent(event.as_mut_ptr()) {
                return None;
            };
            event.assume_init()
        };
        let msg = self.msg_from_event(&event)?;
        Some(msg)
    }

    pub fn wait(&self) -> host::Message {
        loop {
            let event = unsafe {
                let mut event = MaybeUninit::uninit();
                if !sdl::events::SDL_WaitEvent(event.as_mut_ptr()) {
                    panic!();
                };
                event.assume_init()
            };
            if let Some(msg) = self.msg_from_event(&event) {
                return msg;
            }
        }
    }
}

pub struct Surface {
    /// null when running in headless mode
    texture: *mut sdl::render::SDL_Texture,
}

impl Surface {
    /// pixels are RGBA in memory
    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        if self.texture.is_null() {
            return;
        }
        unsafe {
            check(sdl::render::SDL_UpdateTexture(
                self.texture,
                std::ptr::null(),
                pixels.as_ptr() as *const _,
                stride as i32,
            ));
        }
    }
}

pub struct Window {
    /// null when running in headless mode
    window: *mut sdl::video::SDL_Window,
    /// null when running in headless mode
    renderer: *mut sdl::render::SDL_Renderer,
}

impl Window {
    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
        if self.window.is_null() {
            return Surface {
                texture: std::ptr::null_mut(),
            };
        }
        unsafe {
            let texture = check_ptr(sdl::render::SDL_CreateTexture(
                self.renderer,
                // this means RGBA in memory order
                sdl::pixels::SDL_PIXELFORMAT_ABGR8888,
                sdl::render::SDL_TEXTUREACCESS_TARGET,
                width as i32,
                height as i32,
            ));
            Surface { texture }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.window.is_null() {
            return;
        }
        unsafe {
            check(sdl::video::SDL_SetWindowSize(
                self.window,
                width as i32,
                height as i32,
            ));
        }
    }

    pub fn render(&mut self, surface: &mut Surface) {
        if self.window.is_null() {
            return;
        }
        unsafe {
            // For debugging, can verify that the flip covers the entire canvas by starting with red:
            // check(sdl::render::SDL_SetRenderDrawColor(
            //     self.renderer,
            //     255,
            //     0,
            //     0,
            //     255,
            // ));
            // check(sdl::render::SDL_RenderClear(self.renderer));

            // Ignore any alpha in the input when doing the final render copy.
            check(sdl::render::SDL_SetTextureBlendMode(
                surface.texture,
                sdl::blendmode::SDL_BlendMode::NONE,
            ));
            check(sdl::render::SDL_RenderTexture(
                self.renderer,
                surface.texture,
                std::ptr::null(),
                std::ptr::null(),
            ));
            check(sdl::render::SDL_RenderPresent(self.renderer));
        }
    }
}

impl MainThread {
    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        if self.headless {
            return Window {
                window: std::ptr::null_mut(),
                renderer: std::ptr::null_mut(),
            };
        }
        unsafe {
            let window = sdl::video::SDL_CreateWindow(
                CString::new(title).unwrap().as_ptr(),
                width as i32,
                height as i32,
                sdl::video::SDL_WindowFlags::HIGH_PIXEL_DENSITY,
            );
            let renderer = sdl::render::SDL_CreateRenderer(window, std::ptr::null());
            check(sdl::render::SDL_RenderClear(renderer));
            check(sdl::render::SDL_SetDefaultTextureScaleMode(
                renderer,
                sdl::surface::SDL_ScaleMode::NEAREST,
            ));
            Window { window, renderer }
        }
    }
}

impl Host {
    #[allow(unused)] // todo
    pub fn print(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }
}

/// An audio output stream. Null when the host has no usable audio device, in
/// which case writes are discarded — a machine without sound shouldn't stop a
/// program from running.
pub struct AudioStream(*mut sdl::audio::SDL_AudioStream);
unsafe impl Send for AudioStream {}

impl AudioStream {
    /// False when the host has no audio device. Callers should skip producing
    /// audio entirely rather than mixing into nothing.
    pub fn is_open(&self) -> bool {
        !self.0.is_null()
    }

    pub fn queued_bytes(&self) -> u32 {
        if self.0.is_null() {
            // Nothing is queued, because writes are discarded. Saying anything
            // else strands callers that wait for the queue to drain.
            return 0;
        }
        unsafe { sdl::audio::SDL_GetAudioStreamQueued(self.0) as u32 }
    }

    pub fn put_data(&self, data: &[u8]) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            check(sdl::audio::SDL_PutAudioStreamData(
                self.0,
                data.as_ptr() as *const _,
                data.len() as i32,
            ))
        }
    }

    pub fn resume(&self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            check(sdl::audio::SDL_ResumeAudioStreamDevice(self.0));
        }
    }
}

impl Host {
    pub fn poll(&self) -> Option<host::Message> {
        self.main_thread.get().poll()
    }
    pub fn wait(&self) -> host::Message {
        self.main_thread.get().wait()
    }
    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        self.main_thread.get().create_window(title, width, height)
    }

    pub fn create_audio_stream(&self, spec: host::AudioSpec) -> AudioStream {
        unsafe {
            let stream = sdl::audio::SDL_OpenAudioDeviceStream(
                sdl::audio::SDL_AudioDeviceID::DEFAULT_PLAYBACK,
                &sdl::audio::SDL_AudioSpec {
                    freq: spec.sample_rate as i32,
                    channels: spec.channels as i32,
                    format: sdl::audio::SDL_AudioFormat::S16LE,
                },
                None,                 // no callback
                std::ptr::null_mut(), // no userdata
            );
            if stream.is_null() {
                let err = std::ffi::CStr::from_ptr(sdl::error::SDL_GetError());
                log::warn!("no audio output: {}", err.to_string_lossy());
            }
            AudioStream(stream)
        }
    }

    pub fn time(&self) -> u32 {
        unsafe { sdl::timer::SDL_GetTicks() as u32 }
    }

    pub fn console_write(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }
}
