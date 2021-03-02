// MIT/Apache2 License

#![cfg(target_vendor = "apple")]
#![allow(clippy::too-many-arguments)]
#![allow(clippy::useless-conversion)]
//! Equivalent to `core_graphics::CGContext` but with the ability to push its work onto a thread pool.

#[macro_use]
extern crate objc;

pub mod context;
pub mod data_provider;
pub mod error;
pub mod image;
pub mod spawner;

pub(crate) mod util;

pub use context::*;
pub use error::*;
