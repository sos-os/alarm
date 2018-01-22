//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! # Intruder Alarm - ALARM intrusive collections library.
//!
//! _Intrusive_ data structures are data structures whose elements are
//! "aware" of the structures in which they are stored. That is to say
//! that data related to the layout and structure of an intrusive collection
//! is stored by the elements in the collection, rather than internally to
//! them.
//!
//! Intrusive data structures are useful for low-level programming in Rust
//! since they do not explicitly allocate memory. This means that we can use
//! intrusive structures to implement the kernel memory allocator and other
//! kernel subsystems which require structures such as lists prior to
//! the initialization of the kernel heap.
//!
//! This crate currently provides an intrusive linked-list implementation.
//!
//! # Features
//! + `std`: use the Rust standard library (`std`), rather than `core`.
#![crate_name = "intruder_alarm"]
#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]
#![feature(shared)]
#![feature(const_fn)]
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(any(feature = "std", test))]
use std as core;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::{fmt, mem};
use core::default::Default;
use core::ops::Deref;
use core::ptr::NonNull;

pub mod doubly;

/// Trait for references which own their referent.
///
/// # Safety
/// This trait is unsafe to implement because:
/// 1. The referent may not be moved while the owning ref is in an
///    intrusive collection.
/// 2. No references may be created to the referent _during_ an intrusive
///     collection operation.
/// 3. Finally, the implementing reference type must always dereference to
///    the _same_ object.
pub unsafe trait OwningRef<T: ?Sized>: Deref<Target = T> {
    /// Convert this into a raw pointer to the owned referent.
    fn into_ptr(self) -> *const Self::Target;

    /// Convert a raw pointer into an owning reference.
    unsafe fn from_ptr(p: *const Self::Target) -> Self;
}

/// A `Link` provides an [`Option`]-like interface to a [`NonNull`] pointer.
///
///
pub struct Link<T: ?Sized>(Option<NonNull<T>>);

// ===== impl OwningRef =====

unsafe impl<'a, T: ?Sized> OwningRef<T> for &'a T {
    #[inline]
    fn into_ptr(self) -> *const T {
        self
    }
    #[inline]
    unsafe fn from_ptr(p: *const T) -> Self {
        &*p
    }
}

unsafe impl<'a, T: ?Sized> OwningRef<T> for &'a mut T {
    #[inline]
    fn into_ptr(self) -> *const T {
        self
    }
    #[inline]
    unsafe fn from_ptr(p: *const T) -> Self {
        &mut *(p as *mut _)
    }
}

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use core::boxed::Box;

#[cfg(any(feature = "alloc", feature = "std", test))]
unsafe impl<T: ?Sized> OwningRef<T> for Box<T> {
    #[inline]
    fn into_ptr(self) -> *const T {
        Box::into_raw(self)
    }
    #[inline]
    unsafe fn from_ptr(p: *const T) -> Self {
        Box::from_raw(p as *mut T)
    }
}

// ===== impl Link =====

impl<T: ?Sized> Link<T> {
    /// Construct a new empty `Link`.
    #[inline]
    pub const fn none() -> Self {
        Link(None)
    }

    /// Resolve this `Link` to an `Option`.
    ///
    /// # Returns
    ///   - `Some<&'a T>` if the `Link` is not a null pointer
    ///   - `None` if the `Link` is a null pointer
    ///
    /// # Unsafe due to
    ///   - Returning a reference with an arbitrary lifetime
    ///   - Dereferencing a raw pointer
    fn as_ref(&self) -> Option<&T> {
        self.0.as_ref().map(|shared| unsafe { shared.as_ref() })
    }

    /// Mutably resolve this `Link` to an `Option`
    ///
    /// # Returns
    ///   - `Some<&'mut T>` if the `Link` is not a null pointer
    ///   - `None` if the `Link` is a null pointer
    ///
    /// # Unsafe due to
    ///   - Returning a reference with an arbitrary lifetime
    ///   - Dereferencing a raw pointer
    fn as_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|shared| unsafe { shared.as_mut() })
    }

    unsafe fn as_ptr(&mut self) -> Option<*mut T> {
        self.0.as_mut().map(|shared| shared.as_ptr())
    }

    /// Returns true if this link is empty.
    #[inline]
    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns true if this link is non-empty.
    #[inline]
    fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Take `self`, replacing it with `None`
    #[inline]
    fn take(&mut self) -> Option<Link<T>> {
        self.0.take().map(|x| Link(Some(x)))
    }

    /// Swaps the pointed value with `with`, returning the previous pointer.
    #[inline]
    unsafe fn replace<I: Into<Link<T>>>(&mut self, with: I) -> Link<T> {
        mem::replace(self, with.into())
    }

    fn from_owning_ref<R>(reference: R) -> Self
    where
        R: OwningRef<T>,
    {
        Link(NonNull::new(reference.into_ptr() as *mut _))
    }
}

// Cloning a `Link `returns a new unlinked `Link`, allowing types containing
// `Link`s to derive `Clone`.
impl<T: ?Sized> Clone for Link<T> {
    fn clone(&self) -> Self {
        Link::none()
    }
}

impl<T> Default for Link<T> {
    #[inline]
    fn default() -> Self {
        Link::none()
    }
}

impl<T: ?Sized> Copy for Link<T>
where
    Option<NonNull<T>>: Copy,
{
}
// impl<'a, T> From<&'a T> for Link<T> {
//     #[inline]
//     fn from(reference: &'a T) -> Self {
//         Link(Some(NonNull::from(reference)))
//     }
// }

// impl<'a, T> From<&'a mut T> for Link<T> {
//     #[inline]
//     fn from(reference: &'a mut T) -> Self {
//         Link(Some(NonNull::from(reference)))
//     }
// }

// impl<T> From<T> for Link<T> {
//     #[inline]
//     fn from(value: T) -> Self {
//         Link::from(&value)
//     }
// }

// impl<T, R> From<R> for Link<T>

// {
//     fn from(reference: R) -> Self {
//         Link(NonNull::new(reference.into_ptr() as *mut _))
//     }
// }

impl<T: fmt::Debug> fmt::Debug for Link<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            Some(t) => write!(f, "Link::Some({:?})", t),
            None => write!(f, "Link::None"),
        }
    }
}
