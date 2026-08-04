#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --scan-memory
    --exe ~/win/rs/deploy/archive/BasicDD.exe
    --out out/basicdd
    --entry-point 4012d0
    --jump-table 403f1c..403f38
    --jump-table 403f88..403f94
)
cargo run -p tc -- "${args[@]}"
echo cargo run -p basicdd
