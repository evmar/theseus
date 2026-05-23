use std::{
    arch::wasm32,
    sync::{
        Mutex,
        atomic::{self},
    },
};

use web_sys::wasm_bindgen::prelude::*;

use crate::{RECT, host};

pub struct Surface {
    // canvas: web_sys::HtmlCanvasElement,
    // width: u32,
    // context: web_sys::CanvasRenderingContext2d,
    id: i32,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        // let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document
        //     .create_element("canvas")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlCanvasElement>()
        //     .unwrap();
        // canvas.set_width(width);
        // canvas.set_height(height);
        // let context = canvas
        //     .get_context("2d")
        //     .unwrap()
        //     .unwrap()
        //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
        //     .unwrap();
        // Self {
        //     canvas,
        //     width,
        //     context,
        // }
        let id = host::host()
            .chan
            .lock()
            .unwrap()
            .create_surface(width, height);
        Self { id }
    }

    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        // let image_data = web_sys::ImageData::new_with_u8_clamped_array(
        //     wasm_bindgen::Clamped(pixels),
        //     self.width,
        // )
        // .unwrap();
        // self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        todo!();
    }

    pub fn copy(&mut self, window: &mut Window, dst_rect: &RECT, src: &Surface, src_rect: &RECT) {
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
        // self.context
        //     .draw_image_with_html_canvas_element(&surface.canvas, 0.0, 0.0)
        //     .unwrap();
        todo!();
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
        todo!()
    }

    pub fn wait(&self) -> host::Message {
        todo!()
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        Window::new(title, width, height)
    }

    pub fn create_audio_stream(&self, spec: host::AudioSpec) -> AudioStream {
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
export type Addr = number;
export type MsgConsoleWrite = ["console_write", Addr, number, Addr];
export type MsgCreateSurface = ["create_surface", number, number, Addr];
export type MsgCreateWindow = ["create_window", string, number, number, Addr];
export type MsgResizeWindow = ["resize_window", number, number, number];
export type Msg = MsgConsoleWrite | MsgCreateSurface | MsgCreateWindow | MsgResizeWindow;
"#;

impl WebHostSendChannel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn console_write(&mut self, text: &[u8]) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from("console_write"));
        args.push(&JsValue::from(text.as_ptr()));
        args.push(&JsValue::from(text.len()));
        //args.push(&JsValue::from(self.done.as_ptr()));
        self.send(&args);

        // todo: block until the UI has written the response
        // unsafe {
        // wasm32::memory_atomic_wait32(self.done.as_ptr(), 0, -1);
        // }
        // self.done.store(0, atomic::Ordering::SeqCst);
    }

    pub fn create_surface(&mut self, width: u32, height: u32) -> i32 {
        let mut done = 0i32;
        let args = js_sys::Array::new();
        args.push(&JsValue::from("create_surface"));
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        args.push(&JsValue::from(&mut done as *mut _));
        self.send(&args);

        unsafe {
            wasm32::memory_atomic_wait32(&mut done as *mut _, 0, -1);
        }
        log::info!("got done: {}", done);
        done
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> i32 {
        let mut ret = 0i32;
        let args = js_sys::Array::new();
        args.push(&JsValue::from("create_window"));
        args.push(&JsValue::from(title));
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        args.push(&JsValue::from(&mut ret as *mut _));
        self.send(&args);

        unsafe {
            wasm32::memory_atomic_wait32(&mut ret as *mut _, 0, -1);
        }
        ret
    }

    pub fn resize_window(&mut self, id: i32, width: u32, height: u32) {
        let args = js_sys::Array::new();
        args.push(&JsValue::from("resize_window"));
        args.push(&JsValue::from(id));
        args.push(&JsValue::from(width));
        args.push(&JsValue::from(height));
        self.send(&args);
    }

    fn send(&mut self, msg: &JsValue) {
        log::info!("sending: {:?}", msg);
        js_sys::global()
            .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
            .unwrap()
            .post_message(msg)
            .unwrap();
    }
}
