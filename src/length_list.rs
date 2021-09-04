use crate::{Length, Result, Stream};

/// A pull-based [`<list-of-length>`] parser.
///
/// # Example
///
/// ```
/// use svgtypes::{Length, LengthUnit, LengthListParser};
///
/// let mut p = LengthListParser::from("10px 20% 50mm");
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(10.0, LengthUnit::Px));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(20.0, LengthUnit::Percent));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(50.0, LengthUnit::Mm));
/// assert_eq!(p.next().is_none(), true);
/// ```
///
/// [`<list-of-length>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLengthList
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LengthListParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for LengthListParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        LengthListParser(Stream::from(v))
    }
}

impl<'a> Iterator for LengthListParser<'a> {
    type Item = Result<Length>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let v = self.0.parse_list_length();
            if v.is_err() {
                self.0.jump_to_end();
            }

            Some(v)
        }
    }
}
