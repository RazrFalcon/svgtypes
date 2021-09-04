use crate::{Error, Result, Stream, FuzzyEq};

/// List of all path commands.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum PathCommand {
    MoveTo,
    LineTo,
    HorizontalLineTo,
    VerticalLineTo,
    CurveTo,
    SmoothCurveTo,
    Quadratic,
    SmoothQuadratic,
    EllipticalArc,
    ClosePath,
}

/// Representation of the path segment.
///
/// If you want to change the segment type (for example MoveTo to LineTo)
/// you should create a new segment.
/// But you still can change points or make segment relative or absolute.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PathSegment {
    MoveTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    LineTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    HorizontalLineTo {
        abs: bool,
        x: f64,
    },
    VerticalLineTo {
        abs: bool,
        y: f64,
    },
    CurveTo {
        abs: bool,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    SmoothCurveTo {
        abs: bool,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        abs: bool,
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    SmoothQuadratic {
        abs: bool,
        x: f64,
        y: f64,
    },
    EllipticalArc {
        abs: bool,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    },
    ClosePath {
        abs: bool,
    },
}

impl PathSegment {
    /// Sets the segment absolute value.
    pub fn set_absolute(&mut self, new_abs: bool) {
        match *self {
              PathSegment::MoveTo { ref mut abs, .. }
            | PathSegment::LineTo { ref mut abs, .. }
            | PathSegment::HorizontalLineTo { ref mut abs, .. }
            | PathSegment::VerticalLineTo { ref mut abs, .. }
            | PathSegment::CurveTo { ref mut abs, .. }
            | PathSegment::SmoothCurveTo { ref mut abs, .. }
            | PathSegment::Quadratic { ref mut abs, .. }
            | PathSegment::SmoothQuadratic { ref mut abs, .. }
            | PathSegment::EllipticalArc { ref mut abs, .. }
            | PathSegment::ClosePath { ref mut abs, .. } => { *abs = new_abs; }
        }
    }

    /// Returns a segment type.
    pub fn cmd(&self) -> PathCommand {
        match *self {
            PathSegment::MoveTo { .. } => PathCommand::MoveTo,
            PathSegment::LineTo { .. } => PathCommand::LineTo,
            PathSegment::HorizontalLineTo { .. } => PathCommand::HorizontalLineTo,
            PathSegment::VerticalLineTo { .. } => PathCommand::VerticalLineTo,
            PathSegment::CurveTo { .. } => PathCommand::CurveTo,
            PathSegment::SmoothCurveTo { .. } => PathCommand::SmoothCurveTo,
            PathSegment::Quadratic { .. } => PathCommand::Quadratic,
            PathSegment::SmoothQuadratic { .. } => PathCommand::SmoothQuadratic,
            PathSegment::EllipticalArc { .. } => PathCommand::EllipticalArc,
            PathSegment::ClosePath { .. } => PathCommand::ClosePath,
        }
    }

    /// Returns `true` if the segment is absolute.
    #[inline]
    pub fn is_absolute(&self) -> bool {
        match *self {
              PathSegment::MoveTo { abs, .. }
            | PathSegment::LineTo { abs, .. }
            | PathSegment::HorizontalLineTo { abs, .. }
            | PathSegment::VerticalLineTo { abs, .. }
            | PathSegment::CurveTo { abs, .. }
            | PathSegment::SmoothCurveTo { abs, .. }
            | PathSegment::Quadratic { abs, .. }
            | PathSegment::SmoothQuadratic { abs, .. }
            | PathSegment::EllipticalArc { abs, .. }
            | PathSegment::ClosePath { abs, .. } => { abs }
        }
    }

    #[inline]
    /// Returns `true` if the segment is relative.
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Returns the `x` coordinate of the segment if it has one.
    pub fn x(&self) -> Option<f64> {
        match *self {
              PathSegment::MoveTo { x, .. }
            | PathSegment::LineTo { x, .. }
            | PathSegment::HorizontalLineTo { x, .. }
            | PathSegment::CurveTo { x, .. }
            | PathSegment::SmoothCurveTo { x, .. }
            | PathSegment::Quadratic { x, .. }
            | PathSegment::SmoothQuadratic { x, .. }
            | PathSegment::EllipticalArc { x, .. } => Some(x),

              PathSegment::VerticalLineTo { .. }
            | PathSegment::ClosePath { .. } => None,
        }
    }

    /// Returns the `y` coordinate of the segment if it has one.
    pub fn y(&self) -> Option<f64> {
        match *self {
              PathSegment::MoveTo { y, .. }
            | PathSegment::LineTo { y, .. }
            | PathSegment::VerticalLineTo { y, .. }
            | PathSegment::CurveTo { y, .. }
            | PathSegment::SmoothCurveTo { y, .. }
            | PathSegment::Quadratic { y, .. }
            | PathSegment::SmoothQuadratic { y, .. }
            | PathSegment::EllipticalArc { y, .. } => Some(y),

              PathSegment::HorizontalLineTo { .. }
            | PathSegment::ClosePath { .. } => None,
        }
    }
}

impl FuzzyEq for PathSegment {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        use self::PathSegment as Seg;

        // TODO: find a way to wrap it in macro
        match (*self, *other) {
            (Seg::MoveTo { abs, x, y }, Seg::MoveTo { abs: oabs, x: ox, y: oy }) |
            (Seg::LineTo { abs, x, y }, Seg::LineTo { abs: oabs, x: ox, y: oy }) |
            (Seg::SmoothQuadratic { abs, x, y }, Seg::SmoothQuadratic { abs: oabs, x: ox, y: oy }) => {
                abs == oabs && x.fuzzy_eq(&ox) && y.fuzzy_eq(&oy)
            }
            (Seg::HorizontalLineTo { abs, x }, Seg::HorizontalLineTo { abs: oabs, x: ox }) => {
                abs == oabs && x.fuzzy_eq(&ox)
            }
            (Seg::VerticalLineTo { abs, y }, Seg::VerticalLineTo { abs: oabs, y: oy }) => {
                abs == oabs && y.fuzzy_eq(&oy)
            }
            (Seg::CurveTo { abs, x1, y1, x2, y2, x, y },
                Seg::CurveTo { abs: oabs, x1: ox1, y1: oy1, x2: ox2, y2: oy2, x: ox, y: oy }) => {
                    abs == oabs
                &&  x.fuzzy_eq(&ox)  &&  y.fuzzy_eq(&oy)
                && x1.fuzzy_eq(&ox1) && y1.fuzzy_eq(&oy1)
                && x2.fuzzy_eq(&ox2) && y2.fuzzy_eq(&oy2)
            }
            (Seg::SmoothCurveTo { abs, x2, y2, x, y },
                Seg::SmoothCurveTo { abs: oabs, x2: ox2, y2: oy2, x: ox, y: oy }) => {
                abs == oabs
                &&  x.fuzzy_eq(&ox)  &&  y.fuzzy_eq(&oy)
                && x2.fuzzy_eq(&ox2) && y2.fuzzy_eq(&oy2)
            }
            (Seg::Quadratic { abs, x1, y1, x, y },
                Seg::Quadratic { abs: oabs, x1: ox1, y1: oy1, x: ox, y: oy }) => {
                   abs == oabs
                &&  x.fuzzy_eq(&ox)  &&  y.fuzzy_eq(&oy)
                && x1.fuzzy_eq(&ox1) && y1.fuzzy_eq(&oy1)
            }
            (Seg::EllipticalArc { abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y },
                Seg::EllipticalArc { abs: oabs, rx: orx, ry: ory, x_axis_rotation: ox_axis_rotation,
                    large_arc: olarge_arc, sweep: osweep, x: ox, y: oy }) => {
                    abs == oabs
                &&  x.fuzzy_eq(&ox)  &&  y.fuzzy_eq(&oy)
                && rx.fuzzy_eq(&orx) && ry.fuzzy_eq(&ory)
                && x_axis_rotation.fuzzy_eq(&ox_axis_rotation)
                && large_arc == olarge_arc
                && sweep == osweep
            }
            (Seg::ClosePath { abs }, Seg::ClosePath { abs: oabs }) => {
                abs == oabs
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod fuzzy_eq_tests {
    use super::*;

    macro_rules! test {
        ($name:ident,  $seg1:expr, $seg2:expr) => (
        #[test]
        fn $name() {
            assert!($seg1 != $seg2);
            assert!($seg1.fuzzy_eq(&$seg2));
        })
    }

    // TODO: find a better way

    test!(m,
        PathSegment::MoveTo { abs: true, x: 10.0, y: 10.1 + 10.2 },
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.3 }
    );

    test!(l,
        PathSegment::LineTo { abs: true, x: 10.0, y: 10.1 + 10.2 },
        PathSegment::LineTo { abs: true, x: 10.0, y: 20.3 }
    );

    test!(h,
        PathSegment::HorizontalLineTo { abs: true, x: 10.1 + 10.2 },
        PathSegment::HorizontalLineTo { abs: true, x: 20.3 }
    );

    test!(v,
        PathSegment::VerticalLineTo { abs: true, y: 10.1 + 10.2 },
        PathSegment::VerticalLineTo { abs: true, y: 20.3 }
    );

    test!(c,
        PathSegment::CurveTo { abs: true, x1: 10.0, y1: 10.1 + 10.2, x2: 10.0, y2: 10.0, x: 10.0, y: 10.0 },
        PathSegment::CurveTo { abs: true, x1: 10.0, y1: 20.3, x2: 10.0, y2: 10.0, x: 10.0, y: 10.0 }
    );

    test!(s,
        PathSegment::SmoothCurveTo { abs: true, x2: 10.0, y2: 10.1 + 10.2, x: 10.0, y: 10.0 },
        PathSegment::SmoothCurveTo { abs: true, x2: 10.0, y2: 20.3, x: 10.0, y: 10.0 }
    );

    test!(q,
        PathSegment::Quadratic { abs: true, x1: 10.0, y1: 10.1 + 10.2, x: 10.0, y: 10.0 },
        PathSegment::Quadratic { abs: true, x1: 10.0, y1: 20.3, x: 10.0, y: 10.0 }
    );

    test!(t,
        PathSegment::SmoothQuadratic { abs: true, x: 10.0, y: 10.1 + 10.2 },
        PathSegment::SmoothQuadratic { abs: true, x: 10.0, y: 20.3 }
    );

    test!(a,
        PathSegment::EllipticalArc {
            abs: true,
            rx: 100.0,
            ry: 100.0,
            x_axis_rotation: 0.0,
            large_arc: true,
            sweep: true,
            x: 10.1 + 10.2,
            y: 10.0,
        },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 100.0,
            ry: 100.0,
            x_axis_rotation: 0.0,
            large_arc: true,
            sweep: true,
            x: 20.3,
            y: 10.0,
        }
    );
}


/// A pull-based [path data] parser.
///
/// # Errors
///
/// - Most of the `Error` types can occur.
///
/// # Notes
///
/// The library does not support implicit commands, so they will be converted to an explicit one.
/// It mostly affects an implicit MoveTo, which will be converted, according to the spec,
/// into explicit LineTo.
///
/// Example: `M 10 20 30 40 50 60` -> `M 10 20 L 30 40 L 50 60`
///
/// # Example
///
/// ```
/// use svgtypes::{PathParser, PathSegment};
///
/// let mut segments = Vec::new();
/// for segment in PathParser::from("M10-20l30.1.5.1-20z") {
///     segments.push(segment.unwrap());
/// }
///
/// assert_eq!(segments, &[
///     PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
///     PathSegment::LineTo { abs: false, x: 30.1, y: 0.5 },
///     PathSegment::LineTo { abs: false, x: 0.1, y: -20.0 },
///     PathSegment::ClosePath { abs: false },
/// ]);
/// ```
///
/// [path data]: https://www.w3.org/TR/SVG2/paths.html#PathData
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PathParser<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> From<&'a str> for PathParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        PathParser {
            stream: Stream::from(v),
            prev_cmd: None,
        }
    }
}

