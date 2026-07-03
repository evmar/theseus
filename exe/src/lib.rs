mod dos;
mod exports;
mod file;
mod imports;
mod iter;
pub mod pe;
mod relocations;
mod resources;

use anyhow::anyhow;
pub use dos::DOS;
pub use exports::*;
pub use imports::*;
pub use pe::PE;
pub use relocations::*;
pub use resources::*;

/// Read a C-style nul terminated string from a buffer.
/// Various PE structures use these, sometimes with an optional nul.
pub(crate) fn c_str(buf: &[u8]) -> &[u8] {
    let len = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
    &buf[..len]
}

pub enum Parse {
    PE(PE),
    DOS(DOS),
}

pub fn parse(buf: &[u8]) -> anyhow::Result<Parse> {
    let dos = DOS::parse(buf).map_err(|err| anyhow!("reading DOS header: {}", err))?;

    let pe_offset = dos.header.e_lfanew as usize;
    if pe_offset < buf.len() && pe::has_pe_signature(&buf[pe_offset..]) {
        let pe = PE::parse(&buf[pe_offset..])?;
        Ok(Parse::PE(pe))
    } else {
        Ok(Parse::DOS(dos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::IMAGE_SECTION_HEADER;

    #[test]
    fn kkrunchy_header() {
        let mut header = IMAGE_SECTION_HEADER::default();
        header.Name = *b"kkrunchy";
        assert_eq!(header.name().unwrap(), "kkrunchy");
    }

    use std::io::Write;

    #[test]
    fn dos_header() {
        let mut buf: Vec<u8> = Vec::new();
        buf.write(b"MZ").unwrap();
        buf.write(&[0; 0x3a]).unwrap();
        buf.write(&0xFFFFFFFFu32.to_le_bytes()).unwrap();
        assert!(parse(&buf).is_ok()); // no crash
    }
}
