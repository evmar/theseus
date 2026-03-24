#!/bin/sh

set -e

if [[ $1 == "winapi" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/winapi/winapi.exe --out exe/winapi
    cargo build -p winapi-exe
    ./target/debug/winapi-exe
elif [[ $1 == "zig" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/zig_hello/hello.exe --out exe/zig_hello
    cargo build -p zig-hello-exe
    ./target/debug/zig-hello-exe
elif [[ $1 == "chillin" ]]; then
    cargo run -p tc -- \
        --exe ~/win/rs/scratch/demo/chillin-unpacked.exe \
        --out exe/chillin \
        --extern 0x40a3b4
    cargo run --release -p chillin-exe
elif [[ $1 == "thread" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/cpp/thread.exe --out exe/thread --scan-immediates
    cargo build -p thread-exe
    ./target/debug/thread-exe
else
    cargo run -p tc -- --scan --exe ~/win/rs/deploy/archive/BasicDD.exe --out exe/basicdd
    cargo build -p basicdd-exe
fi
