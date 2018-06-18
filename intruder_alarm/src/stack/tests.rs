//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//

use super::Linked;
use super::*;
use quickcheck::TestResult;
use std::default::Default;

#[derive(Default, Debug)]
pub struct NumberedNode {
    pub number: usize,
    next: Link<NumberedNode>,
}

pub type NumberedList = Stack<usize, NumberedNode, Box<NumberedNode>>;

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
    fn next(&self) -> &Link<Self> {
        &self.next
    }

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
        fn push_node_order(x: usize, xs: Vec<usize>) -> TestResult {
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
    fn test_pop_node() {
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

mod unsafe_ref {
    use super::*;
    use UnsafeRef;

    pub type UnsafeList = Stack<usize, NumberedNode, UnsafeRef<NumberedNode>>;

    mod push_node {
        use super::*;
        use UnsafeRef;

        #[test]
        fn not_empty_after_first_push() {
            let mut list = UnsafeList::new();

            assert_eq!(list.peek(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_node(UnsafeRef::boxed(NumberedNode::new(1)));

            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contents_after_first_push() {
            let mut list = UnsafeList::new();
            assert_eq!(list.peek(), None);

            list.push_node(UnsafeRef::boxed(NumberedNode::new(555)));

            assert_eq!(list.peek().unwrap().number, 555);
        }
    }

    quickcheck! {
        fn push_node_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = UnsafeList::new();
            list.push_node(UnsafeRef::boxed(NumberedNode::new(x)));
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_node(UnsafeRef::boxed(NumberedNode::new(x_2)));
                result = TestResult::from_bool(
                    list.peek().unwrap().number == x_2
                );
            }
            result
        }

        fn not_empty_after_push(n: usize) -> bool {
            let mut list = UnsafeList::new();

            assert_eq!(list.peek(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_node(UnsafeRef::boxed(NumberedNode::from(n)));

            !list.is_empty() && list.len() == 1
        }

        fn contents_after_first_push(n: usize) -> bool {
            let mut list = UnsafeList::new();
            assert_eq!(list.peek(), None);
            list.push_node(UnsafeRef::boxed(NumberedNode::from(n)));
            list.peek().unwrap().number == n
        }

        fn extend_sum_len(ys: Vec<usize>, xs: Vec<usize>) -> bool {
            let mut list = UnsafeList::new();
            let total = ys.len() + xs.len();
            let ys = ys.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs = xs.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            for y in ys {
                list.push_node(y);
            }

            list.extend(xs);

            list.len() == total
        }

        fn from_iter_len(xs: Vec<usize>) -> bool {
            let lx = xs.len();
            let xs = xs.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let list = UnsafeList::from_iter(xs);

            list.len() == lx
        }

        fn collect_from_iter_equivalent(xs: Vec<usize>) -> bool {

            let xs1 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs2 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            let mut list1 = UnsafeList::from_iter(xs1);
            let mut list2 = xs2.into_iter().collect::<UnsafeList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_node() == list2.pop_node()
                );
            }
            result
        }

        fn collect_and_loop_push_equivalent(xs: Vec<usize>) -> bool {
            let xs1 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs2 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            let mut list1 = UnsafeList::new();

            for x in xs1 {
                list1.push(x);
            }

            let mut list2 = xs2.into_iter().collect::<UnsafeList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_node().unwrap().number ==
                    list2.pop_node().unwrap().number
                );
            }
            result
        }
    }

    #[test]
    fn contents_after_push_nodes() {
        let mut list = UnsafeList::new();

        list.push_node(UnsafeRef::boxed(NumberedNode::new(0)));
        list.push_node(UnsafeRef::boxed(NumberedNode::new(1)));

        assert_eq!(list.peek().unwrap().number, 1);

        list.push_node(UnsafeRef::boxed(NumberedNode::new(2)));
        assert_eq!(list.peek().unwrap().number, 2);

        list.push_node(UnsafeRef::boxed(NumberedNode::new(3)));
        assert_eq!(list.peek().unwrap().number, 3);

        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_node() {
        let mut list = UnsafeList::new();

        assert_eq!(list.peek(), None);
        assert!(list.is_empty());

        list.push_node(UnsafeRef::boxed(NumberedNode::new(2)));

        assert!(!list.is_empty());

        list.push_node(UnsafeRef::boxed(NumberedNode::new(1)));
        list.push_node(UnsafeRef::boxed(NumberedNode::new(0)));

        assert_eq!(list.peek().unwrap().number, 0);

        list.push_node(UnsafeRef::boxed(NumberedNode::new(3)));

        list.push_node(UnsafeRef::boxed(NumberedNode::new(4)));

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
    fn test_extend() {
        let mut list = UnsafeList::new();

        list.push_node(UnsafeRef::boxed(NumberedNode::from(0)));
        list.push_node(UnsafeRef::boxed(NumberedNode::from(1)));

        assert_eq!(list.peek().unwrap().number, 1);

        let ext = vec![
            UnsafeRef::boxed(NumberedNode::from(3)),
            UnsafeRef::boxed(NumberedNode::from(4)),
        ];
        list.extend(ext);

        assert_eq!(list.peek().unwrap().number, 4);
    }

    #[test]
    fn test_fromiter() {
        let list_a = (0..10)
            .into_iter()
            .map(|i| UnsafeRef::boxed(NumberedNode::from(i)));
        let mut nlist = UnsafeList::from_iter(list_a.rev());

        for i in 0..10 {
            assert_eq!(nlist.pop_node().unwrap().number, i);
        }
    }
}
