// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::char;
use std::str::{self, FromStr};
use std::cmp;

use {
    Angle,
    AngleUnit,
    Error,
    Length,
    LengthUnit,
    Result,
};


/// Extension methods for XML-subset only operations.
pub trait XmlByteExt {
    /// Checks if a byte is a digit.
    ///
    /// `[0-9]`
    fn is_xml_digit(&self) -> bool;

    /// Checks if a byte is a hex digit.
    ///
    /// `[0-9A-Fa-f]`
    fn is_xml_hex_digit(&self) -> bool;

    /// Checks if a byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_xml_space(&self) -> bool;

    /// Checks if a byte is an ASCII char.
    ///
    /// `[A-Za-z]`
    fn is_xml_letter(&self) -> bool;

    /// Checks if a byte is an XML ident char.
    ///
    /// `[A-Za-z]`
    fn is_ident_char(&self) -> bool;
}

impl XmlByteExt for u8 {
    #[inline]
    fn is_xml_digit(&self) -> bool {
        match *self {
            b'0'...b'9' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_hex_digit(&self) -> bool {
        match *self {
            b'0'...b'9'
            | b'A'...b'F'
            | b'a'...b'f' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_space(&self) -> bool {
        match *self {
            b' '
            | b'\t'
            | b'\n'
            | b'\r' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_letter(&self) -> bool {
        match *self {
            b'A'...b'Z' | b'a'...b'z' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_ident_char(&self) -> bool {
        match *self {
            b'0'...b'9'
            | b'A'...b'Z'
            | b'a'...b'z'
            | b'-'
            | b'_' => true,
            _ => false,
        }
    }
}


/// A streaming text parsing interface.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Stream<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(text: &'a str) -> Self {
        Stream {
            text,
            pos: 0,
        }
    }
}

impl<'a> Stream<'a> {
    /// Returns the current position in bytes.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Calculates the current position in chars.
    pub fn calc_char_pos(&self) -> usize {
        self.calc_char_pos_at(self.pos)
    }

    /// Calculates the current position in chars.
    pub fn calc_char_pos_at(&self, byte_pos: usize) -> usize {
        let mut pos = 1;
        for (idx, _) in self.text.char_indices() {
            if idx >= byte_pos {
                break;
            }

            pos += 1;
        }

        pos
    }

    /// Sets current position equal to the end.
    ///
    /// Used to indicate end of parsing on error.
    pub fn jump_to_end(&mut self) {
        self.pos = self.text.len();
    }

    /// Checks if the stream is reached the end.
    ///
    /// Any [`pos()`] value larger than original text length indicates stream end.
    ///
    /// Accessing stream after reaching end via safe methods will produce
    /// an `UnexpectedEndOfStream` error.
    ///
    /// Accessing stream after reaching end via *_unchecked methods will produce
    /// a Rust's bound checking error.
    ///
    /// [`pos()`]: #method.pos
    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    pub fn curr_byte(&self) -> Result<u8> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Panics
    ///
    /// - if the current position is after the end of the data
    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.text.as_bytes()[self.pos]
    }

    /// Checks that current byte is equal to provided.
    ///
    /// Returns `false` if no bytes left.
    #[inline]
    pub fn is_curr_byte_eq(&self, c: u8) -> bool {
        if !self.at_end() {
            self.curr_byte_unchecked() == c
        } else {
            false
        }
    }

    /// Returns a byte from a current stream position if there is one.
    #[inline]
    pub fn get_curr_byte(&self) -> Option<u8> {
        if !self.at_end() {
            Some(self.curr_byte_unchecked())
        } else {
            None
        }
    }

    /// Returns a next byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    pub fn next_byte(&self) -> Result<u8> {
        if self.pos + 1 >= self.text.len() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text.as_bytes()[self.pos + 1])
    }

    /// Returns a char from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    pub fn curr_char(&self) -> Result<char> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_char_unchecked())
    }

    #[inline]
    fn curr_char_unchecked(&self) -> char {
        self.text[self.pos..].chars().next().unwrap()
    }

    /// Advances by `n` bytes.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use svgtypes::Stream;
    ///
    /// let mut s = Stream::from("text");
    /// s.advance(2); // ok
    /// s.advance(20); // will cause a panic via debug_assert!().
    /// ```
    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.text.len());
        self.pos += n;
    }

