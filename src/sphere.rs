use crate::ray::Ray;
use crate::vec::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
