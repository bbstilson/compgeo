use crate::data::{Cone, PolarDirection, Pos2};

pub fn to_the_right(stack: &[Pos2], p2: Pos2) -> bool {
    let p0 = next_to_top(&stack);
    let p1 = top(&stack);
    let cone = Cone {
        a: p0.into(),
        b: p2.into(),
        origin: p1.into(),
    };
    match cone.angle_direction() {
        PolarDirection::Collinear | PolarDirection::CW => true,
        PolarDirection::CCW => false,
    }
}

fn next_to_top<T: Copy>(stack: &[T]) -> T {
    assert!(stack.len() > 1);
    stack[stack.len() - 2]
}

#[test]
fn test_next_to_top() {
    assert_eq!(next_to_top(&vec![1, 2, 3]), 2);
}

fn top<T: Copy>(stack: &[T]) -> T {
    assert!(stack.len() > 1);
    stack[stack.len() - 1]
}

#[test]
fn test_top() {
    assert_eq!(top(&vec![1, 2, 3]), 3);
}
