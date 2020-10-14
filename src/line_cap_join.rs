use std::str::FromStr;

use {Error, Result};

/// [`stroke-linecap`] as defined by the SVG specification.
///
/// Adapted from [lyon](https://docs.rs/lyon_tessellation/0.16.2/src/lyon_tessellation/lib.rs.html#369-385)
///
/// [`stroke-linecap`]: https://svgwg.org/specs/strokes/#StrokeLinecapProperty
///
/// <svg viewBox="0 0 400 399.99998" height="400" width="400">
///   <g transform="translate(0,-652.36229)">
///     <path style="opacity:1;fill:#80b3ff;stroke:#000000;stroke-width:1;stroke-linejoin:round;" d="m 240,983 a 30,30 0 0 1 -25,-15 30,30 0 0 1 0,-30.00001 30,30 0 0 1 25.98076,-15 l 0,30 z"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,782.6 -150,0 0,-60 150,0.5"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" r="10" cy="752.89227" cx="240.86813"/>
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 240,722.6 150,60"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,882 -180,0 0,-60 180,0.4"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" cx="239.86813" cy="852.20868" r="10" />
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 210.1,822.3 180,60"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,983 -150,0 0,-60 150,0.4"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" cx="239.86813" cy="953.39734" r="10" />
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 390,983 -150,-60 L 210,953 l 30,30 -21.5,-9.5 L 210,953 218.3,932.5 240,923.4"/>
///     <text y="757.61273" x="183.65314" style="font-style:normal;font-weight:normal;font-size:20px;line-height:125%;font-family:Sans;text-align:end;text-anchor:end;fill:#000000;stroke:none;">
///        <tspan y="757.61273" x="183.65314">LineCap::Butt</tspan>
///        <tspan y="857.61273" x="183.65314">LineCap::Square</tspan>
///        <tspan y="957.61273" x="183.65314">LineCap::Round</tspan>
///      </text>
///   </g>
/// </svg>
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineCap {
    /// The stroke for each sub-path does not extend beyond its two endpoints.
    /// A zero length sub-path will therefore not have any stroke.
    Butt,
    /// At the end of each sub-path, the shape representing the stroke will be
    /// extended by a rectangle with the same width as the stroke width and
    /// whose length is half of the stroke width. If a sub-path has zero length,
    /// then the resulting effect is that the stroke for that sub-path consists
    /// solely of a square with side length equal to the stroke width, centered
    /// at the sub-path's point.
    Square,
    /// At each end of each sub-path, the shape representing the stroke will be extended
    /// by a half circle with a radius equal to the stroke width.
    /// If a sub-path has zero length, then the resulting effect is that the stroke for
    /// that sub-path consists solely of a full circle centered at the sub-path's point.
    Round,
}

/// # Example
///
/// ```
/// use std::str::FromStr;
/// use svgtypes::LineCap;
///
/// let cap = LineCap::from_str("square").unwrap();
/// assert_eq!(LineCap::Square, cap);
/// ```
impl FromStr for LineCap {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        let text = text.trim();
        match text {
            "butt" => Ok(LineCap::Butt),
            "round" => Ok(LineCap::Round),
            "square" => Ok(LineCap::Square),
            _ => Err(Error::InvalidValue),
        }
    }
}

/// [`stroke-linejoin`] as defined by the SVG specification.
///
/// Adapted from [lyon](https://docs.rs/lyon_tessellation/0.16.2/src/lyon_tessellation/lib.rs.html#369-385)
///
/// [`stroke-linejoin`]: https://svgwg.org/specs/strokes/#StrokeLinejoinProperty
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineJoin {
    /// A sharp corner is to be used to join path segments.
    Miter,
    /// Same as a miter join, but if the miter limit is exceeded,
    /// the miter is clipped at a miter length equal to the miter limit value
    /// multiplied by the stroke width.
    MiterClip,
    /// A round corner is to be used to join path segments.
    Round,
    /// A bevelled corner is to be used to join path segments.
    /// The bevel shape is a triangle that fills the area between the two stroked
    /// segments.
    Bevel,
}

/// # Example
///
/// ```
/// use std::str::FromStr;
/// use svgtypes::LineJoin;
///
/// let join = LineJoin::from_str("round").unwrap();
/// assert_eq!(LineJoin::Round, join);
/// ```
impl FromStr for LineJoin {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        match text.trim() {
            "bevel" => Ok(LineJoin::Bevel),
            "miter" => Ok(LineJoin::Miter),
            "miterclip" => Ok(LineJoin::MiterClip),
            "round" => Ok(LineJoin::Round),
            _ => Err(Error::InvalidValue),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($struc:ident, $name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name() {
                assert_eq!($struc::from_str($text).unwrap(), $result);
            }
        };
    }

    test!(LineCap, parse_butt, "butt", LineCap::Butt);
    test!(LineCap, parse_round, "  round", LineCap::Round);
    test!(LineCap, parse_square, " square ", LineCap::Square);
    test!(LineCap, parse_square_trimmed, "square", LineCap::Square);

    test!(LineJoin, parse_bevel, "   bevel", LineJoin::Bevel);
    test!(LineJoin, parse_miter, "miter   ", LineJoin::Miter);
    test!(LineJoin, parse_miterclip, " miterclip", LineJoin::MiterClip);
    test!(LineJoin, parse_roun, "   round", LineJoin::Round);
}
