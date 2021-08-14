# ex_merge_sort

Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) library.

[![Build Status](https://github.com/winebarrel/ex_merge_sort/workflows/CI/badge.svg)](https://github.com/winebarrel/ex_merge_sort/actions)
[![crates.io](https://img.shields.io/crates/v/ex_merge_sort.svg)](https://crates.io/crates/ex_merge_sort)
[![docs.rs](https://docs.rs/ex_ex_merge_sort/badge.svg)](https://docs.rs/ex_ex_merge_sort)

## Usage

```toml
[dependencies]
ex_merge_sort = "0.3.0"
```

```rust
use ex_merge_sort;

use std::fs::File;
use std::io;

fn main() {
    let f = fs::File::open(opts.file).unwrap();
    let capacity = 1024;
    ex_merge_sort::sort(f, io::stdout(), capacity).unwrap();
}
```
