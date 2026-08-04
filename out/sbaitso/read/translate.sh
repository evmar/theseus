#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    # --trace
    --exe scratch/dos/sbaitso/READ.EXE
    --out out/sbaitso/read
    --jump-table 823:10c4..823:10d2
    # 8fe: call cx
    --entry-point 823:1072
)
cargo run -p tc -- "${args[@]}"
