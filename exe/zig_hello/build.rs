fn main() {
    let wasm = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm";
    if wasm {
        println!("cargo::rustc-link-arg=--import-memory");
    }
}
