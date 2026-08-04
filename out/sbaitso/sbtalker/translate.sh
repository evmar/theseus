#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    --trace
    --jump-table 823:d8e..823:da0
    --exe scratch/dos/sbaitso/SBTALKER.EXE
    --out out/sbaitso/sbtalker
    # Fake return address used after TSR loads.
    --extern 11
    # TSR entry point, called by users of TSR.
    --entry-point 823:898
    # Driver entry point, returned by TSR.
    --entry-point 823:ae2
    # Random jump tables deep in the binary, uhoh.
    --jump-table d72:32c..d72:3ac
    --jump-table 935:3c40..935:3c58
)
cargo run -p tc -- "${args[@]}"
