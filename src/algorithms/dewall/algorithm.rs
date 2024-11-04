use std::collections::HashSet;

use crate::data::{simplex::Simplex, Vec2};

// DeWall: A Fast Divide & Conquer
// Delaunay Triangulation Algorithm in Ed
// P. Cignoniz, C. Montaniz, R. Scopigno
pub fn dewall(points: &[Vec2]) -> Vec<Vec2> {
    // The DeWall (Delaunay Wall) algorithm consists of the following steps:
    // 1) Select the dividing plane a, split P into the two subsets P1 and P2 and construct Σa.
    // 2) Starting from Σa, recursively apply DeWall on P1 and P2 to build Σ1 and Σ2.
    // 3) Return the union of Σa, Σ1, and Σ2.

    // The simplex wall can be simply computed by using an incremental construction
    // approach: a starting simplex is individuated and then Σ is built by adding a new
    // simplex at each step and without having to modify the current triangulation.

    // The incremental construction approach can be easily generalized to Ed
    // triangulations: for each (d-1)-face f, which does not lie on the ConvexHull(P),
    // there are exactly two simplices σ1 and σ2 in Σ, such that σ1 and σ2 share the
    // (d-1)-face f. The algorithm starts by constructing an initial simplex σi ; then,
    // it processes all of the (d-1)-faces of σi: the simplex adjacent to each of them
    // (if it exists, i.e. the face does not belong to the Convex Hull of P) is built
    // and added to the current list of simplices in Σ. All of the new (d-1)-faces of
    // each new simplex are used to up date a data structure, here called Active Face
    // List (AFL). Update of the AFL is as follows: if a new face is already contained
    // in AFL, then it is removed from AFL; otherwise, it is inserted in AFL because its
    // adjacent simplex has not yet been built. The process continues iteratively
    // (extract a face f from AFL, build the simplex σ adjacent to f, update the AFL
    // with the (d-1)-faces of σ, and then again extract another face from AFL) until
    // AFL is empty.

    todo!();
    // Active Face List
    let afl = vec![1];
}

/// Produces a Delaunay d-simplex which is intersected by the plane α.
fn make_first_simplex(points: &[Vec2], a: [Vec2; 2]) {
    // Selects the point p1 ∈ P nearest to the plane. It then selects a second point p2
    // such that p2 is the nearest point to p1 on the other side of α. Then, it searches
    // the point p3 such that the circum-circle around the 1-face (p1, p2) and the point
    // p3 has the minimum radius; (p1, p2, p3) is therefore a 2-face of Σ. The process
    // continues until the required d-simplex is built.
}

/// Given a face f, build the adjacent simplex by applying the DT definition.
fn make_simplex<F: Simplex>(f: F, points: &[Vec2]) {
    // For each point p ∈ P, compute the radius of the hypersphere which circumscribes p
    // and the face f. We choose the point p which, generally speaking, minimizes this
    // radius to build the simplex adjacent to f.

    // selects the point p which minimizes the function dd (Delaunay distance)
}

// Returns the halfspace which contains the new tetrahedra
fn halfspace(f: f32, p: f32) -> HashSet<()> {
    todo!()
}

fn delaunay_distance(f: f32, p: f32) -> f32 {
    // r and c the radius and the center of the circumsphere around f and p
    let radius = 0.0;
    let center = 0.0;
    // if halfspace(f, p).contains(c) {
    //     radius
    // } else {
    //     -radius
    // }
    radius
}

// Σ == Simplex Set?
// Σ
// σ == simplex
// α
// ∈ == element of
// ∉ == not element of
