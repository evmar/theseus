#!/bin/sh

set -e

# For worker wasm output we need to pass a bunch of flags to Rust.
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
for package in winapi-exe mine basicdd; do
    file=$(sed -e 's/-exe//' -e 's/-/_/' <<< "$package")
    if [[ "$desired" != "" && "$desired" != "$file" ]]; then
        continue
    fi
    cargo +nightly build --lib $build_mode -Z build-std=std,panic_abort --target wasm32-unknown-unknown -p $package
    wasm-bindgen --out-dir web/exe/${file} --typescript --target web --reference-types \
        target/wasm32-unknown-unknown/$build_path/${file}.wasm

    if [[ $build_mode == "--release" ]]; then
        path=web/exe/${file}/${file}_bg.wasm
        wasm-opt -Oz -o "$path" "$path"
    fi
done

# build sbaitso in non-worker mode
export RUSTFLAGS=
for package in sbaitso-sbtalker; do
    file=$(sed -e 's/-exe//' -e 's/-/_/' <<< "$package")
    if [[ "$desired" != "" && "$desired" != "$file" ]]; then
        continue
    fi
    cargo build --lib $build_mode --target wasm32-unknown-unknown -p $package
    wasm-bindgen --out-dir web/exe/${file} --typescript --target web --reference-types \
        target/wasm32-unknown-unknown/$build_path/${file}.wasm

    if [[ $build_mode == "--release" ]]; then
        path=web/exe/${file}/${file}_bg.wasm
        wasm-opt -Oz -o "$path" "$path"
    fi
done
