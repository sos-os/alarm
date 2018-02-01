//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
// 

use super::*;
use super::Linked;
use quickcheck::TestResult;
use std::default::Default;

#[derive(Default, Debug)]
pub struct NumberedNode {
    pub number: usize,
    next: Link<NumberedNode>,
}

pub type NumberedList = List<usize, NumberedNode, Box<NumberedNode>>;

impl NumberedNode {
    pub fn new(number: usize) -> Self {
        NumberedNode {
            number: number,
            ..Default::default()
        }
    }
}

impl Linked for NumberedNode {
    #[inline]
    fn next(&self) -> &Link<Self> { &self.next }

    #[inline]
    fn next_mut(&mut self) -> &mut Link<Self> {
        &mut self.next
    }
}

impl AsRef<usize> for NumberedNode {
    fn as_ref(&self) -> &usize {
        &self.number
    }
}

impl AsMut<usize> for NumberedNode {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.number
    }
}

impl PartialEq for NumberedNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.number == rhs.number
    }
}

impl From<usize> for NumberedNode {
    fn from(u: usize) -> NumberedNode {
        NumberedNode::new(u)
    }
}

impl Into<usize> for NumberedNode {
    fn into(self) -> usize {
        self.number
    }
}

mod boxed {
    use super::*;
    use std::boxed::Box;

    mod push_node {
        use super::*;
        use std::boxed::Box;

        #[test]
        fn not_empty_after_first_push() {
            let mut list = NumberedList::new();

            assert_eq!(list.peek(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_node(Box::new(NumberedNode::new(1)));

            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contents_after_first_push() {
            let mut list = NumberedList::new();
            assert_eq!(list.peek(), None);

            list.push_node(Box::new(NumberedNode::new(555)));

            assert_eq!(list.peek().unwrap().number, 555);
        }
    }

    quickcheck! {
        fn push_front_node_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = NumberedList::new();
            list.push_node(Box::new(NumberedNode::new(x)));
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_node(Box::new(NumberedNode::new(x_2)));
                result = TestResult::from_bool(
                    list.peek().unwrap().number == x_2
                );
            }
            result
        }

        fn not_empty_after_push(n: usize) -> bool {
            let mut list = NumberedList::new();

            assert_eq!(list.peek(), None);

            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push(n);

            !list.is_empty() && list.len() == 1
        }

        fn contents_after_first_push(n: usize) -> bool {
            let mut list = NumberedList::new();
            assert_eq!(list.peek(), None);
            list.push(n);
            list.peek().unwrap().number == n
        }
    }

    #[test]
    fn contents_after_push_nodes() {
        let mut list = NumberedList::new();

        list.push_node(Box::new(NumberedNode::new(0)));
        list.push_node(Box::new(NumberedNode::new(1)));

        assert_eq!(list.peek().unwrap().number, 1);

        list.push_node(Box::new(NumberedNode::new(2)));
        assert_eq!(list.peek().unwrap().number, 2);

        list.push_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.peek().unwrap().number, 3);

        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_front_node() {
        let mut list = NumberedList::new();

        assert_eq!(list.peek(), None);
        assert!(list.is_empty());

        list.push_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());

        list.push_node(Box::new(NumberedNode::new(1)));
        list.push_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.peek().unwrap().number, 0);

        list.push_node(Box::new(NumberedNode::new(3)));

        list.push_node(Box::new(NumberedNode::new(4)));

        assert!(!list.is_empty());

        assert_eq!(list.pop_node().unwrap().number, 4);
        assert_eq!(list.pop_node().unwrap().number, 3);
        assert_eq!(list.pop_node().unwrap().number, 0);
        assert_eq!(list.pop_node().unwrap().number, 1);
        assert_eq!(list.pop_node().unwrap().number, 2);

        assert!(list.is_empty());
        assert_eq!(list.pop_node(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list = NumberedList::new();

        assert_eq!(list.peek(), None);
        assert!(list.is_empty());

        list.push_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());

        list.push_node(Box::new(NumberedNode::new(1)));
        list.push_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.peek().unwrap().number, 0);

        list.push_node(Box::new(NumberedNode::new(3)));

        list.push_node(Box::new(NumberedNode::new(4)));

        assert!(!list.is_empty());

        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 0);
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop().unwrap(), 2);

        assert!(list.is_empty());
        assert_eq!(list.pop(), None);
    }
}
