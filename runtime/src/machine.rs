use crate::Cont;
use bitflags::bitflags;
use zerocopy::{FromBytes, IntoBytes};

pub struct Machine {
    pub regs: Regs,
    pub memory: Memory,
    pub blocks: &'static [(u32, fn() -> Cont)],
}

pub struct Memory {
    // TODO: this could be a slice owned by Memory, except that
    // we want to store a Machine in a static.
    pub bytes: &'static mut [u8],
}

impl Memory {
    pub fn read<T: FromBytes>(&self, addr: u32) -> T {
        T::read_from_prefix(&self.bytes[addr as usize..]).unwrap().0
    }

    pub fn write<T: IntoBytes + zerocopy::Immutable>(&mut self, addr: u32, val: T) {
        val.write_to_prefix(&mut self.bytes[addr as usize..])
            .unwrap();
    }
}

pub fn indirect(addr: u32) -> Cont {
    if addr == 0 {
        {
            panic!("null ptr");
        }
    }
    unsafe {
        let index = MACHINE
            .blocks
            .binary_search_by_key(&addr, |(addr, _)| *addr)
            .unwrap_or_else(|_| panic!("jmp to unknown addr {addr:#08x}"));
        Cont(MACHINE.blocks[index].1)
    }
}

pub static mut MACHINE: Machine = Machine {
    regs: Regs {
        eax: 0,
        ecx: 0,
        edx: 0,
        ebx: 0,

        esi: 0,
        edi: 0,
        esp: 0,
        ebp: 0,

        flags: Flags::empty(),

        fs_base: 0, // set when initializing process
    },
    memory: Memory { bytes: &mut [] },
    blocks: &[],
};

impl Regs {
    pub fn get_ax(&self) -> u16 {
        self.eax as u16
    }
    pub fn get_cx(&self) -> u16 {
        self.ecx as u16
    }
    pub fn get_dx(&self) -> u16 {
        self.edx as u16
    }
    pub fn get_bx(&self) -> u16 {
        self.ebx as u16
    }

    pub fn set_ax(&mut self, val: u16) {
        self.eax = (self.eax & 0xFFFF_0000) | (val as u32);
    }
    pub fn set_cx(&mut self, val: u16) {
        self.ecx = (self.ecx & 0xFFFF_0000) | (val as u32);
    }
    pub fn set_dx(&mut self, val: u16) {
        self.edx = (self.edx & 0xFFFF_0000) | (val as u32);
    }
    pub fn set_bx(&mut self, val: u16) {
        self.ebx = (self.ebx & 0xFFFF_0000) | (val as u32);
    }

    pub fn get_al(&self) -> u8 {
        self.eax as u8
    }
    pub fn get_cl(&self) -> u8 {
        self.ecx as u8
    }
    pub fn get_dl(&self) -> u8 {
        self.edx as u8
    }
    pub fn get_bl(&self) -> u8 {
        self.ebx as u8
    }

    pub fn set_al(&mut self, val: u8) {
        self.eax = (self.eax & 0xFFFF_FF00) | (val as u32)
    }
    pub fn set_cl(&mut self, val: u8) {
        self.ecx = (self.ecx & 0xFFFF_FF00) | (val as u32)
    }
    pub fn set_dl(&mut self, val: u8) {
        self.edx = (self.edx & 0xFFFF_FF00) | (val as u32)
    }
    pub fn set_bl(&mut self, val: u8) {
        self.ebx = (self.ebx & 0xFFFF_FF00) | (val as u32)
    }

    pub fn get_ah(&self) -> u8 {
        (self.eax >> 8) as u8
    }
    pub fn get_ch(&self) -> u8 {
        (self.ecx >> 8) as u8
    }
    pub fn get_dh(&self) -> u8 {
        (self.edx >> 8) as u8
    }
    pub fn get_bh(&self) -> u8 {
        (self.ebx >> 8) as u8
    }

    pub fn set_ah(&mut self, val: u8) {
        self.eax = (self.eax & 0xFFFF_00FF) | ((val as u32) << 8)
    }
    pub fn set_ch(&mut self, val: u8) {
        self.ecx = (self.ecx & 0xFFFF_00FF) | ((val as u32) << 8)
    }
    pub fn set_dh(&mut self, val: u8) {
        self.edx = (self.edx & 0xFFFF_00FF) | ((val as u32) << 8)
    }
    pub fn set_bh(&mut self, val: u8) {
        self.ebx = (self.ebx & 0xFFFF_00FF) | ((val as u32) << 8)
    }
}

impl Machine {
    pub fn dump_state(&self) {
        println!(
            "eax={:08x} ecx={:08x} edx={:08x} ebx={:08x}\nesi={:08x} edi={:08x} esp={:08x} ebp={:08x}",
            self.regs.eax,
            self.regs.ecx,
            self.regs.edx,
            self.regs.ebx,
            self.regs.esi,
            self.regs.edi,
            self.regs.esp,
            self.regs.ebp
        );
        println!("stack:");
        for i in 0..8 {
            let addr = self.regs.esp + i * 4;
            println!("{addr:#08x} {:#08x}", self.memory.read::<u32>(addr));
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
