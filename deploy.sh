#!/bin/sh

set -e

./build-wasm.sh --release

rm -rf deploy/*
touch deploy/.nojekyll
cp web/{*.js,*.html} deploy/
for exe in $(ls web/exe); do
    mkdir -p deploy/exe/$exe
    cp web/exe/$exe/{*.wasm,*.js} deploy/exe/$exe/
done
