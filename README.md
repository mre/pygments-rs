# pygments-rs

Rust bindings for [Pygments](http://pygments.org/), a powerful syntax highlighter.

# Requirements

* Rust nightly
* A Python Runtime (2.7 or 3.x)
* [pygments 2.0](https://pypi.python.org/pypi/Pygments)

# How to use

```Rust
extern crate pygments;

use pygments::highlight;

fn main() {
    // Format some code
    let result = pygments::highlight("print [123, 456, None]".to_owned());
    println!("{}", result);
}
```

# License

MIT
