use eframe::egui;

use crate::graham_scan::helpers;

type Point = (String, egui::Pos2);

// https://en.wikipedia.org/wiki/Graham_scan
pub fn graham_scan(points: &[Point]) -> Vec<Point> {
    if points.is_empty() {
        return vec![];
    }

    // The first step in this algorithm is to find the point with the lowest
    // y-coordinate. If the lowest y-coordinate exists in more than one point in the
    // set, the point with the lowest x-coordinate out of the candidates should be
    // chosen.
    let p0 = points
        .iter()
        .min_by(|p1, p2| {
            p1.1.y
                .total_cmp(&p2.1.y)
                .then_with(|| p1.1.x.total_cmp(&p2.1.x))
        })
        .unwrap();

    println!("P0: {p0:?}");

    // Next, the set of points must be sorted in increasing order of the angle they and
    // the point P make with the x-axis.
    let mut points = points
        .to_vec()
        .into_iter()
        .filter(|o| o != p0)
        .collect::<Vec<_>>();

    graham_sort(p0.clone(), &mut points);

    println!("~~ sorted ~~");
    print_ps(&points);

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
    println!();
    let mut stack = vec![p0.clone()];
    for point in &points {
        println!("~~ stack ~~");
        print_ps(&stack);
        println!("checking: {point:?}");
        while stack.len() > 1 && helpers::to_the_right(&stack, point.clone()) {
            // pop the stack if we turn clockwise to reach this point
            println!("popped: {:?}", stack.pop().unwrap());
        }
        println!("pushing point: {point:?}");
        stack.push(point.clone());
        println!();
    }

    println!("~~ final ~~");
    print_ps(&stack);
    stack
}

fn print_ps(points: &[Point]) {
    for (n, p) in points {
        print!("{n} {p} ");
    }
    println!()
}

fn graham_sort(p0: Point, points: &mut [Point]) {
    points.sort_by(|a, b| {
        // Normalize points around p0.
        let a = egui::vec2(a.1.x - p0.1.x, a.1.y - p0.1.y)
            .normalized()
            .to_pos2();
        let b = egui::vec2(b.1.x - p0.1.x, b.1.y - p0.1.y)
            .normalized()
            .to_pos2();

        // x_axis vector from origin offset by lowest point
        let x_axis = egui::pos2(1.0 + p0.1.x, p0.1.y);
        let a1 = helpers::polar_angle(x_axis, p0.1, a);
        let a2 = helpers::polar_angle(x_axis, p0.1, b);
        if a1 <= a2 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
}

#[test]
fn test_graham_sort() {
    let bottom_left = ("bottom_left".to_string(), egui::pos2(-1.0, -1.0));
    let bottom_right = ("bottom_right".to_string(), egui::pos2(1.0, -1.0));
    let top_middle = ("top_middle".to_string(), egui::pos2(0.0, 1.0));
    let center = ("center".to_string(), egui::pos2(0.0, 0.0));
    let mut points = vec![center.clone(), bottom_right.clone(), top_middle.clone()];
    graham_sort(bottom_left, &mut points);
    assert_eq!(points, vec![bottom_right, center, top_middle]);

    let p0 = ("p0".to_string(), egui::pos2(-0.41119027, -0.31959605));
    let p1 = ("p1".to_string(), egui::pos2(-0.033056736, -0.1505971));
    let p2 = ("p2".to_string(), egui::pos2(0.22698152, 0.4522189));
    let p3 = ("p3".to_string(), egui::pos2(-0.034094572, 0.35310435));
    let p4 = ("p4".to_string(), egui::pos2(-0.3797356, 0.35341442));
    let mut points = vec![p4.clone(), p2.clone(), p1.clone(), p3.clone()];
    graham_sort(p0, &mut points);
    assert_eq!(points, vec![p1, p2, p3, p4]);
}

#[test]
fn test_graham_scan() {
    let bottom_left = ("a".to_string(), egui::pos2(-1.0, -1.0));
    let bottom_right = ("b".to_string(), egui::pos2(1.0, -1.0));
    let top_middle = ("c".to_string(), egui::pos2(0.0, 1.0));

    let points = vec![
        bottom_left.clone(),
        bottom_right.clone(),
        top_middle.clone(),
        ("d".to_string(), egui::pos2(0.0, 0.0)), // center
    ];

    assert_eq!(
        graham_scan(&points),
        vec![bottom_left, bottom_right, top_middle]
    );

    let p0 = ("p0".to_string(), egui::pos2(-0.41119027, -0.31959605));
    let p1 = ("p1".to_string(), egui::pos2(-0.033056736, -0.1505971));
    let p2 = ("p2".to_string(), egui::pos2(0.22698152, 0.4522189));
    let p3 = ("p3".to_string(), egui::pos2(-0.034094572, 0.35310435));
    let p4 = ("p4".to_string(), egui::pos2(-0.3797356, 0.35341442));

    let points = vec![p3.clone(), p1.clone(), p0.clone(), p4.clone(), p2.clone()];

    assert_eq!(graham_scan(&points), vec![p0, p1, p2, p4])
}
