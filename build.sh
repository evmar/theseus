#!/bin/sh

set -e

if [[ $1 == "winapi" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/winapi/winapi.exe --out exe/winapi
    cargo run -p winapi-exe
elif [[ $1 == "zig" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/zig_hello/hello.exe --out exe/zig_hello
    cargo run -p zig-hello-exe
elif [[ $1 == "chillin" ]]; then
    # starting at 0x409b42 there is a complex jump table, where the table addr itself
    # is computed from an ip-relative address; it targets code in the 0x409435.. range.
    cargo run -p tc -- \
        --exe ~/win/rs/scratch/demo/chillin-unpacked.exe \
        --out exe/chillin \
        --extern 0x40a3b4 \
        --entry-points 0x409435..0x409b42 \
        --entry-point 0x40969a \
        --entry-point 0x40969e \
        --entry-point 0x4096a2 \
        --entry-point 0x4096a6 \
        --scan-immediates --scan-memory \
        --symbols-csv exe/chillin/ghidra.csv
    cargo build --profile fast -p chillin
elif [[ $1 == "chillin-unpack" ]]; then
    cargo run -p tc -- \
        --exe ~/win/rs/scratch/demo/chillin.exe \
        --out exe/chillin-unpack \
        --extern 0x004085dd
    cargo run -p chillin-unpack
elif [[ $1 == "mofo-unpack" ]]; then
    cargo run -p tc -- \
        --exe ~/win/rs/deploy/archive/demo/psi_mofo.exe \
        --out exe/mofo-unpack \
        --extern 0x41f079
    cargo run -p mofo-unpack
elif [[ $1 == "thread" ]]; then
    cargo run -p tc -- --exe ~/win/rs/exe/cpp/thread.exe --out exe/thread --scan-immediates
    cargo run -p thread-exe
elif [[ $1 == "mine" ]]; then
    wndproc=0x100180a
    wndproc_jmptable=0x1001e0a..0x1001e2e
    cargo run -p tc -- --exe ~/win/rs/deploy/archive/win2k/winmine.exe --out exe/mine \
        --entry-point $wndproc \
        --jump-table $wndproc_jmptable
elif [[ $1 == "animate" ]]; then
    cargo run -p tc -- --exe animate.com --out exe/dos/animate
elif [[ $1 == "animate-unpack" ]]; then
    real_entry=0xff82
    cargo run -p tc -- --exe dos/exe/schwartz_animate/ANIMATE.COM --out exe/dos/animate-unpack \
        --extern $real_entry
elif [[ $1 == "basicdd" ]]; then
    cargo run -p tc -- --scan-memory --exe ~/win/rs/deploy/archive/BasicDD.exe --out exe/basicdd --entry-point 0x4012d0
    echo cargo run -p basicdd
else
    echo "unknown target $1"
fi
