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
    WriteBuffer,
    WriteOptions,
};

/// Representation of the `<list-of-numbers>` type.
#[derive(Clone, PartialEq)]
pub struct NumberList(pub Vec<f64>);

impl_from_vec!(NumberList, NumberList, f64);
impl_vec_defer!(NumberList, f64);
impl_display!(NumberList);
impl_debug_from_display!(NumberList);

/// A pull-based number list parser.
///
/// # Example
///
/// ```rust
/// use svgtypes::NumberListParser;
///
/// let mut p = NumberListParser::from("10, 20 -50");
/// assert_eq!(p.next().unwrap().unwrap(), 10.0);
/// assert_eq!(p.next().unwrap().unwrap(), 20.0);
/// assert_eq!(p.next().unwrap().unwrap(), -50.0);
/// assert_eq!(p.next().is_none(), true);
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NumberListParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for NumberListParser<'a> {
    fn from(v: &'a str) -> Self {
        Self::from(StrSpan::from(v))
    }
}

impl<'a> From<StrSpan<'a>> for NumberListParser<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        NumberListParser(Stream::from(span))
    }
}

impl<'a> Iterator for NumberListParser<'a> {
    type Item = Result<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let v = self.0.parse_list_number();
            if v.is_err() {
                self.0.jump_to_end();
            }

            Some(v)
        }
    }
}

impl_from_str!(NumberList);

impl FromSpan for NumberList {
    fn from_span(span: StrSpan) -> Result<Self> {
        let mut vec = Vec::new();
        for number in NumberListParser::from(span) {
            vec.push(number?);
        }

        Ok(NumberList(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ListSeparator;

    #[test]
    fn write_1() {
        let list = NumberList(vec![1.0, 2.0, 3.0]);

        let mut opt = WriteOptions::default();
        opt.list_separator = ListSeparator::Space;

        assert_eq!(list.with_write_opt(&opt).to_string(), "1 2 3");
    }

    #[test]
    fn write_2() {
        let list = NumberList(vec![1.0, 2.0, 3.0]);

        let mut opt = WriteOptions::default();
        opt.list_separator = ListSeparator::Comma;

        assert_eq!(list.with_write_opt(&opt).to_string(), "1,2,3");
    }

    #[test]
    fn write_3() {
        let list = NumberList(vec![1.0, 2.0, 3.0]);

        let mut opt = WriteOptions::default();
        opt.list_separator = ListSeparator::CommaSpace;

        assert_eq!(list.with_write_opt(&opt).to_string(), "1, 2, 3");
    }
}
