//
//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! # SOS Intrusive Collections
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
#![crate_name = "intrusive_thought"]
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
pub mod doubly;
