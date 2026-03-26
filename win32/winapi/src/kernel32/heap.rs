use runtime::Context;

use crate::{
    dllexport::win32flags,
    heap::Heap,
    kernel32::{self, HANDLE},
    stub,
};

win32flags! {
    pub struct HEAP_FLAGS {
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(ctx: &mut Context, hHeap: HANDLE, dwFlags: HEAP_FLAGS, dwBytes: u32) -> u32 {
    if !dwFlags.is_empty() {
        todo!();
    }

    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    heap.alloc(&mut ctx.memory, dwBytes)
    /*
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        memory.mem().sub32_mut(addr, dwBytes).fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
    */
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
    let addr = state.mappings.alloc("HeapCreate".into(), None, size);
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
    lpMem: u32,
) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    heap.size(&mut ctx.memory, lpMem)
}

#[win32_derive::dllexport]
pub fn HeapFree(
    ctx: &mut Context,
    hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    lpMem: u32,
) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let state = kernel32::lock();
    let heap = state.heaps.get(&hHeap).unwrap();
    heap.free(&mut ctx.memory, lpMem);
    true
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
    _ctx: &mut Context,
    _hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    _lpMem: u32,
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
