// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp;

use xmlparser::{
    XmlByteExt,
};

use super::colors;

use {
    Stream,
    StrSpan,
    Result,
    Error,
    LengthUnit,
    StreamExt,
    Color,
    FromSpan,
};

impl_from_str!(Color);

impl FromSpan for Color {
    /// Parses `Color` from `StrSpan`.
    ///
    /// Parsing is done according to [spec]:
    ///
    /// ```text
    /// color    ::= "#" hexdigit hexdigit hexdigit (hexdigit hexdigit hexdigit)?
    ///              | "rgb(" wsp* integer comma integer comma integer wsp* ")"
    ///              | "rgb(" wsp* integer "%" comma integer "%" comma integer "%" wsp* ")"
    ///              | color-keyword
    /// hexdigit ::= [0-9A-Fa-f]
    /// comma    ::= wsp* "," wsp*
    /// ```
    /// \* The SVG spec has an error. There should be `number`,
    /// not an `integer` for percent values ([details]).
    ///
    /// # Errors
    ///
    ///  - Returns error if a color has an invalid format.
    ///
    ///  - Returns error if `<color>` is followed by `<icccolor>`.
    ///    It's not supported.
    ///
    /// # Notes
    ///
    ///  - Any non-`hexdigit` bytes will be treated as `0`.
    ///  - Allocates heap memory for case-insensitive named colors comparison.
    ///
    /// [spec]: http://www.w3.org/TR/SVG/types.html#DataTypeColor
    /// [details]: https://lists.w3.org/Archives/Public/www-svg/2014Jan/0109.html
    fn from_span(span: StrSpan) -> Result<Self> {
        let mut s = Stream::from(span);

        s.skip_spaces();

        let start = s.pos();

        let mut color = Color::new(0, 0, 0);

        if s.curr_byte()? == b'#' {
            s.advance(1);
            let color_str = s.consume_bytes(|_, c| c.is_xml_hex_digit()).to_str().as_bytes();
            // get color data len until first space or stream end
            match color_str.len() {
                6 => {
                    // #rrggbb
                    color.red   = hex_pair(color_str[0], color_str[1]);
                    color.green = hex_pair(color_str[2], color_str[3]);
                    color.blue  = hex_pair(color_str[4], color_str[5]);
                }
                3 => {
                    // #rgb
                    color.red = short_hex(color_str[0]);
                    color.green = short_hex(color_str[1]);
                    color.blue = short_hex(color_str[2]);
                }
                _ => {
                    return Err(Error::InvalidColor(s.gen_error_pos_from(start)));
                }
            }
        } else if is_rgb(&s) {
            s.advance(4);

            let l = s.parse_list_length()?;

            if l.unit == LengthUnit::Percent {
                fn from_persent(v: f64) -> u8 {
                    let d = 255.0 / 100.0;
                    let n = (v * d).round() as i32;
                    bound(0, n, 255) as u8
                }

                color.red = from_persent(l.num);
                color.green = from_persent(s.parse_list_length()?.num);
                color.blue = from_persent(s.parse_list_length()?.num);
            } else {
                color.red = bound(0, l.num as i32, 255) as u8;
                color.green = bound(0, s.parse_list_integer()?, 255) as u8;
                color.blue = bound(0, s.parse_list_integer()?, 255) as u8;
            }

            s.skip_spaces();
            s.consume_byte(b')')?;
        } else {
            let name = s.consume_name()?.to_str().to_lowercase();
            match colors::from_str(&name) {
                Some(c) => {
                    color = c;
                }
                None => {
                    return Err(Error::InvalidColor(s.gen_error_pos_from(start)));
                }
            }
        }

        // Check that we are at the end of the stream. Otherwise color can be followed by icccolor,
        // which is not supported.
        s.skip_spaces();
        if !s.at_end() {
            // TODO: to UnsupportedColor
            return Err(Error::InvalidColor(s.gen_error_pos()));
        }

        Ok(color)
    }
}

