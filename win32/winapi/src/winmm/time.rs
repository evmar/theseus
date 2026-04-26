use runtime::Context;

use crate::{
    kernel32::{self, get_tick_count},
    stub,
    winmm::{state, winmm_main},
};

pub struct Timer {
    pub period: u32,
    pub next: u32,
    pub callback: u32,
    pub user_data: u32,
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
