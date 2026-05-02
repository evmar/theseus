bitflags::bitflags! {
    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Flags: u32 {
        /// carry
        const CF = 1 << 0;
        /// parity
        const PF = 1 << 2;
        /// zero
        const ZF = 1 << 6;
        /// sign
        const SF = 1 << 7;
        /// direction
        const DF = 1 << 10;
        /// overflow
        const OF = 1 << 11;
        /// cpuid
        const ID = 1 << 21;

        // any flag may be set by operations like SAHF
        const ALL = !0;
    }
}

impl Flags {
    pub fn to_string(&self) -> String {
        self.iter_names()
            .map(|(name, _)| name)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
