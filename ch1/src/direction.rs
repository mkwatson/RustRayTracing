use nalgebra::{Vector3, Unit};

pub type Direction = Vector3<f32>;

pub fn unit(dir: &Direction) -> Unit<Direction> {
    Unit::new_normalize(Direction::clone(dir))
}