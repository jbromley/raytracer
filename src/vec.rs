use std::cmp::PartialEq;
use std::ops::{Add, Sub, Neg, Mul, Div };

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, }
    }

    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0, }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(self.y * other.z() - self.z * other.y(),
                    self.z * other.x() - self.x * other.z(),
                    self.x * other.y() - self.y * other.x(),)
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self * other.x(),
            y: self * other.y(),
            z: self * other.z(),
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector { x: 0.1, y: 0.2, z: 0.3, };
        assert_eq!(v.x(), 0.1);
        assert_eq!(v.y(), 0.2);
        assert_eq!(v.z(), 0.3);

        let w = Vector::new(0.4, 0.5, 0.6);
        assert_eq!(w.x(), 0.4);
        assert_eq!(w.y(), 0.5);
        assert_eq!(w.z(), 0.6);
    }

    #[test]
    fn test_zero() {
        let zero = Vector::zero();
        assert_eq!(zero.x(), 0.0);
        assert_eq!(zero.y(), 0.0);
        assert_eq!(zero.z(), 0.0);
    }

    #[test]
    fn test_length_squared() {
        let p = Vector::new(3.0, 4.0, 5.0);
        assert_approx_eq!(p.length_squared(), 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0);
    }

    #[test]
    fn test_length() {
        let p = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(p.length(), (1.0 + 4.0 + 9.0 as f64).sqrt());
    }

    #[test]
    fn test_dot() {
        let p = Vector::new(0.1, 0.2, 0.3);
        let q = Vector::new(0.4, 0.5, 0.6);
        assert_approx_eq!(p.dot(&q), 0.1 * 0.4 + 0.2 * 0.5 + 0.3 * 0.6);
    }

    #[test]
    fn test_cross() {
        let p = Vector::new(0.1, 0.2, 0.3);
        let q = Vector::new(0.4, 0.5, 0.6);
        let pxq = p.cross(&q);
        let expected = Vector {
            x: 0.2 * 0.6 - 0.3 * 0.5,
            y: 0.3 * 0.4 - 0.1 * 0.6,
            z: 0.1 * 0.5 - 0.2 * 0.4,
        };
        assert_approx_eq!(pxq.x(), expected.x());
        assert_approx_eq!(pxq.y(), expected.y());
        assert_approx_eq!(pxq.z(), expected.z());
    }

    #[test]
    fn test_distance() {
        let p = Vector::new(0.1, 0.2, 0.3);
        let q = Vector::new(0.6, 0.5, 0.4);
        let d = p.distance(&q);
        let expected = (0.25 + 0.09 + 0.01 as f64).sqrt();
        assert_approx_eq!(d, expected);
    }

    #[test]
    fn test_partial_eq() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 3.0, 2.0);
        assert_eq!(v1 + v2, Vector::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vector::new(5.0, 4.0, 3.0);
        let v2 = Vector::new(4.0, 3.0, 2.0);
        assert_eq!(v1 - v2, Vector::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_neg() {
        let v1 = Vector::new(1.0, 1.0, 1.0);
        assert_eq!(-v1, Vector::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_mul_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(v1 * v2, Vector::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let s: f64 = 2.5;
        let expected = Vector::new(2.5, 5.0, 7.5);
        assert_eq!(v1 * s, expected);
        assert_eq!(s * v1, expected);
    }

    #[test]
    fn test_div_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 8.0, 6.0);
        assert_eq!(v1 / v2, Vector::new(0.25, 0.25, 0.5));
    }

    #[test]
    fn test_div_scalar() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let s: f64 = 2.0;
        let expected = Vector::new(0.5, 1.0, 1.5);
        assert_eq!(v1 / s, expected);
    }
}
