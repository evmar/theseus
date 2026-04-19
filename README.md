# Theseus

Theseus is:

1. A compiler that translates 32-bit Windows x86 code to Rust.
2. An implementation of some of the win32 API used by the above.

Together, Theseus takes in a `.exe` file and turns it into a native binary that
doesn't depend on x86 or Windows.

Theseus is very experimental and probably won't work on a program you try.

## Project layout

- tc/: "theseus compiler", exe to Rust generator
- exe/: (mostly) generated exe code for a few test executables, the output of tc
- runtime/: runtime x86 support for the generated programs
- win32/pe/: PE (`.exe` format) parser
- win32/winapi/: Windows API implementation (e.g. kernel32, user32)
- win32/derive/: some Rust macros for winapi

The two halves of Theseus are:

1. The `tc` compiler uses the PE loader to load a `.exe` file and generate
   source code under `exe/`.
2. Building an exe relies on `runtime` for support for x86 operations and
   `win32` for Windows API.
