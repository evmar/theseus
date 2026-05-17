//! The host system interface for things like events, windowing, and audio.

pub struct MouseMessage {
    pub down: bool,
    pub x: u32,
    pub y: u32,
    pub button: u32,
}

pub enum Message {
    Paint,
    MouseDown(MouseMessage),
    MouseUp(MouseMessage),
}

pub trait MessageLoop {
    fn poll(&mut self) -> Option<Message>;
    fn wait(&mut self) -> Message;
}

pub trait Surface {
    fn set_pixels(&mut self, pixels: &[u8], stride: u32);
}

pub trait Window {
    fn create_surface(&mut self, width: u32, height: u32) -> Box<dyn Surface>;
    fn resize(&mut self, width: u32, height: u32);
    fn render(&mut self, surface: &mut dyn Surface);
}

pub trait Windowing {
    fn create_window(&mut self, title: &str, width: u32, height: u32) -> Box<dyn Window>;
}

pub trait Host: Send + MessageLoop + Windowing {
    /// Used to send between threads.
    fn clone(&self) -> Box<dyn Host>;

    fn init(&self);
    fn print(&self, text: &[u8]);
}
