// MIT/Apache2 License

use std::ops;

// container for sync objects
#[derive(Debug, Clone)]
pub struct SyncContainer<T>(T);

unsafe impl<T> Send for SyncContainer<T> {}
unsafe impl<T> Sync for SyncContainer<T> {}

impl<T> SyncContainer<T> {
    #[inline]
    pub unsafe fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> ops::Deref for SyncContainer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for SyncContainer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
