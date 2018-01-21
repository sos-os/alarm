
use super::*;
mod push_node {
    use super::*;
    #[test]
    fn not_empty_after_first_push() {
        let mut a = NumberedNode::new(1);
        let mut list =
            List::<usize, NumberedNode, &mut NumberedNode>::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);


        list.push_front_node(&mut a);

        assert_eq!(list.is_empty(), false);
        assert_eq!(list.len(), 1);

    }

    #[test]
    fn contents_after_first_push() {
        let mut a = NumberedNode::new(555);
        let mut list =
            List::<usize, NumberedNode, &mut NumberedNode>::new();
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);

        list.push_front_node(&mut a);

        assert_eq!(list.tail().unwrap().number, 555);
        assert_eq!(list.head().unwrap().number, 555);
    }

    #[test]
    fn head_tail_equal_after_first_push() {
        let mut a = NumberedNode::new(444);
        let mut list =
            List::<usize, NumberedNode, &mut NumberedNode>::new();
        assert_eq!(list.head(), list.tail());

        list.push_front_node(&mut a);

        assert_eq!(list.head(), list.tail());
    }

    #[test]
    fn head_tail_not_equal_after_second_push() {
        let mut a = NumberedNode::new(444);
        let mut b = NumberedNode::new(555);
        let mut list =
            List::<usize, NumberedNode, &mut NumberedNode>::new();

        list.push_front_node(&mut a);
        list.push_front_node(&mut b);

        assert!(list.head().unwrap() != list.tail().unwrap());
    }
}

#[test]
fn contents_after_push_nodes() {
    let mut a = NumberedNode::new(0);
    let mut b = NumberedNode::new(1);
    let mut c = NumberedNode::new(2);
    let mut d = NumberedNode::new(3);
    let mut list = List::<usize, NumberedNode, &mut NumberedNode>::new();

    list.push_front_node(&mut a);
    list.push_front_node(&mut b);

    assert_eq!(list.tail().unwrap().number, 0);
    assert_eq!(list.head().unwrap().number, 1);

    list.push_back_node(&mut c);
    assert_eq!(list.tail().unwrap().number, 2);
    assert_eq!(list.head().unwrap().number, 1);

    list.push_back_node(&mut d);
    assert_eq!(list.tail().unwrap().number, 3);
    assert_eq!(list.head().unwrap().number, 1);

    assert!(!list.is_empty());
}

// #[test]
// fn test_pop_front_node() {
//     let mut list = List::<usize, NumberedNode, &mut NumberedNode>::new();

//     assert_eq!(list.head(), None);
//     assert_eq!(list.tail(), None);
//     assert!(list.is_empty());

//     list.push_front_node(&mut NumberedNode::new(2));

//     assert!(!list.is_empty());
//     assert_eq!(list.head(), list.tail());

//     list.push_front_node(&mut NumberedNode::new(1));
//     list.push_front_node(&mut NumberedNode::new(0));

//     assert_eq!(list.head().unwrap().number, 0);
//     assert_eq!(list.tail().unwrap().number, 2);

//     list.push_back_node(&mut NumberedNode::new(3));
//     assert_eq!(list.tail().unwrap().number, 3);

//     list.push_back_node(&mut NumberedNode::new(4));
//     assert_eq!(list.tail().unwrap().number, 4);

//     assert!(!list.is_empty());

//     assert_eq!(list.pop_front_node().unwrap().number, 0);
//     assert_eq!(list.pop_front_node().unwrap().number, 1);
//     assert_eq!(list.pop_front_node().unwrap().number, 2);
//     assert_eq!(list.pop_front_node().unwrap().number, 3);
//     assert_eq!(list.pop_front_node().unwrap().number, 4);

//     assert!(list.is_empty());
//     assert_eq!(list.pop_front_node(), None);
// }

// #[test]
// fn test_pop_back_node() {
//     let mut list = List::<usize, NumberedNode, &mut NumberedNode>::new();

//     assert_eq!(list.head(), None);
//     assert_eq!(list.tail(), None);
//     assert!(list.is_empty());

//     list.push_front_node(&mut NumberedNode::new(2));

//     assert!(!list.is_empty());
//     assert_eq!(list.head(), list.tail());

//     list.push_front_node(&mut NumberedNode::new(1));
//     list.push_front_node(&mut NumberedNode::new(0));

//     assert_eq!(list.head().unwrap().number, 0);
//     assert_eq!(list.tail().unwrap().number, 2);

//     list.push_back_node(&mut NumberedNode::new(3));
//     assert_eq!(list.tail().unwrap().number, 3);

//     list.push_back_node(&mut NumberedNode::new(4));
//     assert_eq!(list.tail().unwrap().number, 4);

//     assert!(!list.is_empty());

//     assert_eq!(list.pop_back_node().unwrap().number, 4);
//     assert_eq!(list.pop_back_node().unwrap().number, 3);
//     assert_eq!(list.pop_back_node().unwrap().number, 2);
//     assert_eq!(list.pop_back_node().unwrap().number, 1);
//     assert_eq!(list.pop_back_node().unwrap().number, 0);

//     assert!(list.is_empty());
//     assert_eq!(list.pop_back_node(), None);
// }
