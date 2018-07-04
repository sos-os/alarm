//! A stack using an intrusive linked list implementation of `RawLink`s
//! modified as singly.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.
use super::{Link, OwningRef, UnsafeRef};
use core::{
    iter::{Extend, FromIterator},
    marker::PhantomData,
    mem,
    ops::DerefMut,
};

#[cfg(test)]
mod tests;

//-----------------------------------------------------------------------------
// Public API types
//-----------------------------------------------------------------------------
//  Stack
/// A stack implementation using an intrusive singly-linked list.
///
/// This type is a wrapper around a series of [`Node`]s. It stores [`Link`]s
/// to the head/top [`Node`]s and the size of the stack (a.k.a len of the list).
///
/// `FromIterator` will return a new stack (list) in reverse order because this
/// is a stack.
///
/// # Type parameters
/// - `T`: the type of the items stored by each `N`
/// - `N`: the type of nodes in the stack
/// - `R`: the type of [`OwningRef`] that owns each `N`.
///
/// [`Node`]: trait.Node.html
/// [`Link`]: ../struct.Link.html
/// [`OwningRef]: ../trait.OwningRef.html
#[derive(Default)]
pub struct Stack<T, N, R> {
    /// Link to the top node of the stack.
    top: Link<N>,

    /// Size of the stack.
    len: usize,

    /// Type marker for items stored in the stack.
    _elem_ty: PhantomData<T>,

    /// Type marker for the `OwningRef` type.
    _ref_ty: PhantomData<R>,
}

//  Linked
/// Trait that must be implemented in order to be a member of an intrusive
/// linked list.
pub trait Linked: Sized {
    /// Borrow this element's next [`Link`].
    ///
    /// [`Links`]: struct.Link.html
    fn next(&self) -> &Link<Self>;

    /// Mutably borrow this element's next [`Link`].
    ///
    /// [`Links`]: struct.Link.html
    fn next_mut(&mut self) -> &mut Link<Self>;

    /// De-link this node, returning its' next Link.
    fn take_next(&mut self) -> Link<Self> {
        mem::replace(self.next_mut(), Link::none())
    }
}

//-----------------------------------------------------------------------------
// Implementations
//-----------------------------------------------------------------------------

// ===== impl Stack =====

impl<T, Node, R> Stack<T, Node, R> {
    /// Create a new `Stack` with 0 elements.
    pub const fn new() -> Self {
        Stack {
            top: Link::none(),
            len: 0,
            _elem_ty: PhantomData,
            _ref_ty: PhantomData,
        }
    }

    /// Returns the size of the stack.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the stack is empty, false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows the first node of the stack as an `Option`.
    /// Note that it borrows the head _node_, not the head _element_.
    ///
    /// # Returns
    ///   - `Some(&N)` if the stack has elements
    ///   - `None` if the stack is empty.
    #[inline]
    pub fn peek(&self) -> Option<&Node> {
        self.top.as_ref()
    }

    /// Mutably borrows the tpp node of the stack as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the stack has elements
    ///   - `None` if the stack is empty.
    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut Node> {
        self.top.as_mut()
    }
}

impl<T, Node, Ref> Stack<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
    Ref: DerefMut,
{
    /// Push a node on to the stack.
    pub fn push_node(&mut self, mut node: Ref) -> &mut Self {
        *node.next_mut() = self.top;
        let node = Link::from_owning_ref(node);
        self.top = node;
        self.len += 1;
        self
    }
}

impl<T, Node, Ref> Stack<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
{
    /// Pop a node from the stack.
    pub fn pop_node(&mut self) -> Option<Ref> {
        unsafe {
            self.top.as_ptr().map(|node| {
                self.top = (*node).take_next();
                self.len -= 1;
                Ref::from_ptr(node as *const Node)
            })
        }
    }
}

impl<T, Node, R> Stack<T, Node, R>
where
    Node: AsRef<T>,
{
    /// Borrows the top item of the stack as an `Option`
    ///
    /// # Returns
    ///   - `Some(&T)` if the stack has elements
    ///   - `None` if the stack is empty.
    #[inline]
    pub fn top(&self) -> Option<&T> {
        self.peek().map(Node::as_ref)
    }
}

impl<T, Node, R> Stack<T, Node, R>
where
    Node: AsMut<T>,
{
    /// Mutably borrows the top element of the stack as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut T)` if the stack has elements
    ///   - `None` if the stack is empty.
    #[inline]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.peek_mut().map(Node::as_mut)
    }
}

impl<T, Node> Stack<T, Node, UnsafeRef<Node>>
where
    Node: Linked,
{
    /// Push an item on to the top of the stack.
    #[inline]
    pub fn push<I>(&mut self, item: I) -> &mut Self
    where
        I: Into<UnsafeRef<Node>>,
    {
        self.push_node(item.into())
    }
}

#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use std::boxed::Box;

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> Stack<T, Node, Box<Node>>
where
    Node: From<T>,
    Node: Linked,
{
    /// Push an item on to the stack.
    #[inline]
    pub fn push(&mut self, item: T) -> &mut Self {
        self.push_node(Box::new(Node::from(item)))
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> Stack<T, Node, Box<Node>>
where
    Node: Linked,
    Node: Into<T>,
{
    /// Pop an item from the stack.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|b| (*b).into())
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> Extend<T> for Stack<T, Node, Box<Node>>
where
    Node: From<T> + Linked,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, Node, R> Extend<R> for Stack<T, Node, UnsafeRef<Node>>
where
    R: Into<UnsafeRef<Node>>,
    Node: Linked,
{
    #[inline]
    fn extend<I: IntoIterator<Item = R>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, Node, Ref, E> FromIterator<E> for Stack<T, Node, Ref>
where
    Self: Extend<E>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = E>>(iter: I) -> Self {
        let mut stack = Stack::new();
        stack.extend(iter);
        stack
    }
}
