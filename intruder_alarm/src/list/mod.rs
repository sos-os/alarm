//! An intrusive linked list implementation using `RawLink`s.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.
use cursor::{self, Cursor as CursorTrait};
use Link;
use OwningRef;
use UnsafeRef;

use core::{
    iter::{DoubleEndedIterator, Extend, FromIterator, Iterator},
    marker::PhantomData,
    mem,
    ops::DerefMut,
};

#[cfg(test)]
mod tests;

//-----------------------------------------------------------------------------
// Public API types
//-----------------------------------------------------------------------------
//  List
/// An intrusive doubly-linked list.
///
/// This type is a wrapper around a series of [`Node`]s. It stores [`Link`]s
/// to the head and tail [`Node`]s and the length of the list.
///
/// # Type parameters
/// - `T`: the type of the items stored by each `N`
/// - `N`: the type of nodes in the list
/// - `R`: the type of [`OwningRef`] that owns each `N`.
///
/// [`Node`]: trait.Node.html
/// [`Link`]: ../struct.Link.html
/// [`OwningRef]: ../trait.OwningRef.html
#[derive(Debug, Default)]
pub struct List<T, N, R> {
    /// Link to the head node of the list.
    head: Link<N>,

    /// Link to the tail node of the list.
    tail: Link<N>,

    /// Length of the list.
    len: usize,

    /// Type marker for items stored in the list.
    _elem_ty: PhantomData<T>,

    /// Type marker for the `OwningRef` type.
    _ref_ty: PhantomData<R>,
}

//  Linked
/// Trait that must be implemented in order to be a member of an intrusive
/// linked list.
pub trait Linked: Sized // + Drop
{
    /// Borrow this element's [`Links`].
    ///
    /// [`Links`]: struct.Links.html
    fn links(&self) -> &Links<Self>;

    /// Mutably borrow this element's [`Links`].
    ///
    /// [`Links`]: struct.Links.html
    fn links_mut(&mut self) -> &mut Links<Self>;

    /// De-link this node, returning its' Links.
    fn take_links(&mut self) -> Links<Self> {
        mem::replace(self.links_mut(), Links::new())
    }

    /// Borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    fn next(&self) -> Option<&Self> {
        self.links().next()
    }

    /// Borrow the `prev` element in the list, or `None` if this is the
    /// first.
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

    /// Borrow the `next` linked element, or `None` if this is the last.
    #[inline]
    fn peek_next<T>(&self) -> Option<&T>
    where
        Self: AsRef<T>,
    {
        self.next().map(Self::as_ref)
    }

    /// Borrow the `prev` linked element, or `None` if this is the first.
    #[inline]
    fn peek_prev<T>(&self) -> Option<&T>
    where
        Self: AsRef<T>,
    {
        self.prev().map(Self::as_ref)
    }

    /// Mutably borrow the `next` linked element, or `None` if this is the
    /// last.
    #[inline]
    fn peek_next_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsMut<T>,
    {
        self.next_mut().map(Self::as_mut)
    }

    /// Mutably borrow the `prev` linked element, or `None` if this is the
    /// first.
    #[inline]
    fn peek_prev_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsMut<T>,
    {
        self.prev_mut().map(Self::as_mut)
    }
}

/// Links
#[derive(Default, Debug)]
pub struct Links<T> {
    pub(super) next: Link<T>,
    pub(super) prev: Link<T>,
}

/// A read-only cursor over the elements of a `List`.
#[derive(Debug)]
pub struct Cursor<'a, T: 'a, N: 'a> {
    current: Option<&'a N>,
    _marker: PhantomData<&'a T>,
}

/// A mutable cursor over the elements of a `List`.
#[derive(Debug)]
pub struct CursorMut<'a, T: 'a, N: 'a, R: 'a> {
    current: Link<N>,
    list: &'a mut List<T, N, R>,
}

//-----------------------------------------------------------------------------
// Implementations
//-----------------------------------------------------------------------------

// ===== impl List =====

impl<T, Node, R> List<T, Node, R> {
    /// Create a new `List` with 0 elements.
    pub const fn new() -> Self {
        List {
            head: Link::none(),
            tail: Link::none(),
            len: 0,
            _elem_ty: PhantomData,
            _ref_ty: PhantomData,
        }
    }

