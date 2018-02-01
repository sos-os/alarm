// ••• ALARM: the SOS memory allocator
// --- by Eliza Weisman (eliza@elizas.website)
// ••• and the SOS contributors
//
//  Copyright (c) 2018 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! Cursors allowing bi-directional traversal of data structures.
//!
use core::iter::{self, DoubleEndedIterator, Iterator};

//-----------------------------------------------------------------------------
// Traits
//-----------------------------------------------------------------------------
/// A cursor.
pub trait Cursor {
    /// The type of items "under" the cursor.
    type Item;

    /// Move the cursor one element forward.
    fn move_forward(&mut self);

    /// Move the cursor one element back.
    fn move_back(&mut self);

    /// Move the cursor `n` elements forward.
    #[inline]
    fn seek_forward(&mut self, n: usize) {
        for _ in 0..n {
            self.move_forward();
        }
    }

    /// Move the cursor `n` elements back.
    #[inline]
    fn seek_back(&mut self, n: usize) {
        for _ in 0..n {
            self.move_back();
        }
    }

    /// Return a reference to the item currently under the cursor.
    fn get(&self) -> Option<Self::Item>;

    /// Return a reference to the next element from the cursor's position.
    fn peek_next(&self) -> Option<Self::Item>;

    /// Return a reference to the previous element from the cursor's
    /// position.
    fn peek_back(&self) -> Option<Self::Item>;

    /// Advance the cursor one element and return a reference to that
    /// element.
    #[inline]
    fn next_item(&mut self) -> Option<Self::Item> {
        self.move_forward();
        self.get()
    }

    /// Move the cursor back one element and return a reference to that
    /// element.
    #[inline]
    fn prev_item(&mut self) -> Option<Self::Item> {
        self.move_back();
        self.get()
    }
}

/// A cursor that can mutate the parent data structure.
pub trait CursorMut<'a, T: 'a>: Cursor<Item = &'a mut T> {
    // TODO: some kind of `map`-like mutate in place function?

    /// Remove the element currently under the cursor.
    fn remove(&mut self) -> Option<T>;

    /// Find the first item matching predicate `P` and remove it
    /// from the data structure.
    fn remove_first<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&Self::Item) -> bool;

    /// Find all items matching predicate `P` and remove them
    /// from the data structure.
    fn remove_all<C, P>(&mut self, mut predicate: P) -> C
    where
        P: FnMut(&Self::Item) -> bool,
        C: iter::Extend<T> + iter::FromIterator<T>,
    {
        let mut items = iter::empty::<T>().collect::<C>();
        while let removed @ Some(_) = self.remove_first(&mut predicate) {
            items.extend(removed);
        }
        items
    }

    /// Insert the given item before the cursor's position.
    // TODO: ops::Place impl?
    fn insert_before(&mut self, item: T);

    /// Insert the given item after the cursor's position.
    // TODO: ops::Place impl?
    fn insert_after(&mut self, item: T);
}

/// Conversion into a `Cursor``.
///
/// By implementing `IntoCursor` for a type, you define how it will be
/// converted to a `Cursor`. This is common for types which describe a
/// collection of some kind.
///
/// ...yes, it's just `IntoIterator` for `Cursor`s.
pub trait IntoCursor {
    /// The type of the elements "under" the cursor.
    type Item;

    /// Which kind of cursor are we turning this into?
    type IntoCursor: Cursor<Item = Self::Item>;

    /// Create a cursor from a value.
    fn into_iter(self) -> Self::IntoCursor;
}

// ===== impl Cursor =====

impl<I> Iterator for Cursor<Item = I> {
    type Item = I;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get();
        self.move_forward();
        item
    }
}

impl<I> DoubleEndedIterator for Cursor<Item = I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev_item()
    }
}

// ===== impl IntoCursor =====
// TODO: trait object woes
// impl<I, C> IntoIterator for IntoCursor<Item = I, IntoCursor = C>
// where
//     C: Iterator<Item = I>,
// {
//     type Item = I;
//     type IntoIter = C;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.into_cursor()
//     }
//
// }
