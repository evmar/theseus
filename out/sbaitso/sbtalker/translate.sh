#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
args=(
    #--trace
    --exe scratch/dos/sbaitso/SBTALKER.EXE
    --out out/sbaitso/sbtalker
    # TSR entry point, called by users of TSR.
    --entry-point 823:898
    # Driver entry point, returned by TSR.
    --entry-point 823:ae2
    # command dispatcher, subcommand indexes into jmp table
    --jump-table 823:d8e..823:da0
    # Random jump tables deep in the binary, uhoh.
    --jump-table d72:32c..d72:3ac
    --jump-table 935:3c40..935:3c58

    # BLASTER.DRV
    # This is dynamically loaded as an interface to the underlying Sound Blaster.
    # By loading it a part of translation it's already in memory to be translated
    # ahead of time:
    #   --load 34c4:scratch/dos/sbaitso/BLASTER.DRV
    # But we don't even want all that logic!  Rather than loading it, we patch
    # out the call that loads it:
    --extern 823:c64=noop_load_driver

    # And then provide implementations of all its API entry points:
    --extern 34c4:0000=blaster::api::put_sample
    --extern 34c4:0005=blaster::api::init
    --extern 34c4:000a=blaster::api::flush
    --extern 34c4:000f=blaster::api::get_buffer
    --extern 34c4:0014=blaster::api::noop
)
cargo run -p tc -- "${args[@]}"
