mod parser;
mod segment;
mod writer;

pub use self::parser::*;
pub use self::segment::*;

/// Representation of the SVG [path data].
///
/// [path data]: https://www.w3.org/TR/SVG11/paths.html#PathData
#[derive(Clone, PartialEq, Default)]
pub struct Path(pub Vec<PathSegment>);

impl Path {
    /// Constructs a new path.
    #[inline]
    pub fn new() -> Self {
        Path(Vec::new())
    }

    /// Constructs a new path with a specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Path(Vec::with_capacity(capacity))
    }

    /// Converts path's segments into absolute one in-place.
    ///
    /// Original segments can be mixed (relative, absolute).
    pub fn conv_to_absolute(&mut self) {
        // position of the previous segment
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        // Position of the previous MoveTo segment.
        // When we get 'm'(relative) segment, which is not first segment - then it's
        // relative to previous 'M'(absolute) segment, not to first segment.
        let mut prev_mx = 0.0;
        let mut prev_my = 0.0;

        let mut prev_cmd = PathCommand::MoveTo;
        for seg in self.iter_mut() {
            if seg.cmd() == PathCommand::ClosePath {
                prev_x = prev_mx;
                prev_y = prev_my;

                seg.set_absolute(true);
                continue;
            }

            let offset_x;
            let offset_y;
            if seg.is_relative() {
                if seg.cmd() == PathCommand::MoveTo && prev_cmd == PathCommand::ClosePath {
                    offset_x = prev_mx;
                    offset_y = prev_my;
                } else {
                    offset_x = prev_x;
                    offset_y = prev_y;
                }
            } else {
                offset_x = 0.0;
                offset_y = 0.0;
            }

            if seg.is_relative() {
                shift_segment_data(seg, offset_x, offset_y);
            }

            if seg.cmd() == PathCommand::MoveTo {
                prev_mx = seg.x().unwrap();
                prev_my = seg.y().unwrap();
            }

            seg.set_absolute(true);

            if seg.cmd() == PathCommand::HorizontalLineTo {
                prev_x = seg.x().unwrap();
            } else if seg.cmd() == PathCommand::VerticalLineTo {
                prev_y = seg.y().unwrap();
            } else {
                prev_x = seg.x().unwrap();
                prev_y = seg.y().unwrap();
            }

            prev_cmd = seg.cmd();
        }
    }

    /// Converts path's segments into relative one in-place.
    ///
    /// Original segments can be mixed (relative, absolute).
    pub fn conv_to_relative(&mut self) {
        // NOTE: this method may look like 'conv_to_absolute', but it's a bit different.

        // position of the previous segment
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        // Position of the previous MoveTo segment.
        // When we get 'm'(relative) segment, which is not first segment - then it's
        // relative to previous 'M'(absolute) segment, not to first segment.
        let mut prev_mx = 0.0;
        let mut prev_my = 0.0;

        let mut prev_cmd = PathCommand::MoveTo;
        for seg in self.iter_mut() {
            if seg.cmd() == PathCommand::ClosePath {
                prev_x = prev_mx;
                prev_y = prev_my;

                seg.set_absolute(false);
                continue;
            }

            let offset_x;
            let offset_y;
            if seg.is_absolute() {
                if seg.cmd() == PathCommand::MoveTo && prev_cmd == PathCommand::ClosePath {
                    offset_x = prev_mx;
                    offset_y = prev_my;
                } else {
                    offset_x = prev_x;
                    offset_y = prev_y;
                }
            } else {
                offset_x = 0.0;
                offset_y = 0.0;
            }

            // unlike in 'to_absolute', we should take prev values before changing segment data
            if seg.is_absolute() {
                if seg.cmd() == PathCommand::HorizontalLineTo {
                    prev_x = seg.x().unwrap();
                } else if seg.cmd() == PathCommand::VerticalLineTo {
                    prev_y = seg.y().unwrap();
                } else {
                    prev_x = seg.x().unwrap();
                    prev_y = seg.y().unwrap();
                }
            } else {
                if seg.cmd() == PathCommand::HorizontalLineTo {
                    prev_x += seg.x().unwrap();
                } else if seg.cmd() == PathCommand::VerticalLineTo {
                    prev_y += seg.y().unwrap();
                } else {
                    prev_x += seg.x().unwrap();
                    prev_y += seg.y().unwrap();
                }
            }

            if seg.cmd() == PathCommand::MoveTo {
                if seg.is_absolute() {
                    prev_mx = seg.x().unwrap();
                    prev_my = seg.y().unwrap();
                } else {
                    prev_mx += seg.x().unwrap();
                    prev_my += seg.y().unwrap();
                }
            }

            if seg.is_absolute() {
                shift_segment_data(seg, -offset_x, -offset_y);
            }

            seg.set_absolute(false);

            prev_cmd = seg.cmd();
        }
    }

    /// Appends an absolute MoveTo segment.
    #[inline]
    pub fn push_move_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::MoveTo { abs: true, x, y });
    }

    /// Appends a relative MoveTo segment.
    #[inline]
    pub fn push_rel_move_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::MoveTo { abs: false, x, y });
    }

    /// Appends an absolute ClosePath segment.
    #[inline]
    pub fn push_close_path(&mut self) {
        self.push(PathSegment::ClosePath { abs: true });
    }

    /// Appends a relative ClosePath segment.
    #[inline]
    pub fn push_rel_close_path(&mut self) {
        self.push(PathSegment::ClosePath { abs: false });
    }

    /// Appends an absolute LineTo segment.
    #[inline]
    pub fn push_line_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::LineTo { abs: true, x, y });
    }

    /// Appends a relative LineTo segment.
    #[inline]
    pub fn push_rel_line_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::LineTo { abs: false, x, y });
    }

    /// Appends an absolute HorizontalLineTo segment.
    #[inline]
    pub fn push_hline_to(&mut self, x: f64) {
        self.push(PathSegment::HorizontalLineTo { abs: true, x });
    }

    /// Appends a relative HorizontalLineTo segment.
    #[inline]
    pub fn push_rel_hline_to(&mut self, x: f64) {
        self.push(PathSegment::HorizontalLineTo { abs: false, x });
    }

    /// Appends an absolute VerticalLineTo segment.
    #[inline]
    pub fn push_vline_to(&mut self, y: f64) {
        self.push(PathSegment::VerticalLineTo { abs: true, y });
    }

    /// Appends a relative VerticalLineTo segment.
    #[inline]
    pub fn push_rel_vline_to(&mut self, y: f64) {
        self.push(PathSegment::VerticalLineTo { abs: false, y });
    }

    /// Appends an absolute CurveTo segment.
    #[inline]
    pub fn push_curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::CurveTo { abs: true, x1, y1, x2, y2, x, y });
    }

    /// Appends a relative CurveTo segment.
    #[inline]
    pub fn push_rel_curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::CurveTo { abs: false, x1, y1, x2, y2, x, y });
    }

    /// Appends an absolute SmoothCurveTo segment.
    #[inline]
    pub fn push_smooth_curve_to(&mut self, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::SmoothCurveTo { abs: true, x2, y2, x, y });
    }

    /// Appends a relative SmoothCurveTo segment.
    #[inline]
    pub fn push_rel_smooth_curve_to(&mut self, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::SmoothCurveTo { abs: false, x2, y2, x, y });
    }

    /// Appends an absolute QuadTo segment.
    #[inline]
    pub fn push_quad_to(&mut self, x1: f64, y1: f64, x: f64, y: f64) {
        self.push(PathSegment::Quadratic { abs: true, x1, y1, x, y });
    }

    /// Appends a relative QuadTo segment.
    #[inline]
    pub fn push_rel_quad_to(&mut self, x1: f64, y1: f64, x: f64, y: f64) {
        self.push(PathSegment::Quadratic { abs: false, x1, y1, x, y });
    }

    /// Appends an absolute SmoothQuadTo segment.
    #[inline]
    pub fn push_smooth_quad_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::SmoothQuadratic { abs: true, x, y });
    }

    /// Appends a relative SmoothQuadTo segment.
    #[inline]
    pub fn push_rel_smooth_quad_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::SmoothQuadratic { abs: false, x, y });
    }

    /// Appends an absolute ArcTo segment.
    #[inline]
    pub fn push_arc_to(
        &mut self,
        rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64,
    ) {
        self.push(PathSegment::EllipticalArc {
            abs: true, rx, ry, x_axis_rotation, large_arc, sweep, x, y
        });
    }

    /// Appends a relative ArcTo segment.
    #[inline]
    pub fn push_rel_arc_to(
        &mut self,
        rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64,
    ) {
        self.push(PathSegment::EllipticalArc {
            abs: false, rx, ry, x_axis_rotation, large_arc, sweep, x, y
        });
    }
}

