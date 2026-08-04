#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --scan-memory
    --exe ~/win/rs/deploy/archive/BasicDD.exe
    --out out/basicdd
    --entry-point 4012d0
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p basicdd
