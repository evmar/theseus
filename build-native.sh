#!/bin/sh

set -e

cargo run -p fc ~/win/rs/deploy/archive/zip.exe exe/zip
cargo build -p zip-exe "$@"
