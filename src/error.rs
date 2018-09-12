// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::error;

use xmlparser;

use TextPos;

// TODO: should all errors have a pos?

/// List of all errors.
#[derive(Debug)]
pub enum Error {
    /// An invalid color.
    InvalidColor(TextPos),

    /// An invalid number.
    InvalidNumber(TextPos),

    /// An invalid transform prefix.
    InvalidTransformPrefix(TextPos),

    /// An invalid align type.
    InvalidAlignType(String),

    /// An invalid align slice.
    InvalidAlignSlice(String),

    /// An invalid IRI value.
    InvalidIRI,

    /// An invalid FuncIRI value.
    InvalidFuncIRI,

    /// An invalid paint type.
    InvalidPaint,

    /// A viewBox with a negative or zero size.
    InvalidViewbox,

    // TODO: remove
    /// An XML stream error.
    XmlError(xmlparser::StreamError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidColor(pos) => {
                write!(f, "invalid color at {}", pos)
            }
            Error::InvalidNumber(pos) => {
                write!(f, "invalid number at {}", pos)
            }
            Error::InvalidTransformPrefix(pos) => {
                write!(f, "invalid transform prefix at {}", pos)
            }
            Error::InvalidAlignType(ref kind) => {
                write!(f, "'{}' is an invalid align type", kind)
            }
            Error::InvalidAlignSlice(ref kind) => {
                write!(f, "expected 'meet' or 'slice' not '{}'", kind)
            }
            Error::InvalidIRI => {
                write!(f, "invalid IRI")
            }
            Error::InvalidFuncIRI => {
                write!(f, "invalid FuncIRI")
            }
            Error::InvalidPaint => {
                write!(f, "invalid paint value")
            }
            Error::InvalidViewbox => {
                write!(f, "viewBox should have a positive size")
            }
            Error::XmlError(ref e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "an SVG data parsing error"
    }
}

impl From<xmlparser::StreamError> for Error {
    fn from(v: xmlparser::StreamError) -> Self {
        Error::XmlError(v)
    }
}

/// An alias to `Result<T, Error>`.
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
