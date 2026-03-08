#!/bin/sh

set -e

cargo run -p fc  ~/win/rs/exe/winapi/winapi.exe exe/winapi
cargo build -p winapi-exe "$@"
