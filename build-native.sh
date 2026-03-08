#!/bin/sh

set -e

cargo run -p fc ~/win/rs/deploy/archive/BasicDD.exe exe/basicdd
cargo build -p basicdd-exe "$@"
