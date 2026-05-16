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

pub trait Host: Send + MessageLoop {
    /// Used to send between threads.
    fn clone(&self) -> Box<dyn Host>;

    fn init(&self);
    fn print(&self, text: &[u8]);
}
