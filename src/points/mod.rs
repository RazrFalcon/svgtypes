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
