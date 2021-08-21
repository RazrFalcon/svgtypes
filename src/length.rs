use crate::{Error, FuzzyEq, Result, Stream, WriteBuffer, WriteOptions};

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
/// [`<length>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeLength
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Length {
    pub num: f64, // TODO: rename
    pub unit: LengthUnit,
}

impl Length {
    /// Constructs a new length.
    #[inline]
    pub fn new(num: f64, unit: LengthUnit) -> Length {
        Length { num, unit }
    }

    /// Constructs a new length with `LengthUnit::None`.
    #[inline]
    pub fn new_number(num: f64) -> Length {
        Length { num, unit: LengthUnit::None }
    }

    /// Constructs a new length with a zero number.
    ///
    /// Shorthand for: `Length::new(0.0, Unit::None)`.
    #[inline]
    pub fn zero() -> Length {
        Length { num: 0.0, unit: LengthUnit::None }
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

        Ok(Length::new(l.num, l.unit))
    }
}

impl WriteBuffer for Length {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        self.num.write_buf_opt(opt, buf);

        let t: &[u8] = match self.unit {
            LengthUnit::None => b"",
            LengthUnit::Em => b"em",
            LengthUnit::Ex => b"ex",
            LengthUnit::Px => b"px",
            LengthUnit::In => b"in",
            LengthUnit::Cm => b"cm",
            LengthUnit::Mm => b"mm",
            LengthUnit::Pt => b"pt",
            LengthUnit::Pc => b"pc",
            LengthUnit::Percent => b"%",
        };

        buf.extend_from_slice(t);
    }
}

impl_display!(Length);

impl FuzzyEq for Length {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self.unit != other.unit {
            return false;
        }

        self.num.fuzzy_eq(&other.num)
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

    macro_rules! test_w {
        ($name:ident, $len:expr, $unit:expr, $result:expr) => (
            #[test]
            fn $name() {
                let l = Length::new($len, $unit);
                assert_eq!(l.to_string(), $result);
            }
        )
    }

    test_w!(write_1,  1.0, LengthUnit::None, "1");
    test_w!(write_2,  1.0, LengthUnit::Em, "1em");
    test_w!(write_3,  1.0, LengthUnit::Ex, "1ex");
    test_w!(write_4,  1.0, LengthUnit::Px, "1px");
    test_w!(write_5,  1.0, LengthUnit::In, "1in");
    test_w!(write_6,  1.0, LengthUnit::Cm, "1cm");
    test_w!(write_7,  1.0, LengthUnit::Mm, "1mm");
    test_w!(write_8,  1.0, LengthUnit::Pt, "1pt");
    test_w!(write_9,  1.0, LengthUnit::Pc, "1pc");
    test_w!(write_10, 1.0, LengthUnit::Percent, "1%");
}
