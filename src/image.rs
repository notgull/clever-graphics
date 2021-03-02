// MIT/Apache2 License

use crate::{data_provider::DataProvider, util::SyncContainer};
use core_graphics::{color_space::CGColorSpace, image::CGImage};
use std::mem::ManuallyDrop;

/// An image.
pub struct Image<T> {
    image: ManuallyDrop<SyncContainer<CGImage>>,
    _data_provider: DataProvider<T>,
}

impl<T> Image<T> {
    #[inline]
    pub(crate) fn image(&self) -> &SyncContainer<CGImage> {
        &self.image
    }
}

impl<T: AsRef<[u8]> + Send + Sync> Image<T> {
    #[inline]
    pub fn new(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        colorspace: &CGColorSpace,
        bitmap_info: u32,
        provider: DataProvider<T>,
        should_interpolate: bool,
        rendering_intent: u32,
    ) -> Self {
        let image = CGImage::new(
            width.into(),
            height.into(),
            bits_per_component.into(),
            bits_per_pixel.into(),
            bytes_per_row.into(),
            colorspace,
            bitmap_info,
            provider.provider(),
            should_interpolate,
            rendering_intent,
        );
        Self {
            image: unsafe { ManuallyDrop::new(SyncContainer::new(image)) },
            _data_provider: provider,
        }
    }
}

impl<T> Drop for Image<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.image) };
    }
}
