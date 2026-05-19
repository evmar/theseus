//! Implementation of host interfaces using SDL.

use std::{mem::MaybeUninit, thread::ThreadId};

use sdl3_sys as sdl;

use crate::{RECT, host};

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

/// SDL has many APIs that must be accessed from the main thread, yikes.
/// SingleThreader stores the thread it's created on, then panics if you call .get() from a different thread.
pub struct SingleThreader<T> {
    id: ThreadId,
    data: T,
}

/// Safety: accessors assert we're on the initial thread.
unsafe impl<T> Sync for SingleThreader<T> {}
unsafe impl<T> Send for SingleThreader<T> {}

impl<T> SingleThreader<T> {
    pub fn new(data: T) -> Self {
        Self {
            id: std::thread::current().id(),
            data,
        }
    }

    pub fn get(&self) -> &T {
        assert_eq!(std::thread::current().id(), self.id);
        &self.data
    }
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

fn msg_from_event(event: &sdl::events::SDL_Event) -> Option<host::Message> {
    unsafe {
        use sdl::events::SDL_EventType;
        let typ: sdl::events::SDL_EventType = std::mem::transmute(event.r#type);
        match typ {
            SDL_EventType::WINDOW_EXPOSED => return Some(host::Message::Paint),
            SDL_EventType::MOUSE_BUTTON_DOWN | SDL_EventType::MOUSE_BUTTON_UP => {
                let event = &event.button;
                let button = match event.button as _ {
                    sdl::mouse::SDL_BUTTON_LEFT => 1,
                    sdl::mouse::SDL_BUTTON_RIGHT => 3,
                    sdl::mouse::SDL_BUTTON_MIDDLE => 2,
                    _ => return None,
                };
                let message = host::MouseMessage {
                    x: event.x as u32,
                    y: event.y as u32,
                    button: button as u32,
                };
                if typ == SDL_EventType::MOUSE_BUTTON_DOWN {
                    return Some(host::Message::MouseDown(message));
                } else {
                    return Some(host::Message::MouseUp(message));
                }
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
            sdl::init::SDL_Init(sdl::init::SDL_INIT_VIDEO | sdl::init::SDL_INIT_AUDIO);
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

pub fn rect_to_sdl(rect: &RECT) -> sdl::rect::SDL_FRect {
    sdl::rect::SDL_FRect {
        x: rect.left as f32,
        y: rect.top as f32,
        w: (rect.right - rect.left) as f32,
        h: (rect.bottom - rect.top) as f32,
    }
}

pub struct Surface {
    texture: *mut sdl::render::SDL_Texture,
}

impl Surface {
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

    pub fn copy(&mut self, window: &mut Window, dst_rect: &RECT, src: &Surface, src_rect: &RECT) {
        // To render to a texture, we need to start with a renderer, which we can only get from
        // a window because (I guess?) something about having a GPU context.

        unsafe {
            check(sdl::render::SDL_SetRenderTarget(
                window.renderer,
                self.texture,
            ));
            check(sdl::render::SDL_RenderTexture(
                window.renderer,
                src.texture,
                &rect_to_sdl(src_rect),
                &rect_to_sdl(dst_rect),
            ));
            check(sdl::render::SDL_SetRenderTarget(
                window.renderer,
                std::ptr::null_mut(),
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
                // FML, this means BGRA in memory order
                sdl::pixels::SDL_PIXELFORMAT_ARGB8888,
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
                title.as_ptr() as *const _,
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
    pub fn print(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }
}

pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u32,
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
    pub fn create_audio_stream(&self, spec: AudioSpec) -> AudioStream {
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
}
