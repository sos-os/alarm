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
#![crate_type = "lib"]
// Use `no_std` attribute unless we are running tests or compiling with
// the "std" feature.
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(
    any(feature = "alloc", feature = "std", test),
    feature(box_into_raw_non_null)
)]
#![feature(const_fn)]
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(any(feature = "std", test))]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::{
    borrow::Borrow,
    default::Default,
    fmt,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub mod cursor;
pub use self::cursor::{Cursor, CursorMut};
pub mod list;
pub mod stack;

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

/// An unsafe `OwningRef` backed by a [`NonNull`] pointer.
pub struct UnsafeRef<T: ?Sized>(NonNull<T>);

/// A `Link` provides an [`Option`]-like interface to a [`NonNull`] pointer.
#[derive(Eq, PartialEq)]
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

#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use std::boxed::Box;

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

// ===== impl UnsafeRef =====

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T: ?Sized> UnsafeRef<T> {
    /// Convert a `Box<T>` into an `UnsafeRef<T>`.
    ///
    /// # Note
    /// This is primarily only useful for testing `UnsafeRef` --- if you are
    /// in an environment where you can easily allocate `Box`es, it is
    /// much safer to simply use the `OwningRef` impl for `Box`. Typically,
    /// `UnsafeRef` should only be used when there is no allocator capable
    /// of allocating `Box`es.
    #[inline]
    #[cfg_attr(
        not(test),
        deprecated(note = "Use of `UnsafeRef` is likely to be unnecessary \
                           when `Box` is available.")
    )]
    pub fn from_box(b: Box<T>) -> Self {
        UnsafeRef(Box::into_raw_non_null(b))
    }

    /// Construct a new `UnsafeRef` from a `T`, using `Box::new` to allocate a
    /// memory location for the moved value.
    ///
    /// Observant readers will note that this is essentially the only way to
    /// construct an `UnsafeRef` from a *moved* value, rather than from a
    /// reference --- this is by design. In order to construct an `UnsafeRef`
    /// for a moved value, space in memory must be allocated to contain that
    /// value. If we cannot create `Box`es, than we have no way of allocating
    /// a memory location for a moved value, and we can only construct
    /// `UnsafeRef`s from fixed memory locations.
    ///
    /// # Note
    /// This is primarily only useful for testing `UnsafeRef` --- if you are
    /// in an environment where you can easily allocate `Box`es, it is
    /// much safer to simply use the `OwningRef` impl for `Box`. Typically,
    /// `UnsafeRef` should only be used when there is no allocator capable
    /// of allocating `Box`es.
    #[inline]
    #[cfg_attr(
        not(test),
        deprecated(note = "Use of `UnsafeRef` is likely to be unnecessary \
                           when `Box` is available.")
    )]
    pub fn boxed(t: T) -> Self
    where
        T: Sized,
    {
        UnsafeRef::from_box(Box::new(t))
    }
}
impl<T: ?Sized> Clone for UnsafeRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        UnsafeRef(self.0)
    }
}

impl<T: ?Sized> Deref for UnsafeRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for UnsafeRef<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for UnsafeRef<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> Borrow<T> for UnsafeRef<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: ?Sized> fmt::Debug for UnsafeRef<T>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: ?Sized, F> From<F> for UnsafeRef<T>
where
    NonNull<T>: From<F>,
{
    #[inline]
    fn from(f: F) -> Self {
        UnsafeRef(NonNull::from(f))
    }
}

unsafe impl<T: ?Sized> OwningRef<T> for UnsafeRef<T> {
    #[inline]
    fn into_ptr(self) -> *const T {
        self.0.as_ptr() as *const T
    }
    #[inline]
    unsafe fn from_ptr(p: *const T) -> Self {
        NonNull::new(p as *mut T)
            .map(UnsafeRef)
            .expect("attempted to create OwningRef from null pointer!")
    }
}

impl<T: ?Sized> PartialEq for UnsafeRef<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
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
    fn as_ref<'a>(&'a self) -> Option<&'a T> {
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

impl<T: ?Sized> Copy for Link<T> where Option<NonNull<T>>: Copy {}
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
