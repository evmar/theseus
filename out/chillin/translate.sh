#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/scratch/demo/chillin-unpacked.exe
    --out out/chillin
    --extern 40a3b4=release
    # Starting at 0x409b42 there is a complex jump table, where the table address
    # itself is computed from an IP-relative address; it targets code in the
    # 0x409435.. range.
    --entry-points 409435..409b42
    --entry-point 40969a
    --entry-point 40969e
    --entry-point 4096a2
    --entry-point 4096a6
    --scan-immediates --scan-memory
    --symbols-csv out/chillin/ghidra.csv
)
cargo run -p tc -- "${args[@]}"
echo cargo build --profile fast -p chillin
