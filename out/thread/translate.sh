#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/exe/cpp/thread.exe
    --out out/thread
    --scan-immediates
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p thread-exe
