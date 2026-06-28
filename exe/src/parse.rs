//! Parsing PE files.
//!
//! Caller must call the functions in the proper sequence to successfully parse;
//! use File::parse() for the simpler interface.

use anyhow::bail;
use zerocopy::FromBytes;

use crate::{
    IMAGE_DATA_DIRECTORY, IMAGE_DOS_HEADER, IMAGE_NT_HEADERS32, IMAGE_SECTION_HEADER,
    iter::iter_pod_n,
};

pub fn dos_header(buf: &[u8]) -> anyhow::Result<IMAGE_DOS_HEADER> {
    let header = <IMAGE_DOS_HEADER>::read_from_prefix(buf).unwrap().0;
    if header.e_magic.as_slice() != b"MZ" {
        bail!(
            "invalid DOS signature; wanted 'MZ', got {:?}",
            header.e_magic
        );
    }
    Ok(header)
}

pub fn pe_header(buf: &[u8]) -> anyhow::Result<(IMAGE_NT_HEADERS32, &[u8])> {
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

pub fn data_directory<'a>(
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

pub fn sections<'a>(
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
