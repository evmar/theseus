This crate is the Theseus implementation of the beginnings of the win32 API.

Currently it maps some graphics calls to SDL. In retrowin32 there was a `Host`
trait that was used to forward calls either to SDL (when run natively) or
browser APIs (when run under wasm). I started implementing that here, but
decided the complexity isn't needed for now.