impl<'a> Iterator for PathParser<'a> {
    type Item = Result<PathSegment>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return None;
        }

        let res = next_impl(s, &mut self.prev_cmd);
        if res.is_err() {
            s.jump_to_end();
        }

        Some(res)
    }
}

fn next_impl(s: &mut Stream, prev_cmd: &mut Option<u8>) -> Result<PathSegment> {
    let start = s.pos();

    let has_prev_cmd = prev_cmd.is_some();
    let first_char = s.curr_byte_unchecked();

    if !has_prev_cmd && !is_cmd(first_char) {
        return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
    }

    if !has_prev_cmd {
        if !matches!(first_char, b'M' | b'm') {
            // The first segment must be a MoveTo.
            return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
        }
    }

    // TODO: simplify
    let is_implicit_move_to;
    let cmd: u8;
    if is_cmd(first_char) {
        is_implicit_move_to = false;
        cmd = first_char;
        s.advance(1);
    } else if is_number_start(first_char) && has_prev_cmd {
        // unwrap is safe, because we checked 'has_prev_cmd'
        let p_cmd = prev_cmd.unwrap();

        if p_cmd == b'Z' || p_cmd == b'z' {
            // ClosePath cannot be followed by a number.
            return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
        }

        if p_cmd == b'M' || p_cmd == b'm' {
            // 'If a moveto is followed by multiple pairs of coordinates,
            // the subsequent pairs are treated as implicit lineto commands.'
            // So we parse them as LineTo.
            is_implicit_move_to = true;
            cmd = if is_absolute(p_cmd) { b'L' } else { b'l' };
        } else {
            is_implicit_move_to = false;
            cmd = p_cmd;
        }
    } else {
        return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
    }

    let cmdl = to_relative(cmd);
    let absolute = is_absolute(cmd);
    let token = match cmdl {
        b'm' => {
            PathSegment::MoveTo {
                abs: absolute,
                x: s.parse_list_number()?,
                y: s.parse_list_number()?,
            }
        }
        b'l' => {
            PathSegment::LineTo {
                abs: absolute,
                x: s.parse_list_number()?,
                y: s.parse_list_number()?,
            }
        }
        b'h' => {
            PathSegment::HorizontalLineTo {
                abs: absolute,
                x: s.parse_list_number()?,
            }
        }
        b'v' => {
            PathSegment::VerticalLineTo {
                abs: absolute,
                y: s.parse_list_number()?,
            }
        }
        b'c' => {
            PathSegment::CurveTo {
                abs: absolute,
                x1: s.parse_list_number()?,
                y1: s.parse_list_number()?,
                x2: s.parse_list_number()?,
                y2: s.parse_list_number()?,
                x:  s.parse_list_number()?,
                y:  s.parse_list_number()?,
            }
        }
        b's' => {
            PathSegment::SmoothCurveTo {
                abs: absolute,
                x2: s.parse_list_number()?,
                y2: s.parse_list_number()?,
                x:  s.parse_list_number()?,
                y:  s.parse_list_number()?,
            }
        }
        b'q' => {
            PathSegment::Quadratic {
                abs: absolute,
                x1: s.parse_list_number()?,
                y1: s.parse_list_number()?,
                x:  s.parse_list_number()?,
                y:  s.parse_list_number()?,
            }
        }
        b't' => {
            PathSegment::SmoothQuadratic {
                abs: absolute,
                x: s.parse_list_number()?,
                y: s.parse_list_number()?,
            }
        }
        b'a' => {
            // TODO: radius cannot be negative
            PathSegment::EllipticalArc {
                abs: absolute,
                rx: s.parse_list_number()?,
                ry: s.parse_list_number()?,
                x_axis_rotation: s.parse_list_number()?,
                large_arc: parse_flag(s)?,
                sweep: parse_flag(s)?,
                x: s.parse_list_number()?,
                y: s.parse_list_number()?,
            }
        }
        b'z' => {
            PathSegment::ClosePath {
                abs: absolute,
            }
        }
        _ => unreachable!(),
    };

    *prev_cmd = Some(
        if is_implicit_move_to {
            if absolute { b'M' } else { b'm' }
        } else {
            cmd
        }
    );

    Ok(token)
}

