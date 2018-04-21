// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{
    Path,
    PathSegment,
};

/// A builder for [`Path`].
///
/// # Examples
///
/// Ellipse to path:
///
/// ```
/// use svgtypes::PathBuilder;
///
/// let (cx, cy, rx, ry) = (10.0, 20.0, 5.0, 8.0);
///
/// let path = PathBuilder::with_capacity(6)
///     .move_to(cx + rx, cy)
///     .arc_to(rx, ry, 0.0, false, true, cx,      cy + ry)
///     .arc_to(rx, ry, 0.0, false, true, cx - rx, cy)
///     .arc_to(rx, ry, 0.0, false, true, cx,      cy - ry)
///     .arc_to(rx, ry, 0.0, false, true, cx + rx, cy)
///     .close_path()
///     .finalize();
///
/// assert_eq!(path.to_string(), "M 15 20 A 5 8 0 0 1 10 28 A 5 8 0 0 1 5 20 \
///                               A 5 8 0 0 1 10 12 A 5 8 0 0 1 15 20 Z");
/// ```
///
/// [`Path`]: struct.Path.html
#[allow(missing_debug_implementations)]
pub struct PathBuilder {
    path: Path,
}

impl PathBuilder {
    /// Constructs a new builder.
    pub fn new() -> PathBuilder {
        PathBuilder { path: Path::new() }
    }

    /// Constructs a new builder with a specified capacity.
    pub fn with_capacity(capacity: usize) -> PathBuilder {
        PathBuilder { path: Path::with_capacity(capacity) }
    }

    // TODO: from existing Path

    /// Appends a new absolute MoveTo segment.
    pub fn move_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::MoveTo { abs: true, x, y });
        self
    }

    /// Appends a new relative MoveTo segment.
    pub fn rel_move_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::MoveTo { abs: false, x, y });
        self
    }

    /// Appends a new absolute ClosePath segment.
    pub fn close_path(mut self) -> PathBuilder {
        self.path.push(PathSegment::ClosePath { abs: true });
        self
    }

    /// Appends a new relative ClosePath segment.
    pub fn rel_close_path(mut self) -> PathBuilder {
        self.path.push(PathSegment::ClosePath { abs: false });
        self
    }

    /// Appends a new absolute LineTo segment.
    pub fn line_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::LineTo { abs: true, x, y });
        self
    }

    /// Appends a new relative LineTo segment.
    pub fn rel_line_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::LineTo { abs: false, x, y });
        self
    }

    /// Appends a new absolute HorizontalLineTo segment.
    pub fn hline_to(mut self, x: f64) -> PathBuilder {
        self.path.push(PathSegment::HorizontalLineTo { abs: true, x });
        self
    }

    /// Appends a new relative HorizontalLineTo segment.
    pub fn rel_hline_to(mut self, x: f64) -> PathBuilder {
        self.path.push(PathSegment::HorizontalLineTo { abs: false, x });
        self
    }

    /// Appends a new absolute VerticalLineTo segment.
    pub fn vline_to(mut self, y: f64) -> PathBuilder {
        self.path.push(PathSegment::VerticalLineTo { abs: true, y });
        self
    }

    /// Appends a new relative VerticalLineTo segment.
    pub fn rel_vline_to(mut self, y: f64) -> PathBuilder {
        self.path.push(PathSegment::VerticalLineTo { abs: false, y });
        self
    }

    /// Appends a new absolute CurveTo segment.
    pub fn curve_to(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::CurveTo { abs: true, x1, y1, x2, y2, x, y });
        self
    }

    /// Appends a new relative CurveTo segment.
    pub fn rel_curve_to(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::CurveTo { abs: false, x1, y1, x2, y2, x, y });
        self
    }

    /// Appends a new absolute SmoothCurveTo segment.
    pub fn smooth_curve_to(mut self, x2: f64, y2: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::SmoothCurveTo { abs: true, x2, y2, x, y });
        self
    }

    /// Appends a new relative SmoothCurveTo segment.
    pub fn rel_smooth_curve_to(mut self, x2: f64, y2: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::SmoothCurveTo { abs: false, x2, y2, x, y });
        self
    }

    /// Appends a new absolute QuadTo segment.
    pub fn quad_to(mut self, x1: f64, y1: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::Quadratic { abs: true, x1, y1, x, y });
        self
    }

    /// Appends a new relative QuadTo segment.
    pub fn rel_quad_to(mut self, x1: f64, y1: f64, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::Quadratic { abs: false, x1, y1, x, y });
        self
    }

    /// Appends a new absolute SmoothQuadTo segment.
    pub fn smooth_quad_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::SmoothQuadratic { abs: true, x, y });
        self
    }

    /// Appends a new relative SmoothQuadTo segment.
    pub fn rel_smooth_quad_to(mut self, x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::SmoothQuadratic { abs: false, x, y });
        self
    }

    /// Appends a new absolute ArcTo segment.
    pub fn arc_to(mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool,
                  x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::EllipticalArc { abs: true, rx, ry, x_axis_rotation,
                                                    large_arc, sweep, x, y });
        self
    }

    /// Appends a new relative ArcTo segment.
    pub fn rel_arc_to(mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool,
                      x: f64, y: f64) -> PathBuilder {
        self.path.push(PathSegment::EllipticalArc { abs: false, rx, ry, x_axis_rotation,
                                                    large_arc, sweep, x, y });
        self
    }

    /// Finalizes the build.
    pub fn finalize(self) -> Path {
        self.path
    }
}
