use crate::stream::Stream;
use crate::Error;
use std::str::FromStr;

pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Named(String),
}

pub fn parse_font_families(text: &str) -> Vec<FontFamily> {
    let families = vec![];

    families
}

// #[rustfmt::skip]
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     macro_rules! test {
//         ($name:ident, $text:expr, $result:expr) => (
//             #[test]
//             fn $name() {
//                 assert_eq!(FontStretch::from_str($text).unwrap(), $result);
//             }
//         )
//     }
//
//     // TODO: Add more tests
//     test!(parse_1, "narrower", FontStretch::Condensed);
//
//     macro_rules! test_err {
//         ($name:ident, $text:expr, $result:expr) => (
//             #[test]
//             fn $name() {
//                 assert_eq!(FontStretch::from_str($text).unwrap_err().to_string(), $result);
//             }
//         )
//     }
//
//     test_err!(parse_err_1, "dfg", "invalid value");
// }
