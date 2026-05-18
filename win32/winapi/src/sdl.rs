//! Implementation of host interfaces using SDL.

use std::{cell::RefCell, thread::ThreadId};

use crate::{RECT, host};

/// SDL data structures must all be accessed from the main thread, yikes.
struct SingleThreader<T> {
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

pub struct Host {
    sdl: SingleThreader<RefCell<SDLState>>,
}

impl Host {
    pub fn new() -> Self {
        Self {
            sdl: SingleThreader::new(RefCell::new(SDLState::new())),
        }
    }
}

fn mouse_msg(
    down: bool,
    mouse_btn: sdl3::mouse::MouseButton,
    x: f32,
    y: f32,
) -> host::MouseMessage {
    let button = match mouse_btn {
        sdl3::mouse::MouseButton::Left => 1,
        sdl3::mouse::MouseButton::Right => 2,
        sdl3::mouse::MouseButton::Middle => 3,
        _ => todo!(),
    };
    host::MouseMessage {
        down,
        x: x as u32,
        y: y as u32,
        button,
    }
}

fn msg_from_event(event: sdl3::event::Event) -> Option<host::Message> {
    use sdl3::event::Event;
    match event {
        Event::Window { win_event, .. } => {
            use sdl3::event::WindowEvent;
            match win_event {
                WindowEvent::Shown => return None,
                WindowEvent::Resized(_, _) => return None,
                WindowEvent::FocusGained => return None,
                WindowEvent::FocusLost => return None,
                WindowEvent::Exposed => return Some(host::Message::Paint),
                WindowEvent::MouseEnter => return None,
                WindowEvent::MouseLeave => return None,
                WindowEvent::PixelSizeChanged(_, _) => return None,
                _ => {}
            }
        }
        Event::MouseMotion { .. } => {
            return None;
        }
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => {
            return Some(host::Message::MouseDown(mouse_msg(true, mouse_btn, x, y)));
        }
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => {
            return Some(host::Message::MouseUp(mouse_msg(true, mouse_btn, x, y)));
        }
        Event::AudioDeviceAdded { .. } | Event::ClipboardUpdate { .. } | Event::Unknown { .. } => {
            // ignore
            return None;
        }
        _ => {}
    }
    log::warn!("todo: handle sdl event: {:?}", event);
    None
}

impl Host {
    pub fn poll(&self) -> Option<host::Message> {
        let event = self.sdl.get().borrow_mut().event_pump.poll_event()?;
        let msg = msg_from_event(event)?;
        Some(msg)
    }

    pub fn wait(&self) -> host::Message {
        loop {
            let event = self.sdl.get().borrow_mut().event_pump.wait_event();
            let Some(msg) = msg_from_event(event) else {
                continue;
            };
            return msg;
        }
    }
}

struct SDLState {
    _sdl: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    audio: sdl3::AudioSubsystem,
    event_pump: sdl3::EventPump,
}

impl SDLState {
    pub fn new() -> Self {
        assert!(sdl3::hint::set(sdl3::hint::names::NO_SIGNAL_HANDLERS, "1"));
        assert!(sdl3::hint::set(sdl3::hint::names::RENDER_VSYNC, "1"));
        let _sdl = sdl3::init().unwrap();
        let video = _sdl.video().unwrap();
        let audio = _sdl.audio().unwrap();
        let event_pump = _sdl.event_pump().unwrap();
        Self {
            _sdl,
            video,
            audio,
            event_pump,
        }
    }
}

pub fn rect_to_sdl(rect: &RECT) -> sdl3::rect::Rect {
    sdl3::rect::Rect::new(
        rect.left,
        rect.top,
        (rect.right - rect.left) as u32,
        (rect.bottom - rect.top) as u32,
    )
}

pub struct Surface {
    texture: sdl3::render::Texture,
}

impl Surface {
    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        self.texture.update(None, pixels, stride as usize).unwrap();
    }

    pub fn copy(&mut self, window: &mut Window, dst_rect: &RECT, src: &Surface, src_rect: &RECT) {
        // To render to a texture, we need to start with a canvas, which we can only get from
        // a window because (I guess?) something about having a GPU context.

        window
            .canvas
            .with_texture_canvas(&mut self.texture, |canvas| {
                canvas
                    .copy(&src.texture, rect_to_sdl(src_rect), rect_to_sdl(dst_rect))
                    .unwrap();
            })
            .unwrap();
    }
}

pub struct Window {
    canvas: sdl3::render::WindowCanvas,
}

impl Window {
    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(None, width, height)
            .unwrap();
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        // FML, this means BGRA in memory order
        assert_eq!(texture.format(), sdl3::pixels::PixelFormat::ARGB8888);
        Surface { texture }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let window = self.canvas.window_mut();
        let scale = window.display_scale();
        window
            .set_size(
                (width as f32 * scale) as u32,
                (height as f32 * scale) as u32,
            )
            .unwrap();
    }

    pub fn render(&mut self, surface: &mut Surface) {
        // For debugging, can verify that the flip covers the entire canvas by starting with red:
        // canvas.set_draw_color(sdl3::pixels::Color::RED);
        // canvas.clear();
        // Ignore any alpha in the input when doing the final render copy.
        surface
            .texture
            .set_blend_mode(sdl3::render::BlendMode::None);
        self.canvas.copy(&surface.texture, None, None).unwrap();
        self.canvas.present();
    }
}

impl Host {
    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        let mut canvas = self
            .sdl
            .get()
            .borrow()
            .video
            .window(title, width, height)
            .high_pixel_density()
            .build()
            .unwrap()
            .into_canvas();
        canvas.clear();
        Window { canvas }
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

pub struct AudioStream(SingleThreader<sdl3::audio::AudioStreamOwner>);

impl AudioStream {
    pub fn queued_bytes(&self) -> u32 {
        self.0.get().queued_bytes().unwrap() as u32
    }

    pub fn put_data(&self, data: &[u8]) {
        self.0.get().put_data(data).unwrap();
    }

    pub fn resume(&self) {
        self.0.get().resume().unwrap();
    }
}

impl Host {
    pub fn create_audio_stream(&self, spec: AudioSpec) -> AudioStream {
        let stream = self
            .sdl
            .get()
            .borrow()
            .audio
            .default_playback_device()
            .open_device_stream(Some(&sdl3::audio::AudioSpec {
                freq: Some(spec.sample_rate as i32),
                channels: Some(spec.channels as i32),
                format: Some(sdl3::audio::AudioFormat::S16LE),
            }))
            .unwrap();
        AudioStream(SingleThreader::new(stream))
    }
}
