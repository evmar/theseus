use std::{
    ptr::NonNull,
    sync::{Mutex, MutexGuard},
};

// We want to lock a Mutex<Option<State>> and return a MutexGuard<State>,
// but MutexGuard::map is not yet stable, so use this workaround for now.
pub struct LockedState<S: 'static> {
    _lock: MutexGuard<'static, Option<S>>,
    ptr: NonNull<S>,
}

impl<S> LockedState<S> {
    pub fn from(lock: &'static Mutex<Option<S>>) -> Self {
        let mut lock = lock.lock().unwrap();
        let ptr = NonNull::from_mut(lock.as_mut().unwrap());
        Self { _lock: lock, ptr }
    }
}

impl<S> std::ops::Deref for LockedState<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        // safety: _lock is holding the lock
        unsafe { self.ptr.as_ref() }
    }
}

impl<S> std::ops::DerefMut for LockedState<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // safety: _lock is holding the lock
        unsafe { self.ptr.as_mut() }
    }
}
