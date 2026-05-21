use std::sync::Mutex;

use web_sys::wasm_bindgen::prelude::*;

use crate::{RECT, host};

pub struct Surface {
    canvas: web_sys::HtmlCanvasElement,
    width: u32,
    context: web_sys::CanvasRenderingContext2d,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(width);
        canvas.set_height(height);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Self {
            canvas,
            width,
            context,
        }
    }

    pub fn set_pixels(&mut self, pixels: &[u8], stride: u32) {
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(
            wasm_bindgen::Clamped(pixels),
            self.width,
        )
        .unwrap();
        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

    pub fn copy(&mut self, window: &mut Window, dst_rect: &RECT, src: &Surface, src_rect: &RECT) {
        todo!()
    }
}

pub struct Window {
    dom: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let dom = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = dom
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        dom.set_class_name("window");
        let mut window = Self { dom, context };
        window.resize(width, height);
        window
    }

    pub fn create_surface(&mut self, width: u32, height: u32) -> Surface {
        let surface = Surface::new(width, height);
        self.dom.append_child(&surface.canvas).unwrap();
        surface
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let style = self.dom.style();
        style
            .set_property("width", &format!("{}px", width))
            .unwrap();
        style
            .set_property("height", &format!("{}px", height))
            .unwrap();
        self.context = self
            .dom
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
    }

    pub fn render(&mut self, surface: &mut Surface) {
        self.context
            .draw_image_with_html_canvas_element(&surface.canvas, 0.0, 0.0)
            .unwrap();
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
    console_text: Mutex<Vec<u8>>,
    console: web_sys::HtmlElement,
}

impl Host {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let console = document
            .create_element("pre")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        console.set_id("console");
        body.append_child(&console).unwrap();

        Host {
            console_text: Default::default(),
            console,
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
        web_sys::window().unwrap().performance().unwrap().now() as u32
    }

    pub fn console_write(&self, text: &[u8]) {
        let mut console_text = self.console_text.lock().unwrap();
        console_text.extend_from_slice(text);
        let utf8 = String::from_utf8_lossy(&console_text);
        self.console.set_inner_text(&utf8);
    }
}
