use crate::{Result, Stream};

/// A pull-based [`<list-of-numbers>`] parser.
///
/// # Example
///
/// ```
/// use svgtypes::NumberListParser;
///
/// let mut p = NumberListParser::from("10, 20 -50");
/// assert_eq!(p.next().unwrap().unwrap(), 10.0);
/// assert_eq!(p.next().unwrap().unwrap(), 20.0);
/// assert_eq!(p.next().unwrap().unwrap(), -50.0);
/// assert_eq!(p.next().is_none(), true);
/// ```
///
/// [`<list-of-numbers>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumberList
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NumberListParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for NumberListParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        NumberListParser(Stream::from(v))
    }
}

impl<'a> Iterator for NumberListParser<'a> {
    type Item = Result<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let v = self.0.parse_list_number();
            if v.is_err() {
                self.0.jump_to_end();
            }

            Some(v)
        }
    }
}
