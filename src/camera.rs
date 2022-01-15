use crate::vec::Vector;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Vector,
    pub lower_left: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
}

impl Camera {
    pub fn new() -> Camera {
        // Camera parameters
        let aspect_ratio = 16.0f64 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        // Create instance.
        let origin = Vector::ORIGIN;
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);
        Camera { origin, lower_left, horizontal, vertical, }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new() {
        let camera = Camera::new();
        let width: f64 = 2.0 * 16.0 / 9.0;
        assert!(camera.origin  == Vector::ORIGIN);
        assert!(camera.horizontal == Vector::new(width, 0.0, 0.0));
        assert!(camera.vertical ==  Vector::new(0.0, 2.0, 0.0));
        assert!(camera.lower_left == Vector::new(-width /  2.0, -1.0, -1.0));
    }

    #[test]
    fn test_camera_get_ray() {
        let camera = Camera::new();
        let ray = camera.get_ray(0.5, 0.5);
        assert!(ray.origin == Vector::ORIGIN);
        assert!(ray.direction == Vector::new(0.0, 0.0, -1.0));
    }
}
