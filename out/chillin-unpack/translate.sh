#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/scratch/demo/chillin.exe
    --out out/chillin-unpack
    --extern 004085dd
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p chillin-unpack
