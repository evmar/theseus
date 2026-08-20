//! A representation for the instruction pointer, abstracted over 16/32-bit.
//!
//! In particular, we preserve the seg:ofs of 16-bit IPs, because we want the
//! generated code to generate seg:ofs function names like x0823_1234.

use runtime::SegOfs;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum IP {
    Flat(u32),
    Seg(SegOfs),
}

impl From<u32> for IP {
    fn from(addr: u32) -> Self {
        IP::Flat(addr)
    }
}

impl From<(u16, u16)> for IP {
    fn from(tuple: (u16, u16)) -> Self {
        IP::Seg(tuple.into())
    }
}

impl IP {
    pub fn seg(&self) -> u16 {
        match *self {
            IP::Flat(_) => unreachable!(),
            IP::Seg(addr) => addr.seg,
        }
    }

    pub fn to_addr(&self) -> u32 {
        match *self {
            IP::Flat(addr) => addr,
            IP::Seg(addr) => addr.abs(),
        }
    }

    pub fn local(&self) -> u32 {
        match *self {
            IP::Flat(ip) => ip,
            IP::Seg(addr) => addr.ofs as u32,
        }
    }

    pub fn with_local(&self, local: u32) -> IP {
        match *self {
            IP::Flat(_) => IP::Flat(local),
            IP::Seg(addr) => IP::Seg((addr.seg, local as u16).into()),
        }
    }
}

impl std::fmt::Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IP::Flat(ip) => write!(f, "{ip:08x}"),
            IP::Seg(addr) => write!(f, "{addr}"),
        }
    }
}

impl std::fmt::Debug for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
