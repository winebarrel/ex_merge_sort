# ex_merge_sort

Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) library.

## Usage

```toml
[dependencies]
ex_merge_sort = "0.2.1"
```

```rust
use ex_merge_sort;

use std::fs::File;
use std::io;

fn main() {
    let f = fs::File::open(opts.file).unwrap();
    let capacity = 1024;
    ex_merge_sort::sort(f, io::stdout(), capacity, None).unwrap();
}
```
