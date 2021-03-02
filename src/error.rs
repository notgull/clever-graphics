// MIT/Apache2 License

use objc::runtime::{Object, BOOL};
use objc_exception::Exception;
use std::{error::Error as ErrorTrait, ffi::CStr, fmt, os::raw::c_char};

/// An error resulting from normal operation of one of the drawing functions.
pub struct Error {
    exception: *mut Exception,
    is_nsexception: bool,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    #[inline]
    pub fn from_exception(exception: *mut Exception) -> Self {
        let nsexception = class!(NSException);
        let is_nsexception: BOOL =
            unsafe { msg_send![exception as *const Object, isKindOfClass: nsexception] };
        Self {
            exception,
            is_nsexception: is_nsexception != 0,
        }
    }
}

/// Convenience type for results.
pub type Result<T = ()> = std::result::Result<T, Error>;

/// Macro to try to run the interior expression and catch any errors.
#[macro_export]
macro_rules! objc_try {
    ($e: expr) => {{
        unsafe { objc_exception::r#try(move || $e) }.map_err($crate::Error::from_exception)
    }};
}

impl ErrorTrait for Error {}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum InnerDesc<'a> {
            BlackBox,
            Str(&'a str),
        }

        impl<'a> fmt::Debug for InnerDesc<'a> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::BlackBox => f.write_str("<unknown>"),
                    Self::Str(s) => f.write_str(s),
                }
            }
        }

        let inner_desc = if self.is_nsexception {
            // get the name of the exception
            let name: *const c_char = unsafe {
                let name: *mut Object = msg_send![self.exception as *mut Object, reason];
                msg_send![name, UTF8String]
            };

            let name: &CStr = unsafe { CStr::from_ptr(name) };
            let name: &str = name
                .to_str()
                .expect("Objective-C string should be guaranteed UTF-8");

            InnerDesc::Str(name)
        } else {
            InnerDesc::BlackBox
        };

        f.debug_tuple("Error").field(&inner_desc).finish()
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nsexception {
            let desc: *const c_char = unsafe {
                let desc: *mut Object = msg_send![self.exception as *mut Object, reason];
                msg_send![desc, UTF8String]
            };

            let desc: &CStr = unsafe { CStr::from_ptr(desc) };
            let desc: &str = desc
                .to_str()
                .expect("Objective-C string should be guaranteed UTF-8");

            write!(f, "Caught exception: {}", desc)
        } else {
            f.write_str("An unknown exception was caught")
        }
    }
}
