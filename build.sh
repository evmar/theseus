#!/bin/sh

set -e

cd "$(dirname $0)"

cargo run -p fc ~/win/rs/exe/winapi/winapi.exe exe/winapi
cargo build --profile release --target wasm32-unknown-unknown -p winapi-exe
