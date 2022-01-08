use std::cmp::PartialEq;
use std::fmt;


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
            panic!("Color g out of range: {}", r);
        }
        if b < 0.0 || b > 1.0 {
            panic!("Color b out of range: {}", r);
        }

        let max = 255.0;

        Color {
            r: (max * r) as u8,
            g: (max * g) as u8,
            b: (max * b) as u8,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_color_fmt() {
        let c = Color { r: 16, g: 32, b: 64 };
        assert_eq!(format!("{}", c), "16 32 64");
    }
}
