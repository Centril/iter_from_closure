# `iter_from_closure`

`Iterator<Item = Item>` creation for one-time use iterators from mutable
closures in the form `FnMut() -> Option<Item>`.

Please read [the API documentation here].

## Usage

How to use with cargo:

```rust
[dependencies]
iter_from_closure = "1.0.0"
```

How to use in your crate:

```rust
extern crate itertools;

use iter_from_closure::iter_from_closure;

let mut count = 10;
let iter = iter_from_closure(|| {
    let c = count;
    count = c - 1;
    if c > 0 { Some(c) } else { None }
});

println!("{:?}", iter.collect::<Vec<_>>());
```

## Recent Changes

+ 1.0.0 - Initial version. Unlikely to be more.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0] or the [MIT license],
at your option. This file may not be copied, modified, or distributed except
according to those terms.

[`Iterator<Item = Item>`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
[the API documentation here]: https://docs.rs/iter_from_closure/

[Apache License, Version 2.0]: http://www.apache.org/licenses/LICENSE-2.0
[MIT license]: http://opensource.org/licenses/MIT