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

export RUSTFLAGS

build_mode=
build_path=debug
if [[ "$1" == "--release" ]]; then\
    shift
    build_mode="--release"
    build_path=release
fi


desired="$1"
for package in winapi-exe mine basicdd-exe; do
    file=$(sed -e 's/-exe//' <<< "$package")
    if [[ "$desired" != "" && "$desired" != "$file" ]]; then
        continue
    fi
    cargo +nightly build --lib $build_mode -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p $package
    wasm-bindgen --out-dir web --typescript --target web --reference-types \
        target/wasm32-unknown-unknown/$build_path/${file}.wasm
done
