# EXE parser

This crate parses executable file formats:

- DOS .exe files,
- Windows Portable Executable (PE) files, .exe and .dll.

It doesn't interact with any of the emulation machinery, it just accepts byte
buffers and returns different views on to them.
