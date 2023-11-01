use crate::stream::Stream;
use crate::{Length, LengthUnit};
use crate::directional_position::DirectionalPosition;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Position {
    Length(Length),
    DirectionalPosition(DirectionalPosition)
}

impl Position {
    fn is_vertical(&self) -> bool {
        match self {
            Position::Length(_) => true,
            Position::DirectionalPosition(dp) => dp.is_vertical()
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Position::Length(_) => true,
            Position::DirectionalPosition(dp) => dp.is_horizontal()
        }
    }
}

impl From<Position> for Length {
    fn from(value: Position) -> Self {
        match value {
            Position::Length(l) => l,
            Position::DirectionalPosition(dp) => dp.into()
        }
    }
}

/// Representation of the [`<transform-origin>`] type.
///
/// [`<transform-origin>`]: https://drafts.csswg.org/css-transforms/#transform-origin-property
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct TransformOrigin {
    pub x_offset: Length,
    pub y_offset: Length,
    pub z_offset: Length,
}

impl TransformOrigin {
    /// Constructs a new transform.
    #[inline]
    pub fn new(x_offset: Length, y_offset: Length, z_offset: Length) -> Self {
        TransformOrigin {x_offset, y_offset, z_offset}
    }
}

/// List of possible [`ViewBox`] parsing errors.
#[derive(Clone, Copy, Debug)]
pub enum TransformOriginError {
    /// One of the numbers is invalid.
    MissingPositions,

    /// ViewBox has a negative or zero size.
    InvalidPositions,

    ZIndexIsPercentage
}

// impl std::fmt::Display for TransformOriginError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match *self {
//             TransformOriginError::InvalidNumber => {
//                 write!(f, "viewBox contains an invalid number")
//             }
//             TransformOriginError::InvalidSize => {
//                 write!(f, "viewBox has a negative or zero size")
//             }
//         }
//     }
// }
//
// impl std::error::Error for TransformOriginError {
//     fn description(&self) -> &str {
//         "a viewBox parsing error"
//     }
// }

impl std::str::FromStr for TransformOrigin {
    type Err = TransformOriginError;

    fn from_str(text: &str) -> Result<Self, TransformOriginError> {
        let mut s = Stream::from(text);

        if s.at_end() {
            return Err(TransformOriginError::MissingPositions);
        }

        let mut first_val = None;
        let mut second_val = None;
        let mut third_val = None;

        let mut parse_part= || {
            if let Ok(dp) = s.parse_directional_position() {
                Some(Position::DirectionalPosition(dp))
            } else if let Ok(l) = s.parse_length() {
                Some(Position::Length(l))
            }  else { None }
        };

        first_val = parse_part();
        second_val = parse_part();
        third_val = s.parse_length().ok();

        let result = match (first_val, second_val, third_val) {
            (Some(p), None, None) => {
                let (x_offset, y_offset) = if p.is_horizontal() {
                    (p.into(), DirectionalPosition::Center.into())
                }   else {
                    (DirectionalPosition::Center.into(), p.into())
                };

                TransformOrigin::new(x_offset, y_offset, Length::zero())
            },
            (Some(p1), Some(p2), length) => {
                if let Some(length) = length {
                    if length.unit == LengthUnit::Percent {
                        return Err(TransformOriginError::ZIndexIsPercentage);
                    }
                }

                let check = |pos| {
                    match pos {
                        Position::Length(_) => true,
                        Position::DirectionalPosition(dp) => dp == DirectionalPosition::Center
                    }
                };

                let only_keyword_is_center = check(p1) && check(p2);

                if only_keyword_is_center {
                    TransformOrigin::new(p1.into(), p2.into(), length.unwrap_or_default())
                }   else {
                    // There is at least one of `left`, `right`, `top`, or `bottom`
                    if p1.is_horizontal() && p2.is_vertical() {
                        TransformOrigin::new(p1.into(), p2.into(), length.unwrap_or_default())
                    }   else if p1.is_vertical() && p2.is_horizontal() {
                        TransformOrigin::new(p2.into(), p1.into(), length.unwrap_or_default())
                    }   else {
                        return Err(TransformOriginError::InvalidPositions)
                    }
                }
            }
            _ => unreachable!()
        };

        Ok(result)
    }
}