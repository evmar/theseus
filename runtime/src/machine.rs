use bitflags::bitflags;

#[repr(C)]
#[derive(Debug)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,

    pub flags: Flags,

    pub fs_base: u32,
}

bitflags! {
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

pub static mut REGS: Regs = Regs {
    eax: 0,
    ecx: 0,
    edx: 0,
    ebx: 0,

    esi: 0,
    edi: 0,
    esp: 0x2000,
    ebp: 0x2000,

    flags: Flags::empty(),

    fs_base: 0, // set when initializing process
};

pub static mut MEMORY: *mut u8 = std::ptr::null_mut();
