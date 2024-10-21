use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub enum PolarDirection {
    CW,  // clockwise
    CCW, // counter clockwise
    Collinear,
}

impl Neg for PolarDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::CW => Self::CCW,
            Self::CCW => Self::CW,
            Self::Collinear => Self::Collinear,
        }
    }
}
