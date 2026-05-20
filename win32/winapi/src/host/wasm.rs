use crate::{RECT, host};

pub struct Surface {}

impl Surface {
    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        todo!()
    }

    pub fn copy(&mut self, window: &mut Window, dst_rect: &RECT, src: &Surface, src_rect: &RECT) {
        todo!()
    }
}

pub struct Window {}

impl Window {
    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
        todo!()
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }
    pub fn render(&mut self, surface: &mut Surface) {
        todo!()
    }
}

pub struct AudioStream {}
impl AudioStream {
    pub fn queued_bytes(&self) -> u32 {
        todo!()
    }
    pub fn put_data(&self, data: &[u8]) {
        todo!()
    }

    pub fn resume(&self) {
        todo!()
    }
}

pub struct Host {}

impl Host {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        Host {}
    }

    pub fn poll(&self) -> Option<host::Message> {
        todo!()
    }

    pub fn wait(&self) -> host::Message {
        todo!()
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        todo!()
    }

    pub fn create_audio_stream(&self, spec: host::AudioSpec) -> AudioStream {
        todo!()
    }
}
