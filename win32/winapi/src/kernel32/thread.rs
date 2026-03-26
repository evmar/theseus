use runtime::Context;
use zerocopy::FromBytes as _;

use crate::{
    HANDLE,
    kernel32::{self, Mappings},
    stub,
};

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

pub fn init_thread(ctx: &mut Context, mappings: &mut Mappings, peb_addr: u32) {
    let teb_addr = mappings.alloc(
        format!("thread {} TEB", ctx.thread_id),
        None,
        std::mem::size_of::<TEB>() as u32,
    );
    let buf = &mut ctx.memory.bytes[teb_addr as usize..][..std::mem::size_of::<TEB>()];
    let teb = TEB::mut_from_bytes(buf).unwrap();
    teb.Peb = peb_addr;
    teb.Tib._Self = teb_addr;
    ctx.cpu.regs.fs_base = teb_addr;

    let stack_size = 64 << 10;
    let stack_addr = mappings.alloc(format!("thread 0 stack"), None, stack_size);
    let stack_pointer = stack_addr + stack_size;
    ctx.cpu.regs.esp = stack_pointer;
    ctx.cpu.regs.ebp = stack_pointer;
}

#[allow(unused)]
pub fn teb<'a>(ctx: &'a mut Context) -> &'a TEB {
    let teb_addr = ctx.cpu.regs.fs_base;
    let teb =
        TEB::ref_from_bytes(&ctx.memory.bytes[teb_addr as usize..][..std::mem::size_of::<TEB>()])
            .unwrap();
    teb
}

#[win32_derive::dllexport]
pub fn CreateThread(
    ctx: &mut Context,
    _lpThreadAttributes: u32,
    _dwStackSize: u32,
    lpStartAddress: u32, /* LPTHREAD_START_ROUTINE */
    lpParameter: u32,
    _dwCreationFlags: u32, /* THREAD_CREATION_FLAGS */
    _lpThreadId: u32,
) -> HANDLE {
    let mut lock = kernel32::lock();

    let mut new_ctx = Context {
        cpu: runtime::CPU::default(),
        thread_id: lock.next_thread_id,
        memory: ctx.memory.unsafe_clone(),
        blocks: ctx.blocks,
    };
    lock.next_thread_id += 1;

    init_thread(&mut new_ctx, &mut lock.mappings, teb(ctx).Peb);
    std::thread::Builder::new()
        .name(format!("thread {}@{lpStartAddress:x}", new_ctx.thread_id))
        .spawn(move || {
            let ctx = &mut new_ctx;
            let f = runtime::indirect(ctx, lpStartAddress);
            runtime::call_x86(ctx, f, vec![lpParameter]);
        })
        .unwrap();

    stub!(HANDLE::from_raw(1))
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(ctx: &mut Context) -> u32 {
    ctx.thread_id
}

#[win32_derive::dllexport]
pub fn TlsAlloc(_ctx: &mut Context) -> u32 {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn TlsGetValue(_ctx: &mut Context, _dwTlsIndex: u32) -> u32 {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn TlsSetValue(_ctx: &mut Context, _dwTlsIndex: u32, _lpTlsValue: u32) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(
    _ctx: &mut Context,
    _hThread: HANDLE,
    _nPriority: u32, /* THREAD_PRIORITY */
) -> bool {
    todo!()
}
