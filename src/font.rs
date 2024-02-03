use crate::stream::Stream;
use crate::Error;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Named(String),
}

// impl std::str::FromStr for FontFamily {
//     type Err = Error;
//
//     fn from_str(text: &str) -> Result<Self, Error> {
//         let mut s = Stream::from(text);
//         let font_families = s.parse_font_families()?;
//
//         // Check that we are at the end of the stream. Otherwise color can be followed by icccolor,
//         // which is not supported.
//         s.skip_spaces();
//         if !s.at_end() {
//             return Err(Error::UnexpectedData(s.calc_char_pos()));
//         }
//
//         Ok(color)
//     }
// }

impl<'a> Stream<'a> {
    pub fn parse_font_families(&mut self) -> Result<Vec<FontFamily>, Error> {
        let mut families = vec![];

        while !self.at_end() {
            self.skip_spaces();

            let family = {
                if self.curr_byte()? == b'\'' || self.curr_byte()? == b'\"' {
                    let res = self.parse_string()?;
                    FontFamily::Named(res.to_string())
                } else {
                    let mut idents = vec![];

                    while !self.at_end() {
                        let c = self.chars().next().unwrap();

                        if c != ',' {
                            idents.push(self.parse_ident()?.to_string());
                            self.skip_spaces();
                        } else {
                            break;
                        }
                    }

                    let joined = idents.join(" ");

                    // TODO: No CSS keyword must be matched as a family name...
                    match joined.as_str() {
                        "serif" => FontFamily::Serif,
                        "sans-serif" => FontFamily::SansSerif,
                        "cursive" => FontFamily::Cursive,
                        "fantasy" => FontFamily::Fantasy,
                        "monospace" => FontFamily::Monospace,
                        _ => FontFamily::Named(joined),
                    }
                }
            };

            families.push(family);

            if let Ok(b) = self.curr_byte() {
                if b == b',' {
                    self.advance(1);
                }
            }
        }

        let families = families
            .into_iter()
            .filter(|f| match f {
                FontFamily::Named(s) => !s.is_empty(),
                _ => true,
            })
            .collect();

        Ok(families)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_font_families().unwrap(), $result);
            }
        )
    }

    macro_rules! named {
        ($text:expr) => (
            FontFamily::Named($text.to_string())
        )
    }

    const SERIF: FontFamily = FontFamily::Serif;
    const SANS_SERIF: FontFamily = FontFamily::SansSerif;
    const FANTASY: FontFamily = FontFamily::Fantasy;
    const MONOSPACE: FontFamily = FontFamily::Monospace;
    const CURSIVE: FontFamily = FontFamily::Cursive;

    test!(font_family_1, "Times New Roman", vec![named!("Times New Roman")]);
    test!(font_family_2, "serif", vec![SERIF]);
    test!(font_family_3, "sans-serif", vec![SANS_SERIF]);
    test!(font_family_4, "cursive", vec![CURSIVE]);
    test!(font_family_5, "fantasy", vec![FANTASY]);
    test!(font_family_6, "monospace", vec![MONOSPACE]);
    test!(font_family_7, "'Times New Roman'", vec![named!("Times New Roman")]);
    test!(font_family_8, "'Times New Roman', sans-serif", vec![named!("Times New Roman"), SANS_SERIF]);
    test!(font_family_9, "'Times New Roman', sans-serif", vec![named!("Times New Roman"), SANS_SERIF]);
    test!(font_family_10, "Arial, sans-serif, 'fantasy'", vec![named!("Arial"), SANS_SERIF, named!("fantasy")]);
    test!(font_family_11, "    Arial  , monospace  , 'fantasy'", vec![named!("Arial"), MONOSPACE, named!("fantasy")]);
    test!(font_family_12, "Times    New Roman", vec![named!("Times New Roman")]);
    test!(font_family_13, "\"Times New Roman\", sans-serif, sans-serif, \"Arial\"",
        vec![named!("Times New Roman"), SANS_SERIF, SANS_SERIF, named!("Arial")]
    );
    test!(font_family_14, "Times New Roman,,,Arial", vec![named!("Times New Roman"), named!("Arial")]);
    test!(font_family_15, "简体中文,sans-serif  , ,\"日本語フォント\",Arial",
        vec![named!("简体中文"), SANS_SERIF, named!("日本語フォント"), named!("Arial")]);

    macro_rules! font_family_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Stream::from($text).parse_font_families().unwrap_err().to_string(), $result);
            }
        )
    }
    font_family_err!(font_family_err_1, "Red/Black, sans-serif", "invalid ident");
    // font_family_err!(font_family_err_2, "\"Lucida\" Grande, sans-serif", "invalid ident");
}
