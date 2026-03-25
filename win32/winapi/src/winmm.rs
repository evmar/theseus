#![allow(unused)]

use runtime::Context;
use std::{
    cell::{LazyCell, OnceCell, RefCell},
    sync::{LazyLock, Mutex, MutexGuard},
};

use crate::stub;

#[derive(Default)]
pub struct State {
    timer: RefCell<Option<Timer>>,
}

static STATE: Mutex<State> = Mutex::new(State {
    timer: RefCell::new(None),
});

pub fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}

struct Timer {
    next: u32,
    callback: u32,
}

fn timer_proc() {
    let state = state();
    let mut timer = state.timer.borrow_mut();
    let Some(timer) = timer.as_mut() else { return };
    timer.next = 3;
}

#[derive(Debug)]
pub struct TIME {
    periodic: bool,
    #[allow(unused)]
    event: (), // todo
}

impl crate::dllexport::FromABIParam for TIME {
    fn from_abi(val: u32) -> Self {
        // kind of a bitfield, kind of an enum
        let periodic = (val & 0xF) != 0;
        assert_eq!(periodic, true);
        let event = match val & 0xF0 {
            0x00 => (),      // FUNCTION
            0x10 => todo!(), // EVENT_SET
            0x20 => todo!(), // EVENT_PULSE
            _ => unimplemented!(),
        };
        TIME { periodic, event }
    }
}

#[win32_derive::dllexport]
pub fn timeSetEvent(
    _ctx: &mut Context,
    _uDelay: u32,
    _uResolution: u32,
    _lpTimeProc: u32,
    _dwUser: u32,
    fuEvent: TIME,
) -> u32 {
    assert_eq!(fuEvent.periodic, true);

    stub!(1)
}

#[win32_derive::dllexport]
pub fn timeKillEvent(_ctx: &mut Context, _uTimerID: u32) -> u32 {
    todo!()
}
