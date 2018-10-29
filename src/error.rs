// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::error;

/// List of all errors.
#[derive(Debug)]
pub enum Error {
    /// An input data ended earlier than expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    UnexpectedEndOfStream,

    /// An input text contains unknown data.
    UnexpectedData(usize),

    /// A provided string doesn't have a valid data.
    ///
    /// For example, if we try to parse a color form `zzz`
    /// string - we will get this error.
    /// But if we try to parse a number list like `1.2 zzz`,
    /// then we will get `InvalidNumber`, because at least some data is valid.
    InvalidValue,

    /// An invalid/unexpected character.
    ///
    /// The first byte is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(Vec<u8>, usize),

    /// An unexpected character instead of an XML space.
    ///
    /// The first string is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidString(Vec<String>, usize),

    /// An invalid number.
    InvalidNumber(usize),

    /// A viewBox with a negative or zero size.
    InvalidViewbox,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedEndOfStream => {
                write!(f, "unexpected end of stream")
            }
            Error::UnexpectedData(pos) => {
                write!(f, "unexpected data at position {}", pos)
            }
            Error::InvalidValue => {
                write!(f, "invalid value")
            }
            Error::InvalidChar(ref chars, pos) => {
                // Vec<u8> -> Vec<String>
                let list: Vec<String> =
                    chars.iter().skip(1).map(|c| String::from_utf8(vec![*c]).unwrap()).collect();

                write!(f, "expected '{}' not '{}' at position {}",
                       list.join("', '"), chars[0] as char, pos)
            }
            Error::InvalidString(ref strings, pos) => {
                write!(f, "expected '{}' not '{}' at position {}",
                       strings[1..].join("', '"), strings[0], pos)
            }
            Error::InvalidNumber(pos) => {
                write!(f, "invalid number at position {}", pos)
            }
            Error::InvalidViewbox => {
                write!(f, "viewBox should have a positive size")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "an SVG data parsing error"
    }
}

/// An alias to `Result<T, Error>`.
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