    /// Returns the length of the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty, false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows the first node of the list as an `Option`.
    ///
    /// Note that this is distinct from `front`: this method
    /// borrows the head _node_, not the head _element_.
    ///
    /// # Returns
    ///   - `Some(&N)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn head(&self) -> Option<&Node> {
        self.head.as_ref()
    }

    /// Borrows the last node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn tail(&self) -> Option<&Node> {
        self.tail.as_ref()
    }

    /// Mutably borrows the first node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn head_mut(&mut self) -> Option<&mut Node> {
        self.head.as_mut()
    }

    /// Mutably borrows the last node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn tail_mut(&mut self) -> Option<&mut Node> {
        self.tail.as_mut()
    }
}

impl<T, Node, Ref> List<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
    Ref: DerefMut,
{
    /// Push a node to the head of the list.
    pub fn push_front_node(&mut self, mut node: Ref) -> &mut Self {
        unsafe {
            node.links_mut().next = self.head;
            node.links_mut().prev = Link::none();
            let node = Link::from_owning_ref(node);

            match self.head.0 {
                None => self.tail = node,
                Some(mut head) => head.as_mut().links_mut().prev = node,
            }

            self.head = node;
            self.len += 1;
        };
        self
    }

    /// Push an node to the back of the list.
    pub fn push_back_node(&mut self, mut node: Ref) -> &mut Self {
        unsafe {
            node.links_mut().next = Link::none();
            node.links_mut().prev = self.tail;
            let node = Link::from_owning_ref(node);

            match self.tail.0 {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().links_mut().next = node,
            }

            self.tail = node;
            self.len += 1;
        };
        self
    }
}

impl<T, Node, Ref> List<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
{
    /// Pop a node from the front of the list.
    pub fn pop_front_node(&mut self) -> Option<Ref> {
        unsafe {
            self.head.as_ptr().map(|node| {
                self.head = (*node).take_links().next;

                match self.head.as_mut() {
                    None => self.tail = Link::none(),
                    Some(head) => head.links_mut().prev = Link::none(),
                }

                self.len -= 1;
                Ref::from_ptr(node as *const Node)
            })
        }
    }

    /// Pop a node from the back of the list.
    pub fn pop_back_node(&mut self) -> Option<Ref> {
        unsafe {
            self.tail.as_ptr().map(|node| {
                self.tail = (*node).take_links().prev;

                match self.tail.as_mut() {
                    None => self.head = Link::none(),
                    Some(tail) => tail.links_mut().next = Link::none(),
                }

                self.len -= 1;
                Ref::from_ptr(node as *const Node)
            })
        }
    }
}

impl<T, Node, R> List<T, Node, R>
where
    Node: AsRef<T>,
{
    /// Borrows the first item of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.head().map(Node::as_ref)
    }

    /// Borrows the last item of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.tail().map(Node::as_ref)
    }

    /// Return an immutable `Cursor` over the items of this `List`.
    pub fn cursor<'a>(&'a self) -> Cursor<'a, T, Node> {
        Cursor {
            current: self.head.as_ref(),
            _marker: PhantomData,
        }
    }

    /// Return a mutable `Cursor` over the items of this `List`.
    pub fn cursor_mut<'a>(&'a mut self) -> CursorMut<'a, T, Node, R> {
        CursorMut {
            current: self.head,
            list: self,
        }
    }
}

impl<T, Node, R> List<T, Node, R>
where
    Node: AsMut<T>,
{
    /// Mutably borrows the first element of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head_mut().map(Node::as_mut)
    }

    /// Mutably borrows the last element of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail_mut().map(Node::as_mut)
    }
}

impl<T, Node> List<T, Node, UnsafeRef<Node>>
where
    Node: Linked,
{
    /// Push an item to the front of the list.
    #[inline]
    pub fn push_front<I>(&mut self, item: I) -> &mut Self
    where
        I: Into<UnsafeRef<Node>>,
    {
        self.push_front_node(item.into())
    }

    /// Push an item to the back of the list.
    #[inline]
    pub fn push_back<I>(&mut self, item: I) -> &mut Self
    where
        I: Into<UnsafeRef<Node>>,
    {
        self.push_back_node(item.into())
    }
}

