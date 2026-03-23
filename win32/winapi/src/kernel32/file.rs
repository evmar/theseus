use runtime::Machine;
use runtime::{HOST, Host};

use crate::kernel32::HANDLE;

const STDIN_HFILE: HANDLE = 0xF11E_0001;
const STDOUT_HFILE: HANDLE = 0xF11E_0002;
const STDERR_HFILE: HANDLE = 0xF11E_0003;

#[win32_derive::dllexport]
pub fn GetStdHandle(_m: &mut Machine, nStdHandle: u32) -> u32 {
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
    m: &mut Machine,
    hFile: u32,
    lpBuffer: u32,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: u32,
    o: u32,
) -> u32 {
    let buf = format!(
        "WriteFile({hFile:x} {lpBuffer:x} {nNumberOfBytesToWrite:x} {lpNumberOfBytesWritten:x} {o:x})\n"
    );
    HOST.print(buf.as_bytes());

    if hFile == 0xf11e_0002 || hFile == 0xf11e_0003 {
        let buf = &m.memory.bytes[lpBuffer as usize..][..nNumberOfBytesToWrite as usize];
        HOST.print(buf);
        if lpNumberOfBytesWritten != 0 {
            m.memory
                .write(lpNumberOfBytesWritten, nNumberOfBytesToWrite);
        }
    } else {
        todo!("WriteFile(hFile={hFile:x})");
    }
    return 1;
}

#[win32_derive::dllexport]
pub fn GetFileType(_m: &mut Machine, hFile: HANDLE) -> u32 /* FILE_TYPE */ {
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
pub fn SetHandleCount(_m: &mut Machine, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}
