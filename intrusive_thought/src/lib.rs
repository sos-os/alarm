#![crate_type = "lib"]

#![cfg_attr(not(test), no_std)]
#![feature(shared)]
#![feature(const_fn)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
use std as core;

use core::default::Default;
use core::ptr::Shared;

struct Link<T>(Option<Shared<T>>);

impl<T> Link<T> {

    const fn none() -> Self {
        Link(None)
    }

    fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
            .map(|shared| unsafe { shared.as_ref() })
    }

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