fn shift_segment_data(d: &mut PathSegment, offset_x: f64, offset_y: f64) {
    match *d {
        PathSegment::MoveTo { ref mut x, ref mut y, .. } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::LineTo { ref mut x, ref mut y, .. } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::HorizontalLineTo { ref mut x, .. } => {
            *x += offset_x;
        }
        PathSegment::VerticalLineTo { ref mut y, .. } => {
            *y += offset_y;
        }
        PathSegment::CurveTo { ref mut x1, ref mut y1, ref mut x2, ref mut y2,
            ref mut x, ref mut y, .. } => {
            *x1 += offset_x;
            *y1 += offset_y;
            *x2 += offset_x;
            *y2 += offset_y;
            *x  += offset_x;
            *y  += offset_y;
        }
        PathSegment::SmoothCurveTo { ref mut x2, ref mut y2, ref mut x, ref mut y, .. } => {
            *x2 += offset_x;
            *y2 += offset_y;
            *x  += offset_x;
            *y  += offset_y;
        }
        PathSegment::Quadratic { ref mut x1, ref mut y1, ref mut x, ref mut y, .. } => {
            *x1 += offset_x;
            *y1 += offset_y;
            *x  += offset_x;
            *y  += offset_y;
        }
        PathSegment::SmoothQuadratic { ref mut x, ref mut y, .. } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::EllipticalArc { ref mut x, ref mut y, .. } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::ClosePath { .. } => {}
    }
}

