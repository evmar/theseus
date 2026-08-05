mod exe;
mod flags;
mod fpu;
mod machine;
mod mapping;
mod memory;
mod mmx;
mod ops;
mod registers;

pub use exe::EXEData;
pub use flags::Flags;
pub use machine::{BlockCache, CPU, Context};
pub use mapping::{Mapping, Mappings, round_to_page};
pub use memory::Memory;
pub use ops::*;
pub use registers::Regs;

#[repr(C)]
#[derive(
    zerocopy::FromBytes, zerocopy::IntoBytes, Debug, Clone, Copy, PartialEq, PartialOrd, Eq,
)]
pub struct SegOfs {
    pub ofs: u16,
    pub seg: u16,
}

impl SegOfs {
    pub const fn new(seg: u16, ofs: u16) -> SegOfs {
        SegOfs { seg, ofs }
    }

    pub const fn abs(&self) -> u32 {
        segofs(self.seg, self.ofs)
    }
}

impl From<(u16, u16)> for SegOfs {
    fn from((seg, ofs): (u16, u16)) -> Self {
        SegOfs::new(seg, ofs)
    }
}

impl std::fmt::Display for SegOfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{seg:04x}:{ofs:04x}", seg = self.seg, ofs = self.ofs)
    }
}

pub type ContFn = fn(&mut Context) -> Cont;

#[derive(Clone, Copy)]
pub struct Cont(pub ContFn);

/// When making a call from host to to x86 code, we need a valid return address
/// that is associated with a real function so that the final 'ret' from the
/// called function succeeds, but we never invoke it.
pub const RETURN_FROM_X86_ADDR32: u32 = 0xffff_fffe;
pub const RETURN_FROM_X86_ADDR16: SegOfs = SegOfs::new(0xffff, 0xfffe);

/// Record a code address the static analysis missed, so it can be fed back
/// into tc via --entry-points-file. Set THESEUS_MISSING_ADDRS to a file path.
pub fn log_missing_addr(addr: u32) {
    #[cfg(not(target_family = "wasm"))]
    if let Ok(path) = std::env::var("THESEUS_MISSING_ADDRS") {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            let _ = writeln!(f, "{addr:x}");
        }
    }
}

/// Called by generated code for a statically-known jump target that the
/// analysis didn't produce a block for.
pub fn unknown_block(addr: u32) -> Cont {
    log_missing_addr(addr);
    panic!(
        "jmp to unknown block {addr:#010x}; \
         re-run tc with --entry-points-file (see THESEUS_MISSING_ADDRS)"
    );
}

impl Context {
    /// Call an x86 stdcall function, only returning once the function returns.
    pub fn call32_x86(&mut self, f: Cont, args: Vec<u32>) {
        let esp = self.cpu.regs.esp;
        for arg in args.into_iter().rev() {
            self.push32(arg);
        }
        // Note that return_from_x86 is never called.  When the x86 code returns
        // to it, the stack will have been popped so that esp matches our initial
        // esp and we abort the loop before invoking the continuation.
        self.push32(RETURN_FROM_X86_ADDR32);

        self.cpu_loop(f, esp);
    }

    pub fn cpu_loop(&mut self, mut f: Cont, target_esp: u32) {
        let mut i = 0;
        while self.cpu.regs.esp != target_esp {
            self.recent[i] = f.0;
            i = (i + 1) % self.recent.len();
            f = f.0(self);
        }
    }

    pub fn return_from_x86(&mut self) -> Cont {
        panic!();
    }
}

/// Combine a seg:ofs address into a single flat u32 address.
pub const fn segofs(seg: u16, off: u16) -> u32 {
    ((seg as u32) << 4) + (off as u32)
}