/// Returns `true` if the selected char is the command.
#[inline]
fn is_cmd(c: u8) -> bool {
    match c {
          b'M' | b'm'
        | b'Z' | b'z'
        | b'L' | b'l'
        | b'H' | b'h'
        | b'V' | b'v'
        | b'C' | b'c'
        | b'S' | b's'
        | b'Q' | b'q'
        | b'T' | b't'
        | b'A' | b'a' => true,
        _ => false,
    }
}

/// Returns `true` if the selected char is the absolute command.
#[inline]
fn is_absolute(c: u8) -> bool {
    debug_assert!(is_cmd(c));
    match c {
          b'M'
        | b'Z'
        | b'L'
        | b'H'
        | b'V'
        | b'C'
        | b'S'
        | b'Q'
        | b'T'
        | b'A' => true,
        _ => false,
    }
}

/// Converts the selected command char into the relative command char.
#[inline]
fn to_relative(c: u8) -> u8 {
    debug_assert!(is_cmd(c));
    match c {
        b'M' => b'm',
        b'Z' => b'z',
        b'L' => b'l',
        b'H' => b'h',
        b'V' => b'v',
        b'C' => b'c',
        b'S' => b's',
        b'Q' => b'q',
        b'T' => b't',
        b'A' => b'a',
        _ => c,
    }
}

