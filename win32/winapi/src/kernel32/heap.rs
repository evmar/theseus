use bitflags::bitflags;
use std::rc::Rc;

use crate::{
    FromABIParam,
    heap::Heap,
    kernel32::{self, HANDLE},
    stub,
};
use runtime::{Cont, MACHINE};

#[derive(Debug, Default, PartialEq, Eq, zerocopy::FromBytes)]
pub struct HEAP_FLAGS(u32);
bitflags! {
    impl HEAP_FLAGS: u32 {
    }
}

impl FromABIParam for HEAP_FLAGS {
    fn from_abi(value: u32) -> Self {
        HEAP_FLAGS::from_bits(value).unwrap()
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(hHeap: HANDLE, dwFlags: HEAP_FLAGS, dwBytes: u32) -> u32 {
    if !dwFlags.is_empty() {
        todo!();
    }

    let heaps = kernel32::state().heaps.borrow();
    let heap = heaps.get(&hHeap).unwrap();
    heap.alloc(unsafe { &mut MACHINE.memory }, dwBytes)
    /*
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        memory.mem().sub32_mut(addr, dwBytes).fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
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
    let addr = kernel32::state()
        .mappings
        .borrow_mut()
        .alloc("HeapCreate".into(), 0, size);

    let heap = Heap::new(addr, size);
    kernel32::state()
        .heaps
        .borrow_mut()
        .insert(addr, Rc::new(heap));

    addr
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_hHeap: HANDLE) -> bool {
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn HeapSize(hHeap: HANDLE, dwFlags: u32 /* HEAP_FLAGS */, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let heaps = kernel32::state().heaps.borrow();
    let heap = heaps.get(&hHeap).unwrap();
    heap.size(unsafe { &mut MACHINE.memory }, lpMem)
}

#[win32_derive::dllexport]
pub fn HeapFree(hHeap: HANDLE, dwFlags: u32 /* HEAP_FLAGS */, lpMem: u32) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let heaps = kernel32::state().heaps.borrow();
    let heap = heaps.get(&hHeap).unwrap();
    heap.free(unsafe { &mut MACHINE.memory }, lpMem);
    true
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
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
