use anyhow::bail;
use zerocopy::FromBytes;

use crate::file::IMAGE_DOS_HEADER;

pub struct DOS {
    pub header: IMAGE_DOS_HEADER,
    // pub reloc: Box<[u8]>,
}

impl DOS {
    pub fn parse(buf: &[u8]) -> anyhow::Result<DOS> {
        let header = <IMAGE_DOS_HEADER>::read_from_prefix(buf).unwrap().0;
        if header.e_magic != *b"MZ" {
            bail!(
                "invalid DOS signature; wanted 'MZ', got {:?}",
                header.e_magic
            );
        }
        Ok(DOS { header })
    }

    pub fn header_size(&self) -> usize {
        let paragraph = 16;
        self.header.e_cparhdr as usize * paragraph
    }
}
