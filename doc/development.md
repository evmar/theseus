# Development

## Project layout

- tc/: "theseus compiler", exe to Rust generator
- exe/: (mostly) generated exe code for a few test executables, the output of tc
- runtime/: runtime x86 support for the generated programs
- web/: web hosting implementation for running wasm output
- win32/pe/: PE (`.exe` format) parser
- win32/winapi/: Windows API implementation (e.g. kernel32, user32)
- win32/derive/: some Rust macros for winapi
- dos/: (work in progress) DOS support
- host/: the host environment (SDL or web), used by winapi and DOS

The two halves of Theseus are:

1. The `tc` compiler uses the PE loader to load a `.exe` file and generate
   source code under `exe/`.
2. Building an exe relies on `runtime` for support for x86 operations and
   `win32` for Windows API or `dos` for DOS support.

## Profiling

```
$ cargo instruments -t time -p chillin --profile release --time-limit 10000
```

Run `dsymutil target/release/chillin` to get source info in the profile output.
