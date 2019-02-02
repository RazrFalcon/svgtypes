use {
    Path,
    PathCommand,
    PathSegment,
    WriteBuffer,
    WriteOptions,
};

struct PrevCmd {
    cmd: PathCommand,
    absolute: bool,
    implicit: bool,
}

impl WriteBuffer for Path {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        if self.is_empty() {
            return;
        }

        let mut prev_cmd: Option<PrevCmd> = None;
        let mut prev_coord_has_dot = false;

        for seg in self.iter() {
            let is_written = write_cmd(seg, &mut prev_cmd, opt, buf);
            write_segment(seg, is_written, &mut prev_coord_has_dot, opt, buf);
        }

        if !opt.use_compact_path_notation {
            let len = buf.len();
            buf.truncate(len - 1);
        }
    }
}

fn write_cmd(
    seg: &PathSegment,
    prev_cmd: &mut Option<PrevCmd>,
    opt: &WriteOptions,
    buf: &mut Vec<u8>
) -> bool {
    let mut print_cmd = true;
    if opt.remove_duplicated_path_commands {
        // check that previous command is the same as current
        if let Some(ref pcmd) = *prev_cmd {
            // MoveTo commands can't be skipped
            if pcmd.cmd != PathCommand::MoveTo {
                if seg.cmd() == pcmd.cmd && seg.is_absolute() == pcmd.absolute {
                    print_cmd = false;
                }
            }
        }
    }

    let mut is_implicit = false;
    if opt.use_implicit_lineto_commands {

        let check_implicit = || {
            if let Some(ref pcmd) = *prev_cmd {
                if seg.is_absolute() != pcmd.absolute {
                    return false;
                }

                if pcmd.implicit {
                    if seg.cmd() == PathCommand::LineTo {
                        return true;
                    }
                } else if    pcmd.cmd  == PathCommand::MoveTo
                          && seg.cmd() == PathCommand::LineTo {
                    // if current segment is LineTo and previous was MoveTo
                    return true;
                }
            }

            false
        };

        if check_implicit() {
            is_implicit = true;
            print_cmd = false;
        }
    }

    *prev_cmd = Some(PrevCmd {
        cmd: seg.cmd(),
        absolute: seg.is_absolute(),
        implicit: is_implicit,
    });

    if !print_cmd {
        // we do not update 'prev_cmd' if we do not wrote it
        return false;
    }

    write_cmd_char(seg, buf);

    if !(seg.cmd() == PathCommand::ClosePath || opt.use_compact_path_notation) {
        buf.push(b' ');
    }

    true
}

pub fn write_cmd_char(seg: &PathSegment, buf: &mut Vec<u8>) {
    let cmd: u8 = if seg.is_absolute() {
        match seg.cmd() {
            PathCommand::MoveTo => b'M',
            PathCommand::LineTo => b'L',
            PathCommand::HorizontalLineTo => b'H',
            PathCommand::VerticalLineTo => b'V',
            PathCommand::CurveTo => b'C',
            PathCommand::SmoothCurveTo => b'S',
            PathCommand::Quadratic => b'Q',
            PathCommand::SmoothQuadratic => b'T',
            PathCommand::EllipticalArc => b'A',
            PathCommand::ClosePath => b'Z',
        }
    } else {
        match seg.cmd() {
            PathCommand::MoveTo => b'm',
            PathCommand::LineTo => b'l',
            PathCommand::HorizontalLineTo => b'h',
            PathCommand::VerticalLineTo => b'v',
            PathCommand::CurveTo => b'c',
            PathCommand::SmoothCurveTo => b's',
            PathCommand::Quadratic => b'q',
            PathCommand::SmoothQuadratic => b't',
            PathCommand::EllipticalArc => b'a',
            PathCommand::ClosePath => b'z',
        }
    };
    buf.push(cmd);
}

