use crate::{
    heap::Heap,
    kernel32::{CommandLine, Mappings, UnsafeTickCount},
};
use std::{
    cell::Cell,
    collections::HashMap,
    ptr::NonNull,
    sync::{Mutex, MutexGuard},
};

pub struct State {
    pub mappings: Mappings,
    pub heaps: HashMap<u32, Heap>,
    pub process_heap: Heap,
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub command_line: CommandLine,
    pub environ: Cell<u32>,
    pub next_thread_id: u32,
    pub next_tls_index: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn init_state(image_base: u32, resources: std::ops::Range<u32>) {
    UnsafeTickCount::init();
    let mut state = STATE.lock().unwrap();
    *state = Some(State {
        image_base,
        resources,
        heaps: HashMap::new(),
        mappings: Default::default(),
        process_heap: Default::default(),
        command_line: Default::default(),
        environ: Default::default(),
        next_thread_id: 2,
        next_tls_index: 0,
    });
}

// We want to lock Mutex<Option<State>> and return a MutexGuard<State>,
// but MutexGuard::map is not yet stable, so use this workaround for now.
pub struct LockedState {
    _lock: MutexGuard<'static, Option<State>>,
    ptr: NonNull<State>,
}

impl std::ops::Deref for LockedState {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        // safety: _lock is holding the lock
        unsafe { self.ptr.as_ref() }
    }
}

impl std::ops::DerefMut for LockedState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // safety: _lock is holding the lock
        unsafe { self.ptr.as_mut() }
    }
}

pub type Lock = LockedState;
pub fn lock() -> Lock {
    let mut lock = STATE.lock().unwrap();
    let state = NonNull::from_mut(lock.as_mut().unwrap());
    LockedState {
        _lock: lock,
        ptr: state,
    }
}
