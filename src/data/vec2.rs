use std::ops::{Div, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    // returns the magnitude of the vector that would result from a regular 3D cross
    // product of the input vectors, taking their Z values implicitly as 0
    #[inline]
    pub fn cross(self, other: Self) -> f32 {
        let Self { x: x0, y: y0 } = self; // b
        let Self { x: x1, y: y1 } = other; // c
        (x0 * y1) - (y0 * x1)
    }

    /// The dot-product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    #[inline(always)]
    pub fn perpendicular(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Safe normalize: returns zero if input is zero.
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        //
        let len = self.length();
        if len <= 0.0 {
            self
        } else {
            self / len
        }
    }

    /// Translates `self` relative to `other`.
    #[must_use]
    #[inline(always)]
    pub fn translate(self, other: Self) -> Self {
        let Self { x: x1, y: y1 } = self;
        let Self { x: x0, y: y0 } = other;
        Self {
            x: x1 - x0,
            y: y1 - y0,
        }
    }
}

/// Element-wise division
impl Div<Self> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, factor: f32) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
