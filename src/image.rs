use std::fmt;

use crate::color::Color;

pub struct ImagePpm {
    pub width: u32,
    pub height: u32,
    data: Vec<Color>,
}

impl ImagePpm {
    pub fn new(w: u32, h: u32) -> ImagePpm {
        let num_pixels = (w * h) as usize;
        ImagePpm {
            width: w,
            height: h,
            data: vec![Color::BLACK; num_pixels],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
        if x > self.width - 1 || y > self.height - 1 {
            panic!("ImagePpm setting pixel ({}, {}) out of range ({}, {})", x, y, self.width, self.height);
        }

        let index = (self.width * y + x) as usize;
        self.data[index] = c;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x > self.width - 1 || y > self.height - 1 {
            panic!("ImagePpm getting pixel ({}, {}) out of range ({}, {})", x, y, self.width, self.height);
        }

        let index = (self.width * y + x) as usize;
        self.data[index]
    }
}

impl fmt::Display for ImagePpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                write!(f, "{}\n", self.get_pixel(x, y))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ImagePpm setting pixel")]
    fn test_image_set_pixel() {
        let mut img = ImagePpm::new(4, 4);
        img.set_pixel(8, 0, Color::WHITE);
    }

    #[test]
    fn test_image_get_pixel() {
        let mut img = ImagePpm::new(4, 4);
        img.set_pixel(2, 2, Color::WHITE);
        assert!(img.get_pixel(2, 2) == Color::WHITE);
    }

    #[test]
    #[should_panic(expected = "ImagePpm getting pixel")]
    fn test_image_get_pixel_panic() {
        let img = ImagePpm::new(4, 4);
        img.get_pixel(0, 4);
    }
}
