#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/deploy/archive/demo/psi_mofo.exe
    --out out/mofo-unpack
    --extern 41f079
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p mofo-unpack
