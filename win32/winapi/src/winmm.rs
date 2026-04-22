use std::sync::{Mutex, MutexGuard};

use runtime::Context;

use crate::{
    kernel32::{self, get_tick_count},
    stub,
};

#[derive(Default)]
pub struct State {
    timer: Option<Timer>,
}

static STATE: Mutex<State> = Mutex::new(State { timer: None });

pub fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}

struct Timer {
    period: u32,
    next: u32,
    callback: u32,
    user_data: u32,
}

fn winmm_main(ctx: &mut Context) {
    loop {
        let mut lock = state();
        let Some(timer) = lock.timer.as_mut() else {
            return;
        };

        let now = get_tick_count();
        if now < timer.next {
            let delta = timer.next - now;
            std::thread::sleep(std::time::Duration::from_millis(delta as u64));
        }

        let func = runtime::indirect(ctx, timer.callback);
        let timer_id = 1;
        let user_data = timer.user_data;
        let next = now + timer.period;
        timer.next = next;
        drop(lock);

        // LPTIMECALLBACK
        runtime::call_x86(ctx, func, vec![timer_id, 0, user_data, 0, 0]);
    }
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
    ctx: &mut Context,
    uDelay: u32,
    _uResolution: u32,
    lpTimeProc: u32,
    dwUser: u32,
    fuEvent: TIME,
) -> u32 {
    assert_eq!(fuEvent.periodic, true);

    let mut state = state();
    assert!(state.timer.is_none());
    state.timer = Some(Timer {
        period: uDelay,
        next: get_tick_count() + uDelay,
        callback: lpTimeProc,
        user_data: dwUser,
    });
    kernel32::lock().create_thread(ctx, "winmm".into(), |ctx| {
        winmm_main(ctx);
    });

    stub!(1)
}

#[win32_derive::dllexport]
pub fn timeKillEvent(_ctx: &mut Context, _uTimerID: u32) -> u32 {
    todo!()
}
