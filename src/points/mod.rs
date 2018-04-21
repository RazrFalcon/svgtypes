// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod parser;
mod writer;

pub use self::parser::*;

/// Representation of the [`<list-of-points>`] type.
///
/// [`<list-of-points>`]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
#[derive(Clone, PartialEq)]
pub struct Points(pub Vec<(f64, f64)>);

impl_from_vec!(Points, Points, (f64, f64));
impl_vec_defer!(Points, (f64, f64));
