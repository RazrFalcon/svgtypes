// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Points;

use {
    Error,
    StrSpan,
    FromSpan,
    Result,
    Stream,
    StreamExt,
};

/// A pull-based [`<list-of-points>`] parser.
///
/// Use it for the `points` attribute of the `polygon` and `polyline` elements.
///
/// # Errors
///
/// - Stops on a first invalid character. Follows the same rules as paths parser.
///
/// # Notes
///
/// - If data contains an odd number of coordinates - the last one will be ignored.
///   As SVG spec states.
/// - It doesn't validate that there are more than two coordinate pairs,
///   which is required by the SVG spec.
///
/// # Example
///
/// ```rust
/// use svgtypes::PointsParser;
///
/// let mut p = PointsParser::from("10 20 30 40");
/// assert_eq!(p.next(), Some((10.0, 20.0)));
/// assert_eq!(p.next(), Some((30.0, 40.0)));
/// assert_eq!(p.next(), None);
/// ```
///
/// [`<list-of-points>`]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PointsParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for PointsParser<'a> {
    fn from(v: &'a str) -> Self {
        Self::from(StrSpan::from(v))
    }
}

impl<'a> From<StrSpan<'a>> for PointsParser<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        PointsParser(Stream::from(span))
    }
}

impl<'a> Iterator for PointsParser<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let x = match self.0.parse_list_number() {
                Ok(x) => x,
                Err(_) => return None,
            };

            let y = match self.0.parse_list_number() {
                Ok(y) => y,
                Err(_) => return None,
            };

            Some((x, y))
        }
    }
}

impl_from_str!(Points);

impl FromSpan for Points {
    fn from_span(span: StrSpan) -> Result<Self> {
        // TODO: should contain at least two coordinate pairs
        Ok(Points(PointsParser::from(span).collect()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use {WriteBuffer, WriteOptions, ListSeparator};

    #[test]
    fn parse_1() {
        let points = Points::from_str("10 20 30 40").unwrap();
        assert_eq!(*points, vec![(10.0, 20.0), (30.0, 40.0)]);
    }

    #[test]
    fn parse_2() {
        let points = Points::from_str("10 20 30 40 50").unwrap();
        assert_eq!(*points, vec![(10.0, 20.0), (30.0, 40.0)]);
    }

    #[test]
    fn parse_3() {
        let points = Points::from_str("10 20 30 40").unwrap();
        assert_eq!(points.to_string(), "10 20 30 40");
    }

    #[test]
    fn parse_4() {
        let points = Points::from_str("10 20 30 40").unwrap();

        let opt = WriteOptions {
            list_separator: ListSeparator::Comma,
            .. WriteOptions::default()
        };

        assert_eq!(points.with_write_opt(&opt).to_string(), "10,20,30,40");
    }
}
