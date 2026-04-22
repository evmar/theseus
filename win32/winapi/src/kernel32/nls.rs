use runtime::Context;

use crate::stub;

#[win32_derive::dllexport]
pub fn GetACP(_ctx: &mut Context) -> u32 {
    1252 // windows-1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_ctx: &mut Context, _CodePage: u32, _lpCPInfo: u32) -> bool {
    stub!(false) // fail
}

#[win32_derive::dllexport]
pub fn GetStringTypeA(
    _ctx: &mut Context,
    _Locale: u32,
    _dwInfoType: u32,
    _lpSrcStr: u32,
    _cchSrc: i32,
    _lpCharType: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetStringTypeW(
    _ctx: &mut Context,
    _dwInfoType: u32,
    _lpSrcStr: u32,
    _cchSrc: i32,
    _lpCharType: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn LCMapStringA(
    _ctx: &mut Context,
    _Locale: u32,
    _dwMapFlags: u32,
    _lpSrcStr: u32,
    _cchSrc: i32,
    _lpDestStr: u32,
    _cchDest: i32,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn LCMapStringW(
    _ctx: &mut Context,
    _Locale: u32,
    _dwMapFlags: u32,
    _lpSrcStr: u32,
    _cchSrc: i32,
    _lpDestStr: u32,
    _cchDest: i32,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    _ctx: &mut Context,
    _CodePage: u32,
    _dwFlags: u32, /* MULTI_BYTE_TO_WIDE_CHAR_FLAGS */
    _lpMultiByteStr: u32,
    _cbMultiByte: i32,
    _lpWideCharStr: u32,
    _cchWideChar: i32,
) -> i32 {
    0
    /*
    match CodePage {
        Err(value) => unimplemented!("MultiByteToWideChar code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    // TODO: obey dwFlags
    dwFlags.unwrap();

    let src_addr = lpMultiByteStr;
    let src_len = match cbMultiByte {
        0 => return 0,                                     // TODO: invalid param
        -1 => sys.mem().slicez(src_addr).len() as u32 + 1, // include nul
        len => len as u32,
    };

    let dst = &mut lpWideCharStr;
    if let Some(buf) = dst {
        if buf.len() == 0 {
            *dst = None;
        }
    }

    // TODO: reuse the conversion in winapi/string.rs.
    match dst {
        None => src_len,
        Some(dst) => {
            let src = sys.mem().sub32(src_addr, src_len);
            let mut len = 0;
            for &c in src {
                if c > 0x7f {
                    unimplemented!("unicode");
                }
                dst.put_pod(len, c as u16);
                len += 1;
            }
            len
        }
    }
    */
}
