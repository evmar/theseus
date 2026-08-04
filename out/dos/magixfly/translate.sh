#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --exe scratch/dos/magixfly/magixfly.com
    --out out/dos/magixfly
)
cargo run -p tc -- "${args[@]}"
