use runtime::Context;

use crate::{kernel32::lock, stub};

#[win32_derive::dllexport]
pub fn GetLastError(_ctx: &mut Context) -> u32 {
    0
}

pub type HANDLE = u32;

#[repr(C)]
#[derive(Debug, Default, zerocopy::IntoBytes, zerocopy::Immutable)]
pub struct STARTUPINFOA {
    cb: u32,
    lpReserved: u32,
    lpDesktop: u32,
    lpTitle: u32,
    dwX: u32,
    dwY: u32,
    dwXSize: u32,
    dwYSize: u32,
    dwXCountChars: u32,
    dwYCountChars: u32,
    dwFillAttribute: u32,
    dwFlags: u32,
    wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: u32,
    hStdInput: u32,
    hStdOutput: u32,
    hStdError: u32,
}

#[win32_derive::dllexport]
pub fn GetStartupInfoA(ctx: &mut Context, lpStartupInfo: u32) {
    let size = ctx.memory.read::<u32>(lpStartupInfo);
    if size > 0 && size < std::mem::size_of::<STARTUPINFOA>() as u32 {
        log::error!("GetStartupInfoA: undersized buffer");
        return;
    }

    let info = STARTUPINFOA {
        ..Default::default()
    };
    ctx.memory.write(lpStartupInfo, info);
}

#[win32_derive::dllexport]
pub fn GetVersion(_ctx: &mut Context) -> u32 {
    // Win95, version 4.0.
    (1 << 31) | 0x4
}

#[repr(C)]
#[derive(Debug, Default, zerocopy::IntoBytes, zerocopy::Immutable)]
pub struct OSVERSIONINFO {
    dwOSVersionInfoSize: u32,
    dwMajorVersion: u32,
    dwMinorVersion: u32,
    dwBuildNumber: u32,
    dwPlatformId: u32,
    //szCSDVersion: [u8; 128],
}

#[win32_derive::dllexport]
pub fn GetVersionExA(ctx: &mut Context, lpVersionInformation: u32) -> bool {
    let size = ctx.memory.read::<u32>(lpVersionInformation);
    if size < std::mem::size_of::<OSVERSIONINFO>() as u32 {
        log::error!("GetVersionExA undersized buffer");
        return false;
    }

    let info = OSVERSIONINFO {
        dwMajorVersion: 6, // ? pulled from debugger
        dwPlatformId: 2,   /* VER_PLATFORM_WIN32_NT */
        ..Default::default()
    };
    ctx.memory.write(lpVersionInformation, info);

    true
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_ctx: &mut Context, _ExceptionInfo: u32) -> i32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    _ctx: &mut Context,
    lpAddress: u32,
    dwSize: u32,
    _flAllocationType: u32, /* VIRTUAL_ALLOCATION_TYPE */
    _flProtect: u32,        /* PAGE_PROTECTION_FLAGS */
) -> u32 {
    assert_eq!(lpAddress, 0);
    lock().mappings.alloc("VirtualAlloc".into(), None, dwSize)
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
    _ctx: &mut Context,
    _lpAddress: u32,
    _dwSize: u32,
    _dwFreeType: u32, /* VIRTUAL_FREE_TYPE */
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn WideCharToMultiByte(
    _ctx: &mut Context,
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
pub fn GetOEMCP(_ctx: &mut Context) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn OutputDebugStringA(_ctx: &mut Context, _lpOutputString: u32) {
    todo!()
}

#[win32_derive::dllexport]
pub fn RtlUnwind(
    _ctx: &mut Context,
    _TargetFrame: u32,
    _TargetIp: u32,
    _ExceptionRecord: u32,
    _ReturnValue: u32,
) {
    todo!()
}

/// GetTickCount wants the ticks since the process started.
/// To make this a simple read, we unsafely store the current instant during initialization.
/// Note that this is never mutated after process startup, so it is correctly Sync
/// after that point.
pub struct UnsafeTickCount(std::time::Instant);
unsafe impl Sync for UnsafeTickCount {}

impl UnsafeTickCount {
    pub const fn new_uninitialized() -> Self {
        unsafe { UnsafeTickCount(std::mem::MaybeUninit::zeroed().assume_init()) }
    }

    pub fn init() {
        unsafe {
            START_TICK_COUNT.0 = std::time::Instant::now();
        }
    }

    pub fn get() -> std::time::Instant {
        unsafe { START_TICK_COUNT.0 }
    }
}
static mut START_TICK_COUNT: UnsafeTickCount = UnsafeTickCount::new_uninitialized();

pub fn get_tick_count() -> u32 {
    UnsafeTickCount::get().elapsed().as_millis() as u32
}

#[win32_derive::dllexport]
pub fn GetTickCount(_ctx: &mut Context) -> u32 {
    get_tick_count()
}

#[win32_derive::dllexport]
pub fn WaitForSingleObject(_ctx: &mut Context, _hHandle: HANDLE, _dwMilliseconds: u32) -> u32 /* WAIT_EVENT */
{
    todo!()
}

#[win32_derive::dllexport]
pub fn Sleep(_ctx: &mut Context, dwMilliseconds: u32) {
    std::thread::sleep(std::time::Duration::from_millis(dwMilliseconds as u64));
}

#[win32_derive::dllexport]
pub fn SetPriorityClass(
    _ctx: &mut Context,
    _hProcess: HANDLE,
    _dwPriorityClass: u32, /* PROCESS_CREATION_FLAGS */
) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    _ctx: &mut Context,
    _lpEventAttributes: u32,
    _bManualReset: bool,
    _bInitialState: bool,
    _lpName: u32,
) -> HANDLE {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCurrentThread(_ctx: &mut Context) -> HANDLE {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEvent(_ctx: &mut Context, _hEvent: HANDLE) -> bool {
    todo!()
}
