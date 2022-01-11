use crate::ray::{Ray, Hittable, HitRecord};
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
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root1 = (-half_b - sqrtd) / a;
            let root2 = (-half_b + sqrtd) / a;
            for root in [root1, root2].iter() {
                if t_min < *root && * root < t_max {
                    let p = ray.at(*root);
                    let normal = (p - self.center) / self.radius;
                    return Some(HitRecord::from_ray(*ray, normal, *root))
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vector::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vector::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
        assert_eq!(hit.unwrap().t, 4.0);
    }
}
