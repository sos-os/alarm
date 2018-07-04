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
extern crate hal9000;
extern crate spin;

pub mod frame;
#[cfg(feature = "lend")]
pub mod lend;

pub use self::frame::Allocator as FrameAllocator;
use alloc::allocator::{Alloc, AllocErr, Layout};
use core::ptr;

/// An allocator behind a mutex.
#[derive(Debug)]
pub struct LockedAlloc<A>(spin::Mutex<A>);

// ===== impl LockedAlloc =====

impl<A> core::ops::Deref for LockedAlloc<A> {
    type Target = spin::Mutex<A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<'a, A> Alloc for &'a LockedAlloc<A>
where
    A: Alloc,
{
    unsafe fn alloc(
        &mut self,
        layout: Layout,
    ) -> Result<ptr::NonNull<u8>, AllocErr> {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: ptr::NonNull<u8>, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}

unsafe impl<'a, A> FrameAllocator for &'a LockedAlloc<A>
where
    A: FrameAllocator,
{
    type Frame = A::Frame;
    const FRAME_SIZE: usize = A::FRAME_SIZE;

    unsafe fn alloc(&mut self) -> Result<Self::Frame, AllocErr> {
        self.lock().alloc()
    }

    unsafe fn dealloc(&mut self, frame: Self::Frame) -> Result<(), AllocErr> {
        self.lock().dealloc(frame)
    }
}
