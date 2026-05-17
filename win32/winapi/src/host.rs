//! Implementation of runtime::host using SDL.

use std::thread::ThreadId;

use runtime::host;

/// SDL data structures must all be accessed from the main thread, yikes.
struct SingleThreader<T> {
    id: ThreadId,
    data: Option<T>,
}
unsafe impl<T> Send for SingleThreader<T> {}

impl<T> SingleThreader<T> {
    pub fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: None,
        }
    }
    pub fn get(&mut self) -> &mut T {
        assert_eq!(std::thread::current().id(), self.id);
        self.data.as_mut().unwrap()
    }
}

pub struct SDLHost {
    sdl: SingleThreader<SDLState>,
}

impl SDLHost {
    pub fn new() -> Self {
        Self {
            sdl: SingleThreader {
                id: std::thread::current().id(),
                data: Some(SDLState::new()),
            },
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

impl host::MessageLoop for SDLHost {
    fn poll(&mut self) -> Option<host::Message> {
        let event = self.sdl.get().event_pump.poll_event()?;
        let msg = msg_from_event(event)?;
        Some(msg)
    }

    fn wait(&mut self) -> host::Message {
        loop {
            let event = self.sdl.get().event_pump.wait_event();
            let Some(msg) = msg_from_event(event) else {
                continue;
            };
            return msg;
        }
    }
}

struct SDLState {
    sdl: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    audio: sdl3::AudioSubsystem,
    event_pump: sdl3::EventPump,
}

impl SDLState {
    pub fn new() -> Self {
        assert!(sdl3::hint::set(sdl3::hint::names::NO_SIGNAL_HANDLERS, "1"));
        assert!(sdl3::hint::set(sdl3::hint::names::RENDER_VSYNC, "1"));
        let sdl = sdl3::init().unwrap();
        let video = sdl.video().unwrap();
        let audio = sdl.audio().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        Self {
            sdl,
            video,
            audio,
            event_pump,
        }
    }
}

struct SDLSurface {
    pub texture: sdl3::render::Texture,
}

impl host::Surface for SDLSurface {
    fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        self.texture.update(None, pixels, stride as usize).unwrap();
    }
}

struct SDLWindow {
    pub canvas: sdl3::render::WindowCanvas,
}

impl host::Window for SDLWindow {
    fn create_surface(&mut self, width: u32, height: u32) -> Box<dyn host::Surface> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(None, width, height)
            .unwrap();
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        // FML, this means BGRA in memory order
        assert_eq!(texture.format(), sdl3::pixels::PixelFormat::ARGB8888);
        Box::new(SDLSurface { texture })
    }

    fn resize(&mut self, width: u32, height: u32) {
        let window = self.canvas.window_mut();
        let scale = window.display_scale();
        window
            .set_size(
                (width as f32 * scale) as u32,
                (height as f32 * scale) as u32,
            )
            .unwrap();
    }

    fn render(&mut self, surface: &mut dyn host::Surface) {
        let surface = unsafe { &mut *(surface as *mut _ as *mut SDLSurface) };

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

impl host::Windowing for SDLHost {
    fn create_window(&mut self, title: &str, width: u32, height: u32) -> Box<dyn host::Window> {
        let mut canvas = self
            .sdl
            .get()
            .video
            .window(title, width, height)
            .high_pixel_density()
            .build()
            .unwrap()
            .into_canvas();
        canvas.clear();
        Box::new(SDLWindow { canvas })
    }
}

impl host::Host for SDLHost {
    fn clone(&self) -> Box<dyn host::Host> {
        Box::new(SDLHost {
            sdl: self.sdl.clone(),
        })
    }

    fn init(&self) {
        logger::init();
    }

    fn print(&self, text: &[u8]) {
        use std::io::Write;
        std::io::stdout().write_all(text).unwrap();
    }
}
