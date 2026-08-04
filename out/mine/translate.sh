#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe ~/win/rs/deploy/archive/win2k/winmine.exe
    --out out/mine
    # wndproc
    --entry-point 100180a
    # wndproc jmptable
    --jump-table 1001e0a..1001e2e
)
cargo run -p tc -- "${args[@]}"
