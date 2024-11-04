use crate::data::sphere::Sphere2;

use super::{simplex::Simplex, triangle::Triangle, Point};

pub struct Tetrahedron {
    pub vertices: [Point; 4],
}

impl Simplex for Tetrahedron {
    type Face = Triangle;
    type S = Sphere2;

    fn circumscribe(&self) -> Option<Self::S> {
        todo!()
    }

    fn dimension() -> u32 {
        3
    }

    fn faces(&self) -> Vec<Self::Face> {
        vec![
            Triangle {
                vertices: [self.vertices[0], self.vertices[1], self.vertices[2]],
            },
            Triangle {
                vertices: [self.vertices[0], self.vertices[1], self.vertices[3]],
            },
            Triangle {
                vertices: [self.vertices[0], self.vertices[2], self.vertices[3]],
            },
            Triangle {
                vertices: [self.vertices[1], self.vertices[2], self.vertices[3]],
            },
        ]
    }

    fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    fn volume(&self) -> f32 {
        unimplemented!()
    }
}
