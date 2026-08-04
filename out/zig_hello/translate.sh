#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/exe/zig_hello/hello.exe
    --out out/zig_hello
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p zig-hello-exe
