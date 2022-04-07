use nalgebra::Unit;
use crate::point::Point;
use crate::direction::Direction;
use crate:: color::Color;

pub struct Ray {
    orig: Point,
    dir: Direction,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.orig + t*self.dir;
    }

    pub fn color(&self) -> Color {
        let unit_direction: Unit<Direction> = self.dir.unit();
        let t = 0.5*(unit_direction.y() + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
    }
}


// color ray_color(const ray& r) {
//     vec3 unit_direction = unit_vector(r.direction());
//     auto t = 0.5*(unit_direction.y() + 1.0);
//     return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
// }