    /// Skips whitespaces.
    ///
    /// Accepted values: `' ' \n \r \t`.
    pub fn skip_spaces(&mut self) {
        while !self.at_end() {
            if self.curr_byte_unchecked().is_xml_space() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    /// Checks that the stream starts with a selected text.
    ///
    /// We are using `&[u8]` instead of `&str` for performance reasons.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::Stream;
    ///
    /// let mut s = Stream::from("Some text.");
    /// s.advance(5);
    /// assert_eq!(s.starts_with(b"text"), true);
    /// assert_eq!(s.starts_with(b"long"), false);
    /// ```
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.text.as_bytes()[self.pos..].starts_with(text)
    }

    /// Checks if the stream is starts with a space.
    ///
    /// Uses [`skip_spaces()`](#method.curr_byte) internally.
    pub fn starts_with_space(&self) -> bool {
        if self.at_end() {
            return false;
        }

        let mut is_space = false;

        let c = self.curr_byte_unchecked();

        if c.is_xml_space() {
            is_space = true;
        }

        is_space
    }

    /// Consumes current byte if it's equal to the provided byte.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::Stream;
    ///
    /// let mut s = Stream::from("Some text.");
    /// s.consume_byte(b'S').unwrap();
    /// s.consume_byte(b'o').unwrap();
    /// s.consume_byte(b'm').unwrap();
    /// // s.consume_byte(b'q').unwrap(); // will produce an error
    /// ```
    pub fn consume_byte(&mut self, c: u8) -> Result<()> {
        if self.curr_byte()? != c {
            return Err(
                Error::InvalidChar(
                    vec![self.curr_byte_unchecked(), c],
                    self.calc_char_pos(),
                )
            );
        }

        self.advance(1);
        Ok(())
    }

    /// Consumes selected string.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    pub fn skip_string(&mut self, text: &[u8]) -> Result<()> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        if !self.starts_with(text) {
            let len = cmp::min(text.len(), self.text.len() - self.pos);
            // Collect chars and do not slice a string,
            // because the `len` can be on the char boundary.
            // Which lead to a panic.
            let actual = self.text[self.pos..].chars().take(len).collect();

            // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
            let expected = str::from_utf8(text).unwrap().to_owned();

            return Err(Error::InvalidString(vec![actual, expected], self.calc_char_pos()));
        }

