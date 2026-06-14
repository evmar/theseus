//! Implementation of host interfaces using SDL.

use crate::{self as host, SingleThreader};
use std::{ffi::CString, mem::MaybeUninit};

use sdl3_sys as sdl;

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

pub struct MainThread {}

pub struct Host {
    pub main_thread: SingleThreader<MainThread>,
}

impl Host {
    pub fn new() -> Self {
        Self {
            main_thread: SingleThreader::new(MainThread::new()),
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

fn msg_from_event(event: &sdl::events::SDL_Event) -> Option<host::Message> {
    unsafe {
        use sdl::events::SDL_EventType;
        let typ: sdl::events::SDL_EventType = std::mem::transmute(event.r#type);
        match typ {
            SDL_EventType::WINDOW_EXPOSED => return Some(host::Message::Paint),
            SDL_EventType::MOUSE_MOTION => {
                let event = &event.motion;
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
                let message = host::MouseMessage {
                    x: event.x as u32,
                    y: event.y as u32,
                    button: button,
                    buttons: button,
                };
                if typ == SDL_EventType::MOUSE_BUTTON_DOWN {
                    return Some(host::Message::MouseDown(message));
                } else {
                    return Some(host::Message::MouseUp(message));
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

impl MainThread {
    fn new() -> Self {
        unsafe {
            check(sdl::hints::SDL_SetHint(
                sdl::hints::SDL_HINT_NO_SIGNAL_HANDLERS,
                c"1".as_ptr(),
            ));
            check(sdl::hints::SDL_SetHint(
                sdl::hints::SDL_HINT_RENDER_VSYNC,
                c"1".as_ptr(),
            ));
            check(sdl::init::SDL_Init(
                sdl::init::SDL_INIT_VIDEO | sdl::init::SDL_INIT_AUDIO,
            ));
        }
        Self {}
    }

    pub fn poll(&self) -> Option<host::Message> {
        let event = unsafe {
            let mut event = MaybeUninit::uninit();
            if !sdl::events::SDL_PollEvent(event.as_mut_ptr()) {
                return None;
            };
            event.assume_init()
        };
        let msg = msg_from_event(&event)?;
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
            if let Some(msg) = msg_from_event(&event) {
                return msg;
            }
        }
    }
}

pub struct Surface {
    texture: *mut sdl::render::SDL_Texture,
}

impl Surface {
    /// pixels are RGBA in memory
    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
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
    window: *mut sdl::video::SDL_Window,
    renderer: *mut sdl::render::SDL_Renderer,
}

impl Window {
    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
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
        unsafe {
            check(sdl::video::SDL_SetWindowSize(
                self.window,
                width as i32,
                height as i32,
            ));
        }
    }

    pub fn render(&mut self, surface: &mut Surface) {
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

pub struct AudioStream(*mut sdl::audio::SDL_AudioStream);
unsafe impl Send for AudioStream {}

impl AudioStream {
    pub fn queued_bytes(&self) -> u32 {
        unsafe { sdl::audio::SDL_GetAudioStreamQueued(self.0) as u32 }
    }

    pub fn put_data(&self, data: &[u8]) {
        // self.0.get().put_data(data).unwrap();
        unsafe {
            check(sdl::audio::SDL_PutAudioStreamData(
                self.0,
                data.as_ptr() as *const _,
                data.len() as i32,
            ))
        }
    }

    pub fn resume(&self) {
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
