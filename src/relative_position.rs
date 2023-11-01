use crate::{Error, Stream};

/// List of all SVG relative position units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum RelativePosition {
    Top,
    Center,
    Bottom,
    Right,
    Left,
}

impl std::str::FromStr for RelativePosition {
    type Err = Error;

    #[inline]
    fn from_str(text: &str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let relative_pos = s.parse_relative_position()?;

        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(relative_pos)
    }
}

impl<'a> Stream<'a> {
    /// Parses a relative from the stream [`left`, `center`, `right`, `bottom`, `top`].
    pub fn parse_relative_position(&mut self) -> Result<RelativePosition, Error> {
        self.skip_spaces();

        if self.starts_with(b"left") {
            self.advance(4);
            return Ok(RelativePosition::Left);
        } else if self.starts_with(b"right") {
            self.advance(5);
            return Ok(RelativePosition::Right);
        } else if self.starts_with(b"top") {
            self.advance(3);
            return Ok(RelativePosition::Top);
        } else if self.starts_with(b"bottom") {
            self.advance(6);
            return Ok(RelativePosition::Bottom);
        } else if self.starts_with(b"center") {
            self.advance(6);
            return Ok(RelativePosition::Center);
        } else {
            return Err(Error::InvalidString(
                vec![
                    self.slice_tail().to_string(),
                    "left".to_string(),
                    "right".to_string(),
                    "top".to_string(),
                    "bottom".to_string(),
                    "center".to_string(),
                ],
                self.calc_char_pos(),
            ));
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test_p {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(RelativePosition::from_str($text).unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "left", RelativePosition::Left);
    test_p!(parse_2,  "right", RelativePosition::Right);
    test_p!(parse_3,  "center", RelativePosition::Center);
    test_p!(parse_4,  "top", RelativePosition::Top);
    test_p!(parse_5,  "bottom", RelativePosition::Bottom);

    #[test]
    fn parse_6() {
        let mut s = Stream::from("left,");
        assert_eq!(s.parse_relative_position().unwrap(), RelativePosition::Left);
    }

    #[test]
    fn parse_7() {
        let mut s = Stream::from("left ,");
        assert_eq!(s.parse_relative_position().unwrap(), RelativePosition::Left);
    }

    #[test]
    fn parse_16() {
        let mut s = Stream::from("left center");
        assert_eq!(s.parse_relative_position().unwrap(), RelativePosition::Left);
    }

    #[test]
    fn err_1() {
        let mut s = Stream::from("something");
        assert_eq!(s.parse_relative_position().unwrap_err().to_string(),
                   "expected 'left', 'right', 'top', 'bottom', 'center' not 'something' at position 1");
    }
}
