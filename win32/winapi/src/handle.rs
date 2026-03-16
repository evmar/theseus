//! Structures for working with HWND, HDC, etc. Windows handles.

use std::collections::HashMap;

/// Nullability: following Windows, a given HWND can be null.  We don't attempt to work with
/// Option<HWND> instead etc. for two reasons:
/// 1. Many Windows APIs are not especially clear on nullability.
/// 2. Handles can be either null or invalid, two different states!
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct HANDLE(u32);

impl HANDLE {
    pub const fn from_raw(raw: u32) -> Self {
        HANDLE(raw)
    }
    pub fn to_raw(&self) -> u32 {
        self.0
    }

    // Handles have both null and invalid states, whoopsie.
    // https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    pub fn null() -> Self {
        Self::from_raw(0)
    }
    pub fn invalid() -> Self {
        Self::from_raw(-1i32 as u32)
    }
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    pub fn is_invalid(&self) -> bool {
        self.0 == -1i32 as u32
    }
    pub fn is_null_or_invalid(&self) -> bool {
        self.is_null() || self.is_invalid()
    }

    pub fn to_option(self) -> Option<Self> {
        if self.is_null() || self.is_invalid() {
            None
        } else {
            Some(self)
        }
    }
}

impl std::fmt::Debug for HANDLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            return f.write_str("(null)");
        } else if self.is_invalid() {
            return f.write_str("(invalid)");
        }
        f.write_fmt(format_args!("HANDLE({:x})", self.0))
    }
}

impl crate::dllexport::FromABIParam for HANDLE {
    fn from_abi(val: u32) -> Self {
        Self::from_raw(val)
    }
}

impl Into<crate::dllexport::ABIReturn> for HANDLE {
    fn into(self) -> crate::dllexport::ABIReturn {
        self.0.into()
    }
}

/// Maintains a mapping of HANDLE -> V, vending out new handles.
pub struct Handles<V> {
    map: HashMap<HANDLE, V>,
    next: HANDLE,
}

impl<V> Default for Handles<V> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<V> Handles<V> {
    pub fn new(start: u32) -> Self {
        Handles {
            map: HashMap::default(),
            next: HANDLE::from_raw(start),
        }
    }

    pub fn reserve(&mut self) -> HANDLE {
        let handle = self.next;
        self.next = HANDLE::from_raw(self.next.to_raw() + 1);
        handle
    }

    pub fn set(&mut self, handle: HANDLE, t: V) {
        self.map.insert(handle, t);
    }

    pub fn add(&mut self, t: V) -> HANDLE {
        let handle = self.reserve();
        self.set(handle, t);
        handle
    }

    pub fn get(&self, handle: HANDLE) -> Option<&V> {
        self.map.get(&handle)
    }

    pub fn get_mut(&mut self, handle: HANDLE) -> Option<&mut V> {
        self.map.get_mut(&handle)
    }

    pub fn iter(&self) -> impl Iterator<Item = (HANDLE, &V)> {
        self.map.iter().map(|(k, v)| (*k, v))
    }

    pub fn remove(&mut self, handle: HANDLE) -> Option<V> {
        self.map.remove(&handle)
    }
}
