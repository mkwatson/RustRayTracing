use nalgebra::Vector3;

pub type Color = Vector3<f32>;

pub fn write_color(color: &Color)  {
    println!(
        "{} {} {}", 
        (255.99 * color.x) as i32, 
        (255.99 * color.y) as i32, 
        (255.99 * color.z) as i32
    );
}