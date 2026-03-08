Theseus translates Windows x86 code to Rust, one instruction at a time.

## Project layout

fc/: exe to Rust generator
runtime/: runtime support for generated binary
kernel32/: implementation of Windows kernel32 functions
exe/: generated exe code for a few test executables
