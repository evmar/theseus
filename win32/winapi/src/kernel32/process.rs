use runtime::Context;
use zerocopy::FromBytes;

use crate::{
    heap::Heap,
    kernel32::{self, HANDLE, init_thread, lock, teb},
};

#[win32_derive::dllexport]
pub fn ExitProcess(_ctx: &mut Context, uExitCode: u32) -> u32 {
    std::process::exit(uExitCode as i32);
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
pub fn GetCommandLineA(_ctx: &mut Context) -> u32 {
    lock().command_line.command_line_8
}

fn align_to_4(x: usize) -> usize {
    (x + 4 - 1) & !(4 - 1)
}

pub fn init_process(ctx: &mut Context, state: &mut kernel32::Lock) {
    let process_data_addr = state.mappings.alloc("process data".into(), None, 0x1000);

    let origin = ctx.memory.as_ptr() as usize;
    let buf = &mut ctx.memory[process_data_addr..][..0x1000];

    let command_line = "TODO\0";

    let len = align_to_4(command_line.len());
    let (command_line_16, buf) = buf.split_at_mut(len * 2);
    let (command_line_8, buf) = buf.split_at_mut(len);
    let command_line_16: &mut [u16] = <[u16]>::mut_from_bytes(command_line_16).unwrap();
    for (i, c) in command_line.bytes().enumerate() {
        command_line_8[i] = c;
        command_line_16[i] = c as u16;
    }
    state.command_line = CommandLine {
        command_line_8: (command_line_8.as_ptr() as usize - origin) as u32,
        command_line_16: (command_line_16.as_ptr() as usize - origin) as u32,
    };

    let env = "\0\0";
    let len = align_to_4(env.len() * 2);
    let (env, buf) = buf.split_at_mut(len);
    env.fill(0);
    state.environ.set((env.as_ptr() as usize - origin) as u32);

    let (params, buf) = RTL_USER_PROCESS_PARAMETERS::mut_from_prefix(buf).unwrap();
    params.hStdOutput = 0xF11E_0002;
    params.hStdError = 0xF11E_0003;

    let (peb, _buf) = PEB::mut_from_prefix(buf).unwrap();
    peb.ProcessParameters = (params as *const _ as usize - origin) as u32;

    let heap_size = 4 << 20;
    let heap_addr = state.mappings.alloc("process heap".into(), None, heap_size);
    let process_heap = Heap::new(heap_addr, heap_size);
    peb.ProcessHeap = process_heap.addr;
    state.process_heap = process_heap;

    let peb_addr = (peb as *const _ as usize - origin) as u32;
    init_thread(ctx, &mut state.mappings, peb_addr);
}

// MSDN: "A pseudo handle is a special constant, currently (HANDLE)-1, that is interpreted as the current process handle."
pub const CURRENT_PROCESS_HANDLE: HANDLE = -1i32 as u32;

#[win32_derive::dllexport]
pub fn GetCurrentProcess(_ctx: &mut Context) -> HANDLE {
    CURRENT_PROCESS_HANDLE
}

#[allow(unused)]
fn peb_mut<'a>(ctx: &'a mut Context) -> &'a mut PEB {
    let peb_addr = teb(ctx).Peb;
    let (peb, _) = PEB::mut_from_prefix(&mut ctx.memory[peb_addr..]).unwrap();
    peb
}

#[win32_derive::dllexport]
pub fn GetProcessHeap(_ctx: &mut Context) -> HANDLE {
    lock().process_heap.addr
}

#[win32_derive::dllexport]
pub fn TerminateProcess(_ctx: &mut Context, _hProcess: HANDLE, _uExitCode: u32) -> bool {
    todo!();
}
