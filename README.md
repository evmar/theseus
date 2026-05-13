# Theseus

Theseus is:

1. A compiler that translates 32-bit Windows x86 code to Rust.
2. An implementation of some of the win32 API used by the above.

Together, Theseus takes in a `.exe` file and turns it into a native binary that
doesn't depend on x86 or Windows.

[I wrote a longer blog post motivating the project](https://neugierig.org/software/blog/2026/04/theseus.html).

Theseus is very experimental and probably won't work on a program you try.

See [doc/development.md](doc/development.md) for more details on the code.
