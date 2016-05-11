
![](https://img.shields.io/crates/l/incrust.svg) [![](https://img.shields.io/crates/v/incrust.svg)](https://crates.io/crates/incrust)

## Incrust

> Incrust is a template engine inspired by Jinja2 and written in Rust.

**NOTA BENE**

This is a work in progress and (as may be assumed reasonably enough) may be highly *unstable* just yet.

The implementation is at a very early stage and the API is a subject of changes.

## Installation

Incrust is [available on crates.io](https://crates.io/crates/incrust) and can be included in your Cargo enabled project like this:

```
[dependencies]
incrust = "0.0"
```

Then include it in your code like this:

```rust
extern crate incrust;
```

## Examples

All examples assume a prepared instance of Incrust. For ease of use hashmaps, we use the macro [maplit](https://crates.io/crates/maplit)

```rust
#[macro_use]
extern crate maplit;
extern crate incrust;

use incrust::{Incrust, Args, Var};

fn main() {
    let incrust = Incrust::new();
    // ...
}
```

### Variables

```rust
let result = incrust.render_text("Hello, {{name}}!", &hashmap!{ "name" => Var::ex("World") }.unwrap();
assert_eq!(result, "Hello, World!");
```

### Filters

```rust
let args: Args = hashmap!{ "text" => Var::ex("<Cats & Dogs>") };
let result = incrust.render_text("<h1>{{ text | e }}</h1>", &args).unwrap();
assert_eq!(result, "<h1>&lt;Cats &amp; Dogs&gt;</h1>");
```

### Comments

```rust
let tpl = incrust.parse("<p>Visible {# partially #} paragraph</p>").unwrap();
let result = incrust.render_parsed(tpl, &hashmap!{}).unwrap();
assert_eq!(result, "<p>Visible  paragraph</p>");
```

### Escaping

```rust
let tpl = "Example: {% raw %}{{ mustaches }}{% endraw %}";
let result = incrust.render_text(tpl, &hashmap!{}).unwrap();
assert_eq!(result, "Example: {{ mustaches }}");
```



## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
