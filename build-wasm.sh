#!/bin/sh

set -e

cargo build --target wasm32-unknown-unknown -p winapi-exe
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/winapi_wasm.wasm


cargo build --target wasm32-unknown-unknown -p mine
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/mine_wasm.wasm