#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use std::boxed::Box;

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> List<T, Node, Box<Node>>
where
    Node: From<T>,
    Node: Linked,
{
    /// Push an item to the front of the list.
    #[inline]
    pub fn push_front(&mut self, item: T) -> &mut Self {
        self.push_front_node(Box::new(Node::from(item)))
    }

    /// Push an item to the back of the list.
    #[inline]
    pub fn push_back(&mut self, item: T) -> &mut Self {
        self.push_back_node(Box::new(Node::from(item)))
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> List<T, Node, Box<Node>>
where
    Node: Linked,
    Node: Into<T>,
{
    /// Pop an item from the front of the list.
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|b| (*b).into())
    }

    /// Pop an item from the front of the list.
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|b| (*b).into())
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> Extend<T> for List<T, Node, Box<Node>>
where
    Node: From<T> + Linked,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T, Node, R> Extend<R> for List<T, Node, UnsafeRef<Node>>
where
    R: Into<UnsafeRef<Node>>,
    Node: Linked,
{
    #[inline]
    fn extend<I: IntoIterator<Item = R>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T, Node, Ref, E> FromIterator<E> for List<T, Node, Ref>
where
    Self: Extend<E>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = E>>(iter: I) -> Self {
        let mut list = List::new();
        list.extend(iter);
        list
    }
}

// ===== impl Links =====

impl<T> Links<T> {
    /// Returns a new unlinked set of `Links`.
    #[inline]
    const fn new() -> Self {
        Links {
            next: Link::none(),
            prev: Link::none(),
        }
    }

    /// Borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    fn next(&self) -> Option<&T> {
        self.next.as_ref()
    }

    /// Borrow the `prev` element in the list, or `None` if this is the
    /// first.
    #[inline]
    fn prev(&self) -> Option<&T> {
        self.prev.as_ref()
    }

    /// Mutably borrow the `next` element in the list.
    ///
    /// # Returns
    /// - `Some(&mut T)` if there is a next element in the list.
    /// -  or `None` if this is the last.
    #[inline]
    fn next_mut(&mut self) -> Option<&mut T> {
        self.next.as_mut()
    }

    /// Mutably borrow the `prev` element in the list.
    ///
    /// # Returns
    /// - `Some(&mut T)` if there is a previous element in the list.
    /// -  or `None` if this is the first.
    #[inline]
    fn prev_mut(&mut self) -> Option<&mut T> {
        self.prev.as_mut()
    }
}

impl<T> Clone for Links<T> {
    #[inline]
    fn clone(&self) -> Self {
        Links::new()
    }
}

// ===== impl Cursor =====

impl<'a, T, Node> cursor::Cursor for Cursor<'a, T, Node>
where
    Node: Linked,
    Node: AsRef<T>,
{
    type Item = T;

    fn move_forward(&mut self) -> &mut Self {
        self.current = self.current.and_then(Linked::next);
        self
    }

    fn move_back(&mut self) -> &mut Self {
        self.current = self.current.and_then(Linked::prev);
        self
    }

    fn get(&self) -> Option<&Self::Item> {
        self.current.map(Node::as_ref)
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.current.and_then(Linked::next).map(Node::as_ref)
    }

    fn peek_back(&self) -> Option<&Self::Item> {
        self.current.and_then(Linked::prev).map(Node::as_ref)
    }
}

impl<'a, T, Node> Iterator for Cursor<'a, T, Node>
where
    Node: Linked,
    Node: AsRef<T>,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current.map(Node::as_ref);
        self.move_forward();
        item
    }
}

impl<'a, T, Node> DoubleEndedIterator for Cursor<'a, T, Node>
where
    Node: Linked,
    Node: AsRef<T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.move_back();
        self.current.map(Node::as_ref)
    }
}

// ===== impl CursorMut =====

impl<'a, T, Node, R> cursor::Cursor for CursorMut<'a, T, Node, R>
where
    Node: Linked,
    Node: AsRef<T>,
{
    type Item = T;

    fn move_forward(&mut self) -> &mut Self {
        self.current = self
            .current
            .as_ref()
            .and_then(Linked::next)
            .map(|next| Link::from_owning_ref(UnsafeRef::from(next)))
            .unwrap_or_else(Link::none);
        self
    }

    fn move_back(&mut self) -> &mut Self {
        self.current = self
            .current
            .as_ref()
            .and_then(Linked::prev)
            .map(|prev| Link::from_owning_ref(UnsafeRef::from(prev)))
            .unwrap_or_else(Link::none);
        self
    }

    fn get(&self) -> Option<&Self::Item> {
        self.current.as_ref().map(Node::as_ref)
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.current
            .as_ref()
            .and_then(Linked::next)
            .map(Node::as_ref)
    }

    fn peek_back(&self) -> Option<&Self::Item> {
        self.current
            .as_ref()
            .and_then(Linked::prev)
            .map(Node::as_ref)
    }
}

