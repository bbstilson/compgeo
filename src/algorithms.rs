use eframe::egui;

// https://en.wikipedia.org/wiki/Graham_scan
pub fn grahams_scan(points: &[egui::Vec2]) -> Vec<egui::Vec2> {
    if points.is_empty() {
        return vec![];
    }
    // The first step in this algorithm is to find the point with the lowest
    // y-coordinate. If the lowest y-coordinate exists in more than one point in the
    // set, the point with the lowest x-coordinate out of the candidates should be
    // chosen.
    let p = points
        .iter()
        .min_by(|p1, p2| p1.y.total_cmp(&p2.y).then_with(|| p1.x.total_cmp(&p2.x)))
        .unwrap();

    // Next, the set of points must be sorted in increasing order of the angle they and
    // the point P make with the x-axis.
    let mut points = points
        .to_vec()
        .into_iter()
        .filter(|o| o != p)
        .collect::<Vec<_>>();
    points.sort_by(|p1, p2| ccw(p, p1, p2).to_ordering());

    // For each point, it is first determined whether traveling from the two points
    // immediately preceding this point constitutes making a left turn or a right turn.
    // If a right turn, the second-to-last point is not part of the convex hull, and
    // lies 'inside' it. The same determination is then made for the set of the latest
    // point and the two points that immediately precede the point found to have been
    // inside the hull, and is repeated until a "left turn" set is encountered, at which
    // point the algorithm moves on to the next point in the set of points in the sorted
    // array minus any points that were found to be inside the hull; there is no need to
    // consider these points again. (If at any stage the three points are collinear, one
    // may opt either to discard or to report it, since in some applications it is
    // required to find all points on the boundary of the convex hull.)

    // This process will eventually return to the point at which it started, at which
    // point the algorithm is completed and the stack now contains the points on the
    // convex hull in counterclockwise order.
    let mut stack = vec![];
    for point in &points {
        while stack.len() > 1 && to_the_right(next_to_top(&stack), top(&stack), &point) {
            stack.pop();
        }
        stack.push(&point);
    }
    stack.into_iter().cloned().collect()
}

fn next_to_top<'a>(stack: &'a Vec<&'a egui::Vec2>) -> &'a egui::Vec2 {
    assert!(stack.len() > 1);
    &stack[stack.len() - 2]
}

fn top<'a>(stack: &'a Vec<&'a egui::Vec2>) -> &'a egui::Vec2 {
    assert!(stack.len() > 1);
    &stack[stack.len() - 1]
}

fn to_the_right(v1: &egui::Vec2, v2: &egui::Vec2, v3: &egui::Vec2) -> bool {
    match ccw(v1, v2, v3) {
        Direction::Collinear | Direction::Clockwise => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
    Collinear,
}

impl Direction {
    fn to_ordering(self) -> std::cmp::Ordering {
        match self {
            Direction::Clockwise => std::cmp::Ordering::Greater,
            Direction::CounterClockwise => std::cmp::Ordering::Less,
            Direction::Collinear => std::cmp::Ordering::Equal,
        }
    }
}

fn ccw(v1: &egui::Vec2, v2: &egui::Vec2, v3: &egui::Vec2) -> Direction {
    // Determining whether three points constitute a "left turn" or a "right
    // turn" does not require computing the actual angle between the two line segments,
    // and can actually be achieved with simple arithmetic only. For three points
    // P1 = (x1, y1) P2 = (x2, y2) and P3 = (x3, y3) compute the z-coordinate of the
    // cross product of the two vectors P1XP2 and P1XP3 which is given by the expression
    // (x2 - x1)(y3 - y1) - (y2 - y1)(x3 - x1). If the result is 0, the points are
    // collinear; if it is positive, the three points constitute a "left turn" or
    // counter-clockwise orientation, otherwise a "right turn" or clockwise orientation
    // (for counter-clockwise numbered points).

    let cross = (v2.x - v1.x) * (v3.y - v1.y) - (v2.y - v1.y) * (v3.x - v1.x);
    if cross == 0.0 {
        Direction::Collinear
    } else if cross > 0.0 {
        Direction::CounterClockwise
    } else {
        Direction::Clockwise
    }
}

#[test]
fn test_ccw() {
    //    v0
    //    /\
    //   /  \
    //  /    \
    // /  <-  \
    // v2     v1
    let v0 = egui::vec2(-0.9, -0.9);
    let v1 = egui::vec2(-0.2, -0.2);
    let v2 = egui::vec2(-0.2, -0.7);
    let ans = ccw(&v0, &v1, &v2);
    assert_eq!(ans, Direction::Clockwise);

    //    v0
    //    /\
    //   /  \
    //  /    \
    // /  ->  \
    // v1     v2
    let v0 = egui::vec2(-0.9, -0.9);
    let v1 = egui::vec2(-0.2, -0.7);
    let v2 = egui::vec2(-0.2, -0.2);
    let ans = ccw(&v0, &v1, &v2);
    assert_eq!(ans, Direction::CounterClockwise);
}
