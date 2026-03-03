#!/bin/sh

set -e

cd "$(dirname $0)"

clang -Wall -O -c -nostdlib --target=wasm32 wasm.c
clang -Wall -O -c -nostdlib --target=wasm32 kernel32.c
wasm-ld kernel32.o wasm.o --no-entry --export-dynamic --import-memory -o t.wasm
