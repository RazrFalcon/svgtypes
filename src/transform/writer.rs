// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    FuzzyEq,
    Transform,
    WriteBuffer,
    WriteOptions,
};

impl WriteBuffer for Transform {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        if opt.simplify_transform_matrices {
            write_simplified_transform(self, opt, buf);
        } else {
            write_matrix_transform(self, opt, buf);
        }
    }
}

fn write_matrix_transform(ts: &Transform, opt: &WriteOptions, out: &mut Vec<u8>) {
    // TODO: ListSeparator

    out.extend_from_slice(b"matrix(");
    ts.a.write_buf_opt(opt, out);
    out.push(b' ');
    ts.b.write_buf_opt(opt, out);
    out.push(b' ');
    ts.c.write_buf_opt(opt, out);
    out.push(b' ');
    ts.d.write_buf_opt(opt, out);
    out.push(b' ');
    ts.e.write_buf_opt(opt, out);
    out.push(b' ');
    ts.f.write_buf_opt(opt, out);
    out.push(b')');
}

fn write_simplified_transform(ts: &Transform, opt: &WriteOptions, out: &mut Vec<u8>) {
    if ts.is_translate() {
        out.extend_from_slice(b"translate(");
        ts.e.write_buf_opt(opt, out);

        if ts.f.fuzzy_ne(&0.0) {
            out.push(b' ');
            ts.f.write_buf_opt(opt, out);
        }

        out.push(b')');
    } else if ts.is_scale() {
        out.extend_from_slice(b"scale(");
        ts.a.write_buf_opt(opt, out);

        if ts.a.fuzzy_ne(&ts.d) {
            out.push(b' ');
            ts.d.write_buf_opt(opt, out);
        }

        out.push(b')');
    } else if !ts.has_translate() {
        let a = ts.get_rotate();
        let (sx, sy) = ts.get_scale();
        let (skx, sky) = ts.get_skew();

        if a.fuzzy_eq(&skx) && a.fuzzy_eq(&sky) && sx.fuzzy_eq(&1.0) && sy.fuzzy_eq(&1.0) {
            out.extend_from_slice(b"rotate(");
            a.write_buf_opt(opt, out);
            out.push(b')');
        } else {
            write_matrix_transform(ts, opt, out);
        }
    } else {
        write_matrix_transform(ts, opt, out);
    }
}

impl_display!(Transform);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use {
        WriteOptions,
        WriteBuffer,
    };

    macro_rules! test {
        ($name:ident, $ts:expr, $simplify:expr, $result:expr) => (
            #[test]
            fn $name() {
                let mut opt = WriteOptions::default();
                opt.simplify_transform_matrices = $simplify;
                assert_eq!($ts.with_write_opt(&opt).to_string(), $result);
            }
        )
    }

    test!(write_1,
        Transform::default(), false,
        "matrix(1 0 0 1 0 0)"
    );

    test!(write_2,
        Transform::new(2.0, 0.0, 0.0, 3.0, 20.0, 30.0), false,
        "matrix(2 0 0 3 20 30)"
    );

    test!(write_3,
        Transform::new(1.0, 0.0, 0.0, 1.0, 20.0, 30.0), true,
        "translate(20 30)"
    );

    test!(write_4,
        Transform::new(1.0, 0.0, 0.0, 1.0, 20.0, 0.0), true,
        "translate(20)"
    );

    test!(write_5,
        Transform::new(2.0, 0.0, 0.0, 3.0, 0.0, 0.0), true,
        "scale(2 3)"
    );

    test!(write_6,
        Transform::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0), true,
        "scale(2)"
    );

    test!(write_7,
        Transform::from_str("rotate(30)").unwrap(), true,
        "rotate(30)"
    );

    test!(write_8,
        Transform::from_str("rotate(-45)").unwrap(), true,
        "rotate(-45)"
    );

    test!(write_9,
        Transform::from_str("rotate(33)").unwrap(), true,
        "rotate(33)"
    );

    test!(write_10,
        Transform::from_str("scale(-1)").unwrap(), true,
        "scale(-1)"
    );

    test!(write_11,
        Transform::from_str("scale(-1 1)").unwrap(), true,
        "scale(-1 1)"
    );

    test!(write_12,
        Transform::from_str("scale(1 -1)").unwrap(), true,
        "scale(1 -1)"
    );

    test!(write_13,
        Transform::new(1.0, 0.0, 0.0, 1.0, 20.0, 30.0), false,
        "matrix(1 0 0 1 20 30)"
    );
}
