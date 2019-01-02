/*!

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

## Supported SVG types

| SVG Type                  | Rust Type     | Storage | Parser              |
| ------------------------- | ------------- | ------- | ------------------- |
| [\<color\>]               | Color         | Stack   |                     |
| [\<number\>]              | f64           | Stack   |                     |
| [\<length\>]              | Length        | Stack   |                     |
| [\<viewBox\>]             | ViewBox       | Stack   |                     |
| [\<path\>]                | Path          | Heap    | PathParser          |
| [\<list-of-numbers\>]     | NumberList    | Heap    | NumberListParser    |
| [\<list-of-lengths\>]     | LengthList    | Heap    | LengthListParser    |
| [\<transform-list\>]      | Transform     | Stack   | TransformListParser |
| [\<list-of-points\>]      | Points        | Heap    | PointsParser        |
| [\<style\>]               | -             | -       | StyleParser         |
| [\<paint\>]               | -             | -       | Paint               |

[\<color\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeColor
[\<number\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeNumber
[\<length\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeLength
[\<viewBox\>]: https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute
[\<path\>]: https://www.w3.org/TR/SVG11/paths.html#PathData
[\<list-of-numbers\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
[\<list-of-lengths\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
[\<transform-list\>]: https://www.w3.org/TR/SVG11/types.html#DataTypeTransformList
[\<list-of-points\>]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
[\<style\>]: https://www.w3.org/TR/SVG11/styling.html#StyleAttribute
[\<paint\>]: https://www.w3.org/TR/SVG11/painting.html#SpecifyingPaint

- All types implement from string (`FromStr`) and
  to string traits (`Display`, `WriteBuffer`).
- The library doesn't store transform list as is. It will premultiplied.
- `style` and `paint` types can only be parsed.

## Benefits

- Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
- Access to pull-based parsers.
- Pretty fast.

## Limitations

- Accepts only [normalized](https://www.w3.org/TR/REC-xml/#AVNormalize) values,
  e.g. an input text should not contain `&#x20;` or `&data;`.
- All keywords must be lowercase.
  Case-insensitive parsing is supported only for colors (requires allocation for named colors).
- The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
- Comments inside attributes value supported only for the `style` attribute.
- [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`,
  are not supported. They were deprecated anyway.
- Implicit path commands are not supported. All commands will be parsed as explicit.
- Implicit MoveTo commands will be automatically converted into explicit LineTo.

## Safety

- The library should not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.

## Alternatives

None.
*/

// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/svgtypes/0.4.0")]


#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]


extern crate float_cmp;
extern crate phf;


#[macro_use] mod traits;
mod angle;
mod aspect_ratio;
mod attribute_id;
mod color;
mod element_id;
mod error;
mod length;
mod length_list;
mod number;
mod number_list;
mod options;
mod paint;
mod path;
mod points;
mod stream;
mod style;
mod transform;
mod viewbox;

pub use angle::*;
pub use aspect_ratio::*;
pub use attribute_id::*;
pub use color::*;
pub use element_id::*;
pub use error::*;
pub use length::*;
pub use length_list::*;
pub use number::*;
pub use number_list::*;
pub use options::*;
pub use paint::*;
pub use path::*;
pub use points::*;
pub use stream::*;
pub use style::*;
pub use traits::*;
pub use transform::*;
pub use viewbox::*;
