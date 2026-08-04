This directory contains the hosting implementation for running wasm output.

## Design notes

See <https://neugierig.org/software/blog/2026/05/theseus-wasm.html> for some
design notes particularly around how threading works.

## Developing

- Run `build-wasm.sh` in the root to build the wasm outputs.
- Run `tsc` in this directory to generate the JS output. (TODO: create a
  package.json etc. to use a local tsc.)
- Run `go run static-server.go` to run a local HTTP server to serve the wasm.
  (TODO: maybe switch to vite?)
