use eframe::egui;

pub fn to_the_right(stack: &[(String, egui::Pos2)], p2: (String, egui::Pos2)) -> bool {
    let p0 = next_to_top(&stack);
    let p1 = top(&stack);
    let ans = angle_direction([p0, p1, p2]);
    println!("{ans:?}");
    match ans {
        Direction::Collinear | Direction::CW => true,
        Direction::CCW => false,
    }
}

fn next_to_top<T: Copy>(stack: &[(String, T)]) -> (String, T) {
    assert!(stack.len() > 1);
    stack[stack.len() - 2].clone()
}

#[test]
fn test_next_to_top() {
    assert_eq!(
        next_to_top(&vec![
            ("A".to_string(), 1),
            ("B".to_string(), 2),
            ("C".to_string(), 3)
        ]),
        ("B".to_string(), 2)
    );
}

fn top<T: Copy>(stack: &[(String, T)]) -> (String, T) {
    assert!(stack.len() > 1);
    stack[stack.len() - 1].clone()
}

#[test]
fn test_top() {
    assert_eq!(
        top(&vec![
            ("A".to_string(), 1),
            ("B".to_string(), 2),
            ("C".to_string(), 3)
        ]),
        ("C".to_string(), 3)
    );
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    CW,  // clockwise
    CCW, // counter clockwise
    Collinear,
}

pub fn polar_angle(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2) -> f32 {
    // Compute the z-coordinate of the cross product of the two vectors
    // P0 X P1 and P0 X P2 which is given by the expression:
    // (x1 - x0)(y2 - y0) - (y1 - y0)(x2 - x0).
    (p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x)
}

pub fn angle_direction(ps: [(String, egui::Pos2); 3]) -> Direction {
    println!("{ps:?}");
    let [p0, p1, p2] = ps;
    // For three points if the polar angle is 0.0, the points are collinear;
    // if it is positive, the three points constitute a "left turn" or
    // counter-clockwise orientation, otherwise a "right turn" or clockwise
    // orientation (for counter-clockwise numbered points).
    let angle = polar_angle(p0.1, p1.1, p2.1);
    // println!("{}")
    if angle == 0.0 {
        Direction::Collinear
    } else if angle < 0.0 {
        Direction::CW
    } else {
        Direction::CCW
    }
}

#[test]
fn test_angle_direction() {
    // test colinear
    let ans = angle_direction([
        ("a".to_string(), egui::pos2(0.0, 0.0)),
        ("b".to_string(), egui::pos2(1.0, 0.0)),
        ("c".to_string(), egui::pos2(0.87, 0.0)),
    ]);
    assert_eq!(ans, Direction::Collinear);

    let ans = angle_direction([
        ("a".to_string(), egui::pos2(0.0, 0.0)),
        ("b".to_string(), egui::pos2(0.0, 1.0)),
        ("c".to_string(), egui::pos2(0.0, 0.87)),
    ]);
    assert_eq!(ans, Direction::Collinear);

    //  anchor
    //    /\
    //   /  \
    //  /    \
    // /  <-  \
    // p2     p1
    let p0 = egui::pos2(-0.9, -0.9);
    let p1 = egui::pos2(-0.2, -0.2);
    let p2 = egui::pos2(-0.2, -0.7);
    let ans = angle_direction([
        ("p0".to_string(), p0),
        ("p2".to_string(), p2),
        ("p1".to_string(), p1),
    ]);
    assert_eq!(ans, Direction::CW);

    //  p0
    //    /\
    //   /  \
    //  /    \
    // /  ->  \
    // p1     p2
    let ans = angle_direction([
        ("p0".to_string(), p0),
        ("p1".to_string(), p1),
        ("p2".to_string(), p2),
    ]);
    assert_eq!(ans, Direction::CCW);

    // https://en.wikipedia.org/wiki/File:Graham_Scan.svg
    let p = egui::pos2(0.0, -0.5);
    let a = egui::pos2(0.5, -0.2);
    let b = egui::pos2(0.2, 0.4);
    let c = egui::pos2(0.0, 0.0);
    let d = egui::pos2(-0.5, 0.3);
    assert_eq!(
        angle_direction([
            ("p".to_string(), p),
            ("a".to_string(), a),
            ("b".to_string(), b)
        ]),
        Direction::CCW
    );
    assert_eq!(
        angle_direction([
            ("a".to_string(), a),
            ("b".to_string(), b),
            ("c".to_string(), c)
        ]),
        Direction::CCW
    );
    assert_eq!(
        angle_direction([
            ("b".to_string(), b),
            ("c".to_string(), c),
            ("d".to_string(), d)
        ]),
        Direction::CW
    );
    // Quadrant 1
    // p1 _______ p2
    //   |
    //   |
    //   |
    //   |
    //   p0
    let p0 = egui::pos2(0.0, 0.0);
    let p1 = egui::pos2(0.0, 1.0);
    let p2 = egui::pos2(1.0, 1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // Quadrant 2
    // p1 _______ p2
    //   |
    //   |
    //   |
    //   |
    //   p0
    let p0 = egui::pos2(-1.0, 0.0);
    let p1 = egui::pos2(-1.0, 1.0);
    let p2 = egui::pos2(0.0, 1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // Quadrant 3
    // p1 _______ p2
    //   |
    //   |
    //   |
    //   |
    //   p0
    let p0 = egui::pos2(-1.0, -1.0);
    let p1 = egui::pos2(-1.0, 0.0);
    let p2 = egui::pos2(0.0, 0.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // Quadrant 4
    // p1 _______ p2
    //   |
    //   |
    //   |
    //   |
    //   p0
    let p0 = egui::pos2(0.0, -1.0);
    let p1 = egui::pos2(0.0, 0.0);
    let p2 = egui::pos2(1.0, 0.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    //      p2
    //      /
    //     /
    // p1 /
    //   |
    //   |
    //   |
    //   p0
    let p0 = egui::pos2(0.0, -1.0);
    let p1 = egui::pos2(0.0, 0.0);
    let p2 = egui::pos2(0.5, 1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // p1
    //   |\
    //   | \
    //   |  \
    //   p0  p2
    let p0 = egui::pos2(0.0, -1.0);
    let p1 = egui::pos2(0.0, 0.0);
    let p2 = egui::pos2(0.01, -1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // p0 ____ p1
    //       /
    //      /
    //     /
    //    /
    //  p2
    let p0 = egui::pos2(0.0, 0.0);
    let p1 = egui::pos2(1.0, 0.0);
    let p2 = egui::pos2(0.0, -1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::CW
    );

    // p2 |
    //    |
    //    |
    // p1 |
    //    |
    //    |
    // p0 |
    let p0 = egui::pos2(0.0, -1.0);
    let p1 = egui::pos2(0.0, 0.0);
    let p2 = egui::pos2(0.0, 1.0);
    assert_eq!(
        angle_direction([
            ("p0".to_string(), p0),
            ("p1".to_string(), p1),
            ("p2".to_string(), p2)
        ]),
        Direction::Collinear
    );
}
