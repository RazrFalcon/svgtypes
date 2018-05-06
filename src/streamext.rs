// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use xmlparser::{
    self,
    XmlByteExt,
};

use {
    Error,
    Length,
    LengthUnit,
    Result,
    Stream,
};


/// [`Stream`](struct.Stream.html) additional methods.
pub trait StreamExt<'a> {
    /// Parses number from the stream.
    ///
    /// This method will detect a number length and then
    /// will pass a substring to the `std::from_str` method.
    ///
    /// <https://www.w3.org/TR/SVG/types.html#DataTypeNumber>
    ///
    /// # Errors
    ///
    /// Returns only `InvalidNumber`.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{Stream, StreamExt};
    ///
    /// let mut s = Stream::from("3.14");
    /// assert_eq!(s.parse_number().unwrap(), 3.14);
    /// assert_eq!(s.at_end(), true);
    /// ```
    fn parse_number(&mut self) -> Result<f64>;

    /// Parses number from the list of numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{Stream, StreamExt};
    ///
    /// let mut s = Stream::from("3.14, 12,5 , 20-4");
    /// assert_eq!(s.parse_list_number().unwrap(), 3.14);
    /// assert_eq!(s.parse_list_number().unwrap(), 12.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 5.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 20.0);
    /// assert_eq!(s.parse_list_number().unwrap(), -4.0);
    /// ```
    fn parse_list_number(&mut self) -> Result<f64>;

    /// Parses integer number from the stream.
    ///
    /// Same as [`parse_number()`], but only for integer. Does not refer to any SVG type.
    ///
    /// [`parse_number()`]: #method.parse_number
    fn parse_integer(&mut self) -> Result<i32>;

    /// Parses integer from the list of numbers.
    fn parse_list_integer(&mut self) -> Result<i32>;

    /// Parses length from the stream.
    ///
    /// <https://www.w3.org/TR/SVG/types.html#DataTypeLength>
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{Stream, StreamExt, Length, LengthUnit};
    ///
    /// let mut s = Stream::from("30%");
    /// assert_eq!(s.parse_length().unwrap(), Length::new(30.0, LengthUnit::Percent));
    /// ```
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    fn parse_length(&mut self) -> Result<Length>;

    /// Parses length from the list of lengths.
    fn parse_list_length(&mut self) -> Result<Length>;

    /// Skips digits.
    fn skip_digits(&mut self);

    /// Parses a [IRI].
    ///
    /// By the SVG spec the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// [IRI]: https://www.w3.org/TR/SVG/types.html#DataTypeIRI
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    fn parse_iri(&mut self) -> Result<&'a str>;

    /// Parses a [FuncIRI].
    ///
    /// By the SVG spec the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// [FuncIRI]: https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    fn parse_func_iri(&mut self) -> Result<&'a str>;
}

impl<'a> StreamExt<'a> for Stream<'a> {
    fn parse_number(&mut self) -> Result<f64> {
        // strip off leading blanks
        self.skip_spaces();

        if self.at_end() {
            // empty string
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let start = self.pos();

        macro_rules! gen_err {
            () => ({
                Err(Error::InvalidNumber(self.gen_error_pos_from(start)))
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

        let s = self.slice_back(start).to_str();

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

    fn parse_list_number(&mut self) -> Result<f64> {
        if self.at_end() {
            return Err(xmlparser::StreamError::UnexpectedEndOfStream.into());
        }

        let n = self.parse_number()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(n)
    }

    fn parse_integer(&mut self) -> Result<i32> {
        self.skip_spaces();

        if self.at_end() {
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let start = self.pos();

        macro_rules! gen_err {
            () => ({
                Err(Error::InvalidNumber(self.gen_error_pos_from(start)))
            })
        }

        // consume sign
        match self.curr_byte()? {
            b'+' | b'-' => self.advance(1),
            _ => {}
        }

        // current char must be a digit
        if !self.curr_byte()?.is_xml_digit() {
            return gen_err!();
        }

        self.skip_digits();

        // use default i32 parser now
        let s = self.slice_back(start).to_str();
        match i32::from_str(s) {
            Ok(n) => Ok(n),
            Err(_) => gen_err!(),
        }
    }

    fn parse_list_integer(&mut self) -> Result<i32> {
        if self.at_end() {
            return Err(xmlparser::StreamError::UnexpectedEndOfStream.into());
        }

        let n = self.parse_integer()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(n)
    }

    fn parse_length(&mut self) -> Result<Length> {
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

    fn parse_list_length(&mut self) -> Result<Length> {
        if self.at_end() {
            return Err(xmlparser::StreamError::UnexpectedEndOfStream.into());
        }

        let l = self.parse_length()?;
        self.skip_spaces();
        parse_list_separator(self);
        Ok(l)
    }

    fn skip_digits(&mut self) {
        self.skip_bytes(|_, c| c.is_xml_digit());
    }

    fn parse_iri(&mut self) -> Result<&'a str> {
        let mut _impl = || -> Result<&'a str> {
            self.skip_spaces();
            self.consume_byte(b'#')?;
            let link = self.consume_bytes(|_, c| c != b' ').to_str();
            if !link.is_empty() {
                Ok(link)
            } else {
                Err(Error::InvalidIRI)
            }
        };

        _impl().map_err(|_| Error::InvalidIRI)
    }

    fn parse_func_iri(&mut self) -> Result<&'a str> {
        let mut _impl = || -> Result<&'a str> {
            self.skip_spaces();
            self.skip_string(b"url(")?;
            self.skip_spaces();
            self.consume_byte(b'#')?;
            let link = self.consume_bytes(|_, c| c != b' ' && c != b')').to_str();
            self.skip_spaces();
            self.consume_byte(b')')?;

            if !link.is_empty() {
                Ok(link)
            } else {
                Err(Error::InvalidFuncIRI)
            }
        };

        _impl().map_err(|_| Error::InvalidFuncIRI)
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
    fn integer_1() {
        let mut s = Stream::from("10");
        assert_eq!(s.parse_integer().unwrap(), 10);
    }

    #[test]
    fn integer_err_1() {
        // error because of overflow
        let mut s = Stream::from("10000000000000");
        assert_eq!(s.parse_integer().unwrap_err().to_string(),
                   "invalid number at 1:1");
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
                   "invalid IRI");
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
                   "invalid FuncIRI");
    }

    #[test]
    fn parse_err_func_iri_2() {
        assert_eq!(Stream::from("url(#)").parse_func_iri().unwrap_err().to_string(),
                   "invalid FuncIRI");
    }

    #[test]
    fn parse_err_func_iri_3() {
        assert_eq!(Stream::from("url(# id)").parse_func_iri().unwrap_err().to_string(),
                   "invalid FuncIRI");
    }
}
