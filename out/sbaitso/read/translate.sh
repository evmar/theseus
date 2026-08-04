#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    # --trace
    --exe scratch/dos/sbaitso/READ.EXE
    --out out/sbaitso/read
    --jump-table 10c4..10d2
    # 8fe: call cx
    --entry-point 1072
)
cargo run -p tc -- "${args[@]}"
