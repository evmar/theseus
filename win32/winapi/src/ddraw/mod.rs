use std::{
    cell::{OnceCell, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

mod ddraw;
mod ddraw1;
mod ddraw7;
pub mod types;

pub use ddraw::*;
pub use ddraw1::*;
pub use ddraw7::*;
pub use types::DD;

pub const VTABLES: [(&'static str, &[&str]); 5] = [
    ("IDirectDraw", IDirectDraw::VTABLE_ENTRIES.as_slice()),
    (
        "IDirectDrawSurface",
        IDirectDrawSurface::VTABLE_ENTRIES.as_slice(),
    ),
    ("IDirectDraw7", IDirectDraw7::VTABLE_ENTRIES.as_slice()),
    (
        "IDirectDrawSurface7",
        IDirectDrawSurface7::VTABLE_ENTRIES.as_slice(),
    ),
    (
        "IDirectDrawPalette",
        IDirectDrawPalette::VTABLE_ENTRIES.as_slice(),
    ),
];

#[repr(C)]
#[derive(PartialEq, zerocopy::FromBytes)]
pub struct GUID(pub (u32, u16, u16, [u8; 8]));

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-",
            self.0.0,
            self.0.1,
            self.0.2,
            u16::from_le_bytes(self.0.3[..2].try_into().unwrap())
        )?;
        for b in &self.0.3[2..] {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct State {
    pub ddraw: RefCell<Option<DirectDraw>>,
    pub surf: RefCell<HashMap<u32, Rc<RefCell<Surface>>>>,
    pub palette: RefCell<HashMap<u32, Rc<RefCell<Palette>>>>,
}

impl State {
    pub fn get_ddraw(&self, ptr: u32) -> RefMut<'_, DirectDraw> {
        let ddraw = RefMut::map(self.ddraw.borrow_mut(), |ddraw| ddraw.as_mut().unwrap());
        assert!(ptr == ddraw.addr);
        ddraw
    }
}

// TODO: reuse locking pattern from kernel32
struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}

static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(|| Default::default())
}
