use std::{arch::wasm32, sync::Mutex};

use web_sys::wasm_bindgen::prelude::*;

use crate::{FromABIParam, RECT, host};

pub struct Surface {
    id: i32,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        let id = host::host()
            .chan
            .lock()
            .unwrap()
            .create_surface(width, height);
        Self { id }
    }

    pub fn set_pixels(&mut self, pixels: &[u8], _stride: u32) {
        host::host()
            .chan
            .lock()
            .unwrap()
            .set_pixels(self.id, pixels);
    }

    pub fn copy(
        &mut self,
        _window: &mut Window,
        _dst_rect: &RECT,
        _src: &Surface,
        _src_rect: &RECT,
    ) {
        todo!()
    }
}

pub struct Window {
    id: i32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let id = host::host()
            .chan
            .lock()
            .unwrap()
            .create_window(title, width, height);
        Window { id }
    }

    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
        Surface::new(width, height)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        host::host()
            .chan
            .lock()
            .unwrap()
            .resize_window(self.id, width, height);
    }

    pub fn render(&mut self, surface: &mut Surface) {
        host::host()
            .chan
            .lock()
            .unwrap()
            .render(self.id, surface.id);
    }
}

pub struct AudioStream {}
impl AudioStream {
    pub fn queued_bytes(&self) -> u32 {
        todo!()
    }
    pub fn put_data(&self, _data: &[u8]) {
        todo!()
    }

    pub fn resume(&self) {
        todo!()
    }
}

pub struct Host {
    chan: Mutex<WebHostSendChannel>,
}

impl Host {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        Host {
            chan: Mutex::new(WebHostSendChannel::new()),
        }
    }

    pub fn poll(&self) -> Option<host::Message> {
        self.chan.lock().unwrap().poll_message()
    }

    pub fn wait(&self) -> host::Message {
        self.chan.lock().unwrap().wait_message()
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        Window::new(title, width, height)
    }

    pub fn create_audio_stream(&self, _spec: host::AudioSpec) -> AudioStream {
        todo!()
    }

    pub fn time(&self) -> u32 {
        js_sys::global()
            .unchecked_into::<web_sys::WorkerGlobalScope>()
            .performance()
            .unwrap()
            .now() as u32
    }

    pub fn console_write(&self, text: &[u8]) {
        self.chan.lock().unwrap().console_write(text);
    }
}

struct WebHostSendChannel {}

#[wasm_bindgen(typescript_custom_section)]
const MSG_TYPES: &'static str = r#"
export interface Msg {
    func: string,
    args: unknown[],
    retAddr: number,
}

export interface WasmHost {
    console_write(ptr: number, len: number): void;

    create_surface(width: number, height: number): number;

    create_window(title: string, width: number, height: number): number;
    resize_window(window_id: number, width: number, height: number): void;
    render(window_id: number, surface_id: number): void;

    set_pixels(surface_id: number, ptr: number, len: number): void;

    poll_message(): number[];
    wait_message(): Promise<number[]>;
}
"#;

#[wasm_bindgen]
extern "C" {
    fn send_to_host(msg: &str, args: js_sys::Array, ret: u32);
}

impl WebHostSendChannel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn console_write(&mut self, text: &[u8]) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(text.as_ptr()));
        args.push(&JsValue::from(text.len()));
        self.send_async("console_write", args);
    }

    pub fn create_surface(&mut self, width: u32, height: u32) -> i32 {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        self.send_sync("create_surface", args)
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> i32 {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(title));
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        self.send_sync("create_window", args)
    }

    pub fn resize_window(&mut self, id: i32, width: u32, height: u32) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(id));
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        self.send_async("resize_window", args);
    }

    pub fn render(&mut self, window_id: i32, surface_id: i32) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(window_id));
        args.push(&JsValue::from(surface_id));
        self.send_async("render", args);
    }

    pub fn set_pixels(&mut self, id: i32, pixels: &[u8]) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from(id));
        args.push(&JsValue::from(pixels.as_ptr() as u32));
        args.push(&JsValue::from(pixels.len()));
        self.send_async("set_pixels", args);
    }

    fn parse_message(buf: &[i32; 4]) -> Option<host::Message> {
        // TOOD: some sort of structured encoding
        Some(match buf[0] {
            -1 => return None,
            2 | 3 | 4 => {
                let buttons = host::MouseButton::from_abi(buf[3] as u32);
                let mouse = host::MouseMessage {
                    x: buf[1] as u32,
                    y: buf[2] as u32,
                    button: buttons,
                    buttons: buttons,
                };
                match buf[0] {
                    2 => host::Message::MouseDown(mouse),
                    3 => host::Message::MouseUp(mouse),
                    4 => host::Message::MouseMove(mouse),
                    _ => unreachable!(),
                }
            }
            msg => todo!("host message {msg}"),
        })
    }

    pub fn poll_message(&mut self) -> Option<host::Message> {
        let mut buf = [0i32; 4];
        self.send_sync_buf("poll_message", js_sys::Array::new(), &mut buf);
        Self::parse_message(&buf)
    }

    pub fn wait_message(&mut self) -> host::Message {
        let mut buf = [0i32; 4];
        self.send_sync_buf("wait_message", js_sys::Array::new(), &mut buf);
        Self::parse_message(&buf).unwrap()
    }

    fn send_sync(&mut self, msg: &str, args: js_sys::Array) -> i32 {
        let mut buf = [0i32];
        self.send_sync_buf(msg, args, &mut buf);
        buf[0]
    }

    fn send_sync_buf(&mut self, msg: &str, args: js_sys::Array, buf: &mut [i32]) {
        send_to_host(msg, args, buf.as_ptr() as *mut i32 as u32);
        unsafe {
            wasm32::memory_atomic_wait32(buf.as_ptr() as *mut _, 0, -1);
        }
    }

    fn send_async(&mut self, msg: &str, args: js_sys::Array) {
        send_to_host(msg, args, 0);
    }
}
