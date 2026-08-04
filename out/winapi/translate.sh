#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/exe/winapi/winapi.exe
    --out out/winapi
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p winapi-exe
