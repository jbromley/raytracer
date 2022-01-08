use raytracer::color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let w = (image_width - 1) as f64;
    let h = (image_height - 1) as f64;

    // PPM file header.
    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let c = Color::from_float(x as f64 / w, y as f64 / h, 0.25f64);
            println!("{}", c);
        }
        eprintln!("{} scan lines remaining", y);
    }
}
