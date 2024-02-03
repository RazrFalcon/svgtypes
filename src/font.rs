use crate::stream::Stream;
use crate::Error;
use std::fmt::Display;

pub fn parse_font_families(text: &str) -> Result<Vec<FontFamily>, Error> {
    let mut s = Stream::from(text);
    let font_families = s.parse_font_families()?;

    s.skip_spaces();
    if !s.at_end() {
        return Err(Error::UnexpectedData(s.calc_char_pos()));
    }

    Ok(font_families)
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Named(String),
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FontFamily::Monospace => "monospace".to_string(),
            FontFamily::Serif => "serif".to_string(),
            FontFamily::SansSerif => "sans-serif".to_string(),
            FontFamily::Cursive => "cursive".to_string(),
            FontFamily::Fantasy => "fantasy".to_string(),
            FontFamily::Named(s) => format!("\"{}\"", s),
        };
        write!(f, "{}", str)
    }
}

impl<'a> Stream<'a> {
    pub fn parse_font_families(&mut self) -> Result<Vec<FontFamily>, Error> {
        let mut families = vec![];

        while !self.at_end() {
            self.skip_spaces();

            let family = {
                let ch = self.curr_byte()?;
                if ch == b'\'' || ch == b'\"' {
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
                }   else {
                    break;
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
                assert_eq!(parse_font_families($text).unwrap(), $result);
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

    test!(font_family_16, "", vec![]);

    macro_rules! font_family_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(parse_font_families($text).unwrap_err().to_string(), $result);
            }
        )
    }
    font_family_err!(font_family_err_1, "Red/Black, sans-serif", "invalid ident");
    font_family_err!(font_family_err_2, "\"Lucida\" Grande, sans-serif", "unexpected data at position 10");
    font_family_err!(font_family_err_3, "Ahem!, sans-serif", "invalid ident");
    font_family_err!(font_family_err_4, "test@foo, sans-serif", "invalid ident");
    font_family_err!(font_family_err_5, "#POUND, sans-serif", "invalid ident");
    font_family_err!(font_family_err_6, "Hawaii 5-0, sans-serif", "invalid ident");
}
