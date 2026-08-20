use std::sync::{Mutex, MutexGuard};

use runtime::Context;

mod time;
pub use time::*;
mod wave;
pub use wave::*;
mod misc;
pub use misc::*;
mod mmio;
pub use mmio::*;

#[derive(Default)]
pub struct State {
    timer: Option<Timer>,
    wave: Option<wave::State>,
    mmio: Option<mmio::State>,
}

impl State {
    /// The mmio file table, created on first use (a `static` can't build the
    /// map up front).
    pub fn mmio(&mut self) -> &mut mmio::State {
        self.mmio.get_or_insert_with(Default::default)
    }
}

static STATE: Mutex<State> = Mutex::new(State {
    timer: None,
    wave: None,
    mmio: None,
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
