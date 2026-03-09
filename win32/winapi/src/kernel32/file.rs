use crate::{ABIReturn, kernel32::HANDLE};
use runtime::{Cont, HOST, Host, MACHINE};

#[win32_derive::dllexport]
pub fn GetStdHandle(_x: u32) -> u32 {
    return 0xf11e_0002;
}

#[win32_derive::dllexport]
pub fn WriteFile(
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
        unsafe {
            let buf = &MACHINE.memory.bytes[lpBuffer as usize..][..nNumberOfBytesToWrite as usize];
            HOST.print(buf);
            MACHINE
                .memory
                .write(lpNumberOfBytesWritten, nNumberOfBytesToWrite);
        }
    } else {
        todo!("WriteFile(hFile={hFile:x})");
    }
    return 1;
}

const STDIN_HFILE: HANDLE = 0xF11E_0001;
const STDOUT_HFILE: HANDLE = 0xF11E_0002;
const STDERR_HFILE: HANDLE = 0xF11E_0003;

#[win32_derive::dllexport]
pub fn GetFileType(hFile: HANDLE) -> u32 /* FILE_TYPE */ {
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
pub fn SetHandleCount(uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}
