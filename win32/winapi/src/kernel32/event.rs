use std::sync::{Arc, Condvar, Mutex};

use runtime::Context;

use crate::{HANDLE, kernel32::lock};

pub enum Object {
    Thread,
    Event(Arc<Event>),
}

pub struct Event {
    _name: String,
    manual_reset: bool,
    signaled: Mutex<bool>,
    cond: Condvar,
}

#[win32_derive::dllexport]
pub fn WaitForSingleObject(_ctx: &mut Context, hHandle: HANDLE, dwMilliseconds: u32) -> u32 /* WAIT_EVENT */
{
    let event = {
        let kernel32 = lock();
        let Object::Event(event) = kernel32.objects.get(hHandle).unwrap() else {
            panic!()
        };
        event.clone()
    };

    let mut signaled = event.signaled.lock().unwrap();
    while !*signaled {
        signaled = event
            .cond
            .wait_timeout(
                signaled,
                std::time::Duration::from_millis(dwMilliseconds as u64),
            )
            .unwrap()
            .0;
    }
    if !event.manual_reset {
        *signaled = false;
    }
    0
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    ctx: &mut Context,
    _lpEventAttributes: u32,
    bManualReset: bool,
    bInitialState: bool,
    lpName: u32,
) -> HANDLE {
    let name = ctx.memory.read_str(lpName);
    let event = Event {
        _name: name.to_string(),
        manual_reset: bManualReset,
        signaled: Mutex::new(bInitialState),
        cond: Condvar::new(),
    };
    let mut kernel32 = lock();
    kernel32.objects.add(Object::Event(Arc::new(event)))
}

#[win32_derive::dllexport]
pub fn SetEvent(_ctx: &mut Context, hEvent: HANDLE) -> bool {
    let kernel32 = lock();
    let Object::Event(event) = kernel32.objects.get(hEvent).unwrap() else {
        panic!()
    };
    *event.signaled.lock().unwrap() = true;
    // TODO: the number of threads notified are different between manual reset and auto reset events!
    assert!(!event.manual_reset);
    event.cond.notify_one();
    true
}
