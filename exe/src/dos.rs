use anyhow::bail;
use zerocopy::{FromBytes, IntoBytes};

use crate::{file::IMAGE_DOS_HEADER, iter::iter_pod_n};

#[derive(Debug, zerocopy::FromBytes)]
#[repr(C)]
pub struct Reloc {
    ofs: u16,
    seg: u16,
}

#[derive(Debug)]
pub struct DOS {
    pub header: IMAGE_DOS_HEADER,
    pub relocs: Box<[Reloc]>,
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

        let reloc_len = header.e_crlc;
        let relocs = if reloc_len > 0 {
            iter_pod_n::<Reloc>(buf, header.e_lfarlc as u32, reloc_len as u32).collect::<Vec<_>>()
        } else {
            vec![]
        }
        .into_boxed_slice();

        Ok(DOS { header, relocs })
    }

    pub fn header_size(&self) -> usize {
        let paragraph = 16;
        self.header.e_cparhdr as usize * paragraph
    }

    pub fn apply_relocations(&self, seg: u16, mem: &mut [u8]) {
        for reloc in &self.relocs {
            let ofs = ((reloc.seg as u32) << 4) + reloc.ofs as u32;
            let mem = &mut mem[ofs as usize..][..2];
            let prev = <u16>::read_from_bytes(mem).unwrap();
            let new = prev + seg;
            new.write_to(mem).unwrap();
        }
    }
}
