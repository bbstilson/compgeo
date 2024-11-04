use super::point::Point;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn point2(x: f32, y: f32) -> Point2 {
    Point2 { x, y }
}

impl Point for Point2 {}
