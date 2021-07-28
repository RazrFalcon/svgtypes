use std::io::Write;

use float_cmp::ApproxEqUlps;

use crate::{FuzzyEq, FuzzyZero, WriteBuffer, WriteOptions};

impl FuzzyEq for f32 {
    #[inline]
    fn fuzzy_eq(&self, other: &f32) -> bool {
        self.approx_eq_ulps(other, 4)
    }
}

impl FuzzyEq for f64 {
    #[inline]
    fn fuzzy_eq(&self, other: &f64) -> bool {
        self.approx_eq_ulps(other, 4)
    }
}

impl FuzzyZero for f32 {
    #[inline]
    fn is_fuzzy_zero(&self) -> bool {
        self.fuzzy_eq(&0.0)
    }
}

impl FuzzyZero for f64 {
    #[inline]
    fn is_fuzzy_zero(&self) -> bool {
        self.fuzzy_eq(&0.0)
    }
}


impl WriteBuffer for f64 {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        write_num(self, opt.remove_leading_zero, buf);
    }
}

fn write_num(num: &f64, rm_leading_zero: bool, buf: &mut Vec<u8>) {
    // If number is an integer, it's faster to write it as i32.
    if num.fract().is_fuzzy_zero() {
        write!(buf, "{}", *num as i32).unwrap();
        return;
    }

    // Round numbers up to 11 digits to prevent writing
    // ugly numbers like 29.999999999999996.
    // It's not 100% correct, but differences are insignificant.
    let v = (num * 100_000_000_000.0).round() / 100_000_000_000.0;

    let start_pos = buf.len();

    write!(buf, "{}", v).unwrap();

    if rm_leading_zero {
        let mut has_dot = false;
        let mut pos = 0;
        for c in buf.iter().skip(start_pos) {
            if *c == b'.' {
                has_dot = true;
                break;
            }
            pos += 1;
        }

        if has_dot && buf[start_pos + pos - 1] == b'0' {
            if pos == 2 && num.is_sign_negative() {
                // -0.1 -> -.1
                buf.remove(start_pos + 1);
            } else if pos == 1 && num.is_sign_positive() {
                // 0.1 -> .1
                buf.remove(start_pos);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stream;

    macro_rules! test_p {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                let mut s = Stream::from($text);
                assert_eq!(s.parse_number().unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "0", 0.0);
    test_p!(parse_2,  "1", 1.0);
    test_p!(parse_3,  "-1", -1.0);
    test_p!(parse_4,  " -1 ", -1.0);
    test_p!(parse_5,  "  1  ", 1.0);
    test_p!(parse_6,  ".4", 0.4);
    test_p!(parse_7,  "-.4", -0.4);
    test_p!(parse_8,  "-.4text", -0.4);
    test_p!(parse_9,  "-.01 text", -0.01);
    test_p!(parse_10, "-.01 4", -0.01);
    test_p!(parse_11, ".0000000000008", 0.0000000000008);
    test_p!(parse_12, "1000000000000", 1000000000000.0);
    test_p!(parse_13, "123456.123456", 123456.123456);
    test_p!(parse_14, "+10", 10.0);
    test_p!(parse_15, "1e2", 100.0);
    test_p!(parse_16, "1e+2", 100.0);
    test_p!(parse_17, "1E2", 100.0);
    test_p!(parse_18, "1e-2", 0.01);
    test_p!(parse_19, "1ex", 1.0);
    test_p!(parse_20, "1em", 1.0);
    test_p!(parse_21, "12345678901234567890", 12345678901234567000.0);
    test_p!(parse_22, "0.", 0.0);
    test_p!(parse_23, "1.3e-2", 0.013);
    // test_number!(parse_24, "1e", 1.0); // TODO: this

    macro_rules! test_p_err {
        ($name:ident, $text:expr) => (
            #[test]
            fn $name() {
                let mut s = Stream::from($text);
                assert_eq!(s.parse_number().unwrap_err().to_string(),
                           "invalid number at position 1");
            }
        )
    }

    test_p_err!(parse_err_1, "q");
    test_p_err!(parse_err_2, "");
    test_p_err!(parse_err_3, "-");
    test_p_err!(parse_err_4, "+");
    test_p_err!(parse_err_5, "-q");
    test_p_err!(parse_err_6, ".");
    test_p_err!(parse_err_7, "99999999e99999999");
    test_p_err!(parse_err_8, "-99999999e99999999");

    macro_rules! test_w {
        ($name:ident, $num:expr, $rm_zero:expr, $result:expr) => (
            #[test]
            fn $name() {
                let mut v = Vec::new();
                write_num(&$num, $rm_zero, &mut v);
                assert_eq!(String::from_utf8(v).unwrap(), $result);
            }
        )
    }

    test_w!(write_1,  1.0,                 false, "1");
    test_w!(write_2,  0.0,                 false, "0");
    test_w!(write_3,  -0.0,                false, "0");
    test_w!(write_4,  -1.0,                false, "-1");
    test_w!(write_5,  12345678.12345678,   false, "12345678.123456782");
    test_w!(write_6,  -0.1,                true,  "-.1");
    test_w!(write_7,  0.1,                 true,  ".1");
    test_w!(write_8,  1.0,                 true,  "1");
    test_w!(write_9,  -1.0,                true,  "-1");
    test_w!(write_10, 1.5,                 false, "1.5");
    test_w!(write_11, 0.14186,             false, "0.14186");
    test_w!(write_12, 29.999999999999996,  false, "30");
    test_w!(write_13, 0.49999999999999994, false, "0.5");
    // With some algorithms may produce 4273.680000000001.
    test_w!(write_14, 4273.68,             false, "4273.68");
}
