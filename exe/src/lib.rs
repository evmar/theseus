mod exports;
mod file;
mod imports;
mod iter;
mod parse;
mod relocations;
mod resources;

pub use exports::*;
pub use file::{IMAGE_DIRECTORY_ENTRY, IMAGE_SCN, PE};
pub use imports::*;
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
}
pub fn parse(buf: &[u8]) -> anyhow::Result<Parse> {
    let pe = parse::parse(buf)?;
    Ok(Parse::PE(pe))
}
