use FuzzyEq;

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
            | PathSegment::ClosePath { ref mut abs, .. } => {
                *abs = new_abs;
            }
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
            | PathSegment::ClosePath { abs, .. } => abs,
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

            PathSegment::VerticalLineTo { .. } | PathSegment::ClosePath { .. } => None,
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

            PathSegment::HorizontalLineTo { .. } | PathSegment::ClosePath { .. } => None,
        }
    }
}

impl FuzzyEq for PathSegment {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        use self::PathSegment as Seg;

        // TODO: find a way to wrap it in macro
        match (*self, *other) {
            (
                Seg::MoveTo { abs, x, y },
                Seg::MoveTo {
                    abs: oabs,
                    x: ox,
                    y: oy,
                },
            )
            | (
                Seg::LineTo { abs, x, y },
                Seg::LineTo {
                    abs: oabs,
                    x: ox,
                    y: oy,
                },
            )
            | (
                Seg::SmoothQuadratic { abs, x, y },
                Seg::SmoothQuadratic {
                    abs: oabs,
                    x: ox,
                    y: oy,
                },
            ) => abs == oabs && x.fuzzy_eq(&ox) && y.fuzzy_eq(&oy),
            (Seg::HorizontalLineTo { abs, x }, Seg::HorizontalLineTo { abs: oabs, x: ox }) => {
                abs == oabs && x.fuzzy_eq(&ox)
            }
            (Seg::VerticalLineTo { abs, y }, Seg::VerticalLineTo { abs: oabs, y: oy }) => {
                abs == oabs && y.fuzzy_eq(&oy)
            }
            (
                Seg::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                },
                Seg::CurveTo {
                    abs: oabs,
                    x1: ox1,
                    y1: oy1,
                    x2: ox2,
                    y2: oy2,
                    x: ox,
                    y: oy,
                },
            ) => {
                abs == oabs
                    && x.fuzzy_eq(&ox)
                    && y.fuzzy_eq(&oy)
                    && x1.fuzzy_eq(&ox1)
                    && y1.fuzzy_eq(&oy1)
                    && x2.fuzzy_eq(&ox2)
                    && y2.fuzzy_eq(&oy2)
            }
            (
                Seg::SmoothCurveTo { abs, x2, y2, x, y },
                Seg::SmoothCurveTo {
                    abs: oabs,
                    x2: ox2,
                    y2: oy2,
                    x: ox,
                    y: oy,
                },
            ) => {
                abs == oabs
                    && x.fuzzy_eq(&ox)
                    && y.fuzzy_eq(&oy)
                    && x2.fuzzy_eq(&ox2)
                    && y2.fuzzy_eq(&oy2)
            }
            (
                Seg::Quadratic { abs, x1, y1, x, y },
                Seg::Quadratic {
                    abs: oabs,
                    x1: ox1,
                    y1: oy1,
                    x: ox,
                    y: oy,
                },
            ) => {
                abs == oabs
                    && x.fuzzy_eq(&ox)
                    && y.fuzzy_eq(&oy)
                    && x1.fuzzy_eq(&ox1)
                    && y1.fuzzy_eq(&oy1)
            }
            (
                Seg::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x,
                    y,
                },
                Seg::EllipticalArc {
                    abs: oabs,
                    rx: orx,
                    ry: ory,
                    x_axis_rotation: ox_axis_rotation,
                    large_arc: olarge_arc,
                    sweep: osweep,
                    x: ox,
                    y: oy,
                },
            ) => {
                abs == oabs
                    && x.fuzzy_eq(&ox)
                    && y.fuzzy_eq(&oy)
                    && rx.fuzzy_eq(&orx)
                    && ry.fuzzy_eq(&ory)
                    && x_axis_rotation.fuzzy_eq(&ox_axis_rotation)
                    && large_arc == olarge_arc
                    && sweep == osweep
            }
            (Seg::ClosePath { abs }, Seg::ClosePath { abs: oabs }) => abs == oabs,
            _ => false,
        }
    }
}

#[cfg(test)]
mod fuzzy_eq_tests {
    use super::*;

    macro_rules! test {
        ($name:ident,  $seg1:expr, $seg2:expr) => {
            #[test]
            fn $name() {
                assert!($seg1 != $seg2);
                assert!($seg1.fuzzy_eq(&$seg2));
            }
        };
    }

    // TODO: find a better way

    test!(
        m,
        PathSegment::MoveTo {
            abs: true,
            x: 10.0,
            y: 10.1 + 10.2
        },
        PathSegment::MoveTo {
            abs: true,
            x: 10.0,
            y: 20.3
        }
    );

    test!(
        l,
        PathSegment::LineTo {
            abs: true,
            x: 10.0,
            y: 10.1 + 10.2
        },
        PathSegment::LineTo {
            abs: true,
            x: 10.0,
            y: 20.3
        }
    );

    test!(
        h,
        PathSegment::HorizontalLineTo {
            abs: true,
            x: 10.1 + 10.2
        },
        PathSegment::HorizontalLineTo { abs: true, x: 20.3 }
    );

    test!(
        v,
        PathSegment::VerticalLineTo {
            abs: true,
            y: 10.1 + 10.2
        },
        PathSegment::VerticalLineTo { abs: true, y: 20.3 }
    );

    test!(
        c,
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0,
            y1: 10.1 + 10.2,
            x2: 10.0,
            y2: 10.0,
            x: 10.0,
            y: 10.0
        },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0,
            y1: 20.3,
            x2: 10.0,
            y2: 10.0,
            x: 10.0,
            y: 10.0
        }
    );

    test!(
        s,
        PathSegment::SmoothCurveTo {
            abs: true,
            x2: 10.0,
            y2: 10.1 + 10.2,
            x: 10.0,
            y: 10.0
        },
        PathSegment::SmoothCurveTo {
            abs: true,
            x2: 10.0,
            y2: 20.3,
            x: 10.0,
            y: 10.0
        }
    );

    test!(
        q,
        PathSegment::Quadratic {
            abs: true,
            x1: 10.0,
            y1: 10.1 + 10.2,
            x: 10.0,
            y: 10.0
        },
        PathSegment::Quadratic {
            abs: true,
            x1: 10.0,
            y1: 20.3,
            x: 10.0,
            y: 10.0
        }
    );

    test!(
        t,
        PathSegment::SmoothQuadratic {
            abs: true,
            x: 10.0,
            y: 10.1 + 10.2
        },
        PathSegment::SmoothQuadratic {
            abs: true,
            x: 10.0,
            y: 20.3
        }
    );

    test!(
        a,
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
