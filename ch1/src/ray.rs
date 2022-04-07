use nalgebra::Unit;
use crate::point::Point;
use crate::direction::{Direction, unit};
use crate::color::Color;

pub struct Ray {
    pub orig: Point,
    pub dir: Direction,
}

impl Ray {
    // pub fn at(&self, t: f32) -> Point {
    //     self.orig + t*self.dir
    // }

    pub fn color(&self) -> Color {
        let unit_direction: Unit<Direction> = unit(&self.dir);
        let t = 0.5*(unit_direction.y + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}
