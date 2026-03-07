#!/bin/sh

set -e

cargo run -p fc ~/win/rs/exe/zig_hello/hello.exe exe/zig_hello
cargo build -p zig-hello-exe --bin zig-hello-exe "$@"
