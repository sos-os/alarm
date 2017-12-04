//
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
//! + `use-std`: use the Rust standard library (`std`), rather than `core`.
#![crate_name = "intruder_alarm"]
#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]
#![feature(shared)]
#![feature(const_fn)]
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
use std as core;

use core::default::Default;
use core::ptr::Shared;
use core::{fmt, mem};

/// A `Link` provides an [`Option`]-like interface to a [`Shared`] pointer.
///
///
pub struct Link<T>(Option<Shared<T>>);

impl<T> Link<T> {

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
        self.0.as_ref()
            .map(|shared| unsafe { shared.as_ref() })
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
        self.0.as_mut()
            .map(|shared| unsafe { shared.as_mut() })
    }

    unsafe fn as_ptr(&mut self) -> Option<*mut T> {
        self.0.as_mut().map(|shared| shared.as_ptr())
    }

    /// Returns true if this link is empty.
    #[inline] fn is_none(&self) -> bool { 
        self.0.is_none()
    }

    /// Returns true if this link is non-empty.
    #[inline] fn is_some(&self) -> bool { 
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
}

impl<T> Default for Link<T> {
    #[inline]
    fn default() -> Self {
        Link::none()
    }
}

impl<'a, T> From<&'a T> for Link<T> {
    #[inline]
    fn from(reference: &'a T) -> Self {
        Link(Some(Shared::from(reference)))
    }
}

impl<'a, T> From<&'a mut T> for Link<T> {
    #[inline]
    fn from(reference: &'a mut T) -> Self {
        Link(Some(Shared::from(reference)))
    }
}

impl<T> From<T> for Link<T> {
    #[inline]
    fn from(value: T) -> Self {
        Link::from(&value)
    }
}

impl<T: fmt::Debug> fmt::Debug for Link<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            Some(t) => write!(f, "Link::Some({:?})", t),
            None => write!(f, "Link::None")
        }
    }

}

pub mod doubly;
