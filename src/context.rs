// MIT/Apache2 License

use crate::{image::Image, objc_try, spawner::Spawner, util::SyncContainer};
use core_graphics::{
    context::{CGContext, CGContextRef, CGPathDrawingMode},
    geometry::{CGPoint, CGRect, CGSize},
};
use orphan_crippler::Receiver;
use std::fmt;

/// The context in which drawing takes place.
#[derive(Clone)]
pub struct Context<S> {
    inner: SyncContainer<CGContext>,
    spawner: S,
}

impl<S: fmt::Debug> fmt::Debug for Context<S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Inner;

        impl fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("CGContext")
            }
        }

        f.debug_struct("Context")
            .field("inner", &Inner)
            .field("spawner", &self.spawner)
            .finish()
    }
}

impl<S> Context<S> {
    #[inline]
    pub fn spawner(&self) -> &S {
        &self.spawner
    }
}

impl<S: Spawner> Context<S> {
    /// # Safety
    ///
    /// Use a valid pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut (), spawner: S) -> Self {
        let cref = ptr as *mut CGContextRef;
        let cg = CGContext::from_existing_context_ptr(cref.cast());
        Self {
            inner: SyncContainer::new(cg),
            spawner,
        }
    }

    #[inline]
    pub fn set_rgb_fill_color(&self, r: f64, g: f64, b: f64, a: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.set_rgb_fill_color(r, g, b, a)))
    }

    #[inline]
    pub fn set_rgb_stroke_color(&self, r: f64, g: f64, b: f64, a: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.set_rgb_stroke_color(r, g, b, a)))
    }

    #[inline]
    pub fn save(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || objc_try!(cx.save()))
    }

    #[inline]
    pub fn restore(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || objc_try!(cx.restore()))
    }

    #[inline]
    pub fn move_to_point(&self, x: f64, y: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.move_to_point(x, y)))
    }

    #[inline]
    pub fn add_line_to_point(&self, x: f64, y: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.add_line_to_point(x, y)))
    }

    #[inline]
    pub fn add_curve_to_point(
        &self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.add_curve_to_point(cp1x, cp1y, cp2x, cp2y, x, y)))
    }

    #[inline]
    pub fn close_path(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.close_path()))
    }

    #[inline]
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || {
            objc_try!(cx.fill_rect(CGRect {
                origin: CGPoint { x, y },
                size: CGSize { width, height }
            }))
        })
    }

    #[inline]
    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || {
            objc_try!(cx.stroke_rect(CGRect {
                origin: CGPoint { x, y },
                size: CGSize { width, height }
            }))
        })
    }

    #[inline]
    pub fn fill_ellipse_in_rect(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || {
            objc_try!(cx.fill_ellipse_in_rect(CGRect {
                origin: CGPoint { x, y },
                size: CGSize { width, height }
            }))
        })
    }

    #[inline]
    pub fn stroke_ellipse_in_rect(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || {
            objc_try!(cx.stroke_ellipse_in_rect(CGRect {
                origin: CGPoint { x, y },
                size: CGSize { width, height }
            }))
        })
    }

    #[inline]
    pub fn flush(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner.spawn_blocking(move || objc_try!(cx.flush()))
    }

    #[inline]
    pub fn set_line_width(&self, width: f64) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.set_line_width(width)))
    }

    #[inline]
    pub fn fill_path(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.fill_path()))
    }

    #[inline]
    pub fn stroke_path(&self) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.stroke_path()))
    }

    #[inline]
    pub fn draw_path(&self, options: CGPathDrawingMode) -> Receiver<crate::Result> {
        let cx = self.context();
        self.spawner
            .spawn_blocking(move || objc_try!(cx.draw_path(options)))
    }

    #[inline]
    pub fn draw_image<T>(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        image: &Image<T>,
    ) -> Receiver<crate::Result> {
        let cx = self.context();
        let img = image.image().clone();
        self.spawner.spawn_blocking(move || {
            objc_try!(cx.draw_image(
                CGRect {
                    origin: CGPoint { x, y },
                    size: CGSize { width, height }
                },
                &*img
            ))
        })
    }

    #[inline]
    pub fn inner(&self) -> &CGContext {
        &self.inner
    }

    #[inline]
    fn context(&self) -> SyncContainer<CGContext> {
        self.inner.clone()
    }
}
