use std::ops::{Add, Mul};

use crate::data::sphere::Sphere1;

use super::simplex::Simplex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Simplex for Point {
    type Face = Point;
    type S = Sphere1;

    fn dimension() -> u32 {
        0
    }

    fn faces(&self) -> Vec<Self::Face> {
        vec![]
    }

    fn vertices(&self) -> &[Point] {
        &[]
    }

    fn volume(&self) -> f32 {
        0.0
    }

    fn circumscribe(&self) -> Option<Self::S> {
        // Cannot circumscribe in 1d (unless you're a fancy math chad)
        None
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
