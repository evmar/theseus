This crate is the Theseus implementation of (the beginnings of) the win32 API.

Modules like `ddraw/`, `gdi32/`, etc. are implementing the corresponding Windows
DLL. The code generator, when seeing a call to a function like `DrawText()` in
`gdi32.dll`, will emit function call text like `gdi32::DrawText()`, so the two
are only linked together by having the names match.

The `host/` directory contains the host implementation, which is responsible for
forwarding calls to the appropriate platform-specific implementation. It's
implemented for SDL (native) and browser (wasm, with corresponding code in the
root `web/` directory).
