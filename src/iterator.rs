//! ## Usage
//! ```rust
//! let mut iter = flat_tree::Iterator::new(0);
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(4));
//! assert_eq!(iter.next(), Some(6));
//! assert_eq!(iter.parent(), 5);
//! ```
use super::*;

use std::iter;

/// Iterator over a flat-tree.
#[derive(Debug)]
pub struct Iterator {
  index: usize,
  offset: usize,
  factor: usize,
}

impl Iterator {
  /// Create a new iterator.
  ///
  /// ## Examples
  /// ```rust
  /// use flat_tree::Iterator;
  /// assert_eq!(Iterator::new(0).take(3).collect::<Vec<usize>>(), [2, 4, 6]);
  /// ```
  pub fn new(index: usize) -> Self {
    let mut instance = Self {
      index: 0,
      offset: 0,
      factor: 0,
    };

    instance.seek(index);
    instance
  }

  /// Get the current index.
  #[inline]
  pub fn index(&self) -> usize {
    self.index
  }

  /// Get the current offset.
  #[inline]
  pub fn offset(&self) -> usize {
    self.offset
  }

  /// Seek to a position in the iterator.
  ///
  /// ## Examples
  /// ```rust
  /// let mut iter = flat_tree::Iterator::new(0);
  /// iter.seek(4);
  /// assert_eq!(iter.next(), Some(6));
  /// iter.seek(2);
  /// assert_eq!(iter.next(), Some(4));
  /// ```
  pub fn seek(&mut self, index: usize) {
    self.index = index;
    if is_odd(self.index) {
      self.offset = offset(index);
      self.factor = two_pow(depth(index) + 1);
    } else {
      self.offset = index / 2;
      self.factor = 2;
    }
  }

  /// Check if the position of the iterator is currently on a left node.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).is_left(), true);
  /// assert_eq!(flat_tree::Iterator::new(2).is_left(), false);
  /// assert_eq!(flat_tree::Iterator::new(1).is_left(), true);
  /// ```
  #[inline]
  pub fn is_left(&self) -> bool {
    is_even(self.offset)
  }

  /// Check if the position of the iterator is currently on a right node.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).is_right(), false);
  /// assert_eq!(flat_tree::Iterator::new(2).is_right(), true);
  /// assert_eq!(flat_tree::Iterator::new(1).is_right(), false);
  /// ```
  #[inline]
  pub fn is_right(&self) -> bool {
    is_odd(self.offset)
  }

  /// Move the cursor and get the previous item from the current position.
  ///
  /// ## Examples
  /// ```rust
  /// let mut iter = flat_tree::Iterator::new(6);
  /// assert_eq!(iter.prev(), 4);
  /// assert_eq!(iter.prev(), 2);
  /// assert_eq!(iter.prev(), 0);
  /// ```
  pub fn prev(&mut self) -> usize {
    if self.offset == 0 {
      return self.index;
    }
    self.offset -= 1;
    self.index -= self.factor;
    self.index
  }

  /// Get the sibling for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).sibling(), 2);
  /// assert_eq!(flat_tree::Iterator::new(1).sibling(), 5);
  /// assert_eq!(flat_tree::Iterator::new(4).sibling(), 6);
  /// ```
  pub fn sibling(&mut self) -> usize {
    if self.is_left() {
      self.next().unwrap() // this is always safe
    } else {
      self.prev()
    }
  }

  /// Get the parent for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).parent(), 1);
  /// assert_eq!(flat_tree::Iterator::new(1).parent(), 3);
  /// assert_eq!(flat_tree::Iterator::new(4).parent(), 5);
  /// ```
  pub fn parent(&mut self) -> usize {
    if is_odd(self.offset) {
      self.index -= self.factor / 2;
      self.offset = (self.offset - 1) / 2;
    } else {
      self.index += self.factor / 2;
      self.offset /= 2;
    }
    self.factor *= 2;
    self.index
  }

  /// Get the left_span for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(1).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(3).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(23).left_span(), 16);
  /// assert_eq!(flat_tree::Iterator::new(27).left_span(), 24);
  /// ```
  pub fn left_span(&mut self) -> usize {
    self.index = self.index + 1 - self.factor / 2;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the right_span for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).right_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(1).right_span(), 2);
  /// assert_eq!(flat_tree::Iterator::new(3).right_span(), 6);
  /// assert_eq!(flat_tree::Iterator::new(23).right_span(), 30);
  /// assert_eq!(flat_tree::Iterator::new(27).right_span(), 30);
  /// ```
  pub fn right_span(&mut self) -> usize {
    self.index = self.index + self.factor / 2 - 1;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the left_child for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(1).left_child(), 0);
  /// assert_eq!(flat_tree::Iterator::new(3).left_child(), 1);
  /// assert_eq!(flat_tree::Iterator::new(7).left_child(), 3);
  /// ```
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
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(1).right_child(), 2);
  /// assert_eq!(flat_tree::Iterator::new(3).right_child(), 5);
  /// assert_eq!(flat_tree::Iterator::new(7).right_child(), 11);
  /// ```
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
    self.offset += 1;
    self.index += self.factor;
    Some(self.index)
  }
}

impl Default for Iterator {
  fn default() -> Self {
    Self::new(0)
  }
}

fn two_pow(n: usize) -> usize {
  if n < 31 {
    1 << n
  } else {
    ((1 << 30) * (1 << (n - 30)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_two_pow() {
    assert_eq!(two_pow(0), 1);
    assert_eq!(two_pow(1), 2);
    assert_eq!(two_pow(2), 4);
    assert_eq!(two_pow(3), 8);
    assert_eq!(two_pow(31), 2147483648);
  }

  #[cfg(target_pointer_width = "64")]
  #[test]
  fn test_two_pow_64bit() {
    assert_eq!(two_pow(34), 17179869184);
    assert_eq!(two_pow(63), 9223372036854775808);
  }
}
