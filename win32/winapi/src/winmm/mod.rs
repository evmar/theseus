use std::sync::{Mutex, MutexGuard};

use runtime::Context;

mod time;
pub use time::*;
mod wave;
pub use wave::*;
mod misc;
pub use misc::*;

use crate::host;

#[derive(Default)]
pub struct State {
    timer: Option<Timer>,
    wave: Option<wave::State>,
}

static STATE: Mutex<State> = Mutex::new(State {
    timer: None,
    wave: None,
});

pub fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}

fn winmm_main(ctx: &mut Context) {
    loop {
        let mut lock = state();
        let Some(timer) = lock.timer.as_mut() else {
            return;
        };

        let now = host::host().time();
        if now < timer.next {
            let delta = timer.next - now;
            std::thread::sleep(std::time::Duration::from_millis(delta as u64));
        }

        let func = ctx.indirect(timer.callback);
        let timer_id = 1;
        let user_data = timer.user_data;
        let next = now + timer.period;
        timer.next = next;
        drop(lock);

        // LPTIMECALLBACK
        ctx.call32_x86(func, vec![timer_id, 0, user_data, 0, 0]);
    }
}
