// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::f64;
use std::ops::Mul;

mod parser;
mod writer;

pub use self::parser::*;

use {
    FuzzyEq,
};

/// Representation of the [`<transform>`] type.
///
/// [`<transform>`]: https://www.w3.org/TR/SVG11/coords.html#TransformAttribute
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Transform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Transform {
    /// Constructs a new transform.
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Transform { a, b, c, d, e, f, }
    }

    /// Constructs a new translate transform.
    pub fn new_translate(x: f64, y: f64) -> Self {
        Transform::new(1.0, 0.0, 0.0, 1.0, x, y)
    }

    /// Constructs a new scale transform.
    pub fn new_scale(sx: f64, sy: f64) -> Self {
        Transform::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Constructs a new rotate transform.
    pub fn new_rotate(angle: f64) -> Self {
        let v = (angle / 180.0) * f64::consts::PI;
        let a =  v.cos();
        let b =  v.sin();
        let c = -b;
        let d =  a;
        Transform::new(a, b, c, d, 0.0, 0.0)
    }

    /// Constructs a new rotate transform at the specified position.
    pub fn new_rotate_at(angle: f64, x: f64, y: f64) -> Self {
        let mut ts = Self::default();
        ts.translate(x, y);
        ts.rotate(angle);
        ts.translate(-x, -y);
        ts
    }

    /// Constructs a new skew transform along then X axis.
    pub fn new_skew_x(angle: f64) -> Self {
        let c = ((angle / 180.0) * f64::consts::PI).tan();
        Transform::new(1.0, 0.0, c, 1.0, 0.0, 0.0)
    }

    /// Constructs a new skew transform along then Y axis.
    pub fn new_skew_y(angle: f64) -> Self {
        let b = ((angle / 180.0) * f64::consts::PI).tan();
        Transform::new(1.0, b, 0.0, 1.0, 0.0, 0.0)
    }

    /// Translates the current transform.
    pub fn translate(&mut self, x: f64, y: f64) {
        self.append(&Transform::new_translate(x, y));
    }

    /// Scales the current transform.
    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.append(&Transform::new_scale(sx, sy));
    }

    /// Rotates the current transform.
    pub fn rotate(&mut self, angle: f64) {
        self.append(&Transform::new_rotate(angle));
    }

    /// Rotates the current transform at the specified position.
    pub fn rotate_at(&mut self, angle: f64, x: f64, y: f64) {
        self.translate(x, y);
        self.rotate(angle);
        self.translate(-x, -y);
    }

    /// Skews the current transform along the X axis.
    pub fn skew_x(&mut self, angle: f64) {
        self.append(&Transform::new_skew_x(angle));
    }

    /// Skews the current transform along the Y axis.
    pub fn skew_y(&mut self, angle: f64) {
        self.append(&Transform::new_skew_y(angle));
    }

    /// Appends transform to the current transform.
    pub fn append(&mut self, t: &Transform) {
        // TODO: optimize. No need to create TransformMatrix each time.
        let tm = self.to_matrix() * t.to_matrix();
        self.a = tm.d[0][0];
        self.c = tm.d[1][0];
        self.e = tm.d[2][0];
        self.b = tm.d[0][1];
        self.d = tm.d[1][1];
        self.f = tm.d[2][1];
    }

    fn to_matrix(&self) -> TransformMatrix {
        let mut tm = TransformMatrix::default();
        tm.d[0][0] = self.a;
        tm.d[1][0] = self.c;
        tm.d[2][0] = self.e;
        tm.d[0][1] = self.b;
        tm.d[1][1] = self.d;
        tm.d[2][1] = self.f;
        tm
    }

    /// Returns `true` if the transform is default, aka `(1 0 0 1 0 0)`.
    pub fn is_default(&self) -> bool {
           self.a.fuzzy_eq(&1.0)
        && self.b.fuzzy_eq(&0.0)
        && self.c.fuzzy_eq(&0.0)
        && self.d.fuzzy_eq(&1.0)
        && self.e.fuzzy_eq(&0.0)
        && self.f.fuzzy_eq(&0.0)
    }

    /// Returns `true` if the transform contains only translate part, aka `(1 0 0 1 x y)`.
    pub fn is_translate(&self) -> bool {
           self.a.fuzzy_eq(&1.0)
        && self.b.fuzzy_eq(&0.0)
        && self.c.fuzzy_eq(&0.0)
        && self.d.fuzzy_eq(&1.0)
        && (self.e.fuzzy_ne(&0.0) || self.f.fuzzy_ne(&0.0))
    }

    /// Returns `true` if the transform contains only scale part, aka `(sx 0 0 sy 0 0)`.
    pub fn is_scale(&self) -> bool {
          (self.a.fuzzy_ne(&1.0) || self.d.fuzzy_ne(&1.0))
        && self.b.fuzzy_eq(&0.0)
        && self.c.fuzzy_eq(&0.0)
        && self.e.fuzzy_eq(&0.0)
        && self.f.fuzzy_eq(&0.0)
    }

    /// Returns `true` if the transform contains translate part.
    pub fn has_translate(&self) -> bool {
        self.e.fuzzy_ne(&0.0) || self.f.fuzzy_ne(&0.0)
    }

    /// Returns `true` if the transform contains scale part.
    pub fn has_scale(&self) -> bool {
        let (sx, sy) = self.get_scale();
        sx.fuzzy_ne(&1.0) || sy.fuzzy_ne(&1.0)
    }

    /// Returns `true` if the transform scale is proportional.
    ///
    /// The proportional scale is when `<sx>` equal to `<sy>`.
    pub fn has_proportional_scale(&self) -> bool {
        let (sx, sy) = self.get_scale();
        sx.fuzzy_eq(&sy)
    }

    /// Returns `true` if the transform contains skew part.
    pub fn has_skew(&self) -> bool {
        let (skew_x, skew_y) = self.get_skew();
        skew_x.fuzzy_ne(&0.0) || skew_y.fuzzy_ne(&0.0)
    }

    /// Returns `true` if the transform contains rotate part.
    pub fn has_rotate(&self) -> bool {
        self.get_rotate().fuzzy_ne(&0.0)
    }

    /// Returns transform's translate part.
    pub fn get_translate(&self) -> (f64, f64) {
        (self.e, self.f)
    }

    /// Returns transform's scale part.
    pub fn get_scale(&self) -> (f64, f64) {
        let x_scale = (self.a * self.a + self.c * self.c).sqrt();
        let y_scale = (self.b * self.b + self.d * self.d).sqrt();
        (x_scale, y_scale)
    }

    /// Returns transform's skew part.
    pub fn get_skew(&self) -> (f64, f64) {
        let rad = 180.0 / f64::consts::PI;
        let skew_x = rad * (self.d).atan2(self.c) - 90.0;
        let skew_y = rad * (self.b).atan2(self.a);
        (skew_x, skew_y)
    }

    /// Returns transform's rotate part.
    pub fn get_rotate(&self) -> f64 {
        let rad = 180.0 / f64::consts::PI;
        let mut angle = (-self.b/self.a).atan() * rad;
        if self.b < self.c || self.b > self.c {
            angle = -angle;
        }
        angle
    }

    /// Applies transform to selected coordinates.
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        let new_x = self.a * x + self.c * y + self.e;
        let new_y = self.b * x + self.d * y + self.f;
        (new_x, new_y)
    }

    /// Applies transform to selected coordinates.
    pub fn apply_to(&self, x: &mut f64, y: &mut f64) {
        let tx = *x;
        let ty = *y;
        *x = self.a * tx + self.c * ty + self.e;
        *y = self.b * tx + self.d * ty + self.f;
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }
}

