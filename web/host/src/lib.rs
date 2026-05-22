use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(worker_memory: js_sys::WebAssembly::Memory) {
    logger::init();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("host running");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let console = document
        .create_element("pre")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    console.set_id("console");
    body.append_child(&console).unwrap();

    let mut console_output = vec![];
    let a = Closure::new(move |e: web_sys::MessageEvent| {
        // Note that we can't hang on to the SharedArrayBuffer, because it doesn't grow
        // when the worker memory does(!).  Grabbing it from the wasm memory object does
        // the right thing.
        let buffer = worker_memory
            .buffer()
            .dyn_into::<js_sys::SharedArrayBuffer>()
            .unwrap();
        web_sys::console::log_1(&buffer);

        let arr = e.data().dyn_into::<js_sys::Array>().unwrap();
        let ptr = arr.get(1).as_f64().unwrap() as u32;
        let len = arr.get(2).as_f64().unwrap() as u32;
        let buf = js_sys::Uint8Array::new_with_byte_offset_and_length(&buffer, ptr, len);

        let done = arr.get(3).as_f64().unwrap() as u32;

        let ofs = console_output.len();
        console_output.resize(ofs + len as usize, 0);
        buf.copy_to(&mut console_output[ofs..]);

        // yuck, this encodes/decodes multiple times
        let text = unsafe {
            // safety: buf will not outlive console_output.
            let buf = js_sys::Uint8Array::view(&console_output);
            web_sys::TextDecoder::new()
                .unwrap()
                .decode_with_buffer_source(&buf)
                .unwrap()
        };
        console.set_inner_text(&text);
    });
    window.set_onmessage(Some(a.as_ref().unchecked_ref()));
    a.forget();
}
