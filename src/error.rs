// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xmlparser;

use ErrorPos;

/// List of all errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// An invalid color.
    #[fail(display = "invalid color at {}", _0)]
    InvalidColor(ErrorPos),

    /// An invalid number.
    #[fail(display = "invalid number at {}", _0)]
    InvalidNumber(ErrorPos),

    /// An invalid entity reference.
    #[fail(display = "invalid entity reference at {}", _0)]
    InvalidEntityRef(ErrorPos),

    /// An invalid transform prefix.
    #[fail(display = "invalid transform prefix at {}", _0)]
    InvalidTransformPrefix(ErrorPos),

    /// An invalid align type.
    #[fail(display = "'{}' is an invalid align type", _0)]
    InvalidAlignType(String),

    /// An invalid align slice.
    #[fail(display = "expected 'meet' or 'slice' not '{}'", _0)]
    InvalidAlignSlice(String),

    /// An invalid IRI value.
    #[fail(display = "invalid IRI")]
    InvalidIRI,

    /// An invalid FuncIRI value.
    #[fail(display = "invalid FuncIRI")]
    InvalidFuncIRI,

    /// An invalid paint type.
    #[fail(display = "invalid paint value")]
    InvalidPaint,

    /// A viewBox with a negative or zero size.
    #[fail(display = "viewBox should have a positive size")]
    InvalidViewbox,

    /// An XML stream error.
    #[fail(display = "{}", _0)]
    XmlError(xmlparser::StreamError),
}

impl From<xmlparser::StreamError> for Error {
    fn from(v: xmlparser::StreamError) -> Self {
        Error::XmlError(v)
    }
}

/// An alias to `Result<T, Error>`.
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
