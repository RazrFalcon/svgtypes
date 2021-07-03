## {{crate}}
![Build Status](https://github.com/RazrFalcon/{{crate}}/workflows/{{crate}}/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/{{crate}}.svg)](https://crates.io/crates/{{crate}})
[![Documentation](https://docs.rs/{{crate}}/badge.svg)](https://docs.rs/{{crate}})
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-orange.svg)](https://www.rust-lang.org)

{{readme}}

### Migration from svgparser

This crate is a successor for the [`svgparser`](https://github.com/RazrFalcon/svgparser) crate,
but it differs from it in many ways.

- There is no XML parser or writer. You can use any you like.
- Unlike the `svgparser` this crate not only parse values but can also store and write them.
  Currently, it has a minimal API for manipulating this values.
- No [`AttributeValue`](https://docs.rs/svgparser/0.8.0/svgparser/enum.AttributeValue.html).
  This crate provides only value parsers. You should match attributes and values by yourself.
- No [`ValueId`](https://docs.rs/svgparser/0.8.0/svgparser/enum.ValueId.html).
  It's up to you how to store those values.

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
