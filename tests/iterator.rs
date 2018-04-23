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
