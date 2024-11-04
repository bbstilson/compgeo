use core::f32;

use crate::data::point::point3::Point3;

use super::Sphere;

/// 3d Sphere
pub struct Sphere2 {
    center: Point3,
    radius: f32,
}

impl Sphere for Sphere2 {
    type P = Point3;

    fn area(&self) -> f32 {
        f32::consts::PI * self.radius.powf(2.0) * 4.0
    }

    fn center(&self) -> Self::P {
        self.center
    }

    fn radius(&self) -> f32 {
        self.radius
    }
}