pub fn write_segment(
    data: &PathSegment,
    is_written: bool,
    prev_coord_has_dot: &mut bool,
    opt: &WriteOptions,
    buf: &mut Vec<u8>
) {
    match *data {
          PathSegment::MoveTo { x, y, .. }
        | PathSegment::LineTo { x, y, .. }
        | PathSegment::SmoothQuadratic { x, y, .. } => {
            write_coords(&[x, y], is_written, prev_coord_has_dot, opt, buf);
        }

        PathSegment::HorizontalLineTo { x, .. } => {
            write_coords(&[x], is_written, prev_coord_has_dot, opt, buf);
        }

        PathSegment::VerticalLineTo { y, .. } => {
            write_coords(&[y], is_written, prev_coord_has_dot, opt, buf);
        }

        PathSegment::CurveTo { x1, y1, x2, y2, x, y, .. } => {
            write_coords(&[x1, y1, x2, y2, x, y], is_written,
                         prev_coord_has_dot, opt, buf);
        }

        PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
            write_coords(&[x2, y2, x, y], is_written, prev_coord_has_dot, opt, buf);
        }

        PathSegment::Quadratic { x1, y1, x, y, .. } => {
            write_coords(&[x1, y1, x, y], is_written, prev_coord_has_dot, opt, buf);
        }

        PathSegment::EllipticalArc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, .. } => {
            write_coords(&[rx, ry, x_axis_rotation], is_written,
                         prev_coord_has_dot, opt, buf);

            if opt.use_compact_path_notation {
                // flags must always have a space before it
                buf.push(b' ');
            }

            write_flag(large_arc, buf);
            if !opt.join_arc_to_flags {
                buf.push(b' ');
            }
            write_flag(sweep, buf);
            if !opt.join_arc_to_flags {
                buf.push(b' ');
            }

            // reset, because flags can't have dots
            *prev_coord_has_dot = false;

            // 'is_explicit_cmd' is always 'true'
            // because it's relevant only for first coordinate of the segment
            write_coords(&[x, y], true, prev_coord_has_dot, opt, buf);
        }
        PathSegment::ClosePath { .. } => {
            if !opt.use_compact_path_notation {
                buf.push(b' ');
            }
        }
    }
}

fn write_coords(
    coords: &[f64],
    is_explicit_cmd: bool,
    prev_coord_has_dot: &mut bool,
    opt: &WriteOptions,
    buf: &mut Vec<u8>
) {
    if opt.use_compact_path_notation {
        for (i, num) in coords.iter().enumerate() {
            let start_pos = buf.len() - 1;

            num.write_buf_opt(opt, buf);

            let c = buf[start_pos + 1];

            let write_space = if !*prev_coord_has_dot && c == b'.' {
                !(i == 0 && is_explicit_cmd)
            } else if i == 0 && is_explicit_cmd {
                false
            } else if (c as char).is_digit(10) {
                true
            } else {
                false
            };

            if write_space {
                buf.insert(start_pos + 1, b' ');
            }

            *prev_coord_has_dot = false;
            for c in buf.iter().skip(start_pos) {
                if *c == b'.' {
                    *prev_coord_has_dot = true;
                    break;
                }
            }
        }
    } else {
        for num in coords.iter() {
            num.write_buf_opt(opt, buf);
            buf.push(b' ');
        }
    }
}

fn write_flag(flag: bool, buf: &mut Vec<u8>) {
    buf.push(if flag { b'1' } else { b'0' });
}

