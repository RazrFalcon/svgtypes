// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use {
    Error,
    Path,
    PathSegment,
    Result,
    Stream,
};

macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None
        }
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        let tokens = PathParser::from(text);
        Ok(Path(tokens.collect()))
    }
}

/// A pull-based [path data] parser.
///
/// # Errors
///
/// By the SVG spec any invalid data inside the path data should stop parsing of this path,
/// but not the whole document. So the parser will stop on the first invalid data.
///
/// Example: `M 10 20 L 30 40 #!@$1 L 50 60` -> `M 10 20 L 30 40`
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
/// ```rust
/// use svgtypes::{PathParser, PathSegment};
///
/// let mut p = PathParser::from("M10-20l30.1.5.1-20z");
/// assert_eq!(p.next(), Some(PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 } ));
/// assert_eq!(p.next(), Some(PathSegment::LineTo { abs: false, x: 30.1, y: 0.5 } ));
/// assert_eq!(p.next(), Some(PathSegment::LineTo { abs: false, x: 0.1, y: -20.0 } ));
/// assert_eq!(p.next(), Some(PathSegment::ClosePath { abs: false } ));
/// assert_eq!(p.next(), None);
/// ```
///
/// [path data]: https://www.w3.org/TR/SVG11/paths.html#PathData
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PathParser<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> From<&'a str> for PathParser<'a> {
    fn from(v: &'a str) -> Self {
        PathParser {
            stream: Stream::from(v),
            prev_cmd: None,
        }
    }
}

impl<'a> Iterator for PathParser<'a> {
    type Item = PathSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return None;
        }

        macro_rules! data_error {
            () => ({
                warn!("Invalid path data at {}. The remaining data is ignored.",
                      s.calc_char_pos());
                s.jump_to_end();
                return None;
            })
        }

        macro_rules! try_num {
            ($expr:expr) => (
                match $expr {
                    Ok(v) => v,
                    Err(_) => data_error!(),
                }
            )
        }

        macro_rules! parse_num {
            () => ( try_num!(s.parse_list_number()); )
        }

        let has_prev_cmd = self.prev_cmd.is_some();
        let first_char = s.curr_byte().unwrap(); // TODO: remove unwrap

        if !has_prev_cmd && !is_cmd(first_char) {
            warn!("'{}' is not a command. \
                   The remaining data is ignored.", first_char as char);
            s.jump_to_end();
            return None;
        }

        if !has_prev_cmd {
            match first_char {
                b'M' | b'm' => {}
                _ => {
                    warn!("First segment must be MoveTo. \
                           The remaining data is ignored.");
                    s.jump_to_end();
                    return None;
                }
            }
        }

        // TODO: simplify
        let is_implicit_move_to;
        let cmd: u8;
        if is_cmd(first_char) {
            is_implicit_move_to = false;
            cmd = first_char;
            s.advance(1);
        } else if is_digit(first_char) && has_prev_cmd {
            // unwrap is safe, because we checked 'has_prev_cmd'
            let prev_cmd = self.prev_cmd.unwrap();

            if prev_cmd == b'Z' || prev_cmd == b'z' {
                warn!("ClosePath cannot be followed by a number. \
                       The remaining data is ignored.");
                s.jump_to_end();
                return None;
            }

            if prev_cmd == b'M' || prev_cmd == b'm' {
                // 'If a moveto is followed by multiple pairs of coordinates,
                // the subsequent pairs are treated as implicit lineto commands.'
                // So we parse them as LineTo.
                is_implicit_move_to = true;
                cmd = if is_absolute(prev_cmd) { b'L' } else { b'l' };
            } else {
                is_implicit_move_to = false;
                cmd = prev_cmd;
            }
        } else {
            data_error!();
        }

        let cmdl = to_relative(cmd);
        let absolute = is_absolute(cmd);
        let token = match cmdl {
            b'm' => {
                PathSegment::MoveTo {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'l' => {
                PathSegment::LineTo {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'h' => {
                PathSegment::HorizontalLineTo {
                    abs: absolute,
                    x: parse_num!(),
                }
            }
            b'v' => {
                PathSegment::VerticalLineTo {
                    abs: absolute,
                    y: parse_num!(),
                }
            }
            b'c' => {
                PathSegment::CurveTo {
                    abs: absolute,
                    x1: parse_num!(),
                    y1: parse_num!(),
                    x2: parse_num!(),
                    y2: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b's' => {
                PathSegment::SmoothCurveTo {
                    abs: absolute,
                    x2: parse_num!(),
                    y2: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b'q' => {
                PathSegment::Quadratic {
                    abs: absolute,
                    x1: parse_num!(),
                    y1: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b't' => {
                PathSegment::SmoothQuadratic {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'a' => {
                // TODO: radius cannot be negative
                PathSegment::EllipticalArc {
                    abs: absolute,
                    rx: parse_num!(),
                    ry: parse_num!(),
                    x_axis_rotation: parse_num!(),
                    large_arc: try_opt!(parse_flag(s)),
                    sweep: try_opt!(parse_flag(s)),
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'z' => {
                PathSegment::ClosePath {
                    abs: absolute,
                }
            }
            _ => unreachable!(),
        };

        self.prev_cmd = Some(
            if is_implicit_move_to {
                if is_absolute(cmd) { b'M' } else { b'm' }
            } else {
                cmd
            }
        );

        Some(token)
    }
}

/// Returns `true` if the selected char is the command.
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

fn is_digit(c: u8) -> bool {
    match c {
        b'0'...b'9' | b'.' | b'-' | b'+' => true,
        _ => false,
    }
}

// By the SVG spec 'large-arc' and 'sweep' must contain only one char
// and can be written without any separators, aka: 10 20 30 01 10 20.
fn parse_flag(s: &mut Stream) -> Option<bool> {
    s.skip_spaces();
    let c = try_opt!(s.curr_byte().ok());
    match c {
        b'0' | b'1' => {
            s.advance(1);
            if try_opt!(s.curr_byte().ok()) == b',' {
                s.advance(1);
            }
            s.skip_spaces();

            Some(c == b'1')
        }
        _ => {
            // error type is not relevant since it will be ignored
            None
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
                    assert_eq!(s.next().unwrap(), $seg);
                )*

                assert_eq!(s.next().is_none(), true);
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
