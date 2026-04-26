use std::sync::{Mutex, MutexGuard};

use runtime::Context;

mod time;
pub use time::*;
mod wave;
pub use wave::*;

use crate::kernel32::get_tick_count;

#[derive(Default)]
pub struct State {
    timer: Option<Timer>,
}

static STATE: Mutex<State> = Mutex::new(State { timer: None });

pub fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
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
