//#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use runtime::{Cont, HOST, Host, MACHINE};
use zerocopy::FromBytes;

#[macro_use]
extern crate alloc;

pub fn GetStdHandle(_x: u32) -> u32 {
    return 0xf11e_0002;
}

pub fn stdcall_GetStdHandle() -> Cont {
    unsafe {
        let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
        let ret = *stack.add(0);
        MACHINE.regs.eax = GetStdHandle(*stack.add(1));
        MACHINE.regs.esp += 2 * 4;
        (MACHINE.indirect)(ret)
    }
}

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

pub fn stdcall_WriteFile() -> Cont {
    unsafe {
        let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
        let ret = *stack.add(0);
        MACHINE.regs.eax = WriteFile(
            *stack.add(1),
            *stack.add(2),
            *stack.add(3),
            *stack.add(4),
            *stack.add(5),
        );
        MACHINE.regs.esp += 6 * 4;
        (MACHINE.indirect)(ret)
    }
}

pub fn ExitProcess(uExitCode: u32) -> u32 {
    std::process::exit(uExitCode as i32);
}

pub fn stdcall_ExitProcess() -> Cont {
    unsafe {
        let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
        let ret = *stack.add(0);
        MACHINE.regs.eax = ExitProcess(*stack.add(1));
        MACHINE.regs.esp += 2 * 4;
        (MACHINE.indirect)(ret)
    }
}

pub fn GetLastError() -> u32 {
    0
}

pub fn stdcall_GetLastError() -> Cont {
    unsafe {
        let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
        let ret = *stack.add(0);
        MACHINE.regs.eax = GetLastError();
        MACHINE.regs.esp += 1 * 4;
        (MACHINE.indirect)(ret)
    }
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
