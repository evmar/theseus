This crate defines two Rust macros used for implementing win32 APIs:

1. `#[dllexport]`, which for a function `Foo` creates `Foo_stdcall` that knows
   how to serialize arguments to/from the `runtime` virtual CPU.
2. `#[abi_enum]`, which implements a `from_abi()` helper on enums that maps from
   integer values (as found in e.g. function arguments) to the Rust enum type.
