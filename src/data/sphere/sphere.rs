use crate::data::point::point::Point;

pub trait Sphere {
    type P: Point;

    fn area(&self) -> f32;
    fn center(&self) -> Self::P;
    fn radius(&self) -> f32;
}
