// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    Stream,
    StrSpan,
    Error,
    Result,
    FromSpan,
    WriteBuffer,
    WriteOptions,
};


/// Representation of the `align` value of the [`preserveAspectRatio`] attribute.
///
/// [`preserveAspectRatio`]: https://www.w3.org/TR/SVG/coords.html#PreserveAspectRatioAttribute
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Align {
    None,
    XMinYMin,
    XMidYMin,
    XMaxYMin,
    XMinYMid,
    XMidYMid,
    XMaxYMid,
    XMinYMax,
    XMidYMax,
    XMaxYMax,
}

/// Representation of the [`preserveAspectRatio`] attribute.
///
/// [`preserveAspectRatio`]: https://www.w3.org/TR/SVG/coords.html#PreserveAspectRatioAttribute
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AspectRatio {
    /// `<defer>` value.
    ///
    /// Set to `true` when `defer` value is present.
    pub defer: bool,
    /// `<align>` value.
    pub align: Align,
    /// `<meetOrSlice>` value.
    ///
    /// - Set to `true` when `slice` value is present.
    /// - Set to `false` when `meet` value is present or value is not set at all.
    pub slice: bool,
}

impl FromSpan for AspectRatio {
    fn from_span(span: StrSpan) -> Result<Self> {
        let mut s = Stream::from(span);

        s.skip_spaces();

        let defer = s.starts_with(b"defer");
        if defer {
            s.advance(5);
            s.consume_byte(b' ')?;
            s.skip_spaces();
        }

        let align = s.consume_name()?.to_str();
        let align = match align {
            "none" => Align::None,
            "xMinYMin" => Align::XMinYMin,
            "xMidYMin" => Align::XMidYMin,
            "xMaxYMin" => Align::XMaxYMin,
            "xMinYMid" => Align::XMinYMid,
            "xMidYMid" => Align::XMidYMid,
            "xMaxYMid" => Align::XMaxYMid,
            "xMinYMax" => Align::XMinYMax,
            "xMidYMax" => Align::XMidYMax,
            "xMaxYMax" => Align::XMaxYMax,
            _ => return {
                Err(Error::InvalidAlignType(align.into()))
            }
        };

        s.skip_spaces();

        let mut slice = false;
        if !s.at_end() {
            let v = s.consume_name()?.to_str();
            match v {
                "meet" => {}
                "slice" => slice = true,
                "" => {}
                _ => return {
                    Err(Error::InvalidAlignSlice(v.into()))
                }
            };
        }

        Ok(AspectRatio {
            defer,
            align,
            slice,
        })
    }
}

impl_from_str!(AspectRatio);

impl WriteBuffer for AspectRatio {
    fn write_buf_opt(&self, _: &WriteOptions, buf: &mut Vec<u8>) {
        if self.defer {
            buf.extend_from_slice(b"defer ");
        }

        let align = match self.align {
            Align::None     => "none",
            Align::XMinYMin => "xMinYMin",
            Align::XMidYMin => "xMidYMin",
            Align::XMaxYMin => "xMaxYMin",
            Align::XMinYMid => "xMinYMid",
            Align::XMidYMid => "xMidYMid",
            Align::XMaxYMid => "xMaxYMid",
            Align::XMinYMax => "xMinYMax",
            Align::XMidYMax => "xMidYMax",
            Align::XMaxYMax => "xMaxYMax",
        };

        buf.extend_from_slice(align.as_bytes());

        if self.slice {
            buf.extend_from_slice(b" slice");
        }
    }
}

impl_display!(AspectRatio);

impl Default for AspectRatio {
    fn default() -> Self {
        AspectRatio {
            defer: false,
            align: Align::XMidYMid,
            slice: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                let v = AspectRatio::from_str($text).unwrap();
                assert_eq!(v, $result);
            }
        )
    }

    test!(parse_1, "none", AspectRatio {
        defer: false,
        align: Align::None,
        slice: false,
    });

    test!(parse_2, "defer none", AspectRatio {
        defer: true,
        align: Align::None,
        slice: false,
    });

    test!(parse_3, "xMinYMid", AspectRatio {
        defer: false,
        align: Align::XMinYMid,
        slice: false,
    });

    test!(parse_4, "xMinYMid slice", AspectRatio {
        defer: false,
        align: Align::XMinYMid,
        slice: true,
    });

    test!(parse_5, "xMinYMid meet", AspectRatio {
        defer: false,
        align: Align::XMinYMid,
        slice: false,
    });

    #[test]
    fn write_1() {
        assert_eq!(AspectRatio::default().to_string(), "xMidYMid");
    }

    #[test]
    fn write_2() {
        assert_eq!(AspectRatio {
            defer: true,
            align: Align::None,
            slice: true,
        }.to_string(), "defer none slice");
    }
}
