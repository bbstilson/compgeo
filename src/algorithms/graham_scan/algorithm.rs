use super::helpers;
use crate::data::{pos2, vec2, Cone, Pos2};

// https://en.wikipedia.org/wiki/Graham_scan
pub fn graham_scan(points: &[Pos2]) -> Vec<Pos2> {
    if points.is_empty() {
        return vec![];
    }

    // The first step in this algorithm is to find the point with the lowest
    // y-coordinate. If the lowest y-coordinate exists in more than one point in the
    // set, the point with the lowest x-coordinate out of the candidates should be
    // chosen.
    let p0 = points
        .iter()
        .min_by(|p1, p2| p1.y.total_cmp(&p2.y).then_with(|| p1.x.total_cmp(&p2.x)))
        .unwrap();

    // Next, the set of points must be sorted in increasing order of the angle they and
    // the point P make with the x-axis.
    let mut points = points
        .to_vec()
        .into_iter()
        .filter(|o| o != p0)
        .collect::<Vec<_>>();

    graham_sort(p0.clone(), &mut points);

    // For each point, it is first determined whether traveling from the two points
    // immediately preceding this point constitutes making a left turn or a right turn.
    // If a right turn, the second-to-last point is not part of the convex hull, and
    // lies 'inside' it. The same determination is then made for the set of the latest
    // point and the two points that immediately precede the point found to have been
    // inside the hull, and is repeated until a "left turn" set is encountered, at which
    // point the algorithm moves on to the next point in the set of points in the sorted
    // array minus any points that were found to be inside the hull; there is no need to
    // consider these points again.

    // This process will eventually return to the point at which it started, at which
    // point the algorithm is completed and the stack now contains the points on the
    // convex hull in counterclockwise order.
    let mut stack = vec![p0.clone()];
    for point in &points {
        while stack.len() > 1 && helpers::to_the_right(&stack, point.clone()) {
            // pop the stack if we turn clockwise to reach this point
            stack.pop();
        }
        stack.push(point.clone());
    }

    stack
}

fn graham_sort(p0: Pos2, points: &mut [Pos2]) {
    let p0 = p0;
    points.sort_by(|a, b| {
        let x_axis = pos2(1.0 + p0.x, p0.y);

        let a1 = Cone {
            a: (*a).into(),
            b: x_axis.into(),
            origin: p0.into(),
        };
        let a2 = Cone {
            a: (*b).into(),
            b: x_axis.into(),
            origin: p0.into(),
        };

        if a1.angle() <= a2.angle() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
}

#[test]
fn test_graham_sort() {
    let bottom_left = pos2(-1.0, -1.0);
    let bottom_right = pos2(1.0, -1.0);
    let top_middle = pos2(0.0, 1.0);
    let center = pos2(0.0, 0.0);
    let mut points = vec![center.clone(), bottom_right.clone(), top_middle.clone()];
    graham_sort(bottom_left, &mut points);
    assert_eq!(points, vec![bottom_right, center, top_middle]);

    let p0 = pos2(-0.41119027, -0.31959605);
    let p1 = pos2(-0.033056736, -0.1505971);
    let p2 = pos2(0.22698152, 0.4522189);
    let p3 = pos2(-0.034094572, 0.35310435);
    let p4 = pos2(-0.3797356, 0.35341442);
    let mut points = vec![p4.clone(), p2.clone(), p1.clone(), p3.clone()];
    graham_sort(p0, &mut points);
    assert_eq!(points, vec![p1, p2, p3, p4]);

    let p0 = pos2(0.2, -0.3);
    let p1 = pos2(-0.45289695, -0.099212766);
    let p2 = pos2(-0.18725193, -0.058339);
    let p3 = pos2(-0.26800287, 0.27599692);
    let p4 = pos2(0.03216493, 0.38522828);
    let mut points = vec![p3.clone(), p1.clone(), p4.clone(), p2.clone()];
    graham_sort(p0, &mut points);
    assert_eq!(points, vec![p4, p3, p2, p1,]);
}

#[test]
fn test_graham_scan() {
    let bottom_left = pos2(-1.0, -1.0);
    let bottom_right = pos2(1.0, -1.0);
    let top_middle = pos2(0.0, 1.0);
    let points = vec![
        bottom_left.clone(),
        bottom_right.clone(),
        top_middle.clone(),
        pos2(0.0, 0.0), // centr
    ];
    assert_eq!(
        graham_scan(&points),
        vec![bottom_left, bottom_right, top_middle]
    );

    let p0 = pos2(-0.41119027, -0.31959605);
    let p1 = pos2(-0.033056736, -0.1505971);
    let p2 = pos2(0.22698152, 0.4522189);
    let p3 = pos2(-0.034094572, 0.35310435);
    let p4 = pos2(-0.3797356, 0.35341442);
    let points = vec![p3.clone(), p1.clone(), p0.clone(), p4.clone(), p2.clone()];
    assert_eq!(graham_scan(&points), vec![p0, p1, p2, p4]);

    let p0 = pos2(0.2, -0.3);
    let p1 = pos2(-0.45289695, -0.099212766);
    let p2 = pos2(-0.18725193, -0.058339);
    let p3 = pos2(-0.26800287, 0.27599692);
    let p4 = pos2(0.03216493, 0.38522828);
    let points = vec![p3.clone(), p1.clone(), p0.clone(), p4.clone(), p2.clone()];
    assert_eq!(graham_scan(&points), vec![p0, p4, p3, p1]);
}
