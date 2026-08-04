#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe animate.com
    --out out/dos/animate-unpack-2
    --entry-point ff82
    --extern 100
)
cargo run -p tc -- "${args[@]}"
