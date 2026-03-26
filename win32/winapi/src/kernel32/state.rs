use crate::{
    heap::Heap,
    kernel32::{CommandLine, Mappings},
};
use std::{
    cell::Cell,
    collections::HashMap,
    ptr::NonNull,
    sync::{Mutex, MutexGuard},
};

pub struct State {
    pub start: std::time::Instant,
    pub mappings: Mappings,
    pub heaps: HashMap<u32, Heap>,
    pub process_heap: Heap,
    pub image_base: u32,
    pub resources: std::ops::Range<u32>,
    pub command_line: CommandLine,
    pub environ: Cell<u32>,
    pub next_thread_id: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn init_state(image_base: u32, resources: std::ops::Range<u32>) {
    let mut state = STATE.lock().unwrap();
    *state = Some(State {
        start: std::time::Instant::now(),
        image_base,
        resources,
        heaps: HashMap::new(),
        mappings: Default::default(),
        process_heap: Default::default(),
        command_line: Default::default(),
        environ: Default::default(),
        next_thread_id: 2,
    });
}

pub struct LockedState {
    _lock: MutexGuard<'static, Option<State>>,
    ptr: NonNull<State>,
}

impl std::ops::Deref for LockedState {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl std::ops::DerefMut for LockedState {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
