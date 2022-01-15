use std::sync::Arc;
use std::sync::mpsc::channel;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use threadpool::ThreadPool;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::image::ImagePpm;
use raytracer::ray::{Hittable, Ray, HitRecord};
use raytracer::sphere::Sphere;
use raytracer::vec::Vector;

fn ray_color(r: Ray, world: &[Sphere], depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    if let Some(hit) = hit_world(world, &r, 0.001, f64::INFINITY) {
        let target = hit.p + Vector::random_in_hemisphere(&hit.n);
        // let target = hit.p + hit.n + Vector::random_unit_vector();
        return 0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1)
    }

    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::lerp(Color::WHITE, Color::BACKGROUND, t)
}

fn hit_world(world: &[Sphere], r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
    let image_width: u32 = 1600;
    let image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 64;
    let max_depth = 32;

    // Random number generator
    // let mut rng = rand::thread_rng();
    let dist = Uniform::new(-0.5, 0.5);

    // World
    let world  = Arc::new(vec![
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0),
    ]);

    // Camera
    let camera = Camera::new();

    // Render
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    eprint!("Rendering {} x {}", image_width, image_height);
    let start = Instant::now();
    let mut img = ImagePpm::new(image_width, image_height);

    for y in 0..image_height {
        let tx = tx.clone();
        let w = Arc::clone(&world);
        pool.execute(move || {
            let mut rng = rand::thread_rng();
            for x in 0..image_width {
                let mut c = Color::BLACK;
                for _ in 0..samples_per_pixel {
                    let u = ((x as f64) + dist.sample(&mut rng)) / (image_width - 1) as f64;
                    let v = ((y as f64) + dist.sample(&mut rng)) / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);

                    c += ray_color(r, &w, max_depth);
                }
                c /= samples_per_pixel as f64;
                tx.send((x, y, c.gamma_correct())).expect("Could not set pixel data");
            }
        });
    }
    drop(tx);

    let progress_period = image_width * image_height / 50;
    let mut num_pixels = 0;
    for (x, y, pixel) in rx.iter() {
        img.set_pixel(x, y, pixel);
        num_pixels += 1;
        if num_pixels % progress_period == 0 {
            eprint!(".");
        }
    }

    eprintln!("rendering done in {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    eprint!("Writing image...");
    match img.write() {
        Ok(_) => eprintln!("done in {} ms.", start.elapsed().as_millis()),
        Err(e) => eprintln!("error writing image: {}", e),
    };
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
