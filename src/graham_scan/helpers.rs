use crate::graham_scan::{direction::Direction, vec2::Vec2};

use super::cone::Cone;

type Point<'a> = (&'a str, Vec2);

pub fn to_the_right<'a>(stack: &[Point<'a>], p2: Point<'a>) -> bool {
    let p0 = next_to_top(&stack);
    let p1 = top(&stack);
    let cone = Cone {
        a: p0.1,
        b: p2.1,
        origin: p1.1,
    };
    let ans = cone.angle_direction();
    println!("{ans:?}");
    match ans {
        Direction::Collinear | Direction::CW => true,
        Direction::CCW => false,
    }
}

fn next_to_top<'a, T: Clone>(stack: &'a [(&str, T)]) -> (&'a str, T) {
    assert!(stack.len() > 1);
    stack[stack.len() - 2].clone()
}

#[test]
fn test_next_to_top() {
    assert_eq!(next_to_top(&vec![("A", 1), ("B", 2), ("C", 3)]), ("B", 2));
}

fn top<'a, T: Clone>(stack: &'a [(&'a str, T)]) -> (&'a str, T) {
    assert!(stack.len() > 1);
    stack[stack.len() - 1].clone()
}

#[test]
fn test_top() {
    assert_eq!(top(&vec![("A", 1), ("B", 2), ("C", 3)]), ("C", 3));
}
