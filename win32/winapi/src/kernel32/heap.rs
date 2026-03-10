use std::rc::Rc;

use crate::{
    ABIReturn,
    heap::Heap,
    kernel32::{self, HANDLE},
    stub,
};
use runtime::{Cont, MACHINE};

#[win32_derive::dllexport]
pub fn HeapAlloc(_hHeap: HANDLE, _dwFlags: u32 /* HEAP_FLAGS */, _dwBytes: usize) -> u32 {
    stub!(0)
    /*
    let mut flags = dwFlags.unwrap_or_else(|_| {
        log::warn!("HeapAlloc invalid flags {dwFlags:x?}");
        HeapAllocFlags::empty()
    });
    flags.remove(HeapAllocFlags::HEAP_GENERATE_EXCEPTIONS); // todo: OOM
    flags.remove(HeapAllocFlags::HEAP_NO_SERIALIZE); // todo: threads
    let memory = sys.memory();
    let heap = match memory.heaps.get(&hHeap) {
        None => {
            log::error!("HeapAlloc({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let addr = heap.alloc(memory.mem(), dwBytes);
    if addr == 0 {
        log::warn!("HeapAlloc({hHeap:x}) failed");
    }
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        memory.mem().sub32_mut(addr, dwBytes).fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
    if !flags.is_empty() {
        log::error!("HeapAlloc: unhandled flags {flags:?}");
    }
    addr
    */
}

#[win32_derive::dllexport]
pub fn HeapCreate(
    _flOptions: u32, /* HEAP_FLAGS */
    dwInitialSize: u32,
    _dwMaximumSize: u32,
) -> HANDLE {
    // Currently none of the flags will affect behavior, but we might need to revisit this
    // with exceptions or threads support...
    let size = dwInitialSize.max(20 << 20);
    let addr = kernel32::alloc_mapping("HeapCreate".into(), size);

    let heap = Heap::new(addr, size);
    kernel32::state()
        .heaps
        .borrow_mut()
        .insert(addr, Rc::new(heap));

    addr
}

#[win32_derive::dllexport]
pub fn HeapDestroy(hHeap: HANDLE) -> bool {
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn HeapFree(_hHeap: HANDLE, dwFlags: u32 /* HEAP_FLAGS */, _lpMem: u32) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    /*
    let memory = sys.memory();
    memory.heaps.get(&hHeap).unwrap().free(memory.mem(), lpMem);
    */
    stub!(true)
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
    _hHeap: HANDLE,
    dwFlags: u32, /* HEAP_FLAGS */
    _lpMem: u32,
    _dwBytes: usize,
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
