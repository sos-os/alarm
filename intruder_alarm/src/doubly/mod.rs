//! An intrusive linked list implementation using `RawLink`s.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.
use super::{Link, OwningRef};
use core::marker::PhantomData;
use core::mem;
use core::ops::DerefMut;
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
#[derive(Default)]
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

    /// Borrow the `next` linked element, or `None` if this is the last.
    #[inline]
    fn peek_next<T>(&self) -> Option<&T> where Self: AsRef<T> {
        self.next().map(Self::as_ref)
    }

    /// Borrow the `prev` linked element, or `None` if this is the first.
    #[inline]
    fn peek_prev<T>(&self) -> Option<&T> where Self: AsRef<T> {
        self.prev().map(Self::as_ref)
    }

    /// Mutably borrow the `next` linked element, or `None` if this is the
    /// last.
    #[inline]
    fn peek_next_mut<T>(&mut self) -> Option<&mut T> where Self: AsMut<T> {
        self.next_mut().map(Self::as_mut)
    }

    /// Mutably borrow the `prev` linked element, or `None` if this is the
    /// first.
    #[inline]
    fn peek_prev_mut<T>(&mut self) -> Option<&mut T> where Self: AsMut<T> {
        self.prev_mut().map(Self::as_mut)
    }
}

/// Links
#[derive(Default, Debug)]
pub struct Links<T> {
    pub(super) next: Link<T>,
    pub(super) prev: Link<T>,
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

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use core::boxed::Box;

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

    /// Returns true if this set of links is a member of a list.
    #[inline]
    fn is_linked(&self) -> bool {
        self.next.is_some()
    }
}

impl<T> Clone for Links<T> {
    #[inline]
    fn clone(&self) -> Self {
        Links::new()
    }
}
