mod color;
mod ray;
mod point;
mod direction;

use nalgebra::Vector3;
use crate::ray::Ray;
use crate::point::Point;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Point::new(viewport_width, 0.0, 0.0);
    let vertical = Point::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (0.5*horizontal) - (0.5*vertical) - Vector3::new(0.0, 0.0, focal_length);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {

            let u = i as f32 / (image_width-1) as f32;
            let v = j as f32 / (image_height-1) as f32;
            let r = Ray { orig: origin, dir: (lower_left_corner + u*horizontal + v*vertical - origin) };
            let pixel_color = r.color();

            color::write_color(&pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
