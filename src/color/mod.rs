mod colors;
mod parser;
mod writer;

/// Representation of the [`<color>`] type.
///
/// [`<color>`]: https://www.w3.org/TR/css-color-3/
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    /// Constructs a new `Color` from RGB values.
    #[inline]
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue, alpha: 255 }
    }

    /// Constructs a new `Color` from RGBA values.
    #[inline]
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color { red, green, blue, alpha }
    }

    /// Constructs a new `Color` set to black.
    #[inline]
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// Constructs a new `Color` set to white.
    #[inline]
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }

    /// Constructs a new `Color` set to gray.
    #[inline]
    pub fn gray() -> Color {
        Color::new_rgb(128, 128, 128)
    }

    /// Constructs a new `Color` set to red.
    #[inline]
    pub fn red() -> Color {
        Color::new_rgb(255, 0, 0)
    }

    /// Constructs a new `Color` set to green.
    #[inline]
    pub fn green() -> Color {
        Color::new_rgb(0, 128, 0)
    }

    /// Constructs a new `Color` set to blue.
    #[inline]
    pub fn blue() -> Color {
        Color::new_rgb(0, 0, 255)
    }
}
