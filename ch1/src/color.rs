use nalgebra::Vector3;

pub type Color = Vector3<i32>;

pub fn write_color(color: &Color)  {
    println!("{} {} {}", color.x, color.y, color.z);
}