use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub enum Direction {
    CW,  // clockwise
    CCW, // counter clockwise
    Collinear,
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::CW => Self::CCW,
            Self::CCW => Self::CW,
            Self::Collinear => Self::Collinear,
        }
    }
}
