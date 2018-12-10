// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

use {
    Error,
    Result,
    Stream,
    XmlByteExt,
};

/// A pull-based style parser.
///
/// # Errors
///
/// - Most of the `Error` types can occur.
///
/// # Notes
///
/// - Entity references must be already resolved.
/// - By the SVG spec a `style` attribute can contain any style sheet language,
///   but the library only support CSS2, which is default.
/// - Objects with `-` prefix will be ignored.
/// - All comments are automatically skipped.
///
/// # Example
///
/// ```rust
/// use svgtypes::StyleParser;
///
/// let style = "/* comment */fill:red;";
/// let mut p = StyleParser::from(style);
/// let (name, value) = p.next().unwrap().unwrap();
/// assert_eq!(name, "fill");
/// assert_eq!(value, "red");
/// assert_eq!(p.next().is_none(), true);
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StyleParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for StyleParser<'a> {
    fn from(v: &'a str) -> Self {
        StyleParser(Stream::from(v))
    }
}

impl<'a> Iterator for StyleParser<'a> {
    type Item = Result<(&'a str, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.skip_spaces();

        if self.0.at_end() {
            return None;
        }

        macro_rules! try2 {
            ($expr:expr) => {
                match $expr {
                    Ok(value) => value,
                    Err(e) => {
                        return Some(Err(e.into()));
                    }
                }
            }
        }

        let c = try2!(self.0.curr_byte());
        if c == b'/' {
            try2!(skip_comment(&mut self.0));
            self.next()
        } else if c == b'-' {
            try2!(parse_prefix(&mut self.0));
            self.next()
        } else if c.is_ident_char() {
            Some(parse_attribute(&mut self.0))
        } else {
            // TODO: use custom error type
            let pos = self.0.calc_char_pos();
            self.0.jump_to_end();
            Some(Err(Error::InvalidChar(vec![c, b'/', b'-'], pos).into()))
        }
    }
}

fn skip_comment(stream: &mut Stream) -> Result<()> {
    stream.skip_string(b"/*")?;
    stream.skip_bytes(|_, c| c != b'*');
    stream.skip_string(b"*/")?;
    stream.skip_spaces();

    Ok(())
}

fn parse_attribute<'a>(stream: &mut Stream<'a>) -> Result<(&'a str, &'a str)> {
    let name = stream.consume_ident();

    if name.is_empty() {
        // TODO: this
        // The error type is irrelevant because we will ignore it anyway.
        return Err(Error::UnexpectedEndOfStream.into());
    }

    stream.skip_spaces();
    stream.consume_byte(b':')?;
    stream.skip_spaces();

    let value = if stream.curr_byte()? == b'\'' {
        stream.advance(1);
        let v = stream.consume_bytes(|_, c| c != b'\'');
        stream.consume_byte(b'\'')?;
        v
    } else {
        stream.consume_bytes(|_, c| c != b';' && c != b'/')
    }.trim();

    if value.len() == 0 {
        return Err(Error::UnexpectedEndOfStream.into());
    }

    stream.skip_spaces();

    // ';;;' is valid style data, we need to skip it
    while stream.is_curr_byte_eq(b';') {
        stream.advance(1);
        stream.skip_spaces();
    }

    Ok((name, value))
}

fn parse_prefix(stream: &mut Stream) -> Result<()> {
    // prefixed attributes are not supported, aka '-webkit-*'

    stream.advance(1); // -
    let _ = parse_attribute(stream)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $(($aname:expr, $avalue:expr)),*) => (
            #[test]
            fn $name() {
                let mut s = StyleParser::from($text);
                $(
                    let (name, value) = s.next().unwrap().unwrap();
                    assert_eq!(name, $aname);
                    assert_eq!(value, $avalue);
                )*

                assert_eq!(s.next().is_none(), true);
            }
        )
    }

    test!(parse_1, "fill:none; color:cyan; stroke-width:4.00",
        ("fill", "none"),
        ("color", "cyan"),
        ("stroke-width", "4.00")
    );

    test!(parse_2, "fill:none;",
        ("fill", "none")
    );

    test!(parse_3, "font-size:24px;font-family:'Arial Bold'",
        ("font-size", "24px"),
        ("font-family", "Arial Bold")
    );

    test!(parse_4, "font-size:24px; /* comment */ font-style:normal;",
        ("font-size", "24px"),
        ("font-style", "normal")
    );

    test!(parse_5, "font-size:24px;-font-style:normal;font-stretch:normal;",
        ("font-size", "24px"),
        ("font-stretch", "normal")
    );

    test!(parse_6, "fill:none;-webkit:hi",
        ("fill", "none")
    );

    test!(parse_8, "  fill  :  none  ",
        ("fill", "none")
    );

    test!(parse_10, "/**/", );

    // TODO: technically incorrect, because value with spaces should be quoted
    test!(parse_12, "font-family:Neue Frutiger 65",
        ("font-family", "Neue Frutiger 65")
    );

    test!(parse_13, "/*text*/fill:green/*text*/",
        ("fill", "green")
    );

    test!(parse_14, "  /*text*/ fill:green  /*text*/ ",
        ("fill", "green")
    );

    #[test]
    fn parse_err_1() {
        let mut s = StyleParser::from(":");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-' not ':' at position 1");
    }

    #[test]
    fn parse_err_2() {
        let mut s = StyleParser::from("name:'");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "unexpected end of stream");
    }

    #[test]
    fn parse_err_3() {
        let mut s = StyleParser::from("&\x0a96M*9");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-' not '&' at position 1");
    }

    #[test]
    fn parse_err_4() {
        let mut s = StyleParser::from("/*/**/");
        assert_eq!(s.next().unwrap().is_err(), true);
    }

    #[test]
    fn parse_err_5() {
        let mut s = StyleParser::from("&#x4B2ƿ  ;");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-' not '&' at position 1");
    }

    #[test]
    fn parse_err_6() {
        let mut s = StyleParser::from("{");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-' not '{' at position 1");
    }

    #[test]
    fn parse_err_7() {
        // Non-ASCII text and error pos.

        let mut s = StyleParser::from("fill:красный;&");
        s.next();
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-' not '&' at position 14");
    }
}
