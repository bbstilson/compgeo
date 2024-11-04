use std::ops::Sub;

use super::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for Pos2 {
    fn into(self) -> Vec2 {
        let Pos2 { x, y } = self;
        Vec2 { x, y }
    }
}

pub fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2 { x, y }
}

impl Sub for Pos2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self { x: x1, y: y1 } = self;
        let Self { x: x2, y: y2 } = rhs;
        let x = x1 - x2;
        let y = y1 - y2;
        Self { x, y }
    }
}
