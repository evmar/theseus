mod dll;
mod env;
mod file;
mod heap;
mod misc;
mod nls;
mod process;

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

pub use dll::*;
pub use env::*;
pub use file::*;
pub use heap::*;
pub use misc::*;
pub use nls::*;
pub use process::*;

#[derive(Debug, Default)]
pub struct Mapping {
    #[allow(unused)]
    desc: String,
    addr: u32,
    size: u32,
}

#[derive(Default)]
pub struct State {
    mappings: RefCell<Vec<Mapping>>,
    heaps: RefCell<HashMap<u32, Rc<()>>>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn init_state() {
    let state = State::default();

    {
        let mut mappings = state.mappings.borrow_mut();
        mappings.push(Mapping {
            desc: "null page".into(),
            addr: 0,
            size: 0x1000,
        });
    }

    STATE.0.set(state).unwrap_or_else(|_| panic!());
}

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| panic!())
}

pub fn alloc_mapping(desc: String, size: u32) -> u32 {
    let mut new_mapping = Mapping {
        desc,
        addr: 0,
        size,
    };

    let mut mappings = state().mappings.borrow_mut();
    let mut prev_end = 0;
    for (i, mapping) in mappings.iter().enumerate() {
        let space = mapping.addr - prev_end;
        if space >= size {
            new_mapping.addr = prev_end;
            mappings.insert(i, new_mapping);
            return prev_end;
        }
        prev_end = mapping.addr + mapping.size;
    }
    new_mapping.addr = prev_end;
    mappings.push(new_mapping);
    return prev_end;
}

pub fn dump_mappings() {
    let mappings = state().mappings.borrow();
    println!("{:#x?}", mappings);
}
