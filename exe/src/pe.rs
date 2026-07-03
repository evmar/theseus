//! Parsing PE files.

use anyhow::{anyhow, bail};
use zerocopy::FromBytes;

pub use crate::file::*;
use crate::iter::iter_pod_n;

#[derive(Debug)]
pub struct PE {
    pub header: IMAGE_FILE_HEADER,
    pub opt_header: IMAGE_OPTIONAL_HEADER32,
    pub data_directory: Box<[IMAGE_DATA_DIRECTORY]>,
    pub sections: Box<[IMAGE_SECTION_HEADER]>,
}

impl PE {
    pub fn get_data_directory(
        &self,
        entry: IMAGE_DIRECTORY_ENTRY,
    ) -> Option<&IMAGE_DATA_DIRECTORY> {
        let dir = self.data_directory.get(entry as usize)?;
        if dir.VirtualAddress == 0 {
            return None;
        }
        Some(dir)
    }
}

pub fn has_pe_signature(buf: &[u8]) -> bool {
    buf[..4] == *b"PE\0\0"
}

fn pe_header(buf: &[u8]) -> anyhow::Result<(IMAGE_NT_HEADERS32, &[u8])> {
    let (header, buf) = <IMAGE_NT_HEADERS32>::read_from_prefix(buf).unwrap();
    if header.Signature != *b"PE\0\0" {
        bail!(
            "invalid PE signature; wanted 'PE\\0\\0', got {:x?}",
            header.Signature
        );
    }
    let machine_i386 = 0x14c;
    if header.FileHeader.Machine != machine_i386 {
        bail!(
            "bad machine; wanted {machine_i386:x}, got {:x?}",
            header.FileHeader.Machine
        );
    }
    Ok((header, buf))
}

fn pe_data_directory<'a>(
    header: &IMAGE_NT_HEADERS32,
    buf: &'a [u8],
) -> anyhow::Result<(Box<[IMAGE_DATA_DIRECTORY]>, &'a [u8])> {
    let data_directory =
        iter_pod_n::<IMAGE_DATA_DIRECTORY>(buf, 0, header.OptionalHeader.NumberOfRvaAndSizes)
            .collect();
    let buf = &buf[(std::mem::size_of::<IMAGE_DATA_DIRECTORY>()
        * header.OptionalHeader.NumberOfRvaAndSizes as usize)..];
    Ok((data_directory, buf))
}

fn pe_sections<'a>(
    header: &IMAGE_NT_HEADERS32,
    buf: &'a [u8],
) -> anyhow::Result<(Box<[IMAGE_SECTION_HEADER]>, &'a [u8])> {
    let sections =
        iter_pod_n::<IMAGE_SECTION_HEADER>(buf, 0, header.FileHeader.NumberOfSections as u32)
            .collect();
    let buf = &buf[(std::mem::size_of::<IMAGE_SECTION_HEADER>()
        * header.FileHeader.NumberOfSections as usize)..];
    Ok((sections, buf))
}

impl PE {
    pub fn parse(buf: &[u8]) -> anyhow::Result<PE> {
        let (header, buf) = pe_header(buf).map_err(|err| anyhow!("reading PE header: {}", err))?;
        let (data_directory, buf) = pe_data_directory(&header, buf)
            .map_err(|err| anyhow!("reading data directory: {}", err))?;
        let (sections, _) =
            pe_sections(&header, buf).map_err(|err| anyhow!("reading sections: {}", err))?;
        Ok(PE {
            header: header.FileHeader,
            opt_header: header.OptionalHeader,
            data_directory,
            sections,
        })
    }
}
