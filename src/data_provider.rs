// MIT/Apache2 License

use crate::util::SyncContainer;
use core_graphics::data_provider::CGDataProvider;
use std::{mem::ManuallyDrop, sync::Arc};

pub struct DataProvider<T> {
    provider: ManuallyDrop<SyncContainer<CGDataProvider>>,
    _real_data: T,
}

impl<T> DataProvider<T> {
    #[inline]
    pub fn provider(&self) -> &CGDataProvider {
        &self.provider
    }
}

impl<T: AsRef<[u8]> + Send + Sync> DataProvider<Arc<T>> {
    #[inline]
    pub fn from_arc(val: Arc<T>) -> Self {
        // SAFETY: all involved elements are thread safe
        Self {
            provider: unsafe {
                ManuallyDrop::new(SyncContainer::new(CGDataProvider::from_buffer(val.clone())))
            },
            _real_data: val,
        }
    }
}

impl<T: AsRef<[u8]> + Send + Sync> DataProvider<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        // SAFETY: we hold the data for the duration of our activity
        let provider = unsafe { CGDataProvider::from_slice(val.as_ref()) };
        Self {
            provider: unsafe { ManuallyDrop::new(SyncContainer::new(provider)) },
            _real_data: val,
        }
    }
}

impl<T> Drop for DataProvider<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: we need to drop the provider first to trigger the release mechanism before we drop the real data
        unsafe { ManuallyDrop::drop(&mut self.provider) }
    }
}
