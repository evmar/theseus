pub struct MMX {
    pub mm0: u64,
    pub mm1: u64,
    pub mm2: u64,
    pub mm3: u64,
    pub mm4: u64,
    pub mm5: u64,
    pub mm6: u64,
    pub mm7: u64,
}

impl MMX {
    pub const fn default() -> MMX {
        MMX {
            mm0: 0,
            mm1: 0,
            mm2: 0,
            mm3: 0,
            mm4: 0,
            mm5: 0,
            mm6: 0,
            mm7: 0,
        }
    }
}
