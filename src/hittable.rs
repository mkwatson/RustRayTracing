use crate::point::Point;
use crate::direction::Direction;
use crate::ray::Ray;

pub struct Hit_Record {
    pub p: Point,
    pub normal: Direction,
    pub t: f32
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &Hit_Record) -> Bool;
}