use std::sync::Mutex;

use crate::{ABIReturn, FromABIParam, HANDLE, handle::Handles, locked_state::LockedState};

mod bitmap;
pub use bitmap::*;
mod dc;
pub use dc::*;
mod misc;
pub use misc::*;
mod object;
pub use object::*;

pub type HGDIOBJ = HANDLE;
pub type HBRUSH = HGDIOBJ;
pub type HPEN = HGDIOBJ;

pub struct State {
    pub dcs: Handles<DC>,
    pub objects: Handles<Object>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub type Lock = LockedState<State>;
pub fn lock() -> Lock {
    LockedState::from_or_init(&STATE, || State {
        // avoid low-numbered object handles to avoid conflicting with COLOR_* constants for HBRUSH
        objects: Handles::new(0x1000),
        dcs: Default::default(),
    })
}

#[derive(Debug, Copy, Clone, Default)]
pub struct COLORREF(u32);

impl FromABIParam for COLORREF {
    fn from_abi(val: u32) -> Self {
        Self(val)
    }
}

impl Into<ABIReturn> for COLORREF {
    fn into(self) -> ABIReturn {
        ABIReturn::from(self.0)
    }
}

impl COLORREF {
    pub fn to_pixel(&self) -> [u8; 4] {
        let [r, g, b] = self.to_rgb();
        [r, g, b, 0xff]
    }

    pub fn as_win32(&self) -> u32 {
        self.0
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(u32::from_le_bytes([r, g, b, 0]))
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let [r, g, b, _] = self.0.to_le_bytes();
        [r, g, b]
    }
}
