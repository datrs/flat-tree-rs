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
  offset: usize,
  factor: usize,
}

impl Iterator {
  /// Create a new iterator.
  pub fn new(index: usize) -> Self {
    let mut instance = Self {
      index: 0,
      offset: 0,
      factor: 0,
    };

    if index != 0 {
      instance.seek(index);
    }

    instance
  }

  /// Get the current index.
  #[inline]
  pub fn index(&self) -> usize {
    self.index
  }

  /// Seek to a position in the iterator.
  pub fn seek(&mut self, index: usize) {
    self.index = index;
    if (self.index & 1) > 0 {
      self.offset = offset(index);
      self.factor = two_pow(depth(index) + 1);
    } else {
      self.offset = index / 2;
      self.factor = 2;
    }
  }

  /// Check if the position of the iterator is currently on a left node.
  #[inline]
  pub fn is_left(&mut self) -> bool {
    (self.offset & 1) == 0
  }

  /// Check if the position of the iterator is currently on a right node.
  #[inline]
  pub fn is_right(&mut self) -> bool {
    !self.is_left()
  }

  /// Move the cursor and get the previous item from the current position.
  pub fn prev(&mut self) -> usize {
    if self.offset == 0 {
      return self.index;
    }
    self.offset -= 1;
    self.index -= self.factor;
    self.index
  }

  /// Get the sibling for the current position and move the cursor.
  pub fn sibling(&mut self) -> usize {
    if self.is_left() {
      self.next().unwrap() // We always have a future value.
    } else {
      self.prev()
    }
  }

  /// Get the offset for the current position and move the cursor.
  pub fn offset(&mut self) -> usize {
    self.index = offset(self.index);
    self.index
  }

  /// Get the parent for the current position and move the cursor.
  pub fn parent(&mut self) -> usize {
    if self.offset & 1 > 0 {
      self.index -= self.factor / 2;
      self.offset = self.offset - 1 / 2;
    } else {
      self.index += self.factor / 2;
      self.offset /= 2;
    }
    self.factor *= 2;
    self.index
  }

  /// Get the left_span for the current position and move the cursor.
  pub fn left_span(&mut self) -> usize {
    self.index = self.index - self.factor / 2 + 1;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the right_span for the current position and move the cursor.
  pub fn right_span(&mut self) -> usize {
    self.index = self.index + self.factor / 2 - 1;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the left_child for the current position and move the cursor.
  pub fn left_child(&mut self) -> usize {
    if self.factor == 2 {
      return self.index;
    }
    self.factor /= 2;
    self.index -= self.factor / 2;
    self.offset *= 2;
    self.index
  }

  /// Get the right_child for the current position and move the cursor.
  pub fn right_child(&mut self) -> usize {
    if self.factor == 2 {
      return self.index;
    }
    self.factor /= 2;
    self.index += self.factor / 2;
    self.offset = 2 * self.offset + 1;
    self.index
  }
}

impl iter::Iterator for Iterator {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    self.index += self.factor;
    Some(self.index)
  }
}

fn two_pow(n: usize) -> usize {
  if n < 31 {
    1 << n
  } else {
    ((1 << 30) * (1 << (n - 30)))
  }
}
