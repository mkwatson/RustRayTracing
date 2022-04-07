use nalgebra::{Vector3, magnitude};
use crate::direction::Direction;

pub type Direction = Vector3<i32>;

pub fn unit(dir: &Direction) -> Unit<Direction> {
    dir / magnitude(dir);
}