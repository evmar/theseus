#!/bin/sh

set -e

if [[ $1 == "winapi" ]]; then
    cargo run -p tc ~/win/rs/exe/winapi/winapi.exe exe/winapi
    cargo build -p winapi-exe
    ./target/debug/winapi-exe
elif [[ $1 == "zig" ]]; then
    cargo run -p tc ~/win/rs/exe/zig_hello/hello.exe exe/zig_hello
    cargo build -p zig-hello-exe
    ./target/debug/zig-hello-exe
else
    cargo run -p tc ~/win/rs/deploy/archive/BasicDD.exe exe/basicdd
    cargo build -p basicdd-exe
fi
