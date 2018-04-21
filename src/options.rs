// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


/// A separator type for a list of values.
///
/// <https://www.w3.org/TR/SVG/types.html#DataTypeList>
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ListSeparator {
    /// `10,20`
    Comma,
    /// `10 20`
    Space,
    /// `10, 20`
    CommaSpace,
}

/// Options for SVG types writing.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WriteOptions {
    /// Use #RGB color notation when possible.
    ///
    /// By default all colors written using #RRGGBB notation.
    ///
    /// # Examples
    ///
    /// `#ff0000` -> `#f00`, `#000000` -> `#000`, `#00aa00` -> `#0a0`
    ///
    /// Default: disabled
    pub trim_hex_colors: bool,

    /// Remove leading zero from numbers.
    ///
    /// # Examples
    ///
    /// - `0.1` -> `.1`
    /// - `-0.1` -> `-.1`
    ///
    /// Default: disabled
    pub remove_leading_zero: bool,

    /// Use compact path notation.
    ///
    /// SVG allow us to remove some symbols from path notation without breaking parsing.
    ///
    /// # Examples
    ///
    /// `M 10 -20 A 5.5 0.3 -4 1 1 0 -0.1` -> `M10-20A5.5.3-4 1 1 0-.1`
    ///
    /// Default: disabled
    pub use_compact_path_notation: bool,

    /// Join ArcTo flags.
    ///
    /// Elliptical arc curve segment has flags parameters, which can have values of `0` or `1`.
    /// Since we have fixed-width values, we can skip spaces between them.
    ///
    /// **Note:** Sadly, but most of the viewers doesn't support such notation,
    /// even though it's valid according to the SVG spec.
    ///
    /// # Examples
    ///
    /// `A 5 5 30 1 1 10 10` -> `A 5 5 30 1110 10`
    ///
    /// Default: disabled
    pub join_arc_to_flags: bool,

    /// Remove duplicated commands.
    ///
    /// If a segment has the same type as a previous then we can skip command specifier.
    ///
    /// # Examples
    ///
    /// `M 10 10 L 20 20 L 30 30 L 40 40` -> `M 10 10 L 20 20 30 30 40 40`
    ///
    /// Default: disabled
    pub remove_duplicated_path_commands: bool,

    /// Use implicit LineTo commands.
    ///
    /// 'If a MoveTo is followed by multiple pairs of coordinates,
    /// the subsequent pairs are treated as implicit LineTo commands.'
    ///
    /// # Examples
    ///
    /// `M 10 10 L 20 20 L 30 30` -> `M 10 10 20 20 30 30`
    ///
    /// Default: disabled
    pub use_implicit_lineto_commands: bool,

    /// Simplify transform matrices into short equivalent when possible.
    ///
    /// If not set - all transform will be saved as 'matrix'.
    ///
    /// # Examples
    ///
    /// ```text
    /// matrix(1 0 0 1 10 20) -> translate(10 20)
    /// matrix(1 0 0 1 10 0)  -> translate(10)
    /// matrix(2 0 0 3 0 0)   -> scale(2 3)
    /// matrix(2 0 0 2 0 0)   -> scale(2)
    /// matrix(0 1 -1 0 0 0)  -> rotate(-90)
    /// ```
    ///
    /// Default: disabled
    pub simplify_transform_matrices: bool,

    /// Set the separator type for list types.
    ///
    /// Affects `Points`, `LengthList`, `NumberList` and `Transform`.
    ///
    /// Default: `ListSeparator::Space`
    pub list_separator: ListSeparator,
}

impl Default for WriteOptions {
    fn default() -> WriteOptions {
        WriteOptions {
            trim_hex_colors: false,
            remove_leading_zero: false,
            use_compact_path_notation: false,
            join_arc_to_flags: false,
            remove_duplicated_path_commands: false,
            use_implicit_lineto_commands: false,
            simplify_transform_matrices: false,
            list_separator: ListSeparator::Space,
        }
    }
}

impl WriteOptions {
    /// Writes a selected separator to the output buffer.
    ///
    /// Uses `WriteOptions::list_separator` option.
    pub fn write_separator(&self, out: &mut Vec<u8>) {
        match self.list_separator {
            ListSeparator::Space => out.push(b' '),
            ListSeparator::Comma => out.push(b','),
            ListSeparator::CommaSpace => out.extend_from_slice(b", "),
        }
    }
}
