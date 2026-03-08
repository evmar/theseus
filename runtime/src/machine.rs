use bitflags::bitflags;

pub struct Machine {
    pub regs: Regs,
    pub memory: *mut u8,
}
pub static mut MACHINE: Machine = Machine {
    regs: Regs {
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
    },
    memory: std::ptr::null_mut(),
};

impl Machine {
    pub fn dump_state(&self) {
        unsafe {
            println!(
                "eax={:08x} ecx={:08x} edx={:08x} ebx={:08x}",
                self.regs.eax, self.regs.ecx, self.regs.edx, self.regs.ebx
            );
            println!("stack:");
            for i in 0..8 {
                let addr = self.regs.esp + i * 4;
                println!(
                    "{addr:#08x} {:#08x}",
                    *(self.memory.add(addr as usize) as *const u32)
                );
            }
        }
    }
}

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
