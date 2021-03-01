// MIT/Apache2 License

#![cfg(target_vendor = "apple")]
//! Equivalent to `core_graphics::CGContext` but with the ability to push its work onto a thread pool.

#[macro_use]
extern crate objc;

pub mod context;
pub mod data_provider;
pub mod error;
pub mod image;
pub mod spawner;

pub use context::*;
pub use error::*;
