use runtime::Context;

use crate::{
    Ptr,
    dllexport::win32flags,
    heap::Heap,
    kernel32::{self, HANDLE, lock},
    stub,
};

win32flags! {
    pub struct HEAP_FLAGS {
        const NO_SERIALIZE        = 0x01;
        const GENERATE_EXCEPTIONS = 0x04;
        const ZERO_MEMORY         = 0x08;
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(ctx: &mut Context, hHeap: HANDLE, dwFlags: HEAP_FLAGS, dwBytes: u32) -> u32 {
    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    let addr = heap.alloc(&mut ctx.memory, dwBytes);
    drop(state);
    if addr != 0 && dwFlags.contains(HEAP_FLAGS::ZERO_MEMORY) {
        ctx.memory[addr..][..dwBytes as usize].fill(0);
    }
    addr
}

#[win32_derive::dllexport]
pub fn HeapCreate(
    _ctx: &mut Context,
    _flOptions: u32, /* HEAP_FLAGS */
    dwInitialSize: u32,
    _dwMaximumSize: u32,
) -> HANDLE {
    // Currently none of the flags will affect behavior, but we might need to revisit this
    // with exceptions or threads support...
    let size = dwInitialSize.max(20 << 20);
    let mut state = kernel32::lock();
    let addr = state.mappings.alloc("HeapCreate".into(), size);
    let heap = Heap::new(addr, size);
    state.heaps.insert(addr, heap);
    addr
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_ctx: &mut Context, _hHeap: HANDLE) -> bool {
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn HeapSize(
    ctx: &mut Context,
    hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    lpMem: Ptr<()>,
) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    heap.size(&mut ctx.memory, lpMem.addr)
}

#[win32_derive::dllexport]
pub fn HeapFree(
    ctx: &mut Context,
    hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    lpMem: Ptr<()>,
) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    heap.free(&mut ctx.memory, lpMem.addr);
    true
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
    _ctx: &mut Context,
    _hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    _lpMem: Ptr<()>,
    _dwBytes: u32,
) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapReAlloc flags: {:x}", dwFlags);
    }
    stub!(0)
    /*
    let memory = sys.memory();
    let heap = match memory.heaps.get(&hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let mem = memory.mem();
    let old_size = heap.size(mem, lpMem);
    let new_addr = heap.alloc(mem, dwBytes);
    let copy_size = old_size.min(dwBytes);
    mem.copy(lpMem, new_addr, copy_size);
    heap.free(mem, lpMem);
    new_addr
    */
}

win32flags! {
    pub struct GMEM {
        const MOVEABLE    = 0x0002;
        // const NOCOMPACT   = 0x0010;
        // const NODISCARD   = 0x0020;
        const ZEROINIT    = 0x0040;
        // const MODIFY      = 0x0080;
        // const DISCARDABLE = 0x0100;
        // const NOT_BANKED  = 0x1000;
        // const SHARE       = 0x2000;
        // const DDESHARE    = 0x2000;
        // const NOTIFY      = 0x4000;
        // Lots of obsolete flags, ignore them
        const _ = !0;
    }
}

#[win32_derive::dllexport]
pub fn GlobalAlloc(ctx: &mut Context, uFlags: GMEM, dwBytes: u32) -> u32 {
    assert!(!uFlags.contains(GMEM::MOVEABLE));
    let ptr = lock().process_heap.alloc(&mut ctx.memory, dwBytes);
    if uFlags.contains(GMEM::ZEROINIT) {
        ctx.memory[ptr..][..dwBytes as usize].fill(0);
    }
    ptr
}

#[win32_derive::dllexport]
pub fn GlobalFree(ctx: &mut Context, hMem: Ptr<()>) -> u32 {
    lock().process_heap.free(&mut ctx.memory, hMem.addr);
    0 // success
}

// GlobalAlloc only hands out fixed (non-moveable) memory, so handles and
// pointers are the same value.

#[win32_derive::dllexport]
pub fn GlobalLock(_ctx: &mut Context, hMem: u32) -> u32 {
    hMem
}

#[win32_derive::dllexport]
pub fn GlobalUnlock(_ctx: &mut Context, _hMem: u32) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn GlobalHandle(_ctx: &mut Context, pMem: u32) -> u32 {
    pMem
}
