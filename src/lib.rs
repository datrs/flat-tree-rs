//! You can represent a binary tree in a simple flat list using the following
//! structure:
//!
//!
//! ```text
//!       3
//!   1       5
//! 0   2   4   6  ...
//! ```
//!
//! This module exposes a series of functions to help you build and maintain
//! this data structure.
//!
//! ## See Also
//! - [mafintosh/merkle-tree-stream (JavaScript)](https://github.com/mafintosh/merkle-tree-stream)

#![deny(missing_docs)]

/// Returns the flat-tree of the tree node at the specified depth and offset.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::index(0, 0), 0);
/// assert_eq!(flat_tree::index(0, 1), 2);
/// assert_eq!(flat_tree::index(0, 2), 4);
/// ```
pub fn index(depth: u64, offset: u64) -> u64 {
  (offset << depth + 1) | ((1 << depth) - 1)
}

/// Returns the depth of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::depth(0), 0);
/// assert_eq!(flat_tree::depth(1), 1);
/// assert_eq!(flat_tree::depth(2), 0);
/// assert_eq!(flat_tree::depth(3), 2);
/// assert_eq!(flat_tree::depth(4), 0);
/// ```
pub fn depth(i: u64) -> u64 {
  let mut depth = 0;
  let mut i = i;
  while is_odd(i) {
    i >>= 1;
    depth += 1;
  }
  depth
}

/// Returns the offset of a node with a depth.
pub fn offset_with_depth(i: u64, depth: u64) -> u64 {
  if is_even(i) {
    i / 2
  } else {
    i >> (depth + 1)
  }
}

/// Returns the offset of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::offset(0), 0);
/// assert_eq!(flat_tree::offset(1), 0);
/// assert_eq!(flat_tree::offset(2), 1);
/// assert_eq!(flat_tree::offset(3), 0);
/// assert_eq!(flat_tree::offset(4), 2);
/// ```
pub fn offset(i: u64) -> u64 {
  offset_with_depth(i, depth(i))
}

/// Returns the parent of a node with a depth.
pub fn parent_with_depth(i: u64, depth: u64) -> u64 {
  index(depth + 1, offset_with_depth(i, depth) >> 1)
}

/// Returns the parent of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::index(1, 0), 1);
/// assert_eq!(flat_tree::index(1, 1), 5);
/// assert_eq!(flat_tree::index(2, 0), 3);
///
/// assert_eq!(flat_tree::parent(0), 1);
/// assert_eq!(flat_tree::parent(2), 1);
/// assert_eq!(flat_tree::parent(1), 3);
/// ```
pub fn parent(i: u64) -> u64 {
  parent_with_depth(i, depth(i))
}

/// Returns the sibling of a node with a depth.
pub fn sibling_with_depth(i: u64, depth: u64) -> u64 {
  index(depth, offset(i) ^ 1)
}

/// Returns the sibling of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::sibling(0), 2);
/// assert_eq!(flat_tree::sibling(2), 0);
/// assert_eq!(flat_tree::sibling(1), 5);
/// assert_eq!(flat_tree::sibling(5), 1);
/// ```
pub fn sibling(i: u64) -> u64 {
  sibling_with_depth(i, depth(i))
}

/// Returns the parent's sibling, of a node, with a depth.
pub fn uncle_with_depth(i: u64, depth: u64) -> u64 {
  sibling_with_depth(parent_with_depth(i, depth), depth + 1)
}

/// Returns the parent's sibling, of a node.
pub fn uncle(i: u64) -> u64 {
  uncle_with_depth(i, depth(i))
}

/// Returns both children of a node, with a depth.
pub fn children_with_depth(i: u64, depth: u64) -> Option<(u64, u64)> {
  if is_even(i) {
    None
  } else if depth == 0 {
    Some((i, i))
  } else {
    let offset = offset_with_depth(i, depth) * 2;
    Some((index(depth - 1, offset), index(depth - 1, offset + 1)))
  }
}

/// Returns both children of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::children(0), None);
/// assert_eq!(flat_tree::children(1), Some((0, 2)));
/// assert_eq!(flat_tree::children(3), Some((1, 5)));
/// assert_eq!(flat_tree::children(9), Some((8, 10)));
/// ```
pub fn children(i: u64) -> Option<(u64, u64)> {
  children_with_depth(i, depth(i))
}

/// Returns only the left child of a node, with a depth
// TODO: handle errors
pub fn left_child_with_depth(i: u64, depth: u64) -> Option<u64> {
  if is_even(i) {
    None
  } else if depth == 0 {
    Some(i)
  } else {
    Some(index(depth - 1, offset_with_depth(i, depth) << 1))
  }
}

/// Returns only the left child of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::left_child(0), None);
/// assert_eq!(flat_tree::left_child(1), Some(0));
/// assert_eq!(flat_tree::left_child(3), Some(1));
/// ```
pub fn left_child(i: u64) -> Option<u64> {
  left_child_with_depth(i, depth(i))
}

/// Returns only the left child of a node, with a depth.
pub fn right_child_with_depth(i: u64, depth: u64) -> Option<u64> {
  if is_even(i) {
    None
  } else if depth == 0 {
    Some(i)
  } else {
    Some(index(depth - 1, (offset_with_depth(i, depth) << 1) + 1))
  }
}

