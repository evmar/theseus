#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use zerocopy::FromBytes;

use crate::iter_pod;

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes)]
struct IMAGE_BASE_RELOCATION {
    VirtualAddress: u32,
    SizeOfBlock: u32,
}

/// Iterates IMAGE_BASE_RELOCATION+body blocks.
fn block_iter(mut buf: &[u8]) -> impl Iterator<Item = (u32, &[u8])> {
    std::iter::from_fn(move || {
        if buf.len() == 0 {
            return None;
        }
        let reloc = <IMAGE_BASE_RELOCATION>::read_from_prefix(buf).unwrap().0;
        if reloc.VirtualAddress == 0 {
            // fmod.dll has a block with addr=0, size=8 (header size), and then trailing garbage after that
            return None;
        }
        let body = &buf[std::mem::size_of::<IMAGE_BASE_RELOCATION>()..reloc.SizeOfBlock as usize];
        buf = &buf[reloc.SizeOfBlock as usize..];
        Some((reloc.VirtualAddress, body))
    })
}

pub fn apply_relocs(
    prev_base: u32,
    base: u32,
    relocs: &[u8],
    mut read: impl FnMut(u32) -> u32,
    mut write: impl FnMut(u32, u32),
) {
    // monolife.exe has no IMAGE_DIRECTORY_ENTRY::BASERELOC, but does
    // have a .reloc section that is invalid (?).
    // Note: IMAGE_SECTION_HEADER itself also has some relocation-related fields
    // that appear to only apply to object files (?).

    let offset = base.wrapping_sub(prev_base);

    for (addr, body) in block_iter(relocs) {
        for entry in iter_pod::<u16>(body) {
            let etype = entry >> 12;
            let ofs = entry & 0x0FFF;
            let addr = addr + ofs as u32;
            match etype {
                0 => {} // skip
                3 => {
                    // IMAGE_REL_BASED_HIGHLOW, 32-bit adjustment
                    // win2k's glu32.dll has a relocation offsetting the value 0x6fa7a09
                    // despite the image base being 0x6fac000, so it is a reference to memory
                    // before the image?!
                    let old = read(addr);
                    let new = old.wrapping_add(offset);
                    write(addr, new);
                }
                _ => panic!("unhandled relocation type {etype}"),
            }
        }
    }
}
