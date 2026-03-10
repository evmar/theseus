use crate::{ABIReturn, stub};
use runtime::{Cont, MACHINE};

#[win32_derive::dllexport]
pub fn GetLastError() -> u32 {
    0
}

pub type HANDLE = u32;

#[win32_derive::dllexport]
pub fn GetCommandLineA() -> u32 {
    0
    /*
    let mut state = get_state(sys);
    state.cmdline.cmdline8(sys.memory())
    */
}

#[win32_derive::dllexport]
pub fn GetStartupInfoA(_lpStartupInfo: u32) {
    /*
        // MSVC runtime library passes in uninitialized memory for lpStartupInfo, so don't trust info.cb.
        let info = lpStartupInfo.unwrap();
        let len = std::mem::size_of::<STARTUPINFOA>() as u32;
        unsafe { info.clear_memory(len) };

        info.cb = len;
    */
}

#[win32_derive::dllexport]
pub fn GetVersion() -> u32 {
    // Win95, version 4.0.
    (1 << 31) | 0x4
}

#[win32_derive::dllexport]
pub fn GetVersionExA(_lpVersionInformation: u32) -> bool {
    stub!(false)
    /*
    let info = lpVersionInformation.unwrap();
    if info.dwOSVersionInfoSize < std::mem::size_of::<OSVERSIONINFO>() as u32 {
        log::error!("GetVersionExA undersized buffer");
        return 0;
    }
    unsafe { info.clear_memory(info.dwOSVersionInfoSize) };

    info.dwMajorVersion = 6; // ? pulled from debugger
    info.dwPlatformId = 2 /* VER_PLATFORM_WIN32_NT */;

    1
    */
}

#[win32_derive::dllexport]
pub fn TerminateProcess(_hProcess: HANDLE, _uExitCode: u32) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_ExceptionInfo: u32) -> i32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    _lpAddress: u32,
    _dwSize: usize,
    _flAllocationType: u32, /* VIRTUAL_ALLOCATION_TYPE */
    _flProtect: u32,        /* PAGE_PROTECTION_FLAGS */
) -> u32 {
    0
    /*
    let memory = sys.memory_mut();
    if lpAddress != 0 {
        // Changing flags on an existing address, hopefully.
        match memory
            .mappings
            .vec()
            .iter()
            .find(|&mapping| mapping.contains(lpAddress))
        {
            None => {
                log::error!("failing VirtualAlloc({lpAddress:x}, ...) refers to unknown mapping");
                return 0;
            }
            Some(_) => {
                // adjusting flags on existing mapping, ignore.
                return lpAddress;
            }
        }
    }
    // TODO round dwSize to page boundary

    let mapping = memory
        .mappings
        .alloc(memory.imp.mem(), dwSize, "VirtualAlloc".into());
    mapping.addr
    */
}

#[win32_derive::dllexport]
pub fn VirtualFree(
    _lpAddress: u32,
    _dwSize: usize,
    _dwFreeType: u32, /* VIRTUAL_FREE_TYPE */
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn WideCharToMultiByte(
    _CodePage: u32,
    _dwFlags: u32,
    _lpWideCharStr: u32,
    _cchWideChar: i32,
    _lpMultiByteStr: u32,
    _cbMultiByte: i32,
    _lpDefaultChar: u32,
    _lpUsedDefaultChar: u32,
) -> i32 {
    0
    /*
    match CodePage {
        Err(value) => unimplemented!("WideCharToMultiByte code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    dwFlags.unwrap();

    let src = {
        let len = match cchWideChar {
            0 => todo!(),
            -1 => strlen16(sys.mem().slice(lpWideCharStr..)) + 1, // include nul
            len => len as usize,
        };
        sys.mem().sub32(lpWideCharStr, len as u32 * 2)
    };

    let dst = if cbMultiByte > 0 {
        sys.mem().sub32_mut(lpMultiByteStr, cbMultiByte as u32)
    } else {
        &mut []
    };

    for (i, c) in src.into_iter_pod::<u16>().enumerate() {
        if c > 0x7f {
            unimplemented!("unicode");
        }
        if i < dst.len() {
            dst[i] = c as u8;
        }
    }

    if let Some(used) = lpUsedDefaultChar {
        *used = 0;
    }

    src.len() as u32 / 2
    */
}

#[win32_derive::dllexport]
pub fn GetOEMCP() -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetACP() -> u32 {
    1252 // windows-1252
}
