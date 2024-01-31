use std::str::FromStr;

use crate::Error;

/// Extension methods for XML-subset only operations.
pub(crate) trait ByteExt {
    /// Checks if a byte is a numeric sign.
    fn is_sign(&self) -> bool;

    /// Checks if a byte is a digit.
    ///
    /// `[0-9]`
    fn is_digit(&self) -> bool;

    /// Checks if a byte is a hex digit.
    ///
    /// `[0-9A-Fa-f]`
    fn is_hex_digit(&self) -> bool;

    /// Checks if a byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_space(&self) -> bool;

    /// Checks if a byte is a space.
    ///
    /// `[\r\n]`
    fn is_newline(&self) -> bool;

    /// Checks if a byte is an ASCII char.
    ///
    /// `[A-Za-z]`
    fn is_letter(&self) -> bool;
}

impl ByteExt for u8 {
    #[inline]
    fn is_sign(&self) -> bool {
        matches!(*self, b'+' | b'-')
    }

    #[inline]
    fn is_digit(&self) -> bool {
        matches!(*self, b'0'..=b'9')
    }

    #[inline]
    fn is_hex_digit(&self) -> bool {
        matches!(*self, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')
    }

    #[inline]
    fn is_space(&self) -> bool {
        matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
    }

    #[inline]
    fn is_newline(&self) -> bool {
        matches!(*self, b'\n' | b'\r')
    }

    #[inline]
    fn is_letter(&self) -> bool {
        matches!(*self, b'A'..=b'Z' | b'a'..=b'z')
    }
}

/// A streaming text parsing interface.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stream<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        Stream { text, pos: 0 }
    }
}

