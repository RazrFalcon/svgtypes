## svgtypes
[![Build Status](https://travis-ci.org/RazrFalcon/svgtypes.svg?branch=master)](https://travis-ci.org/RazrFalcon/svgtypes)
[![Crates.io](https://img.shields.io/crates/v/svgtypes.svg)](https://crates.io/crates/svgtypes)
[![Documentation](https://docs.rs/svgtypes/badge.svg)](https://docs.rs/svgtypes)
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-orange.svg)](https://www.rust-lang.org)

*svgtypes* is a collection of parsers, containers and writers for
[SVG 1.1](https://www.w3.org/TR/SVG11/) types.

Usage is simple as:

```rust
use svgtypes::Path;

let path: Path = "M10-20A5.5.3-4 110-.1".parse().unwrap();
assert_eq!(path.to_string(), "M 10 -20 A 5.5 0.3 -4 1 1 0 -0.1");
```

You can also use a low-level, pull-based parser:

```rust
use svgtypes::PathParser;

let p = PathParser::from("M10-20A5.5.3-4 110-.1");
for token in p {
    println!("{:?}", token);
}
```

You can also tweak an output format:

```rust
use svgtypes::{Path, WriteBuffer, WriteOptions};

let path_str = "M10-20A5.5.3-4 110-.1";
let path: Path = path_str.parse().unwrap();

let opt = WriteOptions {
    remove_leading_zero: true,
    use_compact_path_notation: true,
    join_arc_to_flags: true,
    .. WriteOptions::default()
};

assert_eq!(path.with_write_opt(&opt).to_string(), path_str);
```

### Supported SVG types

| SVG Type                  | Rust Type     | Storage | Parser              |
| ------------------------- | ------------- | ------- | ------------------- |
| [\<color\>]               | Color         | Stack   |                     |
| [\<number\>]              | f64           | Stack   |                     |
| [\<length\>]              | Length        | Stack   |                     |
| [\<angle\>]               | Angle         | Stack   |                     |
| [\<viewBox\>]             | ViewBox       | Stack   |                     |
| [\<path\>]                | Path          | Heap    | PathParser          |
| [\<list-of-numbers\>]     | NumberList    | Heap    | NumberListParser    |
| [\<list-of-lengths\>]     | LengthList    | Heap    | LengthListParser    |
| [\<transform-list\>]      | Transform     | Stack   | TransformListParser |
| [\<list-of-points\>]      | Points        | Heap    | PointsParser        |
| [\<paint\>]               | -             | -       | Paint               |

[\<color\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeColor
[\<number\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeNumber
[\<length\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeLength
[\<angle\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeAngle
[\<viewBox\>]: https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute
[\<path\>]: https://www.w3.org/TR/SVG11/paths.html#PathData
[\<list-of-numbers\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
[\<list-of-lengths\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
[\<transform-list\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeTransformList
[\<list-of-points\>]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
[\<paint\>]: https://www.w3.org/TR/SVG11/painting.html#SpecifyingPaint

- All types implement from string (`FromStr`) and
  to string traits (`Display`, `WriteBuffer`).
- The library doesn't store transform list as is. It will premultiplied.
- The `paint` type can only be parsed.

### Benefits

- Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
- Access to pull-based parsers.
- Pretty fast.

### Limitations

- Accepts only [normalized](https://www.w3.org/TR/REC-xml/#AVNormalize) values,
  e.g. an input text should not contain `&#x20;` or `&data;`.
- All keywords must be lowercase.
  Case-insensitive parsing is supported only for colors (requires allocation for named colors).
- The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
- [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`,
  are not supported. They were deprecated anyway.
- Implicit path commands are not supported. All commands will be parsed as explicit.
- Implicit MoveTo commands will be automatically converted into explicit LineTo.

### Safety

- The library should not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.

### Alternatives

None.

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