        self.advance(text.len());
        Ok(())
    }

    /// Consumes bytes by the predicate and returns them.
    ///
    /// The result can be empty.
    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
        where F: Fn(&Stream, u8) -> bool
    {
        let start = self.pos();
        self.skip_bytes(f);
        self.slice_back(start)
    }

    /// Consumes bytes by the predicate.
    pub fn skip_bytes<F>(&mut self, f: F)
        where F: Fn(&Stream, u8) -> bool
    {
        while !self.at_end() {
            let c = self.curr_byte_unchecked();
            if f(self, c) {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    /// Consumes bytes by the predicate and returns them.
    pub fn consume_ident(&mut self) -> &'a str {
        let start = self.pos;
        self.skip_bytes(|_, c| c.is_ident_char());
        self.slice_back(start)
    }

    /// Slices data from `pos` to the current position.
    pub fn slice_back(&self, pos: usize) -> &'a str {
        &self.text[pos..self.pos]
    }

    /// Slices data from the current position to the end.
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..]
    }

    /// Parses number from the stream.
    ///
    /// This method will detect a number length and then
    /// will pass a substring to the `std::from_str` method.
    ///
    /// <https://www.w3.org/TR/SVG11/types.html#DataTypeNumber>
    ///
    /// # Errors
    ///
    /// Returns only `InvalidNumber`.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::Stream;
    ///
    /// let mut s = Stream::from("3.14");
    /// assert_eq!(s.parse_number().unwrap(), 3.14);
    /// assert_eq!(s.at_end(), true);
    /// ```
    pub fn parse_number(&mut self) -> Result<f64> {
        // strip off leading blanks
        self.skip_spaces();

        if self.at_end() {
            // empty string
            return Err(Error::InvalidNumber(self.calc_char_pos()));
        }

        let start = self.pos();

        macro_rules! gen_err {
            () => ({
                Err(Error::InvalidNumber(self.calc_char_pos_at(start)))
            })
        }

        // consume sign
        if let Some(c) = self.get_curr_byte() {
            if c == b'+' || c == b'-' {
                self.advance(1);
            }
        }

        // consume integer
        if let Some(c) = self.get_curr_byte() {
            // current char must be a digit or a dot
            if c.is_xml_digit() {
                self.skip_digits();
            } else if c != b'.' {
                return gen_err!();
            }
        } else {
            return gen_err!();
        }

        // consume fraction
        if let Some(mut c) = self.get_curr_byte() {
            // current char must be a dot or an exponent sign
            if c == b'.' {
                self.advance(1); // skip dot
                self.skip_digits();
                if let Some(c2) = self.get_curr_byte() {
                    // Could have an exponent component.
                    c = c2;
                }
            }

            // TODO: extremely slow for no reason
            if c == b'e' || c == b'E' {
                let c2 = if let Ok(c2) = self.next_byte() {
                    c2
                } else {
                    return gen_err!();
                };

                if c2 != b'm' && c2 != b'x' {
                    self.advance(1); // skip 'e'

                    if let Some(c) = self.get_curr_byte() {
                        if c == b'+' || c == b'-' {
                            self.advance(1); // skip sign
                            self.skip_digits();
                        } else if c.is_xml_digit() {
                            self.skip_digits();
                        } else {
                            // TODO: error
                        }
                    }
                }
            }
        }

        let s = self.slice_back(start);

        // use default f64 parser now
        let r = f64::from_str(s);
        if let Ok(n) = r {
            // inf, nan, etc. are an error
            if n.is_finite() {
                return Ok(n);
            }
        }

        gen_err!()
    }

    /// Parses number from the list of numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::Stream;
    ///
    /// let mut s = Stream::from("3.14, 12,5 , 20-4");
    /// assert_eq!(s.parse_list_number().unwrap(), 3.14);
    /// assert_eq!(s.parse_list_number().unwrap(), 12.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 5.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 20.0);
    /// assert_eq!(s.parse_list_number().unwrap(), -4.0);
    /// ```
    pub fn parse_list_number(&mut self) -> Result<f64> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream.into());
        }

        let n = self.parse_number()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(n)
    }

    /// Parses integer number from the stream.
    ///
    /// Same as [`parse_number()`], but only for integer. Does not refer to any SVG type.
    ///
    /// [`parse_number()`]: #method.parse_number
    pub fn parse_integer(&mut self) -> Result<i32> {
        self.skip_spaces();

        if self.at_end() {
            return Err(Error::InvalidNumber(self.calc_char_pos()));
        }

        let start = self.pos();

        // consume sign
        match self.curr_byte()? {
            b'+' | b'-' => self.advance(1),
            _ => {}
        }

        // current char must be a digit
        if !self.curr_byte()?.is_xml_digit() {
            return Err(Error::InvalidNumber(self.calc_char_pos_at(start)));
        }

        self.skip_digits();

        // use default i32 parser now
        let s = self.slice_back(start);
        match i32::from_str(s) {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::InvalidNumber(self.calc_char_pos_at(start))),
        }
    }

    /// Parses integer from the list of numbers.
    pub fn parse_list_integer(&mut self) -> Result<i32> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream.into());
        }

        let n = self.parse_integer()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(n)
    }

    /// Parses length from the stream.
    ///
    /// <https://www.w3.org/TR/SVG11/types.html#DataTypeLength>
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{Stream, Length, LengthUnit};
    ///
    /// let mut s = Stream::from("30%");
    /// assert_eq!(s.parse_length().unwrap(), Length::new(30.0, LengthUnit::Percent));
    /// ```
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    pub fn parse_length(&mut self) -> Result<Length> {
        self.skip_spaces();

        let n = self.parse_number()?;

        if self.at_end() {
            return Ok(Length::new(n, LengthUnit::None));
        }

        let u = if self.starts_with(b"%") {
            LengthUnit::Percent
        } else if self.starts_with(b"em") {
            LengthUnit::Em
        } else if self.starts_with(b"ex") {
            LengthUnit::Ex
        } else if self.starts_with(b"px") {
            LengthUnit::Px
        } else if self.starts_with(b"in") {
            LengthUnit::In
        } else if self.starts_with(b"cm") {
            LengthUnit::Cm
        } else if self.starts_with(b"mm") {
            LengthUnit::Mm
        } else if self.starts_with(b"pt") {
            LengthUnit::Pt
        } else if self.starts_with(b"pc") {
            LengthUnit::Pc
        } else {
            LengthUnit::None
        };

        match u {
            LengthUnit::Percent => self.advance(1),
            LengthUnit::None => {}
            _ => self.advance(2),
        }

        Ok(Length::new(n, u))
    }

    /// Parses length from the list of lengths.
    pub fn parse_list_length(&mut self) -> Result<Length> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream.into());
        }

        let l = self.parse_length()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(l)
    }

    /// Parses angle from the stream.
    ///
    /// <https://www.w3.org/TR/SVG11/types.html#DataTypeAngle>
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    pub fn parse_angle(&mut self) -> Result<Angle> {
        self.skip_spaces();

        let n = self.parse_number()?;

        if self.at_end() {
            return Ok(Angle::new(n, AngleUnit::Degrees));
        }

        let u = if self.starts_with(b"deg") {
            self.advance(3);
            AngleUnit::Degrees
        } else if self.starts_with(b"grad") {
            self.advance(4);
            AngleUnit::Gradians
        } else if self.starts_with(b"rad") {
            self.advance(3);
            AngleUnit::Radians
        } else {
            AngleUnit::Degrees
        };

        Ok(Angle::new(n, u))
    }

    /// Skips digits.
    pub fn skip_digits(&mut self) {
        self.skip_bytes(|_, c| c.is_xml_digit());
    }

    /// Parses a [IRI].
    ///
    /// By the SVG spec, the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// [IRI]: https://www.w3.org/TR/SVG11/types.html#DataTypeIRI
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    pub fn parse_iri(&mut self) -> Result<&'a str> {
        let mut _impl = || -> Result<&'a str> {
            self.skip_spaces();
            self.consume_byte(b'#')?;
            let link = self.consume_bytes(|_, c| c != b' ');
            if !link.is_empty() {
                Ok(link)
            } else {
                Err(Error::InvalidValue)
            }
        };

        _impl().map_err(|_| Error::InvalidValue)
    }

    /// Parses a [FuncIRI].
    ///
    /// By the SVG spec, the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// [FuncIRI]: https://www.w3.org/TR/SVG11/types.html#DataTypeFuncIRI
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    pub fn parse_func_iri(&mut self) -> Result<&'a str> {
        let mut _impl = || -> Result<&'a str> {
            self.skip_spaces();
            self.skip_string(b"url(")?;
            self.skip_spaces();
            self.consume_byte(b'#')?;
            let link = self.consume_bytes(|_, c| c != b' ' && c != b')');
            self.skip_spaces();
            self.consume_byte(b')')?;

            if !link.is_empty() {
                Ok(link)
            } else {
                Err(Error::InvalidValue)
            }
        };

        _impl().map_err(|_| Error::InvalidValue)
    }
}

