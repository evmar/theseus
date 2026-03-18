use crate::kernel32::{self, HANDLE, state};
use runtime::MACHINE;
use zerocopy::FromBytes;

#[win32_derive::dllexport]
pub fn ExitProcess(uExitCode: u32) -> u32 {
    std::process::exit(uExitCode as i32);
}

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::Immutable, zerocopy::IntoBytes)]
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
#[derive(zerocopy::FromBytes, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::KnownLayout)]
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
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}

#[repr(C)]
#[derive(Debug, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout)]
struct CURDIR {
    DosPath: UNICODE_STRING,
    Handle: u32,
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
    CurrentDirectory: CURDIR,
    DllPath: UNICODE_STRING,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
}

#[derive(Default)]
pub struct CommandLine {
    pub command_line_8: u32,
    pub command_line_16: u32,
}

#[win32_derive::dllexport]
pub fn GetCommandLineA() -> u32 {
    state().command_line.borrow().command_line_8
}

fn align_to_4(x: usize) -> usize {
    (x + 4 - 1) & !(4 - 1)
}

pub fn init_process() {
    unsafe {
        let mut mappings = state().mappings.borrow_mut();
        let stack_size = 64 << 10;
        let stack_addr = mappings.alloc("stack".into(), 0, stack_size);
        MACHINE.regs.esp = stack_addr + stack_size;
        MACHINE.regs.ebp = stack_addr + stack_size;

        let process_data_addr = mappings.alloc("process data".into(), 0, 0x1000);
        drop(mappings);

        let buf = &mut MACHINE.memory.bytes[process_data_addr as usize..][..0x1000];

        let command_line = "TODO\0";

        let len = align_to_4(command_line.len());
        let (command_line_16, buf) = buf.split_at_mut(len * 2);
        let (command_line_8, buf) = buf.split_at_mut(len);
        let command_line_16: &mut [u16] = <[u16]>::mut_from_bytes(command_line_16).unwrap();
        for (i, c) in command_line.bytes().enumerate() {
            command_line_8[i] = c;
            command_line_16[i] = c as u16;
        }
        *state().command_line.borrow_mut() = CommandLine {
            command_line_8: (&raw const *command_line_8)
                .byte_offset_from_unsigned(MACHINE.memory.bytes) as u32,
            command_line_16: (&raw const *command_line_16)
                .byte_offset_from_unsigned(MACHINE.memory.bytes)
                as u32,
        };

        let env = "\0\0";
        let len = align_to_4(env.len() * 2);
        let (env, buf) = buf.split_at_mut(len);
        env.fill(0);
        state()
            .environ
            .set((&raw const *env).byte_offset_from_unsigned(MACHINE.memory.bytes) as u32);

        let (params, buf) = RTL_USER_PROCESS_PARAMETERS::mut_from_prefix(buf).unwrap();
        params.hStdOutput = 0xF11E_0002;
        params.hStdError = 0xF11E_0003;

        let (peb, buf) = PEB::mut_from_prefix(buf).unwrap();
        peb.ProcessParameters =
            (&raw const *params).byte_offset_from_unsigned(MACHINE.memory.bytes) as u32;
        let process_heap = kernel32::heap_create("process heap".into(), 4 << 20);
        peb.ProcessHeap = process_heap.addr;
        *state().process_heap.borrow_mut() = process_heap;

        let (teb, _) = TEB::mut_from_prefix(buf).unwrap();
        teb.Peb = (&raw const *peb).byte_offset_from_unsigned(MACHINE.memory.bytes) as u32;
        teb.Tib._Self = (&raw const *teb).byte_offset_from_unsigned(MACHINE.memory.bytes) as u32;

        MACHINE.regs.fs_base =
            (&raw const *teb).byte_offset_from_unsigned(MACHINE.memory.bytes) as u32;

        state().mappings.borrow().dump();
    }
}

// MSDN: "A pseudo handle is a special constant, currently (HANDLE)-1, that is interpreted as the current process handle."
pub const CURRENT_PROCESS_HANDLE: HANDLE = -1i32 as u32;

#[win32_derive::dllexport]
pub fn GetCurrentProcess() -> HANDLE {
    CURRENT_PROCESS_HANDLE
}

#[allow(unused)]
fn teb(memory: &[u8]) -> &TEB {
    unsafe {
        let teb_addr = MACHINE.regs.fs_base;
        let (teb, _) = TEB::ref_from_prefix(&memory[teb_addr as usize..]).unwrap();
        teb
    }
}

#[allow(unused)]
fn peb_mut(memory: &mut [u8]) -> &mut PEB {
    let peb_addr = teb(memory).Peb;
    let (peb, _) = PEB::mut_from_prefix(&mut memory[peb_addr as usize..]).unwrap();
    peb
}

#[win32_derive::dllexport]
pub fn GetProcessHeap() -> HANDLE {
    state().process_heap.borrow().addr
}
