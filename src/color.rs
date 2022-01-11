use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul};

use crate::vec::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, }
    }

    pub fn from_float(r: f64, g: f64, b: f64) -> Color {
        if r < 0.0 || r > 1.0 {
            panic!("Color r out of range: {}", r);
        }
        if g < 0.0 || g > 1.0 {
            panic!("Color g out of range: {}", g);
        }
        if b < 0.0 || b > 1.0 {
            panic!("Color b out of range: {}", b);
        }

        let max = 255.0;

        Color {
            r: (max * r) as u8,
            g: (max * g) as u8,
            b: (max * b) as u8,
        }
    }

    pub fn from_normal(n: &Vector) -> Color {
        let r = 0.5 * (n.x + 1.0);
        let g = 0.5 * (n.y + 1.0);
        let b = 0.5 * (n.z + 1.0);
        Color::from_float(r, g, b)
    }

    pub fn lerp(start_color: Color, end_color: Color, t: f64) -> Color {
        if t < 0.0 || t > 1.0 {
            panic!("lerp: t = {} out of range", t);
        }

        (1.0 - t) * start_color + t * end_color
    }

    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: add_no_overflow(self.r, other.r),
            g: add_no_overflow(self.g, other.g),
            b: add_no_overflow(self.b, other.b),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        let r = (self.r as f64) * other;
        let g = (self.g as f64) * other;
        let b = (self.b as f64) * other;

        Color {
            r: clamp(r, 0.0, 255.0) as u8,
            g: clamp(g, 0.0, 255.0) as u8,
            b: clamp(b, 0.0, 255.0) as u8,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r = self * (other.r as f64);
        let g = self * (other.g as f64);
        let b = self * (other.b as f64);

        Color {
            r: clamp(r, 0.0, 255.0) as u8,
            g: clamp(g, 0.0, 255.0) as u8,
            b: clamp(b, 0.0, 255.0) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
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

fn add_no_overflow(val1: u8, val2: u8) -> u8 {
    match val1.checked_add(val2) {
        Some(v) => v,
        None => u8::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_f64() {
        let min = 0.0f64;
        let max = 255.0f64;

        assert_eq!(clamp(-1.0, min , max), min);
        assert_eq!(clamp(127.0, min, max), 127.0);
        assert_eq!(clamp(256.0, min, max), max);
    }

    #[test]
    fn test_add_no_overflow() {
        assert_eq!(add_no_overflow(128u8, 64u8), 192u8);
        assert_eq!(add_no_overflow(128u8, 128u8), 255u8)
    }

    #[test]
    fn test_color_eq() {
        let c1 = Color { r: 16, g: 32, b: 64, };
        let c2 = Color { r: 16, g: 32, b: 64, };
        assert!(c1.r == c2.r && c1.g == c2.g && c1.b == c2.b);
    }

    #[test]
    fn test_color_from() {
        let r = 0.0;
        let g = 0.5;
        let b = 1.0;
        let c = Color::from_float(r, g, b);
        assert_eq!(c, Color::new(0, 127, 255));
    }

    #[test]
    fn test_color_from_normal() {
        let n = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(Color::from_normal(&n), Color::new(255, 127, 127));

        let n = Vector::new(-1.0, 1.0, 0.0);
        assert_eq!(Color::from_normal(&n), Color::new(0, 255, 127));

        let n = Vector::new(0.0, -1.0, 1.0);
        assert_eq!(Color::from_normal(&n), Color::new(127, 0, 255));

        let d = 2.0f64.sqrt() / 2.0;
        let n = Vector::new(d, d, 0.0);
        assert_eq!(Color::from_normal(&n), Color { r: 217, g: 217, b: 127, })
    }

    #[test]
    fn test_color_fmt() {
        let c = Color { r: 16, g: 32, b: 64 };
        assert_eq!(format!("{}", c), "16 32 64");
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_color_from_float() {
        let _c = Color::from_float(1.1, 0.5, 0.5);
    }

    #[test]
    fn test_color_add() {
        let c1 = Color::new(128, 128, 128);
        let c2 = Color::new(16, 32, 64);
        let c3 = Color::new(144, 144, 144);

        assert_eq!(c1 + c2, Color::new(144, 160, 192));
        assert_eq!(c1 + c3, Color::white());
    }

    #[test]
    fn test_color_mul_scalar() {
        let c = Color::new(16, 32, 64);
        let m: f64 = 2.0;
        let d: f64 = 0.5;

        assert_eq!(c * m, Color::new(32, 64, 128));
        assert_eq!(m * c, Color::new(32, 64, 128));
        assert_eq!(c * d, Color::new(8, 16, 32));
        assert_eq!(d * c, Color::new(8, 16, 32));

        assert_eq!(16.0f64 * c, Color::new(255, 255, 255));
        assert_eq!(-1.0 * c, Color::new(0, 0, 0));
    }
}
