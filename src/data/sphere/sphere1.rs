use core::f32;

use crate::data::point::Point2;

use super::Sphere;

#[derive(Debug, PartialEq, Clone, Copy)]
/// 2d circle
pub struct Sphere1 {
    pub radius: f32,
    pub center: Point2,
}

impl Sphere for Sphere1 {
    type P = Point2;

    fn area(&self) -> f32 {
        f32::consts::PI * self.radius.powf(2.0)
    }

    fn center(&self) -> Self::P {
        self.center
    }

    fn radius(&self) -> f32 {
        self.radius
    }
}
