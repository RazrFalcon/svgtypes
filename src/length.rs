use crate::{Error, Result, Stream};

/// List of all SVG length units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum LengthUnit {
    None,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
}

/// Representation of the [`<length>`] type.
///
/// [`<length>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Length {
    pub number: f64,
    pub unit: LengthUnit,
}

impl Length {
    /// Constructs a new length.
    #[inline]
    pub fn new(number: f64, unit: LengthUnit) -> Length {
        Length { number, unit }
    }

    /// Constructs a new length with `LengthUnit::None`.
    #[inline]
    pub fn new_number(number: f64) -> Length {
        Length { number, unit: LengthUnit::None }
    }

    /// Constructs a new length with a zero number.
    ///
    /// Shorthand for: `Length::new(0.0, Unit::None)`.
    #[inline]
    pub fn zero() -> Length {
        Length { number: 0.0, unit: LengthUnit::None }
    }
}

impl Default for Length {
    #[inline]
    fn default() -> Self {
        Length::zero()
    }
}

impl std::str::FromStr for Length {
    type Err = Error;

    #[inline]
    fn from_str(text: &str) -> Result<Self> {
        let mut s = Stream::from(text);
        let l = s.parse_length()?;

        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Length::new(l.number, l.unit))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test_p {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Length::from_str($text).unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "1",   Length::new(1.0, LengthUnit::None));
    test_p!(parse_2,  "1em", Length::new(1.0, LengthUnit::Em));
    test_p!(parse_3,  "1ex", Length::new(1.0, LengthUnit::Ex));
    test_p!(parse_4,  "1px", Length::new(1.0, LengthUnit::Px));
    test_p!(parse_5,  "1in", Length::new(1.0, LengthUnit::In));
    test_p!(parse_6,  "1cm", Length::new(1.0, LengthUnit::Cm));
    test_p!(parse_7,  "1mm", Length::new(1.0, LengthUnit::Mm));
    test_p!(parse_8,  "1pt", Length::new(1.0, LengthUnit::Pt));
    test_p!(parse_9,  "1pc", Length::new(1.0, LengthUnit::Pc));
    test_p!(parse_10, "1%",  Length::new(1.0, LengthUnit::Percent));
    test_p!(parse_11, "1e0", Length::new(1.0, LengthUnit::None));
    test_p!(parse_12, "1.0e0", Length::new(1.0, LengthUnit::None));
    test_p!(parse_13, "1.0e0em", Length::new(1.0, LengthUnit::Em));

    #[test]
    fn err_1() {
        let mut s = Stream::from("1q");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
        assert_eq!(s.parse_length().unwrap_err().to_string(),
                   "invalid number at position 2");
    }

    #[test]
    fn err_2() {
        assert_eq!(Length::from_str("1mmx").unwrap_err().to_string(),
                   "unexpected data at position 4");
    }
}