/// A cursor that can mutate the parent data structure.
impl<'a, T, Node, R> cursor::CursorMut<T, Node> for CursorMut<'a, T, Node, R>
where
    Node: Linked,
    Node: AsRef<T> + AsMut<T>,
    R: OwningRef<Node>,
{
    type Ref = R;

    /// Return a reference to the item currently under the cursor.
    fn get_mut(&mut self) -> Option<&mut T> {
        self.current.as_mut().map(Node::as_mut)
    }

    /// Return a reference to the next element from the cursor's position.
    fn peek_next_mut(&mut self) -> Option<&mut T> {
        self.current
            .as_mut()
            .and_then(Linked::next_mut)
            .map(Node::as_mut)
    }

    /// Return a reference to the previous element from the cursor's
    /// position.
    fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.current
            .as_mut()
            .and_then(Linked::prev_mut)
            .map(Node::as_mut)
    }

    /// Remove the element currently under the cursor.
    fn remove_node(&mut self) -> Option<Self::Ref> {
        unsafe {
            self.current.as_ptr().map(|node| {
                // Unlink the node from the list, by changing the node's
                // neighbors to point at each other rather than the node.
                let links = (*node).take_links();
                let mut next = links.next;
                let mut prev = links.prev;

                if let Some(next) = next.as_mut() {
                    next.links_mut().prev = prev;
                }

                if let Some(prev) = prev.as_mut() {
                    prev.links_mut().next = next;
                }

                // Update the list to reflect that the node was unlinked.
                self.list.len -= 1;

                if self.list.head.as_ptr() == Some(node) {
                    self.list.head = next;
                }

                if self.list.tail.as_ptr() == Some(node) {
                    self.list.tail = prev;
                }

                // Update the cursor to point at the next node.
                self.current = next;

                Self::Ref::from_ptr(node as *const Node)
            })
        }
    }

    /// Insert the given item before the cursor's position.
    fn insert_node_before(&mut self, mut node: Self::Ref) -> &mut Self
    where
        Self::Ref: DerefMut,
    {
        // Link the node to the current node and the current node's
        // previous node.
        {
            let links = node.links_mut();
            links.next = self.current;
            links.prev = self
                .current
                .as_ref()
                .map(|current| current.links().prev)
                .unwrap_or_else(Link::none);
        }

        // Link the current node and the current node's old previous
        // node to the new node.
        let node = Link::from_owning_ref(node);
        if let Some(current) = self.current.as_mut() {
            if let Some(mut prev) = current.prev_mut() {
                prev.links_mut().next = node;
            }
            current.links_mut().prev = node;
        }

        // Update the list's state.
        unsafe {
            if self.list.head.as_ptr() == self.current.as_ptr() {
                self.list.head = node;
            }
            if self.list.tail.as_ptr() == self.current.as_ptr() {
                self.list.tail = node;
            }
        }
        self.current = node;
        self.list.len += 1;
        self
    }

    /// Insert the given item after the cursor's position.
    fn insert_node_after(&mut self, mut node: Self::Ref) -> &mut Self
    where
        Self::Ref: DerefMut,
    {
        // Link the node to the current node and the current node's
        // next node.
        {
            let links = node.links_mut();
            links.prev = self.current;
            links.next = self
                .current
                .as_ref()
                .map(|current| current.links().next)
                .unwrap_or_else(Link::none);
        }

        // Link the current node and the current node's old next
        // node to the new node.
        let node = Link::from_owning_ref(node);
        if let Some(current) = self.current.as_mut() {
            if let Some(mut next) = current.next_mut() {
                next.links_mut().prev = node;
            }
            current.links_mut().next = node;
        }

        // Update the list's state.
        unsafe {
            if self.list.tail.as_ptr() == self.current.as_ptr() {
                self.list.tail = node;
            }
        }

        self.list.len += 1;
        self
    }
}