#[inline]
fn from_hex(c: u8) -> u8 {
    match c {
        b'0'...b'9' => c - b'0',
        b'a'...b'f' => c - b'a' + 10,
        b'A'...b'F' => c - b'A' + 10,
        _ => b'0',
    }
}

#[inline]
fn short_hex(c: u8) -> u8 {
    let h = from_hex(c);
    (h << 4) | h
}

#[inline]
fn hex_pair(c1: u8, c2: u8) -> u8 {
    let h1 = from_hex(c1);
    let h2 = from_hex(c2);
    (h1 << 4) | h2
}

fn is_rgb(s: &Stream) -> bool {
    let mut s = s.clone();
    let prefix = s.consume_bytes(|_, c| c != b'(').to_str();
    if s.consume_byte(b'(').is_err() {
        return false;
    }

    #[allow(unused_imports)]
    use std::ascii::AsciiExt;

    prefix.eq_ignore_ascii_case("rgb")
}

#[inline]
fn bound<T: Ord>(min: T, val: T, max: T) -> T {
    cmp::max(min, cmp::min(max, val))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use Color;

    macro_rules! test {
        ($name:ident, $text:expr, $color:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Color::from_str($text).unwrap(), $color);
            }
        };
    }

    test!(
        rrggbb,
        "#ff0000",
        Color::new(255, 0, 0)
    );

    test!(
        rrggbb_upper,
        "#FF0000",
        Color::new(255, 0, 0)
    );

    test!(
        rgb_hex,
        "#f00",
        Color::new(255, 0, 0)
    );

    test!(
        rrggbb_spaced,
        "  #ff0000  ",
        Color::new(255, 0, 0)
    );

    test!(
        rgb_numeric,
        "rgb(254, 203, 231)",
        Color::new(254, 203, 231)
    );

    test!(
        rgb_numeric_spaced,
        " rgb( 77 , 77 , 77 ) ",
        Color::new(77, 77, 77)
    );

    test!(
        rgb_percentage,
        "rgb(50%, 50%, 50%)",
        Color::new(127, 127, 127)
    );

    test!(
        rgb_percentage_overflow,
        "rgb(140%, -10%, 130%)",
        Color::new(255, 0, 255)
    );

    test!(
        rgb_percentage_float,
        "rgb(33.333%,46.666%,93.333%)",
        Color::new(85, 119, 238)
    );

    test!(
        rgb_numeric_upper_case,
        "RGB(254, 203, 231)",
        Color::new(254, 203, 231)
    );

    test!(
        rgb_numeric_mixed_case,
        "RgB(254, 203, 231)",
        Color::new(254, 203, 231)
    );

    test!(
        name_red,
        "red",
        Color::new(255, 0, 0)
    );

    test!(
        name_red_spaced,
        " red ",
        Color::new(255, 0, 0)
    );

    test!(
        name_red_upper_case,
        "RED",
        Color::new(255, 0, 0)
    );

    test!(
        name_red_mixed_case,
        "ReD",
        Color::new(255, 0, 0)
    );

    test!(
        name_cornflowerblue,
        "cornflowerblue",
        Color::new(100, 149, 237)
    );

    macro_rules! test_err {
        ($name:ident, $text:expr, $err:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Color::from_str($text).unwrap_err().to_string(), $err);
            }
        };
    }

    test_err!(
        not_a_color_1,
        "text",
        "invalid color at 1:1"
    );

    test_err!(
        icc_color_not_supported_1,
        "#CD853F icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
        "invalid color at 1:9"
    );

    test_err!(
        icc_color_not_supported_2,
        "red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
        "invalid color at 1:5"
    );

    test_err!(
        invalid_input_1,
        "rgb(-0\x0d",
        "unexpected end of stream"
    );

    test_err!(
        invalid_input_2,
        "#9ߞpx! ;",
        "invalid color at 1:1"
    );
}