impl_from_vec!(Path, Path, PathSegment);
impl_vec_defer!(Path, PathSegment);

#[cfg(test)]
mod to_absolute {
    use std::str::FromStr;
    use super::*;

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let mut path = Path::from_str($in_text).unwrap();
                path.conv_to_absolute();
                assert_eq!(path.to_string(), $out_text);
            }
        )
    }

    test!(line_to,
          "m 10 20 l 20 20",
          "M 10 20 L 30 40");

    test!(close_path,
          "m 10 20 l 20 20 z",
          "M 10 20 L 30 40 Z");

    // test to check that parses implicit MoveTo as LineTo
    test!(implicit_line_to,
          "m 10 20 20 20",
          "M 10 20 L 30 40");

    test!(hline_vline,
          "m 10 20 v 10 h 10 l 10 10",
          "M 10 20 V 30 H 20 L 30 40");

    test!(curve,
          "m 10 20 c 10 10 10 10 10 10",
          "M 10 20 C 20 30 20 30 20 30");

    test!(move_to_1,
          "m 10 20 l 10 10 m 10 10 l 10 10",
          "M 10 20 L 20 30 M 30 40 L 40 50");

    test!(move_to_2,
          "m 10 20 l 10 10 z m 10 10 l 10 10",
          "M 10 20 L 20 30 Z M 20 30 L 30 40");

    test!(move_to_3,
          "m 10 20 l 10 10 Z m 10 10 l 10 10",
          "M 10 20 L 20 30 Z M 20 30 L 30 40");

    // MoveTo after ClosePath can be skipped
    test!(move_to_4,
          "m 10 20 l 10 10 Z l 10 10",
          "M 10 20 L 20 30 Z L 20 30");

    test!(smooth_curve,
          "m 10 20 s 10 10 10 10",
          "M 10 20 S 20 30 20 30");

    test!(quad,
          "m 10 20 q 10 10 10 10",
          "M 10 20 Q 20 30 20 30");

    test!(arc_mixed,
          "M 30 150 a 40 40 0 0 1 65 50 Z m 30 30 A 20 20 0 0 0 125 230 Z \
           m 40 24 a 20 20 0 0 1 65 50 z",
          "M 30 150 A 40 40 0 0 1 95 200 Z M 60 180 A 20 20 0 0 0 125 230 Z \
           M 100 204 A 20 20 0 0 1 165 254 Z");
}

#[cfg(test)]
mod to_relative {
    use std::str::FromStr;
    use super::*;

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let mut path = Path::from_str($in_text).unwrap();
                path.conv_to_relative();
                assert_eq!(path.to_string(), $out_text);
            }
        )
    }

    test!(line_to,
          "M 10 20 L 30 40",
          "m 10 20 l 20 20");

    test!(close_path,
          "M 10 20 L 30 40 Z",
          "m 10 20 l 20 20 z");

    test!(implicit_line_to,
          "M 10 20 30 40",
          "m 10 20 l 20 20");

    test!(hline_vline,
          "M 10 20 V 30 H 20 L 30 40",
          "m 10 20 v 10 h 10 l 10 10");

    test!(curve,
          "M 10 20 C 20 30 20 30 20 30",
          "m 10 20 c 10 10 10 10 10 10");

    test!(move_to_1,
          "M 10 20 L 20 30 M 30 40 L 40 50",
          "m 10 20 l 10 10 m 10 10 l 10 10");

    test!(move_to_2,
          "M 10 20 L 20 30 Z M 20 30 L 30 40",
          "m 10 20 l 10 10 z m 10 10 l 10 10");

    test!(move_to_3,
          "M 10 20 L 20 30 z M 20 30 L 30 40",
          "m 10 20 l 10 10 z m 10 10 l 10 10");

    // MoveTo after ClosePath can be skipped
    test!(move_to_4,
          "M 10 20 L 20 30 Z L 20 30",
          "m 10 20 l 10 10 z l 10 10");

    test!(smooth_curve,
          "M 10 20 S 20 30 20 30",
          "m 10 20 s 10 10 10 10");

    test!(quad,
          "M 10 20 Q 20 30 20 30",
          "m 10 20 q 10 10 10 10");

    test!(arc_mixed,
          "M 30 150 a 40 40 0 0 1 65 50 Z m 30 30 A 20 20 0 0 0 125 230 Z \
           m 40 24 a 20 20 0 0 1 65 50 z",
          "m 30 150 a 40 40 0 0 1 65 50 z m 30 30 a 20 20 0 0 0 65 50 z \
           m 40 24 a 20 20 0 0 1 65 50 z");
}
