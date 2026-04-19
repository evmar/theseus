This crate implements the Theseus Compiler "tc", which consumes a `.exe` file
and generates Rust source code.

CPU operations are generally parsed here, and the generated code calls a
corresponding implementation from the `runtime` crate. For example, given
`inc eax`, this code might generate a call like
`runtime::inc(ctx.cpu.regs.eax)`.
