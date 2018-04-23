extern crate flat_tree;

#[test]
fn iterator() {
  let mut iterator = flat_tree::Iterator::default();
  assert_eq!(iterator.index(), 0);
  assert_eq!(iterator.parent(), 1);
  assert_eq!(iterator.parent(), 3);
  assert_eq!(iterator.parent(), 7);
  assert_eq!(iterator.right_child(), 11);
  assert_eq!(iterator.left_child(), 9);
  assert_eq!(iterator.next(), Some(13));
  assert_eq!(iterator.left_span(), 12);
}

#[test]
fn parent_and_odd_offset() {
  let mut iterator = flat_tree::Iterator::new(10);
  assert_eq!(iterator.index(), 10);
  assert_eq!(iterator.offset(), 5);
  assert_eq!(iterator.parent(), 9);
  assert_eq!(iterator.offset(), 2);
  assert_eq!(iterator.parent(), 11);
  assert_eq!(iterator.offset(), 1);
}

#[test]
fn non_leaf_start() {
  let mut iterator = flat_tree::Iterator::new(1);
  assert_eq!(iterator.index(), 1);
  assert_eq!(iterator.parent(), 3);
  assert_eq!(iterator.parent(), 7);
  assert_eq!(iterator.right_child(), 11);
  assert_eq!(iterator.left_child(), 9);
  assert_eq!(iterator.next(), Some(13));
  assert_eq!(iterator.left_span(), 12);
}
