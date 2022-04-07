mod color;

use crate::color::{Color, write_color};

fn main() {
    // Image

    let image_width = 256;
    let image_height = image_width;

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let b: f32 = 0.25;
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let pixel_color = Color::new((255.999 * r) as i32, (255.999 * g) as i32, (255.999 * b) as i32);
            write_color(&pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
