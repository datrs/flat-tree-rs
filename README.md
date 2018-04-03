# flat-tree

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Rust version of [mafintosh/flat-tree](https://github.com/mafintosh/flat-tree).
Map a binary tree to a list.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate flat_tree;

let parent = flat_tree::parent(0);
println!("parent of 0 is {}", parent);
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/flat-tree.svg?style=flat-square
[2]: https://crates.io/crate/flat-tree
[3]: https://img.shields.io/travis/datrs/flat-tree.svg?style=flat-square
[4]: https://travis-ci.org/datrs/flat-tree
[5]: https://img.shields.io/crates/d/flat-tree.svg?style=flat-square
[6]: https://crates.io/crate/flat-tree
[7]: https://docs.rs/flat-tree/badge.svg
[8]: https://docs.rs/flat-tree
