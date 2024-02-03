use crate::stream::{escape_string, Stream};
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

// pub fn parse_font_families(text: &str) -> Vec<FontFamily> {
//     let families = vec![];
//     let parse_single_family = |mut stream: Stream| {
//         stream.skip_spaces();
//
//         if stream.curr_byte()? == b'\'' || stream.curr_byte()? == b'\"' {
//             let res = stream.parse_string()?;
//             return FontFamily::Named(res);
//         }   else {
//             let mut idents = vec![];
//
//             while let Ok(c) = stream.curr_char() {
//                 if c != ',' {
//                     idents.push(stream.parse_ident()?.to_string());
//                     stream.skip_spaces();
//                 }
//             }
//
//             let joined = idents.join(" ");
//
//             match joined.as_str() {
//                 "serif" => FontFamily::Serif,
//                 "sans-serif" => FontFamily::SansSerif,
//                 "cursive" => FontFamily::Cursive,
//                 "fantasy" => FontFamily::Fantasy,
//                 "monospace" => FontFamily::Monospace,
//                 _ => FontFamily::Named(joined)
//             }
//         }
//     };
//
//     let Ok(escaped) = escape_string(text) else { return families };
//     let mut stream = Stream::from(escaped);
//
//
//
//     families
// }

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
