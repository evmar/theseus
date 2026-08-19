#!/bin/bash
set -e

cd "$(git rev-parse --show-toplevel)"
game_dir=~/projects_hobby/pinball-soccer-98/game_run/Soccer98
args=(
    --exe $game_dir/winpin.exe
    --out out/winpin
    --scan-memory --scan-immediates --scan-prologues
)
# Feedback loop: addresses the scan missed, collected at runtime.
# See THESEUS_MISSING_ADDRS in runtime.
if [[ -f out/winpin/missing.txt ]]; then
    args+=(--entry-points-file out/winpin/missing.txt)
fi
cargo run -p tc -- "${args[@]}"
echo cargo run -p winpin
