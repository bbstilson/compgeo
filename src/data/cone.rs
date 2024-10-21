use crate::data::{PolarDirection, Vec2};

// Cone example:
// [ABC] ==
//        A
//        /
//       /
//      /
//     /_____
//    B      C
#[derive(Debug, Clone, Copy, Default)]
pub struct Cone {
    pub a: Vec2,
    pub b: Vec2,
    pub origin: Vec2,
}

impl Cone {
    /// Returns the cosine angle of the cone.
    pub fn angle(self) -> f32 {
        let Self { a, b, origin } = self;
        // Translate to the origin.
        let a = a.translate(origin);
        let b = b.translate(origin);
        // https://en.wikipedia.org/wiki/Dot_product#Application_to_the_law_of_cosines
        a.dot(b) / (a.length() * b.length())
    }

    pub fn angle_direction(self) -> PolarDirection {
        // For three points if the polar angle is 0.0, the points are collinear;
        // if it is positive, the three points constitute a "left turn" or
        // counter-clockwise orientation, otherwise a "right turn" or clockwise
        // orientation (for counter-clockwise numbered points).
        let Self { a, b, origin } = self;
        // Translate to the origin.
        let a = a.translate(origin);
        let b = b.translate(origin);
        let angle = b.cross(a);
        if angle == 0.0 {
            PolarDirection::Collinear
        } else if angle <= 0.0 {
            PolarDirection::CW
        } else {
            PolarDirection::CCW
        }
    }
}

impl std::fmt::Display for Cone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} -> {}", self.a, self.origin, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::vec2;

    use super::*;

    #[test]
    fn test_get_angle() {
        assert_eq!(
            // 3-4-5 triangle
            Cone {
                a: vec2(3.0, 0.0),
                b: vec2(0.0, 4.0),
                ..Cone::default()
            }
            .angle(),
            // 6-8-10 triangle
            Cone {
                a: vec2(6.0, 0.0),
                b: vec2(0.0, 8.0),
                ..Cone::default()
            }
            .angle()
        )
    }

    #[test]
    fn test_angle_direction() {
        // test colinear
        let ans = Cone {
            a: vec2(1.0, 0.0),
            b: vec2(0.87, 0.0),
            ..Default::default()
        }
        .angle_direction();

        assert_eq!(ans, PolarDirection::Collinear);

        let ans = Cone {
            a: vec2(0.0, 1.0),
            b: vec2(0.0, 0.87),
            ..Default::default()
        }
        .angle_direction();

        assert_eq!(ans, PolarDirection::Collinear);

        //    p0
        //    /\
        //   /  \
        //  /    \
        // /  <-  \
        // p2     p1
        let ans = Cone {
            a: vec2(-0.2, -0.2),
            b: vec2(-0.2, -0.7),
            origin: vec2(-0.9, -0.9),
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::CCW);

        //    p0
        //    /\
        //   /  \
        //  /    \
        // /  ->  \
        // p1     p2
        let ans = Cone {
            a: vec2(-0.2, -0.7),
            b: vec2(-0.2, -0.2),
            origin: vec2(-0.9, -0.9),
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::CW);

        // https://en.wikipedia.org/wiki/File:Graham_Scan.svg
        let p = vec2(0.0, -0.5);
        let a = vec2(0.5, -0.2);
        let b = vec2(0.2, 0.4);
        let c = vec2(0.0, 0.0);
        let d = vec2(-0.5, 0.3);
        let pab = Cone { a: p, b, origin: a }.angle_direction();
        assert_eq!(pab, PolarDirection::CCW);
        let abc = Cone { a, b: c, origin: b }.angle_direction();
        assert_eq!(abc, PolarDirection::CCW);
        let bcd = Cone {
            a: b,
            b: d,
            origin: c,
        }
        .angle_direction();
        assert_eq!(bcd, PolarDirection::CW);

        //      p2
        //      /
        //     /
        // p1 /
        //   |
        //   |
        //   |
        //   p0
        let ans = Cone {
            a: vec2(0.0, -1.0),
            b: vec2(0.5, 1.0),
            ..Default::default()
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::CW);

        //   p1
        //   |\
        //   | \
        //   |  \
        //   p0  p2
        let ans = Cone {
            a: vec2(0.0, -1.0),
            b: vec2(0.01, -1.0),
            ..Default::default()
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::CW);

        // p0 ____ p1
        //       /
        //      /
        //     /
        //    /
        //  p2
        let ans = Cone {
            origin: vec2(1.0, 0.0),
            b: vec2(0.0, -1.0),
            ..Default::default()
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::CW);

        // p2 |
        //    |
        //    |
        // p1 |
        //    |
        //    |
        // p0 |
        let ans = Cone {
            a: vec2(0.0, -1.0),
            b: vec2(0.0, 1.0),
            ..Default::default()
        }
        .angle_direction();
        assert_eq!(ans, PolarDirection::Collinear);
    }
}
