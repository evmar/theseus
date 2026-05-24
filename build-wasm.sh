#!/bin/sh

set -e

# For wasm output we need to pass a bunch of flags to Rust.
# Linker flags end up looking like `-Clink-args=--XXX`, and these two loops accumulate the prefixes.

link_flags="--shared-memory --max-memory=1073741824 --import-memory --export=__heap_base --export=__wasm_init_tls --export=__tls_size --export=__tls_align --export=__tls_base"
link_args=""
for arg in $link_flags; do
    link_args="$link_args link-args=$arg"
done

RUSTFLAGS=""
for arg in target-feature=+atomics $link_args; do
    RUSTFLAGS="$RUSTFLAGS -C$arg"
done

echo "RUSTFLAGS=$RUSTFLAGS"
export RUSTFLAGS
cargo +nightly build -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p winapi-exe
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/winapi_wasm.wasm

cargo +nightly build -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p mine
wasm-bindgen --out-dir web --typescript --target web --reference-types \
    target/wasm32-unknown-unknown/debug/mine_wasm.wasm
