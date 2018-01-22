//! Base types for ALARM allocators
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![feature(alloc, allocator_api)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate core;

pub mod frame;

pub use self::frame::Allocator as FrameAllocator;