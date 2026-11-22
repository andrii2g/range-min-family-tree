# range-min-family-tree

A dependency-free Rust library and CLI for finding the leftmost minimum in an
array interval. The CLI shows how the endpoints meet in a min-Cartesian tree.

## Run the demo

Use two zero-based, inclusive endpoints, then `--` followed by the signed
integer values.

```sh
cargo run -- 1 5 -- 0 4 1 3 1 5
```

```text
[0] 0
└── R [2] 1  * shared ancestor
    ├── L [1] 4  <- left endpoint
    └── R [4] 1  <- right path
        ├── L [3] 3
        └── R [5] 5  <- right endpoint

Left path: 1 -> 2
Right path: 5 -> 4 -> 2
Minimum in [1, 5] is at index 2 (value 1)
```

## Use the library

```rust
use range_min_family_tree::CartesianTree;

let tree = CartesianTree::new(&[0, 4, 1, 3, 1, 5]);
let query = tree.query(1, 5).unwrap();
assert_eq!(query.minimum_index, 2);
assert_eq!(query.left_path, [1, 2]);
assert_eq!(query.right_path, [5, 4, 2]);
```

Both paths include their endpoint and stop at the shared ancestor.
`query` returns `None` for reversed or out-of-bounds endpoints.

An inorder traversal visits the nodes in array order. Each parent has a
smaller `(value, index)` pair than its children, so the lowest common ancestor
of the endpoints is the leftmost minimum in the interval. The monotonic stack
builds the tree in O(n) time and space. A query walks parent links and uses O(h)
time and space, where h is the height of the tree.
[Munro and Wild, §1.2](https://www.wild-inter.net/publications/entropy-trees.pdf)

## Verify

Run `cargo test` to compare every interval against a direct scan for all arrays
of lengths 1–6 over `{0, 1, 2}`. The test checks 19,956 intervals across 1,092
arrays and compares the index returned for each one.
