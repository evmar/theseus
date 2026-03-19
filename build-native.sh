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
    cargo run -p tc -- --exe ~/win/rs/scratch/demo/chillin-unpacked.exe --out exe/chillin
    cargo build -p chillin-exe
    ./target/debug/chillin-exe
else
    cargo run -p tc -- --scan --exe ~/win/rs/deploy/archive/BasicDD.exe --out exe/basicdd
    cargo build -p basicdd-exe
fi
