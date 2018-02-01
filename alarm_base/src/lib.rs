// ••• ALARM: the SOS memory allocator
// --- by Eliza Weisman (eliza@elizas.website)
// ••• and the SOS contributors
//
//  Copyright (c) 2018 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! Base types for ALARM allocators
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![feature(alloc, allocator_api)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate core;
#[cfg(feature = "lend")]
extern crate spin;

pub mod frame;
#[cfg(feature = "lend")]
pub mod lend;

pub use self::frame::Allocator as FrameAllocator;
