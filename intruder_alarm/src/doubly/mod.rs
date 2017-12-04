//! An intrusive linked list implementation using `RawLink`s.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.
use core::mem;
use core::marker::PhantomData;

use super::Link;
#[cfg(test)]
mod tests;

//-----------------------------------------------------------------------------
//  List
/// An intrusive doubly-linked list.
#[derive(Default)]
pub struct List<T, Node> {
    head: Link<Node>,
    tail: Link<Node>,
    len: usize,
    _ty_marker: PhantomData<T>
}

impl<T, Node> List<T, Node> {
    /// Create a new `List` with 0 elements.
    pub const fn new() -> Self {
        List {
            head: Link::none(),
            tail: Link::none(),
            len: 0,
            _ty_marker: PhantomData,
        }
    }

    /// Returns the length of the list.
    #[inline] pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty, false otherwise.
    #[inline] pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
impl<T, Node> List<T, Node> {
    /// Borrows the first node of the list as an `Option`.
    /// 
    /// Note that this is distinct from `front`: this method 
    /// borrows the head _node_, not the head _element_.
    ///
    /// # Returns
    ///   - `Some(&N)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline] pub fn head(&self) -> Option<&Node> {
        self.head.as_ref()
    }

    /// Borrows the last node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline] pub fn tail(&self) -> Option<&Node> {
        self.tail.as_ref()
    }

    /// Mutably borrows the first node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline] pub fn head_mut(&mut self) -> Option<&mut Node> {
        self.head.as_mut()
    }

    /// Mutably borrows the last node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline] pub fn tail_mut(&mut self) -> Option<&mut Node> {
        self.tail.as_mut()
    }

}


impl<T, Node> List<T, Node>
where
    Node: Linked
{
    /// Push a node to the head of the list.
    pub fn push_front_node(&mut self, mut node: Node) -> &mut Self {
        if let Some(ref mut head) = self.head.as_mut() {
            head.push_front(&mut node);
        }
        self.head = Link::from(&node);
        if self.tail.is_none() {
            self.tail = Link::from(&node);
        }
        self.len += 1;
        self
    }

    /// Push an node to the back of the list.
    pub fn push_back_node(&mut self, mut node: Node) -> &mut Self {
        if let Some(ref mut tail) = self.tail.as_mut() {
            tail.push_back(&mut node);
        }
        self.tail = Link::from(&node);
        if self.head.is_none() {
            self.head = Link::from(&node);
        }
        self.len += 1;
        self
    }

    /// Pop a node from the front of the list.
    pub fn pop_front_node(&mut self) -> Option<*mut Node> {
        unsafe {
            self.head.as_ptr().map(|node| {
                self.head = (*node).take_links().next;

                match self.head.as_mut() {
                    None => self.tail = Link::none(),
                    Some(head) => head.links_mut().prev = Link::none(),
                }

                self.len -= 1;
                node
            })
        }
    }

    /// Pop a node from the back of the list.
    pub fn pop_back_node(&mut self) -> Option<*mut Node> {
        unsafe {
            self.tail.as_ptr().map(|node| {
                self.tail = (*node).take_links().prev;

                match self.tail.as_mut() {
                    None => self.head = Link::none(),
                    Some(tail) => tail.links_mut().next = Link::none(),
                }

                self.len -= 1;
                node
            })
        }
    }

}

impl<T, Node> List<T, Node>
where 
    Node: AsRef<T>
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

impl<T, Node> List<T, Node>
where 
    Node: AsMut<T>
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

impl<T, Node> List<T, Node>
where
   Node: From<T>,
   Node: Linked
{
    /// Push an item to the front of the list.
    #[inline] pub fn push_front(&mut self, item: T) -> &mut Self {
        self.push_front_node(Node::from(item))
    }

    /// Push an item to the back of the list.
    #[inline] pub fn push_back(&mut self, item: T) -> &mut Self {
        self.push_back_node(Node::from(item))
    }
}

impl<T, Node> List<T, Node>
where
    // TODO: this is where an adapter trait *might* come in handy...
   *mut Node: Into<T>, 
   Node: Linked,
{
    /// Pop an item from the front of the list.
    #[inline] 
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node()
            .map(Into::into)
    }

    /// Pop an item from the front of the list.
    #[inline] 
    pub fn pop_back(&mut self) -> Option<T>  {
        self.pop_back_node()
            .map(Into::into)
    }
}

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

    /// Push an element to the front of the list.
    fn push_front(&mut self, element: &mut Self) {
        // link the pushed node's prev link back to this node.
        element.links_mut().prev = Link::from(&*self);

        let links = self.links_mut();
        if let Some(next) = links.next.as_mut() {
            // if this node is currently linked to a next node, replace the next
            // node's prev link with the new node.
            next.links_mut().prev = Link::from(&*element);
            // and link the pushed node's next link to this node's old next node.
            element.links_mut().next = Link::from(next);
        }
        // this node's next link points to the pushed node.
        links.next = Link::from(element);
    }

    /// Push an element to the back of the list.
    fn push_back(&mut self, element: &mut Self) {
        // link the pushed node's next link back to this node.
        element.links_mut().next = Link::from(&*self);

        let links = self.links_mut();
        if let Some(prev) = links.prev.as_mut() {
            // if this node is currently linked to a prev node, replace the
            // prev node's next link with the new node.
            prev.links_mut().next = Link::from(&*element);
            // and link the pushed node's prev link to this node's old prev node.
            element.links_mut().prev = Link::from(prev);
        }
        // this node's prev link points to the pushed node.
        links.prev = Link::from(element);
    }

}


/// A linked list node that contains an item.
pub trait Container: Linked {
    /// The type of the item contained in the node.
    type Item;
    /// Construct a new node containing the given item.
    fn new(item: Self::Item) -> Self;

}

//-----------------------------------------------------------------------------
/// Links
#[derive(Default, Debug)]
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

    /// Mutably borrow the `next` element in the list.
    /// 
    /// # Returns
    /// - `Some(&mut T)` if there is a next element in the list.
    /// -  or `None` if this is the last.
    #[inline]
    pub fn next_mut(&mut self) -> Option<&mut T> {
        self.next.as_mut()
    }

    /// Mutably borrow the `prev` element in the list.
    /// 
    /// # Returns
    /// - `Some(&mut T)` if there is a previous element in the list.
    /// -  or `None` if this is the first.
    #[inline]
    pub fn prev_mut(&mut self) -> Option<&mut T> {
        self.prev.as_mut()
    }

    /// Returns true if this set of links is a member of a list.
    #[inline]
    pub fn is_linked(&self) -> bool {
        self.next.is_some()
    }

    // fn set_next<I: Into<Link<T>>>(&mut self, next: I) -> Link<T> {
    //     self.next.replace(next)
    // }

    // fn set_prev<I: Into<Link<T>>>(&mut self, prev: I) -> Link<T> {
    //     self.prev.replace(prev)
    // }
}
