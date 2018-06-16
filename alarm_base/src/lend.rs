//
// ••• ALARM: the SOS memory allocator
// --- by Eliza Weisman (eliza@elizas.website)
// ••• and the SOS contributors
//
//  Copyright (c) 2018 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! Borrowed handles on allocations with fixed (Rust) lifetimes,
//!
//! or, "So You've Always Wished `*mut u8` Could `impl Drop`..."
use alloc::allocator::{Alloc, AllocErr, Layout};
use core::{mem, ops, ptr};

/// An allocator that can provide borrowed handles.
pub trait Lend: Alloc + Sized {

    /// Borrow an allocation for a `T` from this lender.
    fn borrow<T>(self) -> Result<Borrowed<T, Self>, AllocErr>;
}

/// A borrowed handle on a heap allocation with a specified lifetime.
///
/// This automatically deallocates the allocated object when the borrow's
/// lifetime ends. It also ensures that the borrow only lives as long as the
/// allocator that provided it, and that the borrow is dropped if the allocator
/// is dropped.
///
/// # Type Parameters
/// - `T`: the type of the allocated value
/// - `A`: the type of the allocator from which `T` was received.
pub struct Borrowed<T, A>
where
    A: Alloc
{
    /// The allocated value this `Borrowed` handle owns.
    value: ptr::NonNull<T>,

    /// A reference to the allocator that provided us with T.
    allocator: A
}

// ===== impl Lend =====

impl<A> Lend for A
where
    A: Alloc,
{

    /// Borrow an allocation for a `T` from this lender.
    fn borrow<T>(mut self) -> Result<Borrowed<T, Self>, AllocErr> {
        self
            .alloc_one::<T>()
            .map(|value| Borrowed {
                value,
                allocator: self,
            })
    }
}

// ===== impl Borrowed =====

impl<T, A> ops::Deref for Borrowed<T, A>
where
    A: Alloc
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T, A> ops::DerefMut for Borrowed<T, A>
where
    A: Alloc
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<T, A> Drop for Borrowed<T, A>
where
    A: Alloc
{
    fn drop(&mut self) {
        let address = self.value.cast::<u8>();
        let layout = unsafe { Layout::for_value(self.value.as_ref()) };
        // ensure we drop the object _before_ deallocating it, so that
        // the object's `Drop` gets run first.
        mem::drop(address);
        unsafe {
            // lock the allocator and deallocate the object.
            self.allocator.dealloc(address, layout)
        }
    }
}
