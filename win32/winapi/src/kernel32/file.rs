use runtime::Context;
use runtime::{HOST, Host};

use crate::kernel32::HANDLE;

const STDIN_HFILE: HANDLE = 0xF11E_0001;
const STDOUT_HFILE: HANDLE = 0xF11E_0002;
const STDERR_HFILE: HANDLE = 0xF11E_0003;

#[win32_derive::dllexport]
pub fn GetStdHandle(_ctx: &mut Context, nStdHandle: u32) -> u32 {
    match nStdHandle as i32 {
        -10 => STDIN_HFILE,
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => {
            log::error!("GetStdHandle: invalid handle");
            0
        }
    }
}

#[win32_derive::dllexport]
pub fn WriteFile(
    ctx: &mut Context,
    hFile: u32,
    lpBuffer: u32,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: u32,
    lpOverlapped: u32,
) -> u32 {
    assert_eq!(lpOverlapped, 0);
    if hFile == 0xf11e_0002 || hFile == 0xf11e_0003 {
        let buf = &ctx.memory[lpBuffer..][..nNumberOfBytesToWrite as usize];
        HOST.print(buf);
        if lpNumberOfBytesWritten != 0 {
            ctx.memory
                .write(lpNumberOfBytesWritten, nNumberOfBytesToWrite);
        }
    } else {
        todo!("WriteFile(hFile={hFile:x})");
    }
    return 1;
}

#[win32_derive::dllexport]
pub fn GetFileType(_ctx: &mut Context, hFile: HANDLE) -> u32 /* FILE_TYPE */ {
    let FILE_TYPE_CHAR = 0x2;
    let FILE_TYPE_UNKNOWN = 0x8;
    match hFile {
        STDIN_HFILE | STDOUT_HFILE | STDERR_HFILE => return FILE_TYPE_CHAR,
        _ => {}
    }
    /*
    if get_state(sys).files.get(hFile).is_some() {
        return FILE_TYPE_CHAR;
    }
    */

    log::error!("GetFileType({hFile:?}) unknown handle");
    FILE_TYPE_UNKNOWN
}

#[win32_derive::dllexport]
pub fn SetHandleCount(_ctx: &mut Context, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}
