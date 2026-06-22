// Copyright 2024 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! CSS `light-dark()` function parsing.
//!
//! The [`light-dark()`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/light-dark)
//! CSS function enables setting two colors for a property – returning one of the two colors options
//! by detecting if the developer has set a light or dark color scheme.

use alloc::borrow::Cow;
use alloc::string::String;

/// Color scheme preference for resolving `light-dark()` CSS function.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ColorScheme {
    /// Light color scheme - uses the first value in `light-dark(light, dark)`.
    #[default]
    Light,
    /// Dark color scheme - uses the second value in `light-dark(light, dark)`.
    Dark,
}

/// Resolves CSS `light-dark(value1, value2)` function based on the specified color scheme.
///
/// The `light-dark()` CSS function enables setting two values for a property - returning one
/// of the two options based on whether a light or dark color scheme is preferred.
///
/// This function handles:
/// - Nested parentheses (e.g., `light-dark(rgb(0, 0, 0), rgb(255, 255, 255))`)
/// - Recursive `light-dark()` calls
/// - Values with surrounding content (e.g., `fill: light-dark(red, blue) !important`)
///
/// # Arguments
///
/// * `value` - The CSS value that may contain `light-dark()` function
/// * `color_scheme` - The color scheme preference to use
///
/// # Returns
///
/// Returns a `Cow<str>` - borrowed if no `light-dark()` was found, owned if it was resolved.
///
/// # Examples
///
/// ```
/// use svgtypes::{ColorScheme, resolve_light_dark};
///
/// // Light mode extracts first value
/// assert_eq!(resolve_light_dark("light-dark(red, blue)", ColorScheme::Light).as_ref(), "red");
///
/// // Dark mode extracts second value
/// assert_eq!(resolve_light_dark("light-dark(red, blue)", ColorScheme::Dark).as_ref(), "blue");
///
/// // Handles nested functions
/// assert_eq!(
///     resolve_light_dark("light-dark(rgb(0, 0, 0), rgb(255, 255, 255))", ColorScheme::Light).as_ref(),
///     "rgb(0, 0, 0)"
/// );
///
/// // Returns unchanged if no light-dark()
/// assert_eq!(resolve_light_dark("red", ColorScheme::Dark).as_ref(), "red");
/// ```
pub fn resolve_light_dark(value: &str, color_scheme: ColorScheme) -> Cow<'_, str> {
    let Some(start_idx) = value.find("light-dark(") else {
        return Cow::Borrowed(value);
    };

    let func_start = start_idx + "light-dark(".len();
    let rest = &value[func_start..];

    // Find both arguments by tracking parentheses depth
    let mut depth = 1;
    let mut first_arg_end = None;
    let mut second_arg_start = None;
    let mut func_end = None;

    for (i, c) in rest.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    func_end = Some(i);
                    if first_arg_end.is_none() {
                        first_arg_end = Some(i);
                    }
                    break;
                }
            }
            ',' if depth == 1 && first_arg_end.is_none() => {
                first_arg_end = Some(i);
                second_arg_start = Some(i + 1);
            }
            _ => {}
        }
    }

    let Some(first_arg_end) = first_arg_end else {
        return Cow::Borrowed(value);
    };
    let func_end = func_end.unwrap_or(rest.len());

    // Select the appropriate argument based on color scheme
    let selected_arg = match color_scheme {
        ColorScheme::Light => rest[..first_arg_end].trim(),
        ColorScheme::Dark => {
            if let Some(start) = second_arg_start {
                rest[start..func_end].trim()
            } else {
                // No second argument, fall back to first
                rest[..first_arg_end].trim()
            }
        }
    };

    // Reconstruct the value with light-dark() replaced by the selected argument
    let mut result = String::with_capacity(value.len());
    result.push_str(&value[..start_idx]);
    result.push_str(selected_arg);
    // Append any remaining content after the closing parenthesis
    if func_end + 1 < rest.len() {
        result.push_str(&rest[func_end + 1..]);
    }

    // Recursively resolve any remaining light-dark() calls
    match resolve_light_dark(&result, color_scheme) {
        Cow::Borrowed(_) => Cow::Owned(result),
        Cow::Owned(s) => Cow::Owned(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_scheme_simple() {
        assert_eq!(
            resolve_light_dark("light-dark(red, blue)", ColorScheme::Light).as_ref(),
            "red"
        );
    }

    #[test]
    fn dark_scheme_simple() {
        assert_eq!(
            resolve_light_dark("light-dark(red, blue)", ColorScheme::Dark).as_ref(),
            "blue"
        );
    }

    #[test]
    fn no_light_dark() {
        assert_eq!(
            resolve_light_dark("red", ColorScheme::Light).as_ref(),
            "red"
        );
        assert_eq!(resolve_light_dark("red", ColorScheme::Dark).as_ref(), "red");
    }

    #[test]
    fn nested_rgb_light() {
        assert_eq!(
            resolve_light_dark(
                "light-dark(rgb(0, 0, 0), rgb(255, 255, 255))",
                ColorScheme::Light
            )
            .as_ref(),
            "rgb(0, 0, 0)"
        );
    }

    #[test]
    fn nested_rgb_dark() {
        assert_eq!(
            resolve_light_dark(
                "light-dark(rgb(0, 0, 0), rgb(255, 255, 255))",
                ColorScheme::Dark
            )
            .as_ref(),
            "rgb(255, 255, 255)"
        );
    }

    #[test]
    fn surrounding_content() {
        assert_eq!(
            resolve_light_dark("fill: light-dark(red, blue) !important", ColorScheme::Light)
                .as_ref(),
            "fill: red !important"
        );
    }

    #[test]
    fn recursive_light_dark() {
        assert_eq!(
            resolve_light_dark(
                "light-dark(light-dark(a, b), light-dark(c, d))",
                ColorScheme::Light
            )
            .as_ref(),
            "a"
        );
        assert_eq!(
            resolve_light_dark(
                "light-dark(light-dark(a, b), light-dark(c, d))",
                ColorScheme::Dark
            )
            .as_ref(),
            "d"
        );
    }

    #[test]
    fn single_argument_fallback() {
        // If only one argument, use it for both schemes
        assert_eq!(
            resolve_light_dark("light-dark(red)", ColorScheme::Light).as_ref(),
            "red"
        );
        assert_eq!(
            resolve_light_dark("light-dark(red)", ColorScheme::Dark).as_ref(),
            "red"
        );
    }
}
