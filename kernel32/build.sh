#!/bin/sh

set -e

cd "$(dirname $0)"

rustc -O --target wasm32-unknown-unknown --crate-type staticlib --emit obj -C panic=abort src/lib.rs
