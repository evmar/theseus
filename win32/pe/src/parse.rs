//! Parsing PE files.
//!
//! Caller must call the functions in the proper sequence to successfully parse;
//! use File::parse() for the simpler interface.

use crate::{IMAGE_DATA_DIRECTORY, IMAGE_NT_HEADERS32, IMAGE_SECTION_HEADER, iter_pod_n};
use anyhow::bail;
use zerocopy::FromBytes;

pub fn dos_header(buf: &[u8]) -> anyhow::Result<u32> {
    let sig = &buf[0..2];
    if sig != b"MZ" {
        bail!("invalid DOS signature; wanted 'MZ', got {:?}", sig);
    }
    let pe_offset = <u32>::read_from_bytes(&buf[0x3c..][..4]).unwrap();
    if pe_offset > buf.len() as u32 {
        bail!("invalid PE offset in DOS header, might be a DOS executable?");
    }
    Ok(pe_offset)
}

pub fn pe_header(buf: &[u8], ofs: &mut u32) -> anyhow::Result<IMAGE_NT_HEADERS32> {
    let header = <IMAGE_NT_HEADERS32>::read_from_prefix(&buf[*ofs as usize..])
        .unwrap()
        .0;
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
    *ofs += std::mem::size_of::<IMAGE_NT_HEADERS32>() as u32;

    Ok(header)
}

pub fn data_directory(
    header: &IMAGE_NT_HEADERS32,
    buf: &[u8],
    ofs: &mut u32,
) -> anyhow::Result<Box<[IMAGE_DATA_DIRECTORY]>> {
    let data_directory =
        iter_pod_n::<IMAGE_DATA_DIRECTORY>(buf, *ofs, header.OptionalHeader.NumberOfRvaAndSizes)
            .collect();
    *ofs += std::mem::size_of::<IMAGE_DATA_DIRECTORY>() as u32
        * header.OptionalHeader.NumberOfRvaAndSizes;
    Ok(data_directory)
}

pub fn sections(
    header: &IMAGE_NT_HEADERS32,
    buf: &[u8],
    ofs: &mut u32,
) -> anyhow::Result<Box<[IMAGE_SECTION_HEADER]>> {
    let sections =
        iter_pod_n::<IMAGE_SECTION_HEADER>(buf, *ofs, header.FileHeader.NumberOfSections as u32)
            .collect();
    *ofs += std::mem::size_of::<IMAGE_SECTION_HEADER>() as u32
        * header.FileHeader.NumberOfSections as u32;
    Ok(sections)
}
