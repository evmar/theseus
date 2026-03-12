#!/bin/sh

set -e

# cargo run -p tc ~/win/rs/exe/winapi/winapi.exe exe/winapi
# cargo run -p tc ~/win/rs/exe/zig_hello/hello.exe exe/zig_hello
cargo run -p tc ~/win/rs/deploy/archive/BasicDD.exe exe/basicdd
cargo build -p basicdd-exe "$@"
