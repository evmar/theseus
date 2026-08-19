//! String functions: lstr*, CompareString*.

use runtime::Context;

use crate::Ptr;

const CSTR_LESS_THAN: i32 = 1;
const CSTR_EQUAL: i32 = 2;
const CSTR_GREATER_THAN: i32 = 3;

const NORM_IGNORECASE: u32 = 1;

#[win32_derive::dllexport]
pub fn lstrlenA(ctx: &mut Context, lpString: Ptr<u8>) -> i32 {
    // A null string faults on Windows too, so let the null page guard catch it
    // rather than inventing a length.
    ctx.memory.read_str(lpString.addr).len() as i32
}

#[win32_derive::dllexport]
pub fn lstrcpyA(ctx: &mut Context, lpString1: Ptr<u8>, lpString2: Ptr<u8>) -> u32 {
    let src = ctx.memory.read_str(lpString2.addr).to_owned();
    let bytes = src.as_bytes();
    ctx.memory[lpString1.addr..][..bytes.len()].copy_from_slice(bytes);
    ctx.memory
        .write::<u8>(lpString1.addr + bytes.len() as u32, 0);
    lpString1.addr
}

#[win32_derive::dllexport]
pub fn lstrcatA(ctx: &mut Context, lpString1: Ptr<u8>, lpString2: Ptr<u8>) -> u32 {
    let dst_len = ctx.memory.read_str(lpString1.addr).len() as u32;
    let src = ctx.memory.read_str(lpString2.addr).to_owned();
    let bytes = src.as_bytes();
    ctx.memory[lpString1.addr + dst_len..][..bytes.len()].copy_from_slice(bytes);
    ctx.memory
        .write::<u8>(lpString1.addr + dst_len + bytes.len() as u32, 0);
    lpString1.addr
}

fn read_counted_a(ctx: &Context, addr: u32, count: i32) -> Vec<u8> {
    if count < 0 {
        ctx.memory.read_str(addr).as_bytes().to_vec()
    } else {
        ctx.memory[addr..][..count as usize].to_vec()
    }
}

fn read_counted_w(ctx: &Context, addr: u32, count: i32) -> Vec<u16> {
    let mut out = Vec::new();
    let mut addr = addr;
    if count < 0 {
        loop {
            let c = ctx.memory.read::<u16>(addr);
            if c == 0 {
                break;
            }
            out.push(c);
            addr += 2;
        }
    } else {
        for _ in 0..count {
            out.push(ctx.memory.read::<u16>(addr));
            addr += 2;
        }
    }
    out
}

fn compare_ordering(ord: std::cmp::Ordering) -> i32 {
    match ord {
        std::cmp::Ordering::Less => CSTR_LESS_THAN,
        std::cmp::Ordering::Equal => CSTR_EQUAL,
        std::cmp::Ordering::Greater => CSTR_GREATER_THAN,
    }
}

#[win32_derive::dllexport]
pub fn CompareStringA(
    ctx: &mut Context,
    _Locale: u32,
    dwCmpFlags: u32,
    lpString1: Ptr<u8>,
    cchCount1: i32,
    lpString2: Ptr<u8>,
    cchCount2: i32,
) -> i32 {
    let mut a = read_counted_a(ctx, lpString1.addr, cchCount1);
    let mut b = read_counted_a(ctx, lpString2.addr, cchCount2);
    if dwCmpFlags & NORM_IGNORECASE != 0 {
        a.make_ascii_lowercase();
        b.make_ascii_lowercase();
    }
    compare_ordering(a.cmp(&b))
}

#[win32_derive::dllexport]
pub fn CompareStringW(
    ctx: &mut Context,
    _Locale: u32,
    dwCmpFlags: u32,
    lpString1: Ptr<u16>,
    cchCount1: i32,
    lpString2: Ptr<u16>,
    cchCount2: i32,
) -> i32 {
    let mut a = read_counted_w(ctx, lpString1.addr, cchCount1);
    let mut b = read_counted_w(ctx, lpString2.addr, cchCount2);
    if dwCmpFlags & NORM_IGNORECASE != 0 {
        for c in a.iter_mut().chain(b.iter_mut()) {
            if *c < 0x80 {
                *c = (*c as u8).to_ascii_lowercase() as u16;
            }
        }
    }
    compare_ordering(a.cmp(&b))
}
