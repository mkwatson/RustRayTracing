use nalgebra::Unit;
use crate::point::Point;
use crate::direction::{Direction, unit};
use crate::color::Color;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = r.orig - center;
    let a = r.dir.magnitude().powi(2);
    let half_b = oc.dot(&r.dir);
    let c = oc.magnitude().powi(2) - radius*radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt() ) / a;
    }
}
pub struct Ray {
    pub orig: Point,
    pub dir: Direction,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.orig + t*self.dir
    }

    pub fn color(&self) -> Color {
        let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, &self);
        if t > 0.0 {
            let n = unit(&(self.at(t) - Color::new(0.0, 0.0, -1.0)));
            return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
        let unit_direction: Unit<Direction> = unit(&self.dir);
        let t = 0.5*(unit_direction.y + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}
