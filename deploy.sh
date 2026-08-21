#!/bin/sh

set -e

./build-wasm.sh --release
(cd web && npx vite build)

rm -rf deploy/*
touch deploy/.nojekyll
cp -r web/dist/* deploy/
