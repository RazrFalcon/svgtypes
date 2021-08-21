use crate::{Error, FuzzyEq, Result, Stream, WriteBuffer, WriteOptions};

/// List of all SVG angle units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum AngleUnit {
    Degrees,
    Gradians,
    Radians,
    Turns,
}

/// Representation of the [`<angle>`] type.
///
/// [`<angle>`]: https://www.w3.org/TR/css-values-3/#angles
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Angle {
    pub number: f64,
    pub unit: AngleUnit,
}

impl Angle {
    /// Constructs a new angle.
    #[inline]
    pub fn new(number: f64, unit: AngleUnit) -> Angle {
        Angle { number, unit }
    }

    /// Converts angle to degrees.
    #[inline]
    pub fn to_degrees(&self) -> f64 {
        match self.unit {
            AngleUnit::Degrees  => self.number,
            AngleUnit::Gradians => self.number * 180.0 / 200.0,
            AngleUnit::Radians  => self.number.to_degrees(),
            AngleUnit::Turns    => self.number * 360.0,
        }
    }
}

impl std::str::FromStr for Angle {
    type Err = Error;

    #[inline]
    fn from_str(text: &str) -> Result<Self> {
        let mut s = Stream::from(text);
        let l = s.parse_angle()?;

        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Angle::new(l.number, l.unit))
    }
}

impl WriteBuffer for Angle {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        self.number.write_buf_opt(opt, buf);

        let t: &[u8] = match self.unit {
            AngleUnit::Degrees  => b"deg",
            AngleUnit::Gradians => b"grad",
            AngleUnit::Radians  => b"rad",
            AngleUnit::Turns  => b"turn",
        };

        buf.extend_from_slice(t);
    }
}

impl_display!(Angle);

impl FuzzyEq for Angle {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self.unit != other.unit {
            return false;
        }

        self.number.fuzzy_eq(&other.number)
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
                assert_eq!(Angle::from_str($text).unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "1",   Angle::new(1.0, AngleUnit::Degrees));
    test_p!(parse_2,  "1deg", Angle::new(1.0, AngleUnit::Degrees));
    test_p!(parse_3,  "1grad", Angle::new(1.0, AngleUnit::Gradians));
    test_p!(parse_4,  "1rad", Angle::new(1.0, AngleUnit::Radians));
    test_p!(parse_5,  "1turn", Angle::new(1.0, AngleUnit::Turns));

    #[test]
    fn err_1() {
        let mut s = Stream::from("1q");
        assert_eq!(s.parse_angle().unwrap(), Angle::new(1.0, AngleUnit::Degrees));
        assert_eq!(s.parse_angle().unwrap_err().to_string(),
                   "invalid number at position 2");
    }

    #[test]
    fn err_2() {
        assert_eq!(Angle::from_str("1degq").unwrap_err().to_string(),
                   "unexpected data at position 5");
    }

    macro_rules! test_w {
        ($name:ident, $len:expr, $unit:expr, $result:expr) => (
            #[test]
            fn $name() {
                let l = Angle::new($len, $unit);
                assert_eq!(l.to_string(), $result);
            }
        )
    }

    test_w!(write_1,  1.0, AngleUnit::Degrees, "1deg");
    test_w!(write_2,  1.0, AngleUnit::Gradians, "1grad");
    test_w!(write_3,  1.0, AngleUnit::Radians, "1rad");
    test_w!(write_4,  1.0, AngleUnit::Turns, "1turn");
}