impl<'a> Stream<'a> {
    /// Returns the current position in bytes.
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn curr_byte(&self) -> Result<u8, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    #[inline]
    pub fn curr_char(&self) -> Result<char, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text[self.pos..].chars().next().unwrap())
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

    /// Returns a next byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    #[inline]
    pub fn next_byte(&self) -> Result<u8, Error> {
        if self.pos + 1 >= self.text.len() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text.as_bytes()[self.pos + 1])
    }

    /// Advances by `n` bytes.
    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.text.len());
        self.pos += n;
    }

    /// Skips whitespaces.
    ///
    /// Accepted values: `' ' \n \r \t`.
    pub fn skip_spaces(&mut self) {
        while !self.at_end() && self.curr_byte_unchecked().is_space() {
            self.advance(1);
        }
    }

    /// Checks that the stream starts with a selected text.
    ///
    /// We are using `&[u8]` instead of `&str` for performance reasons.
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.text.as_bytes()[self.pos..].starts_with(text)
    }

    pub fn consume_char(&mut self) -> Result<char, Error> {
        let char = self.curr_char()?;
        self.advance(char.len_utf8());
        return Ok(char);
    }

    /// Consumes current byte if it's equal to the provided byte.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    pub fn consume_byte(&mut self, c: u8) -> Result<(), Error> {
        if self.curr_byte()? != c {
            return Err(Error::InvalidChar(
                vec![self.curr_byte_unchecked(), c],
                self.calc_char_pos(),
            ));
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
    pub fn consume_string(&mut self, text: &[u8]) -> Result<(), Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        if !self.starts_with(text) {
            let len = std::cmp::min(text.len(), self.text.len() - self.pos);
            // Collect chars and do not slice a string,
            // because the `len` can be on the char boundary.
            // Which lead to a panic.
            let actual = self.text[self.pos..].chars().take(len).collect();

            // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
            let expected = std::str::from_utf8(text).unwrap().to_owned();

            return Err(Error::InvalidString(
                vec![actual, expected],
                self.calc_char_pos(),
            ));
        }

        self.advance(text.len());
        Ok(())
    }

    /// Consumes bytes by the predicate and returns them.
    ///
    /// The result can be empty.
    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(&Stream, u8) -> bool,
    {
        let start = self.pos();
        self.skip_bytes(f);
        self.slice_back(start)
    }

    /// Consumes bytes by the predicate.
    pub fn skip_bytes<F>(&mut self, f: F)
    where
        F: Fn(&Stream, u8) -> bool,
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

    pub fn parse_escape(&mut self) -> Result<char, Error> {
        let mut processed_char = None;

        if let Ok(b'\\') = self.curr_byte() {
            if self.next_byte()?.is_newline() {
                return Err(Error::InvalidValue);
            }

            self.advance(1);

            if self.curr_byte()?.is_ascii_hexdigit() {
                let mut escape_sequence = String::new();
                let mut counter = 0;

                while let Ok(c) = self.curr_byte() {
                    if c.is_hex_digit() {
                        escape_sequence.push(self.curr_byte()? as char);
                        self.advance(1);

                        counter += 1;

                        if counter == 6 {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if let Ok(num) = u32::from_str_radix(&escape_sequence, 16) {
                    processed_char = Some(char::from_u32(num).ok_or(Error::InvalidValue)?);
                }
            } else {
                processed_char = Some(self.consume_char()?);
            }
        } else {
            return Err(Error::InvalidValue);
        }

        if let Ok(b) = self.curr_byte() {
            if b.is_space() {
                self.advance(1);
            }
        }

        Ok(processed_char.ok_or(Error::InvalidValue)?)
    }

    /// Parse an ident
    /// https://www.w3.org/TR/CSS21/syndata.html#value-def-identifier
    // TODO: return cow str
    pub fn parse_ident(&mut self) -> Result<String, Error> {
        self.skip_spaces();
        let mut ident = String::new();

        let first_char = self.curr_char()?;

        if first_char.is_ascii_alphabetic()
            || first_char == '_'
            || first_char == '-'
            || !first_char.is_ascii()
        {
            ident.push(self.consume_char()?);
        } else if first_char == '\\' {
            ident.push(self.parse_escape()?);
        } else {
            return Err(Error::InvalidValue);
        }

        while let Ok(ch) = self.curr_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || !first_char.is_ascii() {
                ident.push(self.consume_char()?);
            } else if ch == '\\' {
                println!("reached!");
                ident.push(self.parse_escape()?);
            } else {
                break;
            }
        }

        // Just a single hyphen is not allowed
        if ident == "-" {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(ident)
    }

    /// Slices data from `pos` to the current position.
    #[inline]
    pub fn slice_back(&self, pos: usize) -> &'a str {
        &self.text[pos..self.pos]
    }

    /// Slices data from the current position to the end.
    #[inline]
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..]
    }

    /// Parses integer number from the stream.
    ///
    /// Same as [`parse_number()`], but only for integer. Does not refer to any SVG type.
    ///
    /// [`parse_number()`]: #method.parse_number
    pub fn parse_integer(&mut self) -> Result<i32, Error> {
        self.skip_spaces();

        if self.at_end() {
            return Err(Error::InvalidNumber(self.calc_char_pos()));
        }

        let start = self.pos();

        // Consume sign.
        if self.curr_byte()?.is_sign() {
            self.advance(1);
        }

        // The current char must be a digit.
        if !self.curr_byte()?.is_digit() {
            return Err(Error::InvalidNumber(self.calc_char_pos_at(start)));
        }

        self.skip_digits();

        // Use the default i32 parser now.
        let s = self.slice_back(start);
        match i32::from_str(s) {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::InvalidNumber(self.calc_char_pos_at(start))),
        }
    }

    /// Parses integer from a list of numbers.
    pub fn parse_list_integer(&mut self) -> Result<i32, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        let n = self.parse_integer()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(n)
    }

    /// Parses number or percent from the stream.
    ///
    /// Percent value will be normalized.
    pub fn parse_number_or_percent(&mut self) -> Result<f64, Error> {
        self.skip_spaces();

        let n = self.parse_number()?;
        if self.starts_with(b"%") {
            self.advance(1);
            Ok(n / 100.0)
        } else {
            Ok(n)
        }
    }

    /// Parses number or percent from a list of numbers and/or percents.
    pub fn parse_list_number_or_percent(&mut self) -> Result<f64, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        let l = self.parse_number_or_percent()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(l)
    }

    /// Skips digits.
    pub fn skip_digits(&mut self) {
        self.skip_bytes(|_, c| c.is_digit());
    }

    #[inline]
    pub(crate) fn parse_list_separator(&mut self) {
        if self.is_curr_byte_eq(b',') {
            self.advance(1);
        }
    }
}

#[rustfmt::skip]
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

    macro_rules! parse_escape {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_escape().unwrap(), $result);
            }
        )
    }

    parse_escape!(escape_1, "\\\"", '"');
    parse_escape!(escape_2, "\\你", '你');
    parse_escape!(escape_3, "\\38", '8');
    parse_escape!(escape_4, "\\38 ", '8');
    parse_escape!(escape_5, "\\0038", '8');
    parse_escape!(escape_6, "\\000038", '8');
    parse_escape!(escape_7, "\\0038Hi", '8');
    parse_escape!(escape_8, "\\0038 45", '8');

    macro_rules! parse_escape_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_escape().unwrap_err(), $result);
            }
        )
    }

    parse_escape_err!(escape_err_1, "\\", Error::UnexpectedEndOfStream);
    parse_escape_err!(escape_err_2, "\\\n", Error::InvalidValue);

    macro_rules! parse_ident {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_ident().unwrap(), $result);
            }
        )
    }

    parse_ident!(ident_1, "_test", "_test");
    parse_ident!(ident_2, "_te-st", "_te-st");
    parse_ident!(ident_3, "te\\73 \\0074 ", "test");
    parse_ident!(ident_4, "   \\4F60 80abc   ", "你80abc");

    macro_rules! parse_ident_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_ident().unwrap_err(), $result);
            }
        )
    }

    parse_escape_err!(ident_err_1, "-", Error::InvalidValue);
    parse_escape_err!(ident_err_2, "8abc", Error::InvalidValue);
    //TODO
    //parse_escape_err!(ident_err_3, "\\38abc", Error::InvalidValue);
}
