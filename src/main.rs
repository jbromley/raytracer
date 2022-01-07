fn main() {
    let image_width = 256;
    let image_height = 256;

    // PPM file header.
    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let r = x as f64 / ((image_width - 1) as f64);
            let g = y as f64 / ((image_height - 1) as f64);
            let b = 0.25;

            let r = (255.9999 * r) as i32;
            let g = (255.9999 * g) as i32;
            let b = (255.9999 * b) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}
