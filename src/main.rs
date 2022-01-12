use std::time::Instant;

use raytracer::color::Color;
use raytracer::ray::{Hittable, Ray, HitRecord};
use raytracer::sphere::Sphere;
use raytracer::vec::Vector;

fn ray_color(r: Ray, world: &Vec<Sphere>) -> Color {
    match hit_world(world, &r, 0.0, f64::INFINITY) {
        Some(hit_record) => {
            Color::from_normal(&hit_record.n)
        },
        None => {
            let unit_dir = r.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            Color::lerp(Color::WHITE, Color::BACKGROUND, t)
        }
    }
}

fn hit_world(world: &Vec<Sphere>, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest) {
            closest = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // World
    let world  = vec![
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let horiz = Vector::new(viewport_width, 0.0, 0.0);
    let vert = Vector::new(0.0, viewport_height, 0.0);
    let lower_left = Vector::ORIGIN - horiz / 2.0 - vert / 2.0 - Vector::new(0.0, 0.0, focal_length);

    // Render
    eprint!("Rendering {} x {}", image_width, image_height);
    let start = Instant::now();
    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let r = Ray::new(Vector::ORIGIN, lower_left + u * horiz + v * vert - Vector::ORIGIN);
            let c = ray_color(r, &world);
            println!("{}", c);
        }
        if y % 10 == 0 {
            eprint!(".");
        }
    }
    eprintln!("done.");
    eprintln!("{} ms elapsed", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use raytracer::camera::Camera;
    use super::*;

    #[test]
    fn test_hit_world() {
        let camera = Camera::new();
        let world = vec![
            Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
            Sphere::new(Vector::new(0.0, 0.0, -2.0), 0.5),
        ];
        let expected = HitRecord {
            p: Vector::new(0.0, 0.0, -0.5),
            n: Vector::new(0.0, 0.0, 1.0),
            t: 0.5,
            front_face: true,
        };
        let ray = camera.get_ray(0.5, 0.5);
        let hit_record = hit_world(&world, &ray, 0.0, f64::INFINITY).expect("no hit record returned");
        assert!(hit_record.p == expected.p);
        assert!(hit_record.n == expected.n);
        assert!(hit_record.t == 0.5);
        assert!(hit_record.front_face);

        let ray = camera.get_ray(0.0, 0.0);
        let hit_record = hit_world(&world, &camera.get_ray(0.0, 0.0), 0.0, f64::INFINITY);
        match hit_record {
            Some(_hit) => panic!("ray {:?} should not have hit", ray),
            None => (),
        }
    }
}
