// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    Error,
    FromSpan,
    Result,
    Stream,
    StreamExt,
    StrSpan,
    Length,
    WriteBuffer,
    WriteOptions,
};

/// Representation of the `<list-of-length>` type.
#[derive(Clone, PartialEq)]
pub struct LengthList(pub Vec<Length>);

impl_from_vec!(LengthList, LengthList, Length);
impl_vec_defer!(LengthList, Length);
impl_display!(LengthList);
impl_debug_from_display!(LengthList);

/// A pull-based length list parser.
///
/// # Example
///
/// ```rust
/// use svgtypes::{Length, LengthUnit, LengthListParser};
///
/// let mut p = LengthListParser::from("10px 20% 50mm");
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(10.0, LengthUnit::Px));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(20.0, LengthUnit::Percent));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(50.0, LengthUnit::Mm));
/// assert_eq!(p.next().is_none(), true);
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LengthListParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for LengthListParser<'a> {
    fn from(v: &'a str) -> Self {
        Self::from(StrSpan::from(v))
    }
}

impl<'a> From<StrSpan<'a>> for LengthListParser<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        LengthListParser(Stream::from(span))
    }
}

impl<'a> Iterator for LengthListParser<'a> {
    type Item = Result<Length>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let v = self.0.parse_list_length();
            if v.is_err() {
                self.0.jump_to_end();
            }

            Some(v)
        }
    }
}

impl_from_str!(LengthList);

impl FromSpan for LengthList {
    fn from_span(span: StrSpan) -> Result<Self> {
        let mut vec = Vec::new();
        for number in LengthListParser::from(span) {
            vec.push(number?);
        }

        Ok(LengthList(vec))
    }
}
