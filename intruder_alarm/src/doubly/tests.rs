//
//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//

use super::*;
use super::Linked;
use std::default::Default;
use quickcheck::TestResult;

#[derive(Default, Debug)]
pub struct NumberedNode {
    pub number: usize,
    links: Links<NumberedNode>,
}

impl NumberedNode {
    pub fn new(number: usize) -> Self {
        NumberedNode {
            number: number,
            ..Default::default()
        }
    }
}

impl Linked for NumberedNode {
    #[inline] fn links(&self) -> &Links<Self> { &self.links }
    #[inline] fn links_mut(&mut self) -> &mut Links<Self> { &mut self.links }
}
impl PartialEq for NumberedNode {
    fn eq(&self, rhs: &Self) -> bool { self.number == rhs.number }
}

impl From<usize> for NumberedNode {
    fn from(u: usize) -> NumberedNode {
        NumberedNode::new(u)
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
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front_node(Box::new(NumberedNode::new(1)));

            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contents_after_first_push() {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            list.push_front_node(Box::new(NumberedNode::new(555)));

            assert_eq!(list.tail().unwrap().number, 555);
            assert_eq!(list.head().unwrap().number, 555);
        }

        #[test]
        fn head_tail_equal_after_first_push() {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
            assert_eq!(list.head(), list.tail());

            list.push_front_node(Box::new(NumberedNode::new(444)));

            assert_eq!(list.head(), list.tail());
        }

        #[test]
        fn head_tail_not_equal_after_second_push() {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

            list.push_front_node(Box::new(NumberedNode::new(444)));
            list.push_front_node(Box::new(NumberedNode::new(555)));

            assert!(list.head().unwrap() != list.tail().unwrap());
        }
    }

    #[test]
     fn head_tail_not_same_second_push()  {
        let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
        let a = 444;
        let b = 555;

        list.push_front(a);
        list.push_front(b);

        assert!(list.head().unwrap() != list.tail().unwrap());
    }


    quickcheck! {

        fn push_front_node_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
            list.push_front_node(Box::new(NumberedNode::new(x)));
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_front_node(Box::new(NumberedNode::new(x_2)));
                result = TestResult::from_bool(
                    list.tail().unwrap().number == x &&
                    list.head().unwrap().number == x_2
                );
            }
            result
        }

        fn not_empty_after_push(n: usize) -> bool {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front(n);

            !list.is_empty() && list.len() == 1

        }

        fn contents_after_first_push(n: usize) -> bool {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            list.push_front(n);
            list.tail().unwrap().number == n &&
            list.head().unwrap().number == n
        }

        fn head_tail_same_first_push(n: usize) -> bool {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

            list.push_front(n);

            list.head() == list.tail()
        }

        // fn head_tail_not_same_second_push(a: usize, b: usize) -> TestResult {
        //     if a == b {
        //         return TestResult::discard()
        //     };
        //     let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

        //     list.push_front(a);
        //     list.push_front(b);

        //     TestResult::from_bool(list.head().unwrap() != list.tail().unwrap())
        // }

        fn push_front_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();
            list.push_front(x);
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_front(x_2);
                result = TestResult::from_bool(
                    list.tail().unwrap().number == x &&
                    list.head().unwrap().number == x_2
                );
            }
            result
        }
    }

    #[test]
    fn contents_after_push_nodes() {
        let mut list = List::<usize, NumberedNode, Box<NumberedNode>>::new();

        list.push_front_node(Box::new(NumberedNode::new(0)));
        list.push_front_node(Box::new(NumberedNode::new(1)));

        assert_eq!(list.tail().unwrap().number, 0);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(Box::new(NumberedNode::new(2)));
        assert_eq!(list.tail().unwrap().number, 2);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);
        assert_eq!(list.head().unwrap().number, 1);

        assert!(!list.is_empty());
    }
}

//
// #[test]
// fn test_pop_front() {
//     let mut list = List::<usize, NumberedNode>::new();
//
//     assert_eq!(list.head(), None);
//     assert_eq!(list.tail(), None);
//     assert!(list.is_empty());
//
//     list.push_front_node(Box::new(NumberedNode::new(2)));
//
//     assert!(!list.is_empty());
//     assert_eq!(list.head(), list.tail());
//
//     list.push_front_node(Box::new(NumberedNode::new(1)));
//     list.push_front_node(Box::new(NumberedNode::new(0)));
//
//     assert_eq!(list.head().unwrap().number, 0);
//     assert_eq!(list.tail().unwrap().number, 2);
//
//     list.push_back_node(Box::new(NumberedNode::new(3)));
//     assert_eq!(list.tail().unwrap().number, 3);
//
//     list.push_back_node(Box::new(NumberedNode::new(4)));
//     assert_eq!(list.tail().unwrap().number, 4);
//
//     assert!(!list.is_empty());
//
//     assert_eq!(list.pop_front().unwrap().number, 0);
//     assert_eq!(list.pop_front().unwrap().number, 1);
//     assert_eq!(list.pop_front().unwrap().number, 2);
//     assert_eq!(list.pop_front().unwrap().number, 3);
//     assert_eq!(list.pop_front().unwrap().number, 4);
//
//     assert!(list.is_empty());
//     assert_eq!(list.pop_front(), None);
// }
//
// #[test]
// fn test_pop_back() {
//     let mut list = List::<usize, NumberedNode>::new();
//
//     assert_eq!(list.head(), None);
//     assert_eq!(list.tail(), None);
//     assert!(list.is_empty());
//
//     list.push_front_node(Box::new(NumberedNode::new(2)));
//
//     assert!(!list.is_empty());
//     assert_eq!(list.head(), list.tail());
//
//     list.push_front_node(Box::new(NumberedNode::new(1)));
//     list.push_front_node(Box::new(NumberedNode::new(0)));
//
//     assert_eq!(list.head().unwrap().number, 0);
//     assert_eq!(list.tail().unwrap().number, 2);
//
//     list.push_back_node(Box::new(NumberedNode::new(3)));
//     assert_eq!(list.tail().unwrap().number, 3);
//
//     list.push_back_node(Box::new(NumberedNode::new(4)));
//     assert_eq!(list.tail().unwrap().number, 4);
//
//     assert!(!list.is_empty());
//
//     assert_eq!(list.pop_back().unwrap().number, 4);
//     assert_eq!(list.pop_back().unwrap().number, 3);
//     assert_eq!(list.pop_back().unwrap().number, 2);
//     assert_eq!(list.pop_back().unwrap().number, 1);
//     assert_eq!(list.pop_back().unwrap().number, 0);
//
//     assert!(list.is_empty());
//     assert_eq!(list.pop_back(), None);
// }
