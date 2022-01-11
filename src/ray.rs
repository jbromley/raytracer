use crate::vec::Vector;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction, }
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vector,
    pub n: Vector,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vector, n: Vector, t: f64, front_face: bool) -> HitRecord {
        HitRecord { p, n, t, front_face, }
    }

    pub fn from_ray(r: Ray, outward_normal: Vector, t: f64) -> HitRecord {
        let front_face = r.direction * outward_normal < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord {
            p: r.at(t),
            n: normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new() {
        let v1 = Vector::new(0.1, 0.2, 0.3);
        let v2 = Vector::new(0.2, 0.3, 0.4);

        let r = Ray::new(v1, v2);

        assert_approx_eq!(r.origin.x, 0.1);
        assert_approx_eq!(r.origin.y, 0.2);
        assert_approx_eq!(r.origin.z, 0.3);
        assert_approx_eq!(r.direction.x, 0.2);
        assert_approx_eq!(r.direction.y, 0.3);
        assert_approx_eq!(r.direction.z, 0.4);
    }

    #[test]
    fn test_ray_at() {
        let o = Vector::new(1.0, 1.0, 1.0);
        let d = Vector::new(1.0, 2.0, 3.0);
        let r = Ray::new(o, d);
        assert_eq!(r.at(0.5), Vector::new(1.5, 2.0, 2.5));
    }
}