#[inline]
fn parse_list_separator(s: &mut Stream) {
    if s.is_curr_byte_eq(b',') {
        s.advance(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_1() {
        let mut s = Stream::from("10");
        assert_eq!(s.parse_integer().unwrap(), 10);
    }

    #[test]
    fn parse_err_integer_1() {
        // error because of overflow
        let mut s = Stream::from("10000000000000");
        assert_eq!(s.parse_integer().unwrap_err().to_string(),
                   "invalid number at position 1");
    }

    #[test]
    fn parse_length_1() {
        let mut s = Stream::from("1,");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn parse_length_2() {
        let mut s = Stream::from("1 ,");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn parse_length_3() {
        let mut s = Stream::from("1 1");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn parse_iri_1() {
        assert_eq!(Stream::from("#id").parse_iri().unwrap(), "id");
    }

    #[test]
    fn parse_iri_2() {
        assert_eq!(Stream::from("   #id   ").parse_iri().unwrap(), "id");
    }

    #[test]
    fn parse_iri_3() {
        assert_eq!(Stream::from("   #id   text").parse_iri().unwrap(), "id");
    }

    #[test]
    fn parse_iri_4() {
        assert_eq!(Stream::from("#1").parse_iri().unwrap(), "1");
    }

    #[test]
    fn parse_err_iri_1() {
        assert_eq!(Stream::from("# id").parse_iri().unwrap_err().to_string(),
                   "invalid value");
    }

    #[test]
    fn parse_func_iri_1() {
        assert_eq!(Stream::from("url(#id)").parse_func_iri().unwrap(), "id");
    }

    #[test]
    fn parse_func_iri_2() {
        assert_eq!(Stream::from("url(#1)").parse_func_iri().unwrap(), "1");
    }

    #[test]
    fn parse_func_iri_3() {
        assert_eq!(Stream::from("    url(    #id    )   ").parse_func_iri().unwrap(), "id");
    }

    #[test]
    fn parse_err_func_iri_1() {
        assert_eq!(Stream::from("url ( #1 )").parse_func_iri().unwrap_err().to_string(),
                   "invalid value");
    }

    #[test]
    fn parse_err_func_iri_2() {
        assert_eq!(Stream::from("url(#)").parse_func_iri().unwrap_err().to_string(),
                   "invalid value");
    }

    #[test]
    fn parse_err_func_iri_3() {
        assert_eq!(Stream::from("url(# id)").parse_func_iri().unwrap_err().to_string(),
                   "invalid value");
    }
}
