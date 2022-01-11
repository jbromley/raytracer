use raytracer::color::Color;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec::Vector;

fn ray_color(r: Ray) -> Color {
    let sphere = Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5);

    if sphere.hit(&r) {
        return Color::new(255, 0, 0)
    }

    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::lerp(Color::white(), Color::new(127, 178, 255), t)
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let horiz = Vector::new(viewport_width, 0.0, 0.0);
    let vert = Vector::new(0.0, viewport_height, 0.0);
    let lower_left = Vector::origin() - horiz / 2.0 - vert / 2.0 - Vector::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let r = Ray::new(Vector::origin(), lower_left + u * horiz + v * vert - Vector::origin());
            let c = ray_color(r);
            println!("{}", c);
        }
        eprintln!("{} scan lines remaining", y);
    }
}
