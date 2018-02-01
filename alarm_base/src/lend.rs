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
use spin;

/// An allocator that can provide borrowed handles.
pub trait Lend {
    /// The type of the allocator providing the allocation.
    ///
    /// This has to be an associated type rather than `Self`
    /// so that `Lend` can be implemented for e.g. `Mutex<A>`.
    type Allocator: Alloc;

    /// Borrow an allocation for a `T` from this lender.
    fn borrow<'a, T>(
        &'a self,
    ) -> Result<Borrowed<'a, T, Self::Allocator>, AllocErr>;
}

/// A borrowed handle on a heap allocation with a specified lifetime.
///
/// This automatically deallocates the allocated object when the borrow's
/// lifetime ends. It also ensures that the borrow only lives as long as the
/// allocator that provided it, and that the borrow is dropped if the allocator
/// is dropped.
///
/// # Type Parameters
/// - `'alloc`: the lifetime of the allocator from which this object was
///             recieved.
/// - `T`: the type of the allocated value
/// - `A`: the type of the allocator from which `T` was received.
pub struct Borrowed<'alloc, T, A>
where
    A: Alloc + 'alloc,
{
    /// The allocated value this `Borrowed` handle owns.
    value: ptr::NonNull<T>,

    /// A reference to the allocator that provided us with T.
    allocator: &'alloc ::spin::Mutex<A>,
}

// ===== impl Lend =====

impl<A> Lend for spin::Mutex<A>
where
    A: Alloc,
{
    type Allocator = A;

    /// Borrow an allocation for a `T` from this lender.
    fn borrow<'a, T>(
        &'a self,
    ) -> Result<Borrowed<'a, T, Self::Allocator>, AllocErr> {
        let value = self.lock().alloc_one::<T>();
        value.map(|value| Borrowed {
            value,
            allocator: self,
        })
    }
}

// ===== impl Borrowed =====

impl<'alloc, T, A> ops::Deref for Borrowed<'alloc, T, A>
where
    A: Alloc + 'alloc,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<'alloc, T, A> ops::DerefMut for Borrowed<'alloc, T, A>
where
    A: Alloc + 'alloc,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<'alloc, T, A> Drop for Borrowed<'alloc, T, A>
where
    A: Alloc + 'alloc,
{
    fn drop(&mut self) {
        let address = self.value.as_ptr();
        let layout = unsafe { Layout::for_value(self.value.as_ref()) };
        // ensure we drop the object _before_ deallocating it, so that
        // the object's `Drop` gets run first.
        mem::drop(address);
        unsafe {
            // lock the allocator and deallocate the object.
            self.allocator.lock().dealloc(address as *mut _, layout)
        }
    }
}
