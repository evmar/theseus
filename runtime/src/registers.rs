#[repr(C)]
#[derive(Debug, Default)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,

    pub fs_base: u32,
}

impl Regs {
    pub const fn default() -> Regs {
        Regs {
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,

            esi: 0,
            edi: 0,
            esp: 0,
            ebp: 0,

            fs_base: 0, // set when initializing process
        }
    }

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
    pub fn get_si(&self) -> u16 {
        self.esi as u16
    }
    pub fn get_di(&self) -> u16 {
        self.edi as u16
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
    pub fn set_si(&mut self, val: u16) {
        self.esi = (self.esi & 0xFFFF_0000) | (val as u32);
    }
    pub fn set_di(&mut self, val: u16) {
        self.edi = (self.edi & 0xFFFF_0000) | (val as u32);
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

    pub fn dump(&self) {
        println!(
            "eax={:08x} ecx={:08x} edx={:08x} ebx={:08x}\nesi={:08x} edi={:08x} esp={:08x} ebp={:08x}",
            self.eax, self.ecx, self.edx, self.ebx, self.esi, self.edi, self.esp, self.ebp
        );
    }
}
