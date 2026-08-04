#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe animate2.com
    --out out/dos/animate
    --entry-point 1126
    --entry-point 1165
)
cargo run -p tc -- "${args[@]}"
