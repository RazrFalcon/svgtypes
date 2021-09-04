use float_cmp::ApproxEqUlps;

/// A trait for fuzzy/approximate equality comparisons of float numbers.
pub trait FuzzyEq<Rhs: ?Sized = Self> {
    /// Returns `true` if values are approximately equal.
    fn fuzzy_eq(&self, other: &Rhs) -> bool;

    /// Returns `true` if values are not approximately equal.
    #[inline]
    fn fuzzy_ne(&self, other: &Rhs) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl<T: FuzzyEq> FuzzyEq for Vec<T> {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in self.iter().zip(other.iter()) {
            if a.fuzzy_ne(b) {
                return false;
            }
        }

        true
    }
}

/// A trait for fuzzy/approximate comparisons of float numbers.
pub trait FuzzyZero: FuzzyEq {
    /// Returns `true` if the number is approximately zero.
    fn is_fuzzy_zero(&self) -> bool;
}

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


#[cfg(test)]
mod tests {
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
}
