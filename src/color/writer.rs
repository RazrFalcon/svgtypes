use crate::{Color, WriteBuffer, WriteOptions};

static CHARS: &[u8] = b"0123456789abcdef";

#[inline]
fn int2hex(n: u8) -> (u8, u8) {
    (CHARS[(n >> 4) as usize], CHARS[(n & 0xf) as usize])
}

impl WriteBuffer for Color {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        // TODO: rgb() support
        // TODO: color name support

        buf.push(b'#');
        let (r1, r2) = int2hex(self.red);
        let (g1, g2) = int2hex(self.green);
        let (b1, b2) = int2hex(self.blue);

        if opt.trim_hex_colors && r1 == r2 && g1 == g2 && b1 == b2 {
            buf.push(r1);
            buf.push(g1);
            buf.push(b1);
        } else {
            buf.push(r1);
            buf.push(r2);
            buf.push(g1);
            buf.push(g2);
            buf.push(b1);
            buf.push(b2);
        }
    }
}

impl_display!(Color);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WriteOptions, WriteBuffer};

    macro_rules! test {
        ($name:ident, $c:expr, $trim:expr, $result:expr) => (
            #[test]
            fn $name() {
                let mut opt = WriteOptions::default();
                opt.trim_hex_colors = $trim;
                assert_eq!($c.with_write_opt(&opt).to_string(), $result);
            }
        )
    }

    test!(write_1, Color::new(255, 0, 0), false, "#ff0000");
    test!(write_2, Color::new(255, 127, 5), false, "#ff7f05");
    test!(write_3, Color::new(255, 0, 0), true, "#f00");
    test!(write_4, Color::new(255, 127, 5), true, "#ff7f05");
}
