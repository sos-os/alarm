//! An intrusive linked list implementation using `RawLink`s.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.

use super::Link;

//-----------------------------------------------------------------------------
//  Linked
/// Trait that must be implemented in order to be a member of an intrusive
/// linked list.
pub trait Linked: Sized {
    /// Borrow this element's [`Links`].
    ///
    /// [`Links`]: struct.Links.html
    fn links(&self) -> &Links<Self>;

    /// Mutably borrow this element's [`Links`].
    ///
    /// [`Links`]: struct.Links.html
    fn links_mut(&mut self) -> &mut Links<Self>;

    /// Borrow the `next` element in the list, or `None` if this is the last.
    #[inline]
    fn next(&self) -> Option<&Self> {
        self.links().next()
    }

    /// Borrow the `prev` element in the list, or `None` if this is the first.
    #[inline]
    fn prev(&self) -> Option<&Self> {
        self.links().prev()
    }

    /// Mutably borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    fn next_mut(&mut self) -> Option<&mut Self> {
        self.links_mut().next_mut()
    }

    /// Mutably borrow the `prev` element in the list, or `None` if this is the
    /// first.
    #[inline]
    fn prev_mut(&mut self) -> Option<&mut Self> {
        self.links_mut().prev_mut()
    }

    /// Push an element to the front of the list.
    fn push_front(&mut self, mut element: Self) {
        // link the pushed node's prev link back to this node.
        element.links_mut().prev = Link::from(&*self);

        let links = self.links_mut();
        if let Some(next) = links.next.as_mut() {
            // if this node is currently linked to a next node, replace the next
            // node's prev link with the new node.
            next.links_mut().prev = Link::from(&element);
            // and link the pushed node's next link to this node's old next node.
            element.links_mut().next = Link::from(next);
        }
        // this node's next link points to the pushed node.
        links.next = Link::from(&element);
    }

    /// Push an element to the back of the list.
    fn push_back(&mut self, mut element: Self) {
        // link the pushed node's next link back to this node.
        element.links_mut().next = Link::from(&*self);

        let links = self.links_mut();
        if let Some(prev) = links.prev.as_mut() {
            // if this node is currently linked to a prev node, replace the
            // prev node's next link with the new node.
            prev.links_mut().next = Link::from(&element);
            // and link the pushed node's prev link to this node's old prev node.
            element.links_mut().prev = Link::from(prev);
        }
        // this node's prev link points to the pushed node.
        links.prev = Link::from(&element);
    }
}

//-----------------------------------------------------------------------------
/// Links
#[derive(Default)]
pub struct Links<T> {
    pub(super) next: Link<T>,
    pub(super) prev: Link<T>,
}

impl<T> Links<T> {
    /// Returns a new unlinked set of `Links`.
    #[inline]
    pub const fn new() -> Self {
        Links {
            next: Link::none(),
            prev: Link::none(),
        }
    }

    /// Borrow the `next` element in the list, or `None` if this is the last.
    #[inline]
    pub fn next(&self) -> Option<&T> {
        self.next.as_ref()
    }

    /// Borrow the `prev` element in the list, or `None` if this is the first.
    #[inline]
    pub fn prev(&self) -> Option<&T> {
        self.prev.as_ref()
    }

    /// Mutably borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    pub fn next_mut(&mut self) -> Option<&mut T> {
        self.next.as_mut()
    }

    /// Mutably borrow the `prev` element in the list, or `None` if this is the
    /// first.
    #[inline]
    pub fn prev_mut(&mut self) -> Option<&mut T> {
        self.prev.as_mut()
    }

}
