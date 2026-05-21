use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
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

    let a = Closure::<dyn Fn()>::new(|| {
        log::info!("onmessage");
    });
    window.set_onmessage(Some(a.as_ref().unchecked_ref()));
    a.forget();
}
