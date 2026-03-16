Theseus translates Windows x86 code to Rust, one instruction at a time.

## Project layout

tc/: exe to Rust generator
runtime/: runtime support for generated binary
win32/winapi/: implementation of Windows kernel32 functions
exe/: generated exe code (output of tc) for a few test executables