#[inline]
fn is_number_start(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'.' | b'-' | b'+')
}

// By the SVG spec 'large-arc' and 'sweep' must contain only one char
// and can be written without any separators, e.g.: 10 20 30 01 10 20.
fn parse_flag(s: &mut Stream) -> Result<bool> {
    s.skip_spaces();

    let c = s.curr_byte()?;
    match c {
        b'0' | b'1' => {
            s.advance(1);
            if s.is_curr_byte_eq(b',') {
                s.advance(1);
            }
            s.skip_spaces();

            Ok(c == b'1')
        }
        _ => {
            Err(Error::UnexpectedData(s.calc_char_pos_at(s.pos())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $( $seg:expr ),*) => (
            #[test]
            fn $name() {
                let mut s = PathParser::from($text);
                $(
                    assert_eq!(s.next().unwrap().unwrap(), $seg);
                )*

                if let Some(res) = s.next() {
                    assert!(res.is_err());
                }
            }
        )
    }

    test!(null, "", );
    test!(not_a_path, "q", );
    test!(not_a_move_to, "L 20 30", );
    test!(stop_on_err_1, "M 10 20 L 30 40 L 50",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 }
    );

    test!(move_to_1, "M 10 20", PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 });
    test!(move_to_2, "m 10 20", PathSegment::MoveTo { abs: false, x: 10.0, y: 20.0 });
    test!(move_to_3, "M 10 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::LineTo { abs: true, x: 50.0, y: 60.0 }
    );
    test!(move_to_4, "M 10 20 30 40 50 60 M 70 80 90 100 110 120",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::LineTo { abs: true, x: 50.0, y: 60.0 },
        PathSegment::MoveTo { abs: true, x: 70.0, y: 80.0 },
        PathSegment::LineTo { abs: true, x: 90.0, y: 100.0 },
        PathSegment::LineTo { abs: true, x: 110.0, y: 120.0 }
    );

    test!(arc_to_1, "M 10 20 A 5 5 30 1 1 20 20",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 20.0, y: 20.0
        }
    );

    test!(arc_to_2, "M 10 20 a 5 5 30 0 0 20 20",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::EllipticalArc {
            abs: false,
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: false, sweep: false,
            x: 20.0, y: 20.0
        }
    );

    test!(arc_to_10, "M10-20A5.5.3-4 010-.1",
        PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 5.5, ry: 0.3,
            x_axis_rotation: -4.0,
            large_arc: false, sweep: true,
            x: 0.0, y: -0.1
        }
    );

    test!(separator_1, "M 10 20 L 5 15 C 10 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_2, "M 10, 20 L 5, 15 C 10, 20 30, 40 50, 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_3, "M 10,20 L 5,15 C 10,20 30,40 50,60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_4, "M10, 20 L5, 15 C10, 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_5, "M10 20V30H40V50H60Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::VerticalLineTo { abs: true, y: 30.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 40.0 },
        PathSegment::VerticalLineTo { abs: true, y: 50.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 60.0 },
        PathSegment::ClosePath { abs: true }
    );

    test!(all_segments_1, "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 S 130 140 150 160
        Q 170 180 190 200 T 210 220 A 50 50 30 1 1 230 240 Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 50.0 },
        PathSegment::VerticalLineTo { abs: true, y: 60.0 },
        PathSegment::CurveTo {
            abs: true,
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        },
        PathSegment::SmoothCurveTo {
            abs: true,
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        },
        PathSegment::Quadratic {
            abs: true,
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        },
        PathSegment::SmoothQuadratic { abs: true, x: 210.0, y: 220.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        },
        PathSegment::ClosePath { abs: true }
    );

    test!(all_segments_2, "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 s 130 140 150 160
        q 170 180 190 200 t 210 220 a 50 50 30 1 1 230 240 z",
        PathSegment::MoveTo { abs: false, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: false, x: 30.0, y: 40.0 },
        PathSegment::HorizontalLineTo { abs: false, x: 50.0 },
        PathSegment::VerticalLineTo { abs: false, y: 60.0 },
        PathSegment::CurveTo {
            abs: false,
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        },
        PathSegment::SmoothCurveTo {
            abs: false,
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        },
        PathSegment::Quadratic {
            abs: false,
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        },
        PathSegment::SmoothQuadratic { abs: false, x: 210.0, y: 220.0 },
        PathSegment::EllipticalArc {
            abs: false,
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        },
        PathSegment::ClosePath { abs: false }
    );

    test!(close_path_1, "M10 20 L 30 40 ZM 100 200 L 300 400",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::MoveTo { abs: true, x: 100.0, y: 200.0 },
        PathSegment::LineTo { abs: true, x: 300.0, y: 400.0 }
    );

    test!(close_path_2, "M10 20 L 30 40 zM 100 200 L 300 400",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: false },
        PathSegment::MoveTo { abs: true, x: 100.0, y: 200.0 },
        PathSegment::LineTo { abs: true, x: 300.0, y: 400.0 }
    );

    test!(close_path_3, "M10 20 L 30 40 Z Z Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::ClosePath { abs: true },
        PathSegment::ClosePath { abs: true }
    );

    // first token should be EndOfStream
    test!(invalid_1, "M\t.", );

    // ClosePath can't be followed by a number
    test!(invalid_2, "M 0 0 Z 2",
        PathSegment::MoveTo { abs: true, x: 0.0, y: 0.0 },
        PathSegment::ClosePath { abs: true }
    );

    // ClosePath can be followed by any command
    test!(invalid_3, "M 0 0 Z H 10",
        PathSegment::MoveTo { abs: true, x: 0.0, y: 0.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::HorizontalLineTo { abs: true, x: 10.0 }
    );
}
