# flat-tree

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

> Rust version of [flat-tree](https://github.com/mafintosh/flat-tree). A series of functions to map a binary tree to a list.

## Usage

``` rs
extern crate flat_tree;

let parent = flat_tree::parent(0);
println!("parent of 0 is {}", parent);
```

## API

#### `index(depth: u64, offset: u64) -> u64`

Returns the flat-tree of the tree node at the specified depth and offset

#### `depth(node: u64) -> u64`

Returns the depth of a node

#### `offset(node: u64) -> u64`

Returns the offset of a node

#### `parent(node: u64) -> u64`

Returns the parent of a node

#### `sibling(node: u64) -> u64`

Returns the sibling of a node

#### `uncle(node: u64) -> u64`

Returns the parent's sibling of a node

#### `children(node: u64) -> (u64, u64)`

Returns both children of a node.

#### `left_child(node: u64) -> u64`

Returns only the left child of a node

#### `right_child(node: u64) -> u64`

Returns only the right child of a node

#### `right_span(node: u64) -> u64`

Returns the right most node in the tree that i spans

#### `left_span(node: u64) -> u64`

Returns the left most node in the tree that i spans

#### `spans(node: u64) -> (u64, u64)`

Returns the left and right most nodes in the tree that i spans

#### `count(node: u64) -> u64`

Returns how many nodes are in the tree that the node spans

#### `full_roots(node: u64) -> Vec<u64>`

Returns all the previous fully rooted trees before the node

## License

MIT

[1]: https://img.shields.io/crates/v/flat-tree.svg?style=flat-square
[2]: https://crates.io/crate/flat-tree
[3]: https://img.shields.io/travis/datrs/flat-tree.svg?style=flat-square
[4]: https://travis-ci.org/datrs/flat-tree
[5]: https://img.shields.io/crates/d/flat-tree.svg?style=flat-square
[6]: https://crates.io/crate/flat-tree
[7]: https://docs.rs/flat-tree/badge.svg
[8]: https://docs.rs/flat-tree
