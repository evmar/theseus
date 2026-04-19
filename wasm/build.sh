#!/bin/sh

echo 'this has bitrotted'
exit 1

set -e

# cargo run -p fc ~/win/rs/exe/winapi/winapi.exe exe/winapi
# cargo build --profile release --target wasm32-unknown-unknown -p winapi-exe

cargo run -p fc ~/win/rs/exe/zig_hello/hello.exe exe/zig_hello
#cargo build --profile release --target wasm32-unknown-unknown -p zig-hello-exe
cargo build -p zig-hello-exe --bin zig-hello-exe
