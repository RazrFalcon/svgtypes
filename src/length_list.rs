use std::str::FromStr;

use {Error, Length, Result, Stream, WriteBuffer, WriteOptions};

/// Representation of the [`<list-of-length>`] type.
///
/// [`<list-of-length>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
#[derive(Clone, PartialEq, Default)]
pub struct LengthList(pub Vec<Length>);

impl_from_vec!(LengthList, LengthList, Length);
impl_vec_defer!(LengthList, Length);
impl_display!(LengthList);
impl_debug_from_display!(LengthList);

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
/// [`<list-of-length>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeList
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

impl FromStr for LengthList {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        let mut vec = Vec::new();
        for number in LengthListParser::from(text) {
            vec.push(number?);
        }

        Ok(LengthList(vec))
    }
}
