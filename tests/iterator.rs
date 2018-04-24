extern crate flat_tree;

#[test]
fn iterator() {
  let mut iterator = flat_tree::Iterator::default();
  assert_eq!(iterator.index(), 0);
  assert_eq!(iterator.prev(), 0);
  assert_eq!(iterator.is_left(), true);
  assert_eq!(iterator.is_right(), false);
  assert_eq!(iterator.parent(), 1);
  assert_eq!(iterator.offset(), 0);
  assert_eq!(iterator.parent(), 3);
  assert_eq!(iterator.offset(), 0);
  assert_eq!(iterator.parent(), 7);
  assert_eq!(iterator.offset(), 0);
  assert_eq!(iterator.right_child(), 11);
  assert_eq!(iterator.left_child(), 9);
  assert_eq!(iterator.next(), Some(13));
  assert_eq!(iterator.is_left(), false);
  assert_eq!(iterator.is_right(), true);
  assert_eq!(iterator.left_span(), 12);
  assert_eq!(iterator.next(), Some(14));
  assert_eq!(iterator.next(), Some(16));
  assert_eq!(iterator.offset(), 8);
  assert_eq!(iterator.parent(), 17);
  assert_eq!(iterator.offset(), 4);
  assert_eq!(iterator.parent(), 19);
  assert_eq!(iterator.offset(), 2);
  assert_eq!(iterator.parent(), 23);
  assert_eq!(iterator.offset(), 1);
  assert_eq!(iterator.right_span(), 30);

  iterator.seek(23);

  assert_eq!(iterator.index(), 23);
  assert_eq!(iterator.offset(), 1);
  assert_eq!(iterator.right_child(), 27);
  assert_eq!(iterator.sibling(), 19);
  assert_eq!(iterator.prev(), 11);
  assert_eq!(iterator.left_child(), 9);
  assert_eq!(iterator.prev(), 5);
  assert_eq!(iterator.left_child(), 4);
  assert_eq!(iterator.prev(), 2);
  assert_eq!(iterator.prev(), 0);
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