/// Returns only the left child of a node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::right_child(0), None);
/// assert_eq!(flat_tree::right_child(1), Some(2));
/// assert_eq!(flat_tree::right_child(3), Some(5));
/// ```
// TODO: handle errors
pub fn right_child(i: u64) -> Option<u64> {
  right_child_with_depth(i, depth(i))
}

/// Returns the right most node in the tree that the node spans, with a depth.
pub fn right_span_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    i
  } else {
    (offset_with_depth(i, depth) + 1) * (2 << depth) - 2
  }
}

/// Returns the right most node in the tree that the node spans.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::right_span(0), 0);
/// assert_eq!(flat_tree::right_span(1), 2);
/// assert_eq!(flat_tree::right_span(3), 6);
/// assert_eq!(flat_tree::right_span(23), 30);
/// assert_eq!(flat_tree::right_span(27), 30);
/// ```
pub fn right_span(i: u64) -> u64 {
  right_span_with_depth(i, depth(i))
}

/// Returns the left most node in the tree that the node spans, with a depth.
pub fn left_span_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    i
  } else {
    offset_with_depth(i, depth) * (2 << depth)
  }
}

/// Returns the left most node in the tree that it spans.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::left_span(0), 0);
/// assert_eq!(flat_tree::left_span(1), 0);
/// assert_eq!(flat_tree::left_span(3), 0);
/// assert_eq!(flat_tree::left_span(23), 16);
/// assert_eq!(flat_tree::left_span(27), 24);
/// ```
pub fn left_span(i: u64) -> u64 {
  left_span_with_depth(i, depth(i))
}

/// Returns the left and right most nodes in the tree that the node spans, with
/// a depth.
pub fn spans_with_depth(i: u64, depth: u64) -> (u64, u64) {
  (
    left_span_with_depth(i, depth),
    right_span_with_depth(i, depth),
  )
}

/// Returns the left and right most nodes in the tree that the node spans.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::spans(0), (0, 0));
/// assert_eq!(flat_tree::spans(1), (0, 2));
/// assert_eq!(flat_tree::spans(3), (0, 6));
/// assert_eq!(flat_tree::spans(23), (16, 30));
/// assert_eq!(flat_tree::spans(27), (24, 30));
/// ```
pub fn spans(i: u64) -> (u64, u64) {
  spans_with_depth(i, depth(i))
}

/// Returns how many nodes are in the tree that the node spans, with a depth.
pub fn count_with_depth(_: u64, depth: u64) -> u64 {
  (2 << depth) - 1
}

/// Returns how many nodes are in the tree that the node spans.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::count(0), 1);
/// assert_eq!(flat_tree::count(1), 3);
/// assert_eq!(flat_tree::count(3), 7);
/// assert_eq!(flat_tree::count(5), 3);
/// assert_eq!(flat_tree::count(23), 15);
/// assert_eq!(flat_tree::count(27), 7);
/// ```
pub fn count(i: u64) -> u64 {
  count_with_depth(i, depth(i))
}

/// Returns all the previous fully rooted trees before the node.
///
/// ## Examples
/// ```rust
/// assert_eq!(flat_tree::full_roots(0), []);
/// assert_eq!(flat_tree::full_roots(2), [0]);
/// assert_eq!(flat_tree::full_roots(8), [3]);
/// assert_eq!(flat_tree::full_roots(20), [7, 17]);
/// assert_eq!(flat_tree::full_roots(18), [7, 16]);
/// assert_eq!(flat_tree::full_roots(16), [7]);
/// ```
pub fn full_roots(i: u64) -> Vec<u64> {
  let mut result = Vec::with_capacity(64);

  if is_odd(i) {
    result
  } else {
    let mut tmp = i >> 1;
    let mut offset = 0;
    let mut factor = 1;

    loop {
      if tmp == 0 {
        break;
      }
      while factor * 2 <= tmp {
        factor *= 2;
      }
      result.push(offset + factor - 1);
      offset += 2 * factor;
      tmp -= factor;
      factor = 1;
    }

    result
  }
}

#[inline(always)]
fn is_even(num: u64) -> bool {
  (num & 1) == 0
}
#[test]
fn test_is_even() {
  assert_eq!(is_even(0), true);
  assert_eq!(is_even(1), false);
  assert_eq!(is_even(2), true);
  assert_eq!(is_even(3), false);
}

#[inline(always)]
fn is_odd(num: u64) -> bool {
  (num & 1) != 0
}
#[test]
fn test_is_odd() {
  assert_eq!(is_odd(0), false);
  assert_eq!(is_odd(1), true);
  assert_eq!(is_odd(2), false);
  assert_eq!(is_odd(3), true);
}

#[test]
fn test_parent_gt_int32() {
  assert_eq!(parent(10_000_000_000), 10_000_000_001);
}

#[test]
fn test_child_to_parent_to_child() {
  let mut child = 0;
  for _ in 0..50 {
    child = parent(child)
  }
  assert_eq!(child, 1125899906842623);
  for _ in 0..50 {
    child = left_child(child).expect("no valid number returned");
  }
  assert_eq!(child, 0);
}
