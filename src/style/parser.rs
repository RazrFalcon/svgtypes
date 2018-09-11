// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::str;

use xmlparser::{
    StreamError,
    Reference,
};

use {
    Error,
    Result,
    Stream,
    StrSpan,
};

// TODO: prefix
// TODO: comment

/// Style token.
#[derive(PartialEq)]
pub enum StyleToken<'a> {
    /// Tuple contains attribute's name and value.
    Attribute(StrSpan<'a>, StrSpan<'a>),
    /// Tuple contains ENTITY reference. Just a name without `&` and `;`.
    EntityRef(&'a str),
}

impl<'a> fmt::Debug for StyleToken<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StyleToken::Attribute(name, value) =>
                write!(f, "SvgAttribute({:?}, {:?})", name, value),
            StyleToken::EntityRef(name) =>
                write!(f, "EntityRef({})", name),
        }
    }
}

/// A pull-based style parser.
///
/// # Errors
///
/// - Most of the `Error` types can occur.
///
/// # Notes
///
/// - By the SVG spec a `style` attribute can contain any style sheet language,
///   but the library only support CSS2, which is default.
/// - Objects with `-` prefix will be ignored since.
/// - All comments are automatically skipped.
///
/// # Example
///
/// ```rust
/// use svgtypes::{StyleParser, StyleToken};
///
/// let style = "/* comment */fill:red;";
/// let mut p = StyleParser::from(style);
/// if let StyleToken::Attribute(name, value) = p.next().unwrap().unwrap() {
///     assert_eq!(name.to_str(), "fill");
///     assert_eq!(value.to_str(), "red");
/// }
/// assert_eq!(p.next().is_none(), true);
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StyleParser<'a> {
    stream: Stream<'a>,
}

impl<'a> From<&'a str> for StyleParser<'a> {
    fn from(v: &'a str) -> Self {
        Self::from(StrSpan::from(v))
    }
}

impl<'a> From<StrSpan<'a>> for StyleParser<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        StyleParser {
            stream: Stream::from(span),
        }
    }
}

impl<'a> Iterator for StyleParser<'a> {
    type Item = Result<StyleToken<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.skip_spaces();

        if self.stream.at_end() {
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

        let c = try2!(self.stream.curr_byte());
        if c == b'/' {
            try2!(skip_comment(&mut self.stream));
            self.next()
        } else if c == b'-' {
            try2!(parse_prefix(&mut self.stream));
            self.next()
        } else if c == b'&' {
            Some(parse_entity_ref(&mut self.stream))
        } else if is_ident_char(c) {
            Some(parse_attribute(&mut self.stream))
        } else {
            // TODO: use custom error type
            let pos = self.stream.gen_error_pos();
            self.stream.jump_to_end();
            Some(Err(StreamError::InvalidChar(vec![c, b'/', b'-', b'&'], pos).into()))
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

fn parse_attribute<'a>(stream: &mut Stream<'a>) -> Result<StyleToken<'a>> {
    let name = stream.consume_bytes(|_, c| is_ident_char(c));

    if name.is_empty() {
        // TODO: this
        // The error type is irrelevant because we will ignore it anyway.
        return Err(StreamError::UnexpectedEndOfStream.into());
    }

    stream.skip_spaces();
    stream.consume_byte(b':')?;
    stream.skip_spaces();

    let value = if stream.curr_byte()? == b'\'' {
        stream.advance(1);
        let v = stream.consume_bytes(|_, c| c != b'\'');
        stream.consume_byte(b'\'')?;
        v
    } else if stream.starts_with(b"&apos;") {
        stream.advance(6);
        let v = stream.consume_bytes(|_, c| c != b'&');
        stream.skip_string(b"&apos;")?;
        v
    } else {
        stream.consume_bytes(|_, c| c != b';' && c != b'/')
    }.trim();

    if value.len() == 0 {
        return Err(StreamError::UnexpectedEndOfStream.into());
    }

    stream.skip_spaces();

    // ';;;' is valid style data, we need to skip it
    while stream.is_curr_byte_eq(b';') {
        stream.advance(1);
        stream.skip_spaces();
    }

    Ok(StyleToken::Attribute(name, value))
}

fn parse_entity_ref<'a>(stream: &mut Stream<'a>) -> Result<StyleToken<'a>> {
    match stream.consume_reference()? {
        Reference::EntityRef(name) => {
            Ok(StyleToken::EntityRef(name))
        }
        Reference::CharRef(_) => {
            // TODO: wrong, should be parsed as a string
            Err(Error::InvalidEntityRef(stream.gen_error_pos()))
        }
    }
}

fn parse_prefix(stream: &mut Stream) -> Result<()> {
    // prefixed attributes are not supported, aka '-webkit-*'

    stream.advance(1); // -
    let t = parse_attribute(stream)?;

    if let StyleToken::Attribute(name, _) = t {
        warn!("Style attribute '-{}' is skipped.", name);
    }

    Ok(())
}

// TODO: to xmlparser traits
fn is_ident_char(c: u8) -> bool {
    match c {
        b'0'...b'9'
        | b'A'...b'Z'
        | b'a'...b'z'
        | b'-'
        | b'_' => true,
        _ => false,
    }
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
                    match s.next().unwrap().unwrap() {
                        StyleToken::Attribute(name, value) => {
                            assert_eq!(name.to_str(), $aname);
                            assert_eq!(value.to_str(), $avalue);
                        },
                        _ => unreachable!(),
                    }
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

    test!(parse_7, "font-family:&apos;Verdana&apos;",
        ("font-family", "Verdana")
    );

    test!(parse_8, "  fill  :  none  ",
        ("fill", "none")
    );

    #[test]
    fn parse_9() {
        let mut s = StyleParser::from("&st0; &st1;");
        assert_eq!(s.next().unwrap().unwrap(), StyleToken::EntityRef("st0"));
        assert_eq!(s.next().unwrap().unwrap(), StyleToken::EntityRef("st1"));
        assert_eq!(s.next().is_none(), true);
    }

    test!(parse_10, "/**/", );

    test!(parse_11, "font-family:Cantarell;-inkscape-font-specification:&apos;Cantarell Bold&apos;",
        ("font-family", "Cantarell")
    );

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
                   "expected '/', '-', '&' not ':' at 1:1");
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
                   "invalid reference");
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
                   "invalid reference");
    }

    #[test]
    fn parse_err_6() {
        let mut s = StyleParser::from("{");
        assert_eq!(s.next().unwrap().unwrap_err().to_string(),
                   "expected '/', '-', '&' not '{' at 1:1");
    }
}
