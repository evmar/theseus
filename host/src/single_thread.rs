use std::thread::ThreadId;

/// SDL has many APIs that must be accessed from the main thread, yikes.
/// SingleThreader stores the thread it's created on, then panics if you call .get() from a different thread.
pub struct SingleThreader<T> {
    id: ThreadId,
    data: T,
}

/// Safety: accessors assert we're on the initial thread.
unsafe impl<T> Sync for SingleThreader<T> {}
unsafe impl<T> Send for SingleThreader<T> {}

impl<T> SingleThreader<T> {
    pub fn new(data: T) -> Self {
        Self {
            id: std::thread::current().id(),
            data,
        }
    }

    pub fn get(&self) -> &T {
        assert_eq!(std::thread::current().id(), self.id);
        &self.data
    }
}
