extern crate flat_tree;

#[test]
/// Postmortem: offset was incorrectly calculated when finding a parent for a
/// node with an odd offset.
fn parent_and_odd_offset() {
  let mut iterator = flat_tree::Iterator::new(10);
  assert_eq!(iterator.index(), 10);
  assert_eq!(iterator.offset(), 5);
  assert_eq!(iterator.parent(), 9);
  assert_eq!(iterator.offset(), 2);
  assert_eq!(iterator.parent(), 11);
  assert_eq!(iterator.offset(), 1);
}
