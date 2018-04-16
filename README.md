# flat-tree

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Map a binary tree to a list. Adapted from
[mafintosh/flat-tree](https://github.com/mafintosh/flat-tree).

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate flat_tree;

let parent = flat_tree::parent(0);
println!("parent of 0 is {}", parent);
```

## Why?
You can represent a binary tree in a simple flat list using the following
structure:

```text
      3
  1       5
0   2   4   6  ...
```

This module exposes a series of functions to help you build and maintain
this data structure.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/flat-tree.svg?style=flat-square
[2]: https://crates.io/crates/flat-tree
[3]: https://img.shields.io/travis/datrs/flat-tree.svg?style=flat-square
[4]: https://travis-ci.org/datrs/flat-tree
[5]: https://img.shields.io/crates/d/flat-tree.svg?style=flat-square
[6]: https://crates.io/crates/flat-tree
[7]: https://docs.rs/flat-tree/badge.svg
[8]: https://docs.rs/flat-tree
