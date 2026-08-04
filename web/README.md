# Theseus web support

This directory contains the hosting implementation for running wasm output.

## Design notes

See <https://neugierig.org/software/blog/2026/05/theseus-wasm.html> for some
design notes particularly around how threading works.

## Developing

You need `vite` for serving. Install with `brew install vite` or `npm install`.

- `build-wasm.sh` in the root to build the wasm outputs.
- `npx vite` in this directory to run the dev server.

Vite will hot reload when you edit any input files.

# Releasing

- `npx vite build` to build the site.
- `npx vite preview` to run a web server to verify the built site works.
