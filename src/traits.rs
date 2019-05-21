use std::fmt;

use {
    WriteOptions,
};

/// A trait for writing data to the buffer.
pub trait WriteBuffer {
    /// Writes data to the `Vec<u8>` buffer using specified `WriteOptions`.
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>);

    /// Writes data to the `Vec<u8>` buffer using default `WriteOptions`.
    fn write_buf(&self, buf: &mut Vec<u8>) {
        self.write_buf_opt(&WriteOptions::default(), buf);
    }

    /// Returns an object that implements `fmt::Display` using provided write options.
    fn with_write_opt<'a>(&'a self, opt: &'a WriteOptions) -> DisplaySvg<'a, Self>
        where Self: Sized
    {
        DisplaySvg { value: self, opt }
    }
}

impl<T: WriteBuffer> WriteBuffer for Vec<T> {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        for (n, l) in self.iter().enumerate() {
            l.write_buf_opt(opt, buf);
            if n < self.len() - 1 {
                opt.write_separator(buf);
            }
        }
    }
}

/// A wrapper to use `fmt::Display` with [`WriteOptions`].
///
/// Should be used via `WriteBuffer::with_write_opt`.
///
/// # Example
///
/// ```
/// use svgtypes::{Transform, WriteOptions, WriteBuffer, DisplaySvg};
///
/// let ts = Transform::new(1.0, 0.0, 0.0, 1.0, 10.0, 20.0);
/// assert_eq!(ts.to_string(), "matrix(1 0 0 1 10 20)");
///
/// let opt = WriteOptions {
///     simplify_transform_matrices: true,
///     .. WriteOptions::default()
/// };
/// assert_eq!(ts.with_write_opt(&opt).to_string(), "translate(10 20)");
/// ```
///
/// [`WriteOptions`]: struct.WriteOptions.html
pub struct DisplaySvg<'a, T: 'a + WriteBuffer> {
    value: &'a T,
    opt: &'a WriteOptions,
}

impl<'a, T: WriteBuffer> fmt::Debug for DisplaySvg<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use Display.
        write!(f, "{}", self)
    }
}

impl<'a, T: WriteBuffer> fmt::Display for DisplaySvg<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        let mut out = Vec::with_capacity(32);
        self.value.write_buf_opt(self.opt, &mut out);
        write!(f, "{}", str::from_utf8(&out).unwrap())
    }
}

macro_rules! impl_display {
    ($t:ty) => (
        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.with_write_opt(&WriteOptions::default()))
            }
        }
    )
}

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


macro_rules! impl_vec_defer {
    ($t:ty, $tt:ty) => (
        impl ::std::ops::Deref for $t {
            type Target = Vec<$tt>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )
}

macro_rules! impl_from_vec {
    ($t:ty, $te:expr, $s:ty) => (
        impl From<Vec<$s>> for $t {
            fn from(v: Vec<$s>) -> Self {
                $te(v)
            }
        }
    )
}

macro_rules! impl_debug_from_display {
    ($t:ty) => (
        impl ::std::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Overload Display.
                write!(f, "{}", &self)
            }
        }
    )
}
