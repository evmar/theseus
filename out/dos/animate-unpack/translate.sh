#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe scratch/dos/schwartz_animate/ANIMATE.COM
    --out out/dos/animate-unpack
    --extern ff82
)
cargo run -p tc -- "${args[@]}"