impl_display!(Path);
impl_debug_from_display!(Path);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use WriteOptions;

    #[test]
    fn write_1() {
        let mut path = Path::new();
        path.push(PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 });
        path.push(PathSegment::LineTo { abs: true, x: 10.0, y: 20.0 });
        assert_eq!(path.to_string(), "M 10 20 L 10 20");
    }

    #[test]
    fn write_2() {
        let path = Path::from_str("M 10 20 l 10 20").unwrap();
        assert_eq!(path.to_string(), "M 10 20 l 10 20");
    }

    #[test]
    fn write_3() {
        let path = Path::from_str(
            "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 \
             S 130 140 150 160 Q 170 180 190 200 T 210 220 \
             A 50 50 30 1 1 230 240 Z").unwrap();
        assert_eq!(path.to_string(),
            "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 \
             S 130 140 150 160 Q 170 180 190 200 T 210 220 \
             A 50 50 30 1 1 230 240 Z");
    }

    #[test]
    fn write_4() {
        let path = Path::from_str(
            "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 \
             s 130 140 150 160 q 170 180 190 200 t 210 220 \
             a 50 50 30 1 1 230 240 z").unwrap();
        assert_eq!(path.to_string(),
            "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 \
             s 130 140 150 160 q 170 180 190 200 t 210 220 \
             a 50 50 30 1 1 230 240 z");
    }

    #[test]
    fn write_5() {
        let path = Path::from_str("").unwrap();
        assert_eq!(path.to_string(), "");
    }

    macro_rules! test_write_opt {
        ($name:ident, $in_text:expr, $out_text:expr, $flag:ident) => (
            #[test]
            fn $name() {
                let path = Path::from_str($in_text).unwrap();

                let mut opt = WriteOptions::default();
                opt.$flag = true;

                assert_eq!(path.with_write_opt(&opt).to_string(), $out_text);
            }
        )
    }

    test_write_opt!(write_6,
        "M 10 20 L 30 40 L 50 60 l 70 80",
        "M 10 20 L 30 40 50 60 l 70 80",
        remove_duplicated_path_commands);

    test_write_opt!(write_7,
        "M 10 20 30 40 50 60",
        "M 10 20 L 30 40 50 60",
        remove_duplicated_path_commands);

    test_write_opt!(write_8,
        "M 10 20 L 30 40",
        "M10 20L30 40",
        use_compact_path_notation);

    test_write_opt!(write_9,
        "M 10 20 V 30 H 40 V 50 H 60 Z",
        "M10 20V30H40V50H60Z",
        use_compact_path_notation);

    #[test]
    fn write_10() {
        let path = Path::from_str("M 10 -20 A 5.5 0.3 -4 1 1 0 -0.1").unwrap();

        let mut opt = WriteOptions::default();
        opt.use_compact_path_notation = true;
        opt.join_arc_to_flags = true;
        opt.remove_leading_zero = true;

        assert_eq!(path.with_write_opt(&opt).to_string(), "M10-20A5.5.3-4 110-.1");
    }

    test_write_opt!(write_11,
        "M 10-10 a 1 1 0 1 1 -1 1",
        "M10-10a1 1 0 1 1 -1 1",
        use_compact_path_notation);

    test_write_opt!(write_12,
        "M 10-10 a 1 1 0 1 1 0.1 1",
        "M10-10a1 1 0 1 1 0.1 1",
        use_compact_path_notation);

    test_write_opt!(write_13,
        "M 10 20 L 30 40 L 50 60 H 10",
        "M 10 20 30 40 50 60 H 10",
        use_implicit_lineto_commands);

    // should be ignored, because of different 'absolute' values
    test_write_opt!(write_14,
        "M 10 20 l 30 40 L 50 60",
        "M 10 20 l 30 40 L 50 60",
        use_implicit_lineto_commands);

    test_write_opt!(write_15,
        "M 10 20 L 30 40 l 50 60 L 50 60",
        "M 10 20 30 40 l 50 60 L 50 60",
        use_implicit_lineto_commands);

    test_write_opt!(write_16,
        "M 10 20 L 30 40 l 50 60",
        "M 10 20 30 40 l 50 60",
        use_implicit_lineto_commands);

    test_write_opt!(write_17,
        "M 10 20 L 30 40 L 50 60 M 10 20 L 30 40 L 50 60",
        "M 10 20 30 40 50 60 M 10 20 30 40 50 60",
        use_implicit_lineto_commands);

    #[test]
    fn write_18() {
        let path = Path::from_str("M 10 20 L 30 40 L 50 60 M 10 20 L 30 40 L 50 60").unwrap();

        let mut opt = WriteOptions::default();
        opt.use_implicit_lineto_commands = true;
        opt.remove_duplicated_path_commands = true;

        assert_eq!(path.with_write_opt(&opt).to_string(), "M 10 20 30 40 50 60 M 10 20 30 40 50 60");
    }

    #[test]
    fn write_19() {
        let path = Path::from_str("m10 20 A 10 10 0 1 0 0 0 A 2 2 0 1 0 2 0").unwrap();

        let mut opt = WriteOptions::default();
        opt.use_compact_path_notation = true;
        opt.remove_duplicated_path_commands = true;
        opt.remove_leading_zero = true;

        // may generate as 'm10 20A10 10 0 1 0 0 0 2 2 0 1 0  2 0' <- two spaces

        assert_eq!(path.with_write_opt(&opt).to_string(), "m10 20A10 10 0 1 0 0 0 2 2 0 1 0 2 0");
    }

    #[test]
    fn write_20() {
        let path = Path::from_str("M 0.1 0.1 L 1 0.1 2 -0.1").unwrap();

        let mut opt = WriteOptions::default();
        opt.use_compact_path_notation = true;
        opt.remove_duplicated_path_commands = true;
        opt.remove_leading_zero = true;

        assert_eq!(path.with_write_opt(&opt).to_string(), "M.1.1L1 .1 2-.1");
    }

    test_write_opt!(write_21,
        "M 10 20 M 30 40 M 50 60 L 30 40",
        "M 10 20 M 30 40 M 50 60 L 30 40",
        remove_duplicated_path_commands);
}
