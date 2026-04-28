use std::{cell::Cell, collections::HashMap, sync::Mutex};

use crate::{
    Handles,
    heap::Heap,
    kernel32::{self, CommandLine, Mappings, Object, UnsafeTickCount},
    locked_state::LockedState,
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
    pub dll_loader: Box<dyn kernel32::DLLLoader>,
    pub objects: Handles<Object>,
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
        dll_loader: Box::new(()),
        objects: Handles::new(0x1000),
    });
}

pub type Lock = LockedState<State>;
pub fn lock() -> Lock {
    LockedState::from(&STATE)
}
