use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul};
use float_cmp::approx_eq;

use crate::vec::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, }
    }

    pub fn from_normal(n: &Vector) -> Color {
        let r = 0.5 * (n.x + 1.0);
        let g = 0.5 * (n.y + 1.0);
        let b = 0.5 * (n.z + 1.0);
        Color { r, g, b, }
    }

    pub fn lerp(start_color: Color, end_color: Color, t: f64) -> Color {
        if t < 0.0 || t > 1.0 {
            panic!("lerp: t = {} out of range", t);
        }

        (1.0 - t) * start_color + t * end_color
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        approx_eq!(f64, self.r, other.r, ulps = 2)
            && approx_eq!(f64, self.g, other.g, ulps = 2)
            && approx_eq!(f64, self.b, other.b, ulps = 2)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        let r = self.r * other;
        let g = self.g * other;
        let b = self.b * other;
        Color { r, g, b }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r = self * other.r;
        let g = self * other.g;
        let b = self * other.b;
        Color { r, g, b, }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (clamp(self.r, 0.0, 1.0) * 255.0) as u8;
        let g = (clamp(self.g, 0.0, 1.0) * 255.0) as u8;
        let b = (clamp(self.b, 0.0, 1.0) * 255.0) as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}

fn clamp<T>(val: T, min: T, max: T) -> T
where T: PartialOrd
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_clamp_f64() {
        let min = 0.0f64;
        let max = 255.0f64;

        assert_eq!(clamp(-1.0, min , max), min);
        assert_eq!(clamp(127.0, min, max), 127.0);
        assert_eq!(clamp(256.0, min, max), max);
    }

    #[test]
    fn test_color_eq() {
        let c1 = Color { r: 0.1, g: 0.2, b: 0.3, };
        let c2 = Color { r: 0.1, g: 0.1 + 0.1, b: 0.1 + 0.2, };
        assert!(c1 == c2);
    }

    #[test]
    fn test_color_from_normal() {
        let n = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(Color::from_normal(&n), Color::new(1.0, 0.5, 0.5));

        let n = Vector::new(-1.0, 1.0, 0.0);
        assert_eq!(Color::from_normal(&n), Color::new(0.0, 1.0, 0.5));

        let n = Vector::new(0.0, -1.0, 1.0);
        assert_eq!(Color::from_normal(&n), Color::new(0.5, 0.0, 1.0));

        let d = SQRT_2 / 2.0;
        let c = 0.5 * (1.0 + d);
        let n = Vector::new(d, d, 0.0);
        assert_eq!(Color::from_normal(&n), Color { r: c, g: c, b: 0.5, })
    }

    #[test]
    fn test_color_fmt() {
        let c = Color { r: 0.0625, g: 0.125, b: 0.25 };
        assert_eq!(format!("{}", c), "15 31 63");

        let c= Color { r: 0.0, g: 0.5, b: 1.0, };
        assert_eq!(format!("{}", c), "0 127 255");
    }

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.5, 0.5, 0.5);
        let c2 = Color::new(0.0625, 0.125, 0.25);

        assert_eq!(c1 + c2, Color::new(0.5625, 0.625, 0.75));
    }

    #[test]
    fn test_color_mul_scalar() {
        let c = Color::new(0.2, 0.4, 0.5);
        let m: f64 = 2.0;
        let d: f64 = 0.5;

        assert_eq!(c * m, Color::new(0.4, 0.8, 1.0));
        assert_eq!(m * c, Color::new(0.4, 0.8, 1.0));
        assert_eq!(c * d, Color::new(0.1, 0.2, 0.25));
        assert_eq!(d * c, Color::new(0.1, 0.2, 0.25));
    }
}
