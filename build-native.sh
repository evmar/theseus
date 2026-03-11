#!/bin/sh

set -e

# cargo run -p fc ~/win/rs/exe/winapi/winapi.exe exe/winapi
# cargo run -p fc ~/win/rs/exe/zig_hello/hello.exe exe/zig_hello
cargo run -p fc ~/win/rs/deploy/archive/BasicDD.exe exe/basicdd
cargo build -p basicdd-exe "$@"