impl FuzzyEq for Transform {
    fn fuzzy_eq(&self, other: &Self) -> bool {
           self.a.fuzzy_eq(&other.a)
        && self.b.fuzzy_eq(&other.b)
        && self.c.fuzzy_eq(&other.c)
        && self.d.fuzzy_eq(&other.d)
        && self.e.fuzzy_eq(&other.e)
        && self.f.fuzzy_eq(&other.f)
    }
}

struct TransformMatrix {
    d: [[f64; 3]; 3]
}

impl Default for TransformMatrix {
    fn default() -> TransformMatrix {
        TransformMatrix {
            d: [[1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]]
        }
    }
}

impl Mul for TransformMatrix {
    type Output = TransformMatrix;

    fn mul(self, other: TransformMatrix) -> TransformMatrix {
        let mut res = TransformMatrix::default();
        for row in 0..3 {
            for col in 0..3 {
                let mut sum = 0.0;
                for j in 0..3 {
                    sum += self.d[j][row] * other.d[col][j];
                }
                res.d[col][row] = sum;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn api_1() {
        let mut ts = Transform::default();
        ts.translate(10.0, 20.0);
        assert_eq!(Transform::from_str("translate(10 20)").unwrap(), ts);
    }

    #[test]
    fn api_2() {
        let mut ts = Transform::default();
        ts.scale(2.0, 3.0);
        assert_eq!(Transform::from_str("scale(2 3)").unwrap(), ts);
    }

    #[test]
    fn api_3() {
        let mut ts = Transform::default();
        ts.skew_x(20.0);
        assert_eq!(Transform::from_str("skewX(20)").unwrap(), ts);
    }

    #[test]
    fn api_4() {
        let mut ts = Transform::default();
        ts.skew_y(20.0);
        assert_eq!(Transform::from_str("skewY(20)").unwrap(), ts);
    }

    #[test]
    fn api_5() {
        let mut ts = Transform::default();
        ts.rotate(20.0);
        assert_eq!(Transform::from_str("rotate(20)").unwrap(), ts);
    }
}
