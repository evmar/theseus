use crate::ABIReturn;
use runtime::{Cont, HOST, Host, MACHINE};
use zerocopy::FromBytes;

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
            let buf = core::slice::from_raw_parts(
                MACHINE.memory.add(lpBuffer as usize),
                nNumberOfBytesToWrite as usize,
            );
            HOST.print(buf);
            *(MACHINE.memory.add(lpNumberOfBytesWritten as usize) as *mut u32) =
                nNumberOfBytesToWrite;
        }
    } else {
        todo!("WriteFile(hFile={hFile:x})");
    }
    return 1;
}

#[win32_derive::dllexport]
pub fn ExitProcess(uExitCode: u32) -> u32 {
    std::process::exit(uExitCode as i32);
}

#[win32_derive::dllexport]
pub fn GetLastError() -> u32 {
    0
}

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout)]
pub struct NT_TIB {
    ExceptionList: u32,
    StackBase: u32,
    StackLimit: u32,
    SubSystemTib: u32,
    FiberData: u32,
    ArbitraryUserPointer: u32,
    _Self: u32,
}

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout)]
pub struct TEB {
    pub Tib: NT_TIB,
    pub EnvironmentPointer: u32,
    pub ClientId_UniqueProcess: u32,
    pub ClientId_UniqueThread: u32,
    pub ActiveRpcHandle: u32,
    pub ThreadLocalStoragePointer: u32,
    pub Peb: u32,
    pub LastErrorValue: u32,
    pub CountOfOwnedCriticalSections: u32,
    pub CsrClientThread: u32,
    pub Win32ThreadInfo: u32,
    pub User32Reserved: [u32; 26],
    pub UserReserved: [u32; 5],
    pub WOW32Reserved: u32,
    pub CurrentLocale: u32,
    // TODO: ... there are many more fields here
    pub padding: [u32; 20],

    // This is at the wrong offset, but it shouldn't matter.
    pub TlsSlots: [u32; 64],
}

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout)]
pub struct PEB {
    pub InheritedAddressSpace: u8,
    pub ReadImageFileExecOptions: u8,
    pub BeingDebugged: u8,
    pub SpareBool: u8,
    pub Mutant: u32,
    pub ImageBaseAddress: u32,
    pub LdrData: u32,
    /* 0x10 */
    pub ProcessParameters: u32,
    pub SubSystemData: u32,
    pub ProcessHeap: u32,
    // TODO: more fields
}

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout)]
struct RTL_USER_PROCESS_PARAMETERS {
    AllocationSize: u32,
    Size: u32,
    Flags: u32,
    DebugFlags: u32,
    ConsoleHandle: u32,
    ConsoleFlags: u32,
    hStdInput: u32,
    hStdOutput: u32,
    hStdError: u32,
    // CurrentDirectory: CURDIR,
    // DllPath: UNICODE_STRING,
    // ImagePathName: UNICODE_STRING,
    // CommandLine: UNICODE_STRING,
}

pub fn init_process() {
    unsafe {
        let buf = core::slice::from_raw_parts_mut(MACHINE.memory.add(0x1000), 0x1000);

        let (params, buf) = RTL_USER_PROCESS_PARAMETERS::mut_from_prefix(buf).unwrap();
        params.hStdOutput = 0xF11E_0002;
        params.hStdError = 0xF11E_0003;

        let (peb, buf) = PEB::mut_from_prefix(buf).unwrap();
        peb.ProcessParameters =
            (&raw const *params).byte_offset_from_unsigned(MACHINE.memory) as u32;

        let (teb, _) = TEB::mut_from_prefix(buf).unwrap();
        teb.Peb = (&raw const *peb).byte_offset_from_unsigned(MACHINE.memory) as u32;
        teb.Tib._Self = (&raw const *teb).byte_offset_from_unsigned(MACHINE.memory) as u32;

        MACHINE.regs.fs_base = (&raw const *teb).byte_offset_from_unsigned(MACHINE.memory) as u32;
    }
}

pub type PSTR = u32;
pub type PWSTR = u32;
pub type HANDLE = u32;

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_penv: PSTR) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(_penv: PWSTR) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCommandLineA() -> PSTR {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_CodePage: u32, _lpCPInfo: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCurrentProcess() -> HANDLE {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings() -> PSTR {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(_lpName: PSTR, _lpBuffer: PSTR, _nSize: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetFileType(_hFile: HANDLE) -> u32 /* FILE_TYPE */ {
    todo!()
}

pub type HMODULE = u32;

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(_hModule: HMODULE, _lpFilename: PSTR, _nSize: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(_lpModuleName: PSTR) -> HMODULE {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetStartupInfoA(_lpStartupInfo: u32) {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetStringTypeA(
    _Locale: u32,
    _dwInfoType: u32,
    _lpSrcStr: PSTR,
    _cchSrc: i32,
    _lpCharType: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetStringTypeW(_dwInfoType: u32, _lpSrcStr: PWSTR, _cchSrc: i32, _lpCharType: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetVersion() -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetVersionExA(_lpVersionInformation: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn HeapAlloc(_hHeap: HANDLE, _dwFlags: u32 /* HEAP_FLAGS */, _dwBytes: usize) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn HeapCreate(
    _flOptions: u32, /* HEAP_FLAGS */
    _dwInitialSize: usize,
    _dwMaximumSize: usize,
) -> HANDLE {
    todo!()
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_hHeap: HANDLE) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn HeapFree(_hHeap: HANDLE, _dwFlags: u32 /* HEAP_FLAGS */, _lpMem: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
    _hHeap: HANDLE,
    _dwFlags: u32, /* HEAP_FLAGS */
    _lpMem: u32,
    _dwBytes: usize,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn LCMapStringA(
    _Locale: u32,
    _dwMapFlags: u32,
    _lpSrcStr: PSTR,
    _cchSrc: i32,
    _lpDestStr: PSTR,
    _cchDest: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn LCMapStringW(
    _Locale: u32,
    _dwMapFlags: u32,
    _lpSrcStr: PWSTR,
    _cchSrc: i32,
    _lpDestStr: PWSTR,
    _cchDest: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(_lpLibFileName: PSTR) -> HMODULE {
    todo!()
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    _CodePage: u32,
    _dwFlags: u32, /* MULTI_BYTE_TO_WIDE_CHAR_FLAGS */
    _lpMultiByteStr: PSTR,
    _cbMultiByte: i32,
    _lpWideCharStr: PWSTR,
    _cchWideChar: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetHandleCount(_uNumber: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TerminateProcess(_hProcess: HANDLE, _uExitCode: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_ExceptionInfo: u32) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    _lpAddress: u32,
    _dwSize: usize,
    _flAllocationType: u32, /* VIRTUAL_ALLOCATION_TYPE */
    _flProtect: u32,        /* PAGE_PROTECTION_FLAGS */
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn VirtualFree(
    _lpAddress: u32,
    _dwSize: usize,
    _dwFreeType: u32, /* VIRTUAL_FREE_TYPE */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn WideCharToMultiByte(
    _CodePage: u32,
    _dwFlags: u32,
    _lpWideCharStr: PWSTR,
    _cchWideChar: i32,
    _lpMultiByteStr: PSTR,
    _cbMultiByte: i32,
    _lpDefaultChar: PSTR,
    _lpUsedDefaultChar: u32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetOEMCP() -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetACP() -> u32 {
    todo!()
}
