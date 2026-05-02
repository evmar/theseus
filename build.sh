#!/bin/sh

set -e

if [[ $1 == "winapi" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/winapi/winapi.exe --out exe/winapi
    cargo run -p winapi-exe
elif [[ $1 == "zig" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/zig_hello/hello.exe --out exe/zig_hello
    cargo run -p zig-hello-exe
elif [[ $1 == "chillin" ]]; then
    # starting at 0x409b42 there is a complex jump table, where the table addr itself
    # is computed from an ip-relative address
    cargo run -p tc -- \
        --exe ~/win/rs/scratch/demo/chillin-unpacked.exe \
        --out exe/chillin \
        --extern 0x40a3b4 \
        --entry-point 0x409435 \
        --entry-point 0x409739 \
        --entry-point 0x40950a \
        --entry-point 0x4094ad \
        --scan-immediates --scan-memory
    cargo build --profile fast -p chillin
elif [[ $1 == "chillin-unpack" ]]; then
    cargo run -p tc -- \
        --exe ~/win/rs/scratch/demo/chillin.exe \
        --out exe/chillin-unpack \
        --extern 0x004085dd
    cargo run -p chillin-unpack
elif [[ $1 == "thread" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/cpp/thread.exe --out exe/thread --scan-immediates
    cargo run -p thread-exe
else
    cargo run -p tc -- --scan-memory --exe ~/win/rs/deploy/archive/BasicDD.exe --out exe/basicdd
    echo cargo run -p basicdd-exe
fi
