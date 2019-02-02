mod colors;
mod parser;
mod writer;

/// Representation of the [`<color>`] type.
///
/// [`<color>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeColor
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Constructs a new `Color` from `red`, `green` and `blue` values.
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// Constructs a new `Color` set to black.
    pub fn black() -> Color {
        Color::new(0, 0, 0)
    }

    /// Constructs a new `Color` set to white.
    pub fn white() -> Color {
        Color::new(255, 255, 255)
    }

    /// Constructs a new `Color` set to gray.
    pub fn gray() -> Color {
        Color::new(128, 128, 128)
    }

    /// Constructs a new `Color` set to red.
    pub fn red() -> Color {
        Color::new(255, 0, 0)
    }

    /// Constructs a new `Color` set to green.
    pub fn green() -> Color {
        Color::new(0, 128, 0)
    }

    /// Constructs a new `Color` set to blue.
    pub fn blue() -> Color {
        Color::new(0, 0, 255)
    }
}
