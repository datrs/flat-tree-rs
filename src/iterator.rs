//! ## Usage
//! ```rust
//! let mut iter = flat_tree::Iterator::new(0);
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.parent(), 7);
//! ```
use super::*;

use std::iter;

/// Iterator over a flat-tree.
pub struct Iterator {
  index: usize,
}

impl Iterator {
  /// Create a new iterator.
  pub fn new(index: usize) -> Self {
    Self { index }
  }

  /// Seek to a position in the iterator.
  pub fn seek(&mut self, index: usize) {
    self.index = index;
  }

  /// Get the offset for the current position.
  pub fn offset(&mut self) -> usize {
    self.index = offset(self.index);
    self.index
  }

  /// Get the parent for the current position.
  pub fn parent(&mut self) -> usize {
    self.index = parent(self.index);
    self.index
  }

  /// Get the sibling for the current position.
  pub fn sibling(&mut self) -> usize {
    self.index = sibling(self.index);
    self.index
  }

  /// Get the uncle for the current position.
  pub fn uncle(&mut self) -> usize {
    self.index = uncle(self.index);
    self.index
  }

  /// Get the left_child for the current position.
  pub fn left_child(&mut self) -> Option<usize> {
    let res = left_child(self.index);
    if let Some(index) = res {
      self.index = index;
    }
    res
  }

  /// Get the right_child for the current position.
  pub fn right_child(&mut self) -> Option<usize> {
    let res = right_child(self.index);
    if let Some(index) = res {
      self.index = index;
    }
    res
  }

  /// Get the left_span for the current position.
  pub fn left_span(&mut self) -> usize {
    self.index = left_span(self.index);
    self.index
  }

  /// Get the right_span for the current position.
  pub fn right_span(&mut self) -> usize {
    self.index = right_span(self.index);
    self.index
  }

  /// Get the count for the current position.
  pub fn count(&mut self) -> usize {
    self.index = count(self.index);
    self.index
  }
}

impl iter::Iterator for Iterator {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.index;
    self.index += 1;
    Some(index)
  }
}
