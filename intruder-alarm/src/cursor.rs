// ••• ALARM: the SOS memory allocator
// --- by Eliza Weisman (eliza@elizas.website)
// ••• and the SOS contributors
//
//  Copyright (c) 2018 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! Cursors allowing bi-directional traversal of data structures.
use core::{iter, ops};
use crate::OwningRef;

//-----------------------------------------------------------------------------
// Traits
//-----------------------------------------------------------------------------
/// A cursor.
pub trait Cursor {
    /// The type of items "under" the cursor.
    type Item;

    /// Move the cursor one element forward.
    fn move_forward(&mut self) -> &mut Self;

    /// Move the cursor one element back.
    fn move_back(&mut self) -> &mut Self;

    /// Move the cursor `n` elements forward.
    #[inline]
    fn seek_forward(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.move_forward();
        }
        self
    }

    /// Move the cursor `n` elements back.
    #[inline]
    fn seek_back(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.move_back();
        }
        self
    }

    /// Return a reference to the item currently under the cursor.
    fn get(&self) -> Option<&Self::Item>;

    /// Return a reference to the next element from the cursor's position.
    fn peek_next(&self) -> Option<&Self::Item>;

    /// Return a reference to the previous element from the cursor's
    /// position.
    fn peek_back(&self) -> Option<&Self::Item>;

    /// Advance the cursor one element and return a reference to that
    /// element.
    #[inline]
    fn next_item(&mut self) -> Option<&Self::Item> {
        self.move_forward().get()
    }

    /// Move the cursor back one element and return a reference to that
    /// element.
    #[inline]
    fn prev_item(&mut self) -> Option<&Self::Item> {
        self.move_back().get()
    }
}

/// A cursor that can mutate the parent data structure.
pub trait CursorMut<T, N>: Cursor<Item = T>
where
    N: AsRef<T> + AsMut<T>,
{
    /// The type of [`OwningRef`] used by the parent data structure.
    type Ref: OwningRef<N>;

    /// Return a reference to the item currently under the cursor.
    fn get_mut(&mut self) -> Option<&mut T>;

    /// Return a reference to the next element from the cursor's position.
    fn peek_next_mut(&mut self) -> Option<&mut T>;

    /// Return a reference to the previous element from the cursor's
    /// position.
    fn peek_back_mut(&mut self) -> Option<&mut T>;

    /// Advance the cursor one element and return a mutable reference to that
    /// element.
    #[inline]
    fn next_item_mut(&mut self) -> Option<&mut T> {
        self.move_forward().get_mut()
    }

    /// Move the cursor back one element and return a mutable reference to
    /// that element.
    #[inline]
    fn prev_item_mut(&mut self) -> Option<&mut T> {
        self.move_back().get_mut()
    }

    /// Remove the node currently under the cursor.
    fn remove_node(&mut self) -> Option<Self::Ref>;

    /// Find the first node matching predicate `P` and remove it.
    fn remove_first_node<P>(&mut self, mut predicate: P) -> Option<Self::Ref>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        // XXX: This would be much more cleanly expressed with a `while let`,
        //      but that doesn't quite work for Borrow Checker Reasons...
        loop {
            // First, test if the cursor is over an item, and if the item
            // matches the predicate.
            let matches = if let Some(ref item) = self.get() {
                predicate(item)
            } else {
                // If we're not over an item, then we've scanned the entire
                // structure and haven't found a match, so we `None`.
                return None;
            };

            // If the item matches, remove and return it.
            if matches {
                return self.remove_node();
            }

            // Otherwise, continue searching.
            self.move_forward();
        }
    }

    /// Find all nodes matching predicate `P` and remove them
    /// from the data structure.
    fn remove_all_nodes<C, P>(&mut self, mut predicate: P) -> C
    where
        P: FnMut(&Self::Item) -> bool,
        C: iter::Extend<Self::Ref> + iter::FromIterator<Self::Ref>,
    {
        let mut items = iter::empty::<Self::Ref>().collect::<C>();
        while let removed @ Some(_) = self.remove_first_node(&mut predicate) {
            items.extend(removed);
        }
        items
    }

    /// Insert the given node before the cursor's position.
    // TODO: ops::Place impl?
    fn insert_node_before(&mut self, node: Self::Ref) -> &mut Self
    where
        Self::Ref: ops::DerefMut;

    /// Insert the given node after the cursor's position.
    // TODO: ops::Place impl?
    fn insert_node_after(&mut self, node: Self::Ref) -> &mut Self
    where
        Self::Ref: ops::DerefMut;

    /// Iterate over each item in the data structure and mutate it in place
    /// with function `f`.
    fn map_in_place<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Self::Item),
    {
        loop {
            let done = if let Some(ref mut i) = self.get_mut() {
                // If there is an item under the cursor, mutate it with `f`.
                f(i);
                false
            } else {
                // Otherwise, we've reached the end of the data structure.
                true
            };

            if done {
                return self;
            }

            // Advance the cursor to the next element.
            self.move_forward();
        }
    }
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
    fn into_cursor(self) -> Self::IntoCursor;
}
