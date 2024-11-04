use crate::data::sphere::Sphere;

use super::Point;

pub trait Simplex {
    type Face: Simplex;
    type S: Sphere;

    fn dimension() -> u32;
    fn faces(&self) -> Vec<Self::Face>;
    fn vertices(&self) -> &[Point];
    fn volume(&self) -> f32;
    fn circumscribe(&self) -> Option<Self::S>;
}
