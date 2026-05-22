#!/bin/sh

set -e

#exports="-Clink-args=--export=__heap_base -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base"
#RUSTFLAGS="-C target-feature=+atomics,+bulk-memory -C link-arg=--shared-memory -C link-arg=--import-memory  $exports"

export RUSTFLAGS='-Ctarget-feature=+atomics -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory -Clink-args=--export=__heap_base -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base'
cargo +nightly build -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p winapi-exe
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/winapi_wasm.wasm

cargo +nightly build -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p mine
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/mine_wasm.wasm
