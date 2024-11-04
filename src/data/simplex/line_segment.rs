use crate::data::sphere::Sphere1;

use super::{Point, Simplex};

pub struct LineSegment {
    pub vertices: [Point; 2],
}

impl LineSegment {
    pub fn intersection_point(&self, other: &Self) -> Option<Point> {
        let [a, b] = self.vertices;
        let [c, d] = other.vertices;
        // Line AB represented as a1x + b1y = c1
        let a1 = b.y - a.y;
        let b1 = a.x - b.x;
        let c1 = a1 * (a.x) + b1 * (a.y);

        // Line CD represented as a2x + b2y = c2
        let a2 = d.y - c.y;
        let b2 = c.x - d.x;
        let c2 = a2 * (c.x) + b2 * (c.y);

        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0.0 {
            // The lines are parallel.
            None
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            Some(Point { x, y })
        }
    }
}

impl Simplex for LineSegment {
    type Face = Point;
    type S = Sphere1;

    fn dimension() -> u32 {
        1
    }

    fn faces(&self) -> Vec<Self::Face> {
        vec![self.vertices[0], self.vertices[1]]
    }

    fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    fn volume(&self) -> f32 {
        let faces = self.faces();
        let a = faces[0];
        let b = faces[1];
        let x = b.x - a.x;
        let y = b.y - a.y;
        x.hypot(y)
    }

    fn circumscribe(&self) -> Option<Self::S> {
        // Cannot circumscribe in 2d (unless you're a fancy math chad)
        None
    }
}

#[test]
fn test_volume() {
    let a = Point { x: 2.0, y: 1.0 };
    let b = Point { x: 8.0, y: 4.0 };
    let line_ab = LineSegment { vertices: [a, b] };
    let line_ba = LineSegment { vertices: [b, a] };

    assert!((line_ab.volume() - 6.7082039325).abs() <= f32::EPSILON);
    assert!((line_ba.volume() - 6.7082039325).abs() <= f32::EPSILON);
}
