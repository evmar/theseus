#!/bin/sh

set -e

cd "$(dirname $0)"

cargo build --profile release --target wasm32-unknown-unknown -p winapi-exe
