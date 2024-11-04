use crate::data::{point::Point2, sphere::Sphere1};

use super::{line_segment::LineSegment, simplex::Simplex, Point};

pub struct Triangle {
    pub vertices: [Point; 3],
}

impl Simplex for Triangle {
    type Face = LineSegment;
    type S = Sphere1;

    fn circumscribe(&self) -> Option<Self::S> {
        // https://en.wikipedia.org/wiki/Circumcircle#Circumcenter_coordinates
        let [a, b, c] = self.vertices;
        let Point { x: ax, y: ay } = a;
        let Point { x: bx, y: by } = b;
        let Point { x: cx, y: cy } = c;
        let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
        let ux = ((ax * ax + ay * ay) * (by - cy)
            + (bx * bx + by * by) * (cy - ay)
            + (cx * cx + cy * cy) * (ay - by))
            / d;
        let uy = ((ax * ax + ay * ay) * (cx - bx)
            + (bx * bx + by * by) * (ax - cx)
            + (cx * cx + cy * cy) * (bx - ax))
            / d;
        let center = Point2 { x: ux, y: uy };
        let x = ux - a.x;
        let y = b.y - uy;
        let radius = x.hypot(y);
        Some(Sphere1 { radius, center })
    }

    fn dimension() -> u32 {
        2
    }

    fn faces(&self) -> Vec<Self::Face> {
        vec![
            LineSegment {
                vertices: [self.vertices[0], self.vertices[1]],
            },
            LineSegment {
                vertices: [self.vertices[1], self.vertices[2]],
            },
            LineSegment {
                vertices: [self.vertices[0], self.vertices[2]],
            },
        ]
    }

    fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    fn volume(&self) -> f32 {
        let faces = self.faces();
        let a = faces[0].volume();
        let b = faces[1].volume();
        let c = faces[2].volume();
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

#[test]
fn test_volume() {
    let a = Point { x: 1.0, y: 3.0 };
    let b = Point { x: 3.0, y: 5.0 };
    let c = Point { x: 7.0, y: 2.0 };
    let t = Triangle {
        vertices: [a, b, c],
    };

    let expected = 7.0;
    let actual = t.volume();

    assert!(
        (actual - expected).abs() <= f32::EPSILON * 10.0,
        "Expected {expected} got {actual}"
    );
}